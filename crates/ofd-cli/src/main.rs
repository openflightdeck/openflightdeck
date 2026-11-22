//! `ofd`: Headless-Planer und Fixture-Läufer. Später auch der MCP-Kanal.

// Das CLI gibt seine Ergebnisse bewusst auf stdout aus.
#![allow(clippy::print_stdout)]

fn main() {
    println!("ofd {}", ofd_core::VERSION);
}
