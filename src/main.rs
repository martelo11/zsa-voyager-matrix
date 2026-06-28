use clap::Parser;
use kontroll::Kontroll;
use rand::Rng;
use std::collections::HashSet;
use tokio::time::{interval, sleep, Duration};

#[derive(Parser, Debug)]
#[command(name = "zsa-voyager-matrix")]
#[command(about = "Matrix rain animation for ZSA Voyager keyboard LEDs")]
struct Args {
    #[arg(short, long, default_value = "#69c11d")]
    color: String,

    #[arg(short, long, default_value = "20")]
    fps: u64,

    #[arg(short, long, default_value = "6")]
    drops: usize,
}

struct Drop {
    column: usize,
    head_row: f32,
    length: usize,
    speed: f32,
}

fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some((r, g, b))
}

fn pos_to_led(x: usize, y: usize) -> Option<usize> {
    let layout: [[usize; 12]; 5] = [
        [ 0,  1,  2,  3,  4,  5,  26, 27, 28, 29, 30, 31],
        [ 6,  7,  8,  9, 10, 11, 32, 33, 34, 35, 36, 37],
        [12, 13, 14, 15, 16, 17, 38, 39, 40, 41, 42, 43],
        [18, 19, 20, 21, 22, 23, 44, 45, 46, 47, 48, 49],
        [60, 60, 60, 60, 24, 25, 50, 51, 60, 60, 60, 60],
    ];
    let led = layout[y][x];
    if led < 52 { Some(led) } else { None }
}

fn create_drops(count: usize) -> Vec<Drop> {
    let mut rng = rand::thread_rng();
    (0..count)
        .map(|_| Drop {
            column: rng.gen_range(0..12),
            head_row: rng.gen_range(-4.0..0.0),
            length: rng.gen_range(2..=4),
            speed: rng.gen_range(0.15..0.45),
        })
        .collect()
}

fn update_drops(drops: &mut [Drop]) {
    let mut rng = rand::thread_rng();
    for drop in drops.iter_mut() {
        drop.head_row += drop.speed;
        if drop.head_row - drop.length as f32 > 4.0 {
            drop.column = rng.gen_range(0..12);
            drop.head_row = -(rng.gen_range(2.0..6.0));
            drop.length = rng.gen_range(2..=4);
            drop.speed = rng.gen_range(0.15..0.45);
        }
    }
}

fn get_lit_leds(drops: &[Drop]) -> HashSet<usize> {
    let mut lit = HashSet::new();
    for drop in drops {
        let head = drop.head_row.floor() as isize;
        for offset in 0..drop.length as isize {
            let row = head - offset;
            if row >= 0 && row < 5 {
                if let Some(led) = pos_to_led(drop.column, row as usize) {
                    lit.insert(led);
                }
            }
        }
    }
    lit
}

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("zsa-voyager-matrix: {}", e);
        std::process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let (r, g, b) = hex_to_rgb(&args.color).ok_or("Invalid hex color")?;

    eprintln!("zsa-voyager-matrix: connecting to Keymapp...");
    let api = Kontroll::new(None).await.map_err(|e| format!("Keymapp connection failed: {}", e))?;

    // Keymapp auto-connects; ignore "already connected" errors
    let _ = api.connect_any().await;
    eprintln!("zsa-voyager-matrix: flashing all LEDs to confirm...");

    // Startup flash: all LEDs on for 1 second so user immediately sees it works
    api.set_rgb_all(r, g, b, 1).await.map_err(|e| format!("set_rgb_all failed: {}", e))?;
    sleep(Duration::from_secs(1)).await;
    api.set_rgb_all(0, 0, 0, 1).await.ok();
    eprintln!("zsa-voyager-matrix: starting animation (color: #{}, fps: {}, drops: {})", args.color, args.fps, args.drops);

    let mut drops = create_drops(args.drops);
    let mut prev_lit: HashSet<usize> = HashSet::new();

    let period = Duration::from_millis(1000 / args.fps.max(1));
    let mut ticker = interval(period);

    let mut sigterm = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())?;

    loop {
        tokio::select! {
            _ = ticker.tick() => {
                update_drops(&mut drops);
                let lit = get_lit_leds(&drops);

                for &led in prev_lit.difference(&lit) {
                    let _ = api.set_rgb_led(led, 0, 0, 0, 1).await;
                }

                for &led in lit.difference(&prev_lit) {
                    let _ = api.set_rgb_led(led, r, g, b, 1).await;
                }

                prev_lit = lit;
            }

            _ = tokio::signal::ctrl_c() => break,
            _ = sigterm.recv() => break,
        }
    }

    eprintln!("zsa-voyager-matrix: restoring LEDs...");
    let _ = api.restore_rgb_leds().await;

    Ok(())
}
