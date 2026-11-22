# Status

Stand: 22.11.2026

Diese Datei ist der Einstieg für jede Session. Am Ende einer Session aktualisieren: erledigt, in Arbeit, nächster Schritt.

## M1 Spike (in Arbeit)

Abnahmekriterium: iPad zeigt eine lokale PMTiles-Karte mit Overlay aus Rust bei 10 Hz flüssig. Details und Messwerte in [spike-m1.md](spike-m1.md).

| Punkt | Stand |
| --- | --- |
| `ofd-ffi` mit UniFFI, simulierter Flug in `ofd_core::spike` | erledigt |
| `cargo xtask ios` (XCFramework, Swift Package `apps/ios/OFDCore`) | erledigt |
| `cargo xtask spike-tiles` (synthetische PMTiles-Testkarte) | erledigt |
| Spike-App `apps/ios` baut für Simulator und Gerät | erledigt |
| Messung im Simulator oder auf dem iPad | offen: auf dem Mac fehlt die iOS-Simulator-Laufzeit |
| macOS-Job in CI für den iOS-Build | offen |

## M0 Fundament (abgeschlossen bis auf Organisatorisches)

Abnahmekriterium: Repo, AGENTS.md, parity.md, `cargo xtask ci` grün in CI; Domains und GitHub-Organisation gesichert; openAIP-Freigabe angefragt.

| Punkt | Stand |
| --- | --- |
| GitHub-Organisation `openflightdeck` und Repository | erledigt |
| Lizenzdateien (MPL-2.0, MIT, Apache-2.0) und DCO | erledigt |
| AGENTS.md, CONTRIBUTING.md | erledigt |
| docs/STATUS.md, docs/parity.md | erledigt |
| Cargo-Workspace mit allen zehn Crates als Skelett | erledigt |
| `cargo xtask ci` (fmt, clippy, Tests, Schichten und Lizenzen) | erledigt, grün in CI |
| GitHub Actions: xtask ci, cargo-deny, DCO-Prüfung | erledigt, grün |
| Domains openflightdeck.com, .org, .app | offen (Rene) |
| openAIP: API-Schlüssel und schriftliche Freigabe | offen (Rene) |
| open flightmaps: Lizenztext | offen (Rene) |

## Nächste Schritte

1. Messung des Spikes im Simulator oder auf dem iPad, Ergebnis in spike-m1.md eintragen.
2. Branch-Schutz für `main` einrichten (CI und DCO als Pflicht-Checks, Review für `ofd-units`, `ofd-geo`, `ofd-alerts` per CODEOWNERS).
3. Parallel möglich: M2 beginnen mit `ofd-units` (Höhen, Kurse, Distanzen, Geschwindigkeiten als Typen).

## Offene Punkte

- `ofd-ffi` hat eigene Lints mit `unsafe_code = "deny"` und erlaubt unsafe nur für den von UniFFI erzeugten Code (begründet in `lib.rs`).
- Labels auf der Karte brauchen lokale Glyphen (Font-PBFs); der Spike zeichnet deshalb noch keine Texte.
- Testpiloten, Enroute-Team, NOTAM-Quelle, UL-Regeln und Hosting: siehe Implementierungsplan, „Offene Punkte“.
