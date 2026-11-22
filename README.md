# OpenFlightDeck

Freies Open-Source-EFB für VFR in Deutschland, zuerst für das iPad. Rust-Kern mit UniFFI-Brücke, SwiftUI-Oberfläche, MapLibre Native und ausschließlich offenen Daten (openAIP, open flightmaps, OSM, Copernicus DEM, DWD, NOAA).

> **Status:** Phase 0, Meilenstein M0 (Fundament). Noch nichts davon ist zum Fliegen geeignet. Aktueller Stand in [docs/STATUS.md](docs/STATUS.md).

## Aufbau

```
crates/     Rust-Kern (ofd-units, ofd-geo, ofd-navdata, ofd-wx, ofd-traffic,
            ofd-flight, ofd-alerts, ofd-core, ofd-ffi, ofd-cli)
xtask/      Build- und Prüfaufgaben (cargo xtask ci)
docs/       Konzept, Implementierungsplan, STATUS.md, parity.md, decisions/
website/    Landingpage (statisch, ohne Build-Schritt)
```

Später kommen `apps/ios/`, `pipeline/`, `design/` und `fixtures/` dazu (siehe Implementierungsplan).

## Entwickeln

Voraussetzung ist eine Rust-Toolchain über rustup; `rust-toolchain.toml` wählt den stabilen Kanal mit rustfmt und clippy.

```bash
cargo xtask ci
```

Das ist das einzige Gate, lokal und in GitHub Actions: Formatierung, Clippy mit Warnungen als Fehler, alle Tests und die Prüfung der Schichten und Lizenzen. Weitere Aufgaben zeigt `cargo xtask help`.

Regeln für Menschen und Coding-Agenten stehen in [AGENTS.md](AGENTS.md), Beiträge laufen nach [CONTRIBUTING.md](CONTRIBUTING.md) mit DCO-Sign-off.

## Lizenz

| Teil | Lizenz |
| --- | --- |
| Bibliotheken: `ofd-units`, `ofd-geo`, `ofd-navdata`, `ofd-wx`, `ofd-traffic`, `ofd-flight`, `ofd-alerts`, `xtask` | MIT oder Apache-2.0, nach Wahl ([LICENSE-MIT](LICENSE-MIT), [LICENSE-APACHE](LICENSE-APACHE)) |
| App und alles andere: `ofd-core`, `ofd-ffi`, `ofd-cli`, iOS-App, Pipeline, Website, Dokumentation | MPL-2.0 ([LICENSE-MPL-2.0](LICENSE-MPL-2.0)) |

Maßgeblich ist das Feld `license` in der jeweiligen `Cargo.toml`; `cargo xtask ci` prüft, dass es zur Tabelle passt. Die Schriften in `website/fonts/` stehen unter der SIL Open Font License 1.1 (Lizenztexte dort). Kartendaten und Navigationsdaten unterliegen den Lizenzen ihrer Quellen.

## Haftungsausschluss

OpenFlightDeck ist kein zugelassenes Navigationsmittel. Es ersetzt weder die amtliche Karte noch die Flugvorbereitung nach den geltenden Vorschriften. Die Verantwortung liegt immer beim verantwortlichen Luftfahrzeugführer.
