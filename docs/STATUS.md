# Status

Stand: 22.11.2026

Diese Datei ist der Einstieg für jede Session. Am Ende einer Session aktualisieren: erledigt, in Arbeit, nächster Schritt.

## Aktueller Meilenstein: M0 Fundament

Abnahmekriterium: Repo, AGENTS.md, parity.md, `cargo xtask ci` grün in CI; Domains und GitHub-Organisation gesichert; openAIP-Freigabe angefragt.

| Punkt | Stand |
| --- | --- |
| GitHub-Organisation `openflightdeck` und Repository | erledigt |
| Lizenzdateien (MPL-2.0, MIT, Apache-2.0) und DCO | erledigt |
| AGENTS.md, CONTRIBUTING.md | erledigt |
| docs/STATUS.md, docs/parity.md | erledigt |
| Cargo-Workspace mit allen zehn Crates als Skelett | erledigt |
| `cargo xtask ci` (fmt, clippy, Tests, Schichten und Lizenzen) | lokal grün |
| GitHub Actions: xtask ci, cargo-deny, DCO-Prüfung | angelegt, erster Lauf nach dem Push |
| Domains openflightdeck.com, .org, .app | offen (Rene) |
| openAIP: API-Schlüssel und schriftliche Freigabe | offen (Rene) |
| open flightmaps: Lizenztext | offen (Rene) |

## Nächste Schritte

1. Ersten CI-Lauf auf GitHub prüfen, Branch-Schutz für `main` einrichten (CI und DCO als Pflicht-Checks, Review für `ofd-units`, `ofd-geo`, `ofd-alerts` per CODEOWNERS).
2. M1 Spike: iPad zeigt lokale PMTiles-Karte mit Overlay aus Rust bei 10 Hz (`ofd-ffi` mit UniFFI, `apps/ios/`).
3. Parallel möglich: M2 beginnen mit `ofd-units` (Höhen, Kurse, Distanzen, Geschwindigkeiten als Typen).

## Offene Punkte

- Schriften in `website/fonts/` brauchen die OFL-Lizenztexte mit den Copyright-Zeilen der jeweiligen Schrift.
- `ofd-ffi` verbietet aktuell `unsafe` wie alle Crates; UniFFI erzeugt `unsafe`-Code, dafür braucht die Crate im Spike eine begründete Ausnahme.
- Testpiloten, Enroute-Team, NOTAM-Quelle, UL-Regeln und Hosting: siehe Implementierungsplan, „Offene Punkte“.
