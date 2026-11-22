# AGENTS.md

Regeln für Coding-Agenten und Menschen, die an OpenFlightDeck arbeiten. OpenFlightDeck ist ein EFB: Ein falscher Luftraum, eine verwechselte Höhe oder ein Absturz im Flug kann Menschen gefährden. Lieber langsamer und nachweislich richtig.

## Jede Session beginnt so

1. `docs/STATUS.md` lesen: aktueller Meilenstein, was in Arbeit ist, offene Punkte.
2. Die betroffene Zeile in `docs/parity.md` suchen. Gibt es keine, zuerst eine anlegen (Funktion, Akzeptanzkriterium, Test-ID).
3. Nur bei Bedarf tiefer lesen: `docs/OpenFlightDeck - Implementierungsplan Phase 0 und 1.md` für Architektur und Meilensteine, `docs/OpenFlightDeck - Konzept und Research.md` für Hintergründe, `docs/decisions/` für getroffene Entscheidungen.

## Arbeitsweise

- **Eine Aufgabe pro Branch.** Branch von `main`, kleine Pull Requests, ein Thema pro PR.
- **Parallel arbeiten nur in getrennten git-Worktrees** mit eigenem `target/`-Verzeichnis (`CARGO_TARGET_DIR`).
- **`cargo xtask ci` muss grün sein**, bevor ein PR geöffnet oder als fertig gemeldet wird. Das ist das einzige Gate; es prüft Formatierung, Clippy mit `-D warnings`, alle Tests und die Schichten.
- **Ein grüner Test ist der Nachweis, nicht die Behauptung.** Eine Zeile in `parity.md` wird erst auf „erledigt“ gesetzt, wenn der Test mit ihrer Test-ID existiert und grün ist.
- **Jede Fehlerbehebung bekommt einen Regressionstest**, der ohne die Behebung rot ist.
- **Am Ende der Session `docs/STATUS.md` aktualisieren:** was erledigt ist, was offen ist, was der nächste Schritt wäre.
- Commits mit `git commit -s` (DCO, siehe `CONTRIBUTING.md`).

## Code-Regeln im Rust-Kern

- **Kein Panic im Produktivcode.** Kein `unwrap`, `expect`, `panic!`, `todo!`, `unimplemented!`; Fehler als `Result` mit eigenem Fehlertyp. In Tests ist `unwrap` erlaubt. Clippy erzwingt das über die Workspace-Lints.
- **Kein `unsafe`.** Einzige künftige Ausnahme ist `ofd-ffi`, und nur mit begründetem Kommentar und Review.
- **Einheiten und Bezugssysteme sind Typen.** Höhen nie als nackte Zahl, sondern mit Bezug (AMSL, AGL, FL); Kurse mit Bezug (rechtweisend, missweisend). Umrechnungen nur über `ofd-units`.
- **Alles ist ein Command.** Jede Nutzeraktion ist ein Befehl in `ofd-core` mit ID, Parametern und Test. SwiftUI, `ofd-cli` und ein späterer MCP-Kanal rufen dieselben Commands auf; Logik gehört nie in die Oberfläche.
- **Schichten einhalten.** Die erlaubten Abhängigkeiten stehen in `LAYERS` in `xtask/src/main.rs`. Eine neue Crate wird dort eingetragen, sonst schlägt `cargo xtask ci` fehl. Bibliotheken (MIT/Apache-2.0) hängen nie von MPL-Crates ab.
- **Neue Abhängigkeiten sparsam** und nur mit Lizenzen aus `deny.toml`. Keine GPL, LGPL oder AGPL.
- Doku-Kommentare auf Deutsch, Bezeichner auf Englisch.

## Fliegerisch kritische Bereiche

- **Review-Pflicht:** Änderungen an `ofd-units`, `ofd-geo` und `ofd-alerts` gehen nie ohne Durchsicht durch einen Maintainer nach `main`. Agenten öffnen dafür einen PR und mergen nicht selbst.
- **Fixtures sind Wahrheit nur nach menschlicher Prüfung.** Agenten dürfen Fixtures vorschlagen, aber nicht als geprüft markieren. Prüfung gegen die ICAO-Karte mit Angabe von Stand und Quelle.
- **Quellen nie stillschweigend zusammenführen.** Widersprechen sich openAIP und open flightmaps, wird der Konflikt protokolliert, nicht aufgelöst.
- **Datenalter und Quelle sind immer sichtbar.** Kein Code-Pfad zeigt Daten ohne Herkunft an.

## Nicht tun

- Nicht direkt auf `main` pushen, keine Force-Pushes auf `main`.
- Keine Funktionen außerhalb des MVP-Scopes (siehe Implementierungsplan, „Bewusst nicht im MVP“) ohne Rücksprache.
- Keine Daten aus Quellen einbauen, deren Lizenz nicht geklärt ist.
- Keine Tests abschwächen oder löschen, um CI grün zu bekommen.
