//! Spike M1: simulierter Flug, der dynamische Overlays erzeugt.
//!
//! Dient nur dazu, die Brücke zur App bei 10 bis 20 Hz zu messen. Die Werte
//! sind bewusst nackte `f64` ohne Bezugssystem-Typen, weil `ofd-units` erst in
//! M2 entsteht. Nichts hier ist fliegerisch geprüft; das Modul wird ersetzt,
//! sobald `ofd-flight` und `ofd-alerts` existieren.

use std::collections::VecDeque;
use std::f64::consts::PI;
use std::fmt::Write as _;

const EARTH_RADIUS_M: f64 = 6_371_008.8;
const KT_TO_MPS: f64 = 1852.0 / 3600.0;

/// Eigene Position und Zustand in einem Frame.
#[derive(Debug, Clone, PartialEq)]
pub struct Ownship {
    /// Breite in Grad (WGS84).
    pub lat: f64,
    /// Länge in Grad (WGS84).
    pub lon: f64,
    /// Rechtweisender Kurs über Grund in Grad, 0 bis 360.
    pub track_deg: f64,
    /// Geschwindigkeit über Grund in Knoten.
    pub ground_speed_kt: f64,
}

/// Alles, was die Oberfläche pro Frame zeichnen muss.
#[derive(Debug, Clone, PartialEq)]
pub struct OverlayFrame {
    /// Laufende Nummer des Frames.
    pub sequence: u64,
    /// Eigene Position.
    pub ownship: Ownship,
    /// Ob die eigene Position in der Warnzone liegt.
    pub alert_active: bool,
    /// Distanz zum Rand der Warnzone in Metern, negativ innerhalb.
    pub alert_distance_m: f64,
    /// GeoJSON-FeatureCollection mit Track (LineString) und Warnzone (Polygon).
    pub geojson: String,
}

/// Simulierter Kreisflug um einen Mittelpunkt mit einer festen Warnzone.
#[derive(Debug, Clone)]
pub struct SimulatedFlight {
    center_lat: f64,
    center_lon: f64,
    radius_m: f64,
    ground_speed_kt: f64,
    zone_lat: f64,
    zone_lon: f64,
    zone_radius_m: f64,
    trail: VecDeque<(f64, f64)>,
    trail_len: usize,
    sequence: u64,
}

impl SimulatedFlight {
    /// Kreisflug mit `radius_m` um den Mittelpunkt bei `ground_speed_kt`.
    /// Die Warnzone liegt auf der Kreisbahn im Norden, damit sie regelmäßig
    /// durchflogen wird.
    pub fn new(center_lat: f64, center_lon: f64, radius_m: f64, ground_speed_kt: f64) -> Self {
        let (zone_lat, zone_lon) = destination(center_lat, center_lon, 0.0, radius_m);
        Self {
            center_lat,
            center_lon,
            radius_m: radius_m.max(1.0),
            ground_speed_kt,
            zone_lat,
            zone_lon,
            zone_radius_m: radius_m * 0.3,
            trail: VecDeque::new(),
            trail_len: 300,
            sequence: 0,
        }
    }

    /// Berechnet den Frame zum Zeitpunkt `t_s` (Sekunden seit Start).
    pub fn frame_at(&mut self, t_s: f64) -> OverlayFrame {
        let speed_mps = self.ground_speed_kt * KT_TO_MPS;
        // Winkel auf dem Kreis, im Uhrzeigersinn ab Norden.
        let angle_deg = (speed_mps * t_s / self.radius_m)
            .to_degrees()
            .rem_euclid(360.0);
        let (lat, lon) = destination(self.center_lat, self.center_lon, angle_deg, self.radius_m);
        let track_deg = (angle_deg + 90.0).rem_euclid(360.0);

        self.trail.push_back((lat, lon));
        while self.trail.len() > self.trail_len {
            self.trail.pop_front();
        }
        self.sequence += 1;

        let alert_distance_m =
            distance_m(lat, lon, self.zone_lat, self.zone_lon) - self.zone_radius_m;
        let alert_active = alert_distance_m <= 0.0;

        OverlayFrame {
            sequence: self.sequence,
            ownship: Ownship {
                lat,
                lon,
                track_deg,
                ground_speed_kt: self.ground_speed_kt,
            },
            alert_active,
            alert_distance_m,
            geojson: self.geojson(alert_active),
        }
    }

    fn geojson(&self, alert_active: bool) -> String {
        let mut out = String::with_capacity(16 * 1024);
        out.push_str(r#"{"type":"FeatureCollection","features":["#);
        out.push_str(r#"{"type":"Feature","properties":{"kind":"track"},"geometry":{"type":"LineString","coordinates":["#);
        push_coords(&mut out, self.trail.iter().copied());
        out.push_str("]}},");
        let _ = write!(
            out,
            r#"{{"type":"Feature","properties":{{"kind":"alert-zone","active":{alert_active}}},"geometry":{{"type":"Polygon","coordinates":[["#
        );
        let ring = (0..=64).map(|i| {
            let bearing = f64::from(i) * 360.0 / 64.0;
            destination(self.zone_lat, self.zone_lon, bearing, self.zone_radius_m)
        });
        push_coords(&mut out, ring);
        out.push_str("]]}}]}");
        out
    }
}

fn push_coords(out: &mut String, coords: impl Iterator<Item = (f64, f64)>) {
    for (i, (lat, lon)) in coords.enumerate() {
        if i > 0 {
            out.push(',');
        }
        // GeoJSON erwartet [Länge, Breite]; 6 Nachkommastellen sind ~0,1 m.
        let _ = write!(out, "[{lon:.6},{lat:.6}]");
    }
}

/// Zielpunkt auf der Kugel von (`lat`, `lon`) aus mit Peilung und Distanz.
fn destination(lat: f64, lon: f64, bearing_deg: f64, distance_m: f64) -> (f64, f64) {
    let delta = distance_m / EARTH_RADIUS_M;
    let theta = bearing_deg.to_radians();
    let phi1 = lat.to_radians();
    let lambda1 = lon.to_radians();
    let phi2 = (phi1.sin() * delta.cos() + phi1.cos() * delta.sin() * theta.cos()).asin();
    let lambda2 = lambda1
        + (theta.sin() * delta.sin() * phi1.cos()).atan2(delta.cos() - phi1.sin() * phi2.sin());
    let lon2 = (lambda2 + 3.0 * PI).rem_euclid(2.0 * PI) - PI;
    (phi2.to_degrees(), lon2.to_degrees())
}

/// Großkreisdistanz nach Haversine in Metern.
fn distance_m(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let dphi = (lat2 - lat1).to_radians();
    let dlambda = (lon2 - lon1).to_radians();
    let a = (dphi / 2.0).sin().powi(2)
        + lat1.to_radians().cos() * lat2.to_radians().cos() * (dlambda / 2.0).sin().powi(2);
    2.0 * EARTH_RADIUS_M * a.sqrt().asin()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Schönhagen (EDAZ), grob.
    const LAT: f64 = 52.2036;
    const LON: f64 = 13.1586;

    #[test]
    fn stays_on_circle() {
        let mut flight = SimulatedFlight::new(LAT, LON, 5000.0, 100.0);
        for i in 0..200 {
            let f = flight.frame_at(f64::from(i) * 0.1);
            let r = distance_m(LAT, LON, f.ownship.lat, f.ownship.lon);
            assert!((r - 5000.0).abs() < 1.0, "Radius {r}");
        }
    }

    #[test]
    fn starts_inside_zone_and_leaves_it() {
        let mut flight = SimulatedFlight::new(LAT, LON, 5000.0, 100.0);
        assert!(flight.frame_at(0.0).alert_active);
        // Nach einem Viertelkreis (~7854 m bei 51,4 m/s, ~153 s) ist sie draußen.
        assert!(!flight.frame_at(153.0).alert_active);
    }

    #[test]
    fn trail_is_bounded_and_geojson_is_valid() {
        let mut flight = SimulatedFlight::new(LAT, LON, 5000.0, 100.0);
        let mut last = None;
        for i in 0..1000 {
            last = Some(flight.frame_at(f64::from(i) * 0.1));
        }
        let frame = last.unwrap();
        assert_eq!(frame.sequence, 1000);
        let value: serde_json::Value = serde_json::from_str(&frame.geojson).unwrap();
        let track = &value["features"][0]["geometry"]["coordinates"];
        assert_eq!(track.as_array().unwrap().len(), 300);
        let ring = &value["features"][1]["geometry"]["coordinates"][0];
        assert_eq!(ring.as_array().unwrap().len(), 65);
    }
}
