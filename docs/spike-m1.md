# Spike M1: UniFFI bei 10 bis 20 Hz und lokale PMTiles

Stand: 22.11.2026

Abnahmekriterium laut Implementierungsplan: Das iPad zeigt eine lokale PMTiles-Karte mit Overlay aus Rust bei 10 Hz flüssig.

## Offene Fragen

1. Bleiben UniFFI-Aufrufe mit Overlay-Updates bei 10 bis 20 Hz auf dem iPad flüssig?
2. Lädt MapLibre Native lokale PMTiles-Dateien aus dem App-Bundle zuverlässig?

## Aufbau

- **Kern:** `ofd_core::spike::SimulatedFlight` fliegt einen Kreis mit 5 km Radius um Schönhagen (EDAZ) bei 100 kt und liefert pro Aufruf eigene Position, Kurs, Status einer Warnzone und ein GeoJSON mit den letzten 300 Trackpunkten und dem Polygon der Warnzone (rund 8 KB). Bewusst ohne `ofd-units`, das Modul fliegt raus, sobald M2 bis M7 stehen.
- **Brücke:** `ofd-ffi` exportiert das Objekt per UniFFI-Proc-Macros. `cargo xtask ios` baut die statische Bibliothek für Gerät und Simulator, erzeugt die Swift-Bindings und packt beides als XCFramework in das Swift Package `apps/ios/OFDCore`.
- **App:** `apps/ios` (XcodeGen), SwiftUI mit MapLibre Native iOS 6.31.0. Die Karte kommt aus `spike.pmtiles` im Bundle über `pmtiles://file:///…`. Ein `CADisplayLink` ruft den Kern mit 10, 20 oder 60 Hz auf, setzt das GeoJSON als `MLNShapeSource`, zentriert die Karte und dreht sie bei Track-up.
- **Messung in der App** (eingeblendet und alle 2 s im Log unter `org.openflightdeck.spike`): tatsächliche Tick-Rate, Jitter (p95), Dauer von `frameAt` inklusive FFI, Dauer des Overlay-Updates in MapLibre und gerenderte Kartenframes pro Sekunde.

## Ergebnisse

| Messung | Umgebung | Ergebnis |
| --- | --- | --- |
| Kernkosten pro Frame ohne FFI (`ofd spike-bench`, Release) | Mac, Apple Silicon | Ø 0,027 ms, p99 0,05 ms, max 0,12 ms bei 8,1 KB GeoJSON |
| App baut für Simulator und Gerät (arm64) | Xcode 27.0, iOS SDK 27.0 | ja |
| PMTiles lädt im Bundle | iPad / Simulator | offen |
| Tick-Rate, Jitter, FFI- und Update-Dauer bei 10, 20, 60 Hz | iPad / Simulator | offen |
| Kartenframes pro Sekunde mit Track-up | iPad | offen |

Der Kern ist mit rund 0,03 ms pro Frame um mehr als drei Größenordnungen unter dem Budget von 100 ms (10 Hz) bzw. 50 ms (20 Hz). Die eigentliche Frage ist daher, was das Kopieren des GeoJSON über die Brücke und das Neusetzen der `MLNShapeSource` in MapLibre kostet. Das lässt sich nur auf dem Gerät oder im Simulator messen.

## Bewertung

Wird ergänzt, sobald die Messung auf dem iPad gelaufen ist. Als flüssig gilt: Tick-Rate innerhalb 5 % des Sollwerts, Jitter p95 unter 10 ms, `frameAt` plus Overlay-Update p95 unter 8 ms (halbes 60-Hz-Frame), Karte ohne sichtbares Ruckeln bei Track-up.
