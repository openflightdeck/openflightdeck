//! UniFFI-Schnittstelle des Kerns für Swift (später Kotlin).
//!
//! Kartengeometrie fließt nicht durch diese Brücke, nur Anzeigezustand und
//! kleine GeoJSON-Overlays. Für den Spike M1 gibt es nur den simulierten Flug.

// UniFFI erzeugt in den Export-Makros unsafe-Code für die C-Schnittstelle.
// Eigener unsafe-Code ist hier weiterhin nicht erlaubt.
#![allow(unsafe_code)]

use std::sync::Mutex;

use ofd_core::spike;

uniffi::setup_scaffolding!();

/// Version des Kerns.
#[uniffi::export]
pub fn core_version() -> String {
    ofd_core::VERSION.to_owned()
}

/// Eigene Position in einem Frame.
#[derive(Debug, Clone, uniffi::Record)]
pub struct Ownship {
    /// Breite in Grad (WGS84).
    pub lat: f64,
    /// Länge in Grad (WGS84).
    pub lon: f64,
    /// Rechtweisender Kurs über Grund in Grad.
    pub track_deg: f64,
    /// Geschwindigkeit über Grund in Knoten.
    pub ground_speed_kt: f64,
}

/// Anzeigezustand für einen Frame.
#[derive(Debug, Clone, uniffi::Record)]
pub struct OverlayFrame {
    /// Laufende Nummer.
    pub sequence: u64,
    /// Eigene Position.
    pub ownship: Ownship,
    /// Ob die Warnzone aktiv ist.
    pub alert_active: bool,
    /// Distanz zum Rand der Warnzone in Metern, negativ innerhalb.
    pub alert_distance_m: f64,
    /// GeoJSON-FeatureCollection mit Track und Warnzone.
    pub geojson: String,
}

impl From<spike::OverlayFrame> for OverlayFrame {
    fn from(f: spike::OverlayFrame) -> Self {
        Self {
            sequence: f.sequence,
            ownship: Ownship {
                lat: f.ownship.lat,
                lon: f.ownship.lon,
                track_deg: f.ownship.track_deg,
                ground_speed_kt: f.ownship.ground_speed_kt,
            },
            alert_active: f.alert_active,
            alert_distance_m: f.alert_distance_m,
            geojson: f.geojson,
        }
    }
}

/// Simulierter Kreisflug für den Spike.
#[derive(Debug, uniffi::Object)]
pub struct SimulatedFlight {
    inner: Mutex<spike::SimulatedFlight>,
}

#[uniffi::export]
impl SimulatedFlight {
    /// Kreisflug mit `radius_m` um den Mittelpunkt bei `ground_speed_kt`.
    #[uniffi::constructor]
    pub fn new(center_lat: f64, center_lon: f64, radius_m: f64, ground_speed_kt: f64) -> Self {
        Self {
            inner: Mutex::new(spike::SimulatedFlight::new(
                center_lat,
                center_lon,
                radius_m,
                ground_speed_kt,
            )),
        }
    }

    /// Frame zum Zeitpunkt `t_s` in Sekunden seit Start.
    pub fn frame_at(&self, t_s: f64) -> OverlayFrame {
        // Ein vergifteter Mutex kann hier nur nach einem Panic entstehen; der
        // Zustand ist trotzdem konsistent, also weiterrechnen statt abbrechen.
        let mut flight = self
            .inner
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        flight.frame_at(t_s).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frames_count_up() {
        let flight = SimulatedFlight::new(52.2, 13.16, 5000.0, 100.0);
        assert_eq!(flight.frame_at(0.0).sequence, 1);
        assert_eq!(flight.frame_at(0.1).sequence, 2);
        assert!(!core_version().is_empty());
    }
}
