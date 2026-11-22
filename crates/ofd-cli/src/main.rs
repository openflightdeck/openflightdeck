//! `ofd`: Headless-Planer und Fixture-Läufer. Später auch der MCP-Kanal.

// Das CLI gibt seine Ergebnisse bewusst auf stdout aus.
#![allow(clippy::print_stdout)]

use std::time::Instant;

use ofd_core::spike::SimulatedFlight;

fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("spike-bench") => spike_bench(),
        _ => println!(
            "ofd {}\n\nBefehle:\n  spike-bench   Kosten eines Overlay-Frames im Kern messen (Spike M1)",
            ofd_core::VERSION
        ),
    }
}

/// Misst, wie lange der Kern für einen Overlay-Frame braucht, ohne FFI.
fn spike_bench() {
    const FRAMES: u32 = 20_000;
    let mut flight = SimulatedFlight::new(52.2036, 13.1586, 5000.0, 100.0);
    // Aufwärmen, bis der Track voll ist.
    for i in 0..400 {
        flight.frame_at(f64::from(i) * 0.1);
    }
    let mut samples = Vec::with_capacity(FRAMES as usize);
    let mut bytes = 0;
    for i in 0..FRAMES {
        let start = Instant::now();
        let frame = flight.frame_at(40.0 + f64::from(i) * 0.1);
        samples.push(start.elapsed().as_secs_f64() * 1000.0);
        bytes = frame.geojson.len();
    }
    samples.sort_by(f64::total_cmp);
    let mean = samples.iter().sum::<f64>() / f64::from(FRAMES);
    let pick = |p: f64| {
        let idx = ((samples.len().saturating_sub(1)) as f64 * p) as usize;
        samples.get(idx).copied().unwrap_or(0.0)
    };
    println!(
        "{FRAMES} Frames: Ø {mean:.4} ms, p95 {:.4} ms, p99 {:.4} ms, max {:.4} ms, GeoJSON {:.1} KB",
        pick(0.95),
        pick(0.99),
        pick(1.0),
        bytes as f64 / 1024.0
    );
}
