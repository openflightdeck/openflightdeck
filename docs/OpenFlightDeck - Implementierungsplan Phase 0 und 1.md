# OpenFlightDeck – Implementierungsplan Phase 0 und 1

Stand: 22.11.2026

## Ziel und Scope

Ziel von Phase 0 und 1 ist eine iPad-App für VFR in Deutschland, die beim Fliegen besser zu bedienen ist als jede freie Alternative und dabei verlässlich korrekte Luftraumdaten zeigt. Alles, was nicht dazu beiträgt, kommt später.

### Im MVP

- Moving Map offline mit eigener ICAO-artiger Vektorkarte, Track-up und North-up
- Luftraumwarnung mit Vorausschau, Frequenz und Vertikalprofil entlang des Tracks
- „Was ist hier“ per Tipp auf die Karte: Luftraum, Flugplatz, Distanz, Peilung, Zeit
- Routenplanung am Boden (Rubber-Band), Navlog mit Kurs, Zeit und Kraftstoff
- Flugplatzinfos: Pisten, Frequenzen, Höhe, Platzrunde wo vorhanden
- METAR/TAF mit Flugkategorie auf der Karte, DWD-Radar als Overlay
- Verkehr über GDL90 und FLARM-NMEA (Stratux, SkyEcho, FLARM-Geräte)
- Direct-To und Nearest mit großem Button, Track-Aufzeichnung mit GPX-Export
- Tag-, Nacht- und Farbsehschwäche-Theme, Datenalter und Quelle immer sichtbar

### Bewusst nicht im MVP

- IFR, Plates, Airways, Flugplanaufgabe
- NOTAMs (keine freie Quelle für Deutschland geklärt)
- Android, Web-Planer, Accounts, Cloud-Sync
- Masse und Schwerpunkt, Performance, Logbuch
- Synthetic Vision

### Definition of Done für Phase 1

- 20 dokumentierte Testflüge mit gültigem PIC ohne Absturz und ohne falsche Luftraumwarnung
- TestFlight-Beta mit mindestens 50 Piloten
- Über 99,5 % absturzfreie Sessions
- Jede Luftraumdarstellung in Deutschland stichprobenartig gegen die ICAO-Karte geprüft

## Architekturentscheidungen

Rust besitzt alles, was fliegerisch richtig sein muss; die iPad-App zeichnet nur und liest Sensoren. So läuft dieselbe Logik später unverändert auf Android, im Web und in Tests.

| Entscheidung | Wahl | Begründung |
| --- | --- | --- |
| Kernlogik | Rust-Workspace | Luftraumgeometrie, Warnungen, Route, Navlog, Datenzugriff an einer Stelle, testbar ohne Gerät |
| Brücke zur App | UniFFI, Ferrostar-Muster | erzeugt Swift-Bindings automatisch, später Kotlin für Android |
| Oberfläche | SwiftUI, iPad zuerst | Split View, VoiceOver, Hintergrundortung, beste UX |
| Karte | MapLibre Native iOS (Metal) | produktionsreif, Vektorkarten, Labels mit Kollisionserkennung, freie Styles |
| Kartenformat | PMTiles, offline auf dem Gerät | MapLibre Native liest PMTiles nativ: pmtiles:// wird seit MapLibre Native iOS v6.10.0 (Januar 2025) unterstützt; die URL muss vollständig angegeben sein |
| Navdaten auf dem Gerät | SQLite mit R*Tree, über rusqlite | schnelle räumliche Abfragen („Was ist hier“, Warnungen), ein File pro Datenpaket |
| Datenquellen MVP | openAIP, open flightmaps, OSM, Copernicus DEM, NOAA, DWD | alle frei nutzbar; openAIP-Freigabe schriftlich bestätigen lassen |
| Datenaufbereitung | eigener Server-Job, nicht auf dem Gerät | App lädt fertige, signierte Pakete; keine Abhängigkeit von Drittanbieter-APIs im Flug |
| Lizenz | MPL-2.0 für die App, MIT/Apache-2.0 für Bibliotheken | App-Store-tauglich, Bibliotheken für andere Projekte nutzbar |

Zwei Dinge legen wir jetzt schon fest, weil sie später teuer zu ändern sind:

- **Alles ist ein Command:** Jede Nutzeraktion ist ein Befehl im Rust-Kern mit ID, Parametern und Test. SwiftUI, CLI und ein späterer MCP-Kanal rufen dieselben Befehle auf. Das macht die App für Coding-Agenten steuerbar und testbar.
- **Einheiten und Bezugssysteme explizit:** Höhen immer mit Bezug (AMSL, AGL, FL), Winkel immer mit Bezug (rechtweisend oder missweisend), als eigene Typen im Rust-Kern. Verwechslungen hier sind die gefährlichsten Fehler eines EFB.

## Repository, Tooling und agentischer Workflow

Ein Monorepo unter github.com/openflightdeck enthält Kern, App, Datenpipeline und Dokumentation, damit Agenten und Menschen immer den ganzen Zusammenhang sehen.

### Struktur

```
openflightdeck/
  crates/
    ofd-units      Typen für Höhen, Winkel, Distanzen mit Bezugssystem
    ofd-geo        Geodäsie, Luftraumgeometrie, Schnitte mit Route und Profil
    ofd-navdata    Import openAIP/open flightmaps, SQLite mit R*Tree, Abfragen
    ofd-wx         METAR/TAF-Parser, Flugkategorien, Radar-Metadaten
    ofd-traffic    GDL90, FLARM-NMEA, Geräteerkennung
    ofd-flight     Route, Navlog, Kraftstoff, Flugzustand, Track-Aufzeichnung
    ofd-alerts     Luftraum-Vorausschau, Priorisierung, Entprellung
    ofd-core       App-Zustand, Commands, Effekte
    ofd-ffi        UniFFI-Bindings für Swift
    ofd-cli        Headless-Planer, Fixture-Läufer, später MCP-Kanal
  pipeline/        Server-Jobs: Datenpakete, Vektorkacheln, Style-JSON
  apps/ios/        SwiftUI-App mit MapLibre
  design/          Tokens, Symbolik-Spezifikation, Komponentenkatalog
  fixtures/        geprüfte Testdaten: Lufträume, Routen, GDL90-Mitschnitte
  docs/            parity.md, decisions/, STATUS.md
  xtask/           Build- und Prüfaufgaben
  AGENTS.md
```

### Werkzeuge

- **Rust:** stable Toolchain, `cargo xtask ci` als einziges Gate (fmt, clippy mit Warnungen als Fehler, Tests, Layer-Prüfung)
- **iOS:** Xcode, Swift Package für die generierten Bindings, XCFramework aus `ofd-ffi`
- **CI:** GitHub Actions, Linux für den Kern, macOS-Runner für den iOS-Build
- **Sicherheit:** cargo-deny für Lizenzen und Abhängigkeiten, signierte Releases, SBOM

### Arbeiten mit Claude Code

- **AGENTS.md** legt die Regeln fest: Session startet mit `docs/STATUS.md`, eine Aufgabe pro Branch, kein Panic im Produktivcode, jede Fehlerbehebung bekommt einen Regressionstest, `cargo xtask ci` muss grün sein.
- **docs/parity.md** listet jede Funktion mit Akzeptanzkriterium und Test-ID. Agenten arbeiten Zeile für Zeile ab; ein grüner Test ist der Nachweis, nicht die Behauptung.
- **Agenten im Kern, Mensch bei der Fliegerei:** Agenten bauen vor allem in den Rust-Crates, wo Tests schnell und eindeutig sind. Fixtures zu Lufträumen, Höhen und Missweisung prüfst du selbst gegen die ICAO-Karte, bevor sie als Wahrheit gelten.
- **Parallele Agenten** arbeiten in getrennten git-Worktrees mit eigenem Build-Verzeichnis.
- **Review-Regel:** Änderungen an `ofd-geo`, `ofd-alerts` und `ofd-units` gehen nie ohne deine Durchsicht in `main`.

## Datenpipeline

Die Pipeline läuft auf einem kleinen Server und erzeugt pro Region versionierte, signierte Pakete; die App lädt sie und funktioniert danach komplett offline. Das ist der erste Baustein, weil ohne korrekte Daten keine Karte und keine Warnung entsteht.

### Ablauf

1. **Abholen:** openAIP (Flugplätze, Lufträume, Navaids, Hindernisse) und open flightmaps (Meldepunkte, Platzrunden, Naturschutzgebiete) per API bzw. Export, dazu OSM-Basisdaten für Deutschland und Copernicus DEM GLO-30.
2. **Normalisieren:** In Rust (`ofd-navdata`) in ein gemeinsames Modell überführen: Luftraumklasse, Unter- und Obergrenze mit Bezug, Geometrie, Quelle, Gültigkeit. Konflikte zwischen Quellen werden protokolliert, nicht stillschweigend gelöst.
3. **Prüfen:** Automatische Checks (geschlossene Polygone, plausible Höhen, keine doppelten Objekte) plus Vergleich gegen geprüfte Fixtures. Ein fehlgeschlagener Check stoppt die Veröffentlichung.
4. **Bauen:** Navdaten als SQLite mit R*Tree-Index; Kartenkacheln als PMTiles (Basiskarte, Geländeschummerung, Luftfahrtlayer getrennt); Style-JSON aus der Symbolik-Spezifikation generiert.
5. **Ausliefern:** Paket mit Manifest (Version, Quellen, Datenstand, Prüfsummen, Signatur) auf einem statischen Host; die App prüft die Signatur vor der Nutzung.
6. **Aktualisieren:** Wöchentlicher Lauf plus Lauf zum AIRAC-Stichtag (alle 28 Tage); die App zeigt das Datenalter dauerhaft an und warnt bei veralteten Paketen.

### Wetter

METAR, TAF und SIGMET kommen aus den Sammeldateien von aviationweather.gov, die der Server minütlich spiegelt; so bleibt die App unter dem Limit der NOAA-API. DWD-Radar wird als Kachelsatz bereitgestellt. Wetter ist online-only und klar als ergänzend gekennzeichnet.

### Kosten und Betrieb

Für das MVP reicht ein kleiner VPS plus statischer Speicher (z.B. ein S3-kompatibler Bucket). Grobe Größenordnung: niedriger zweistelliger Euro-Betrag pro Monat (Schätzung). Alle Attributionen (openAIP, open flightmaps, OSM, Copernicus/DLR/Airbus, DWD, NOAA) erscheinen in einem Credits-Bildschirm.

## Rust-Kern

Der Kern wird von unten nach oben gebaut: erst Einheiten und Geometrie, dann Daten, dann Logik, zuletzt die Brücke zur App. Jede Schicht ist ohne iPad vollständig testbar.

| Reihenfolge | Crate | Liefert | Kernabhängigkeiten |
| --- | --- | --- | --- |
| 1 | ofd-units | Höhe (AMSL/AGL/FL), Kurs (rechtweisend/missweisend), Distanz, Geschwindigkeit als eigene Typen | uom oder eigene Newtypes |
| 2 | ofd-geo | Großkreis, Peilung, Luftraum-Polygone, Schnitt Route–Luftraum, Profil entlang Track, Missweisung | geo, geographiclib-rs |
| 3 | ofd-navdata | Import, SQLite-Schema mit R*Tree, Abfragen: „Was ist hier“, Flugplätze im Umkreis, Lufträume entlang Route | rusqlite, serde |
| 4 | ofd-wx | METAR/TAF-Parsing, Flugkategorie (VFR, MVFR, IFR, LIFR), Alter der Meldung | metar-taf-parser |
| 5 | ofd-traffic | GDL90 über UDP, FLARM-NMEA über TCP, Ziel-Liste mit relativer Höhe, NoTrack-Regel | gdl90 (erweitert), eigener NMEA-Parser |
| 6 | ofd-flight | Route, Navlog (Kurs, Zeit, Kraftstoff, Wind), Flugzustand, Track-Aufzeichnung, GPX | ofd-geo |
| 7 | ofd-alerts | Luftraum-Vorausschau (Zeit bis Eintritt), Hinweis/Vorsicht/Warnung, Entprellung, Quittieren | ofd-geo, ofd-flight |
| 8 | ofd-core | App-Zustand, Commands, Ereignisse an die Oberfläche | alle obigen |
| 9 | ofd-ffi | UniFFI-Schnittstelle, XCFramework für iOS | uniffi |

### Schnittstelle zur App

Die App gibt Positionen (10 Hz), Geräteereignisse und Nutzeraktionen in den Kern. Der Kern liefert einen kompakten Anzeigezustand zurück: eigene Position, aktive Warnung, nächster Wegpunkt mit ETE, Verkehrsziele, Profil-Daten. Kartengeometrie fließt nicht durch die Brücke; MapLibre liest die PMTiles direkt, der Kern liefert nur dynamische Overlays (Route, Track, Verkehr, Warnzonen) als kleines GeoJSON.

### Erster Prototyp-Spike

Vor dem eigentlichen Bau prüft ein Spike die zwei offenen technischen Fragen: ob UniFFI-Aufrufe bei 10 bis 20 Hz mit Overlay-Updates auf dem iPad flüssig bleiben, und ob MapLibre lokale PMTiles-Dateien aus dem App-Container zuverlässig lädt.

## iPad-App

Die App hat zwei klar getrennte Modi: Planen am Boden mit allen Eingaben, Fliegen mit großen Zielen und nur Tippen und Wischen. Gleiche Aktion, gleicher Ort, in beiden Modi.

### Bildschirme

| Bildschirm | Inhalt | Modus |
| --- | --- | --- |
| Karte (Fly) | vollflächige Karte, Datenleiste oben (GS, TRK, ALT, nächster Wegpunkt, ETE), einklappbares Vertikalprofil unten, großer Direct-To/Nearest-Button | Flug |
| Warnbanner | Hinweis, Vorsicht (Amber), Warnung (Rot mit Ton), Quittieren mit einem Tipp | Flug |
| Was ist hier | Sheet nach Tipp auf die Karte: Lufträume mit Grenzen, Flugplatz, Distanz, Peilung, Zeit | beide |
| Planen | Route per Rubber-Band und Suche, Navlog-Tabelle, Kraftstoff, Wind | Boden |
| Flugplatz | Pisten, Frequenzen, Höhe, METAR/TAF, Platzrunde | beide |
| Verkehr | Geräteverbindung, Status, Liste der Ziele | beide |
| Daten | Pakete laden, Datenstand, Quellen, Credits | Boden |
| Einstellungen | Themes, Einheiten, Warnschwellen, Datenleiste | Boden |

### Kartenintegration

MapLibre Native als SwiftUI-View; Basiskarte, Gelände und Luftfahrtlayer kommen aus lokalen PMTiles, dynamische Overlays aus dem Rust-Kern. Track-up rotiert die Karte, die Beschriftungen bleiben lesbar. Die Symbolik folgt ICAO Annex 4 und der DFS-Konvention; statische Sperrgebiete werden entsättigt gezeichnet, damit kräftiges Rot den aktiven Warnungen vorbehalten bleibt.

### Ortung und Sensoren

- CoreLocation mit Hintergrundmodus für Navigation, damit Warnungen auch bei gesperrtem Bildschirm laufen
- Positionspriorität: externer Empfänger (GDL90-Ownship), internes GNSS, manueller Fallback
- Local-Network-Freigabe für UDP und TCP zu Stratux und FLARM
- Verlust der Position wird sofort sichtbar: Flugzeugsymbol verschwindet, Banner erscheint

### Designsystem

Tokens in physischen Millimetern statt Punkten: mindestens 15 mm Touch-Fläche, 22 mm für Primäraktionen im Flug, Feedback unter 100 ms, 200 ms Entprellung nach einer Berührung. Drei Themes: Tag, Nacht (dunkel, kein Rotfilter), Farbsehschwäche (Muster statt Farbe). Die Tokens liegen in `design/` und werden für Swift und den Kartenstyle generiert.

## Meilensteine

Zehn Meilensteine führen vom leeren Repository zur TestFlight-Beta; jeder endet mit einem prüfbaren Kriterium, nicht mit einem Datum. M0 bis M4 bilden Phase 0, M5 bis M9 Phase 1.

| # | Meilenstein | Abnahmekriterium |
| --- | --- | --- |
| M0 | Fundament | Repo, AGENTS.md, parity.md, `cargo xtask ci` grün in CI; Domains und GitHub-Organisation gesichert; openAIP-Freigabe angefragt |
| M1 | Spike | iPad zeigt eine lokale PMTiles-Karte mit Overlay aus Rust bei 10 Hz flüssig |
| M2 | Einheiten und Geometrie | `ofd-units` und `ofd-geo` mit Tests; Schnitt Route–Luftraum stimmt für 20 geprüfte Fixture-Fälle |
| M3 | Datenpipeline Deutschland | Paket mit Navdaten und Karte für DE, Prüfungen grün, signiert ausgeliefert |
| M4 | Headless-Planer | `ofd-cli` plant eine Route durch Deutschland und listet alle durchflogenen Lufträume mit korrekten Grenzen |
| M5 | Moving Map | iPad-App mit Karte, eigener Position, Track-up, „Was ist hier“, Tag- und Nachttheme |
| M6 | Planen und Navlog | Route am Boden bauen, Navlog mit Wind und Kraftstoff, Direct-To, Nearest |
| M7 | Warnungen und Profil | Luftraum-Vorausschau mit drei Stufen und Ton, Vertikalprofil; Simulatorflüge ohne Fehlalarm |
| M8 | Wetter und Verkehr | METAR/TAF auf der Karte, DWD-Radar, Verkehr von Stratux und FLARM mit relativer Höhe |
| M9 | Beta | Track-Aufzeichnung, Credits, Disclaimer; 20 Testflüge, TestFlight mit 50 Piloten |

Die Reihenfolge ist bewusst so gewählt, dass die fliegerisch heikelsten Teile (Geometrie, Daten, Warnungen) früh und ohne Oberfläche getestet werden. Die App kommt erst dazu, wenn der Kern nachweislich richtig rechnet.

## Tests und Qualität

Qualität wird auf vier Ebenen geprüft, von schnell und automatisch bis langsam und echt; erst wenn alle vier grün sind, geht eine Version an Tester.

1. **Unit- und Property-Tests im Kern:** Geometrie, Einheiten, Navlog und Warnlogik; Fuzzing für alle Parser (openAIP, METAR, GDL90, NMEA). Läuft bei jedem Commit.
2. **Fixture-Tests:** von dir geprüfte Wahrheiten, z.B. „Route EDAZ–EDDC schneidet diese Lufträume mit diesen Grenzen“. Jede gefundene Abweichung im Flug wird zu einem neuen Fixture.
3. **Simulator- und Replay-Tests:** aufgezeichnete GDL90- und NMEA-Ströme sowie simulierte Flüge laufen headless durch `ofd-cli`; geprüft werden Warnzeitpunkte und Fehlalarme.
4. **Golden-Screenshots und Testflüge:** Kartendarstellung je Theme gegen Referenzbilder, Bedientests auf dem iPad, dann echte Testflüge mit Protokoll.

Zusätzlich gilt ab dem ersten Release: Absturzberichte ohne personenbezogene Daten, sichtbarer Datenstand in der App und ein klarer Disclaimer beim ersten Start.

## Erste Schritte und offene Punkte

Die ersten Schritte sind organisatorisch und in wenigen Abenden erledigt; danach startet M0 im Repository.

### Diese Woche

- [ ] openflightdeck.com, .org und .app registrieren
- [ ] GitHub-Organisation „openflightdeck“ anlegen, Lizenzdateien (MPL-2.0, MIT/Apache-2.0) und DCO einrichten
- [ ] openAIP: API-Schlüssel beantragen und Nutzung für eine freie App schriftlich bestätigen lassen
- [ ] open flightmaps: Lizenztext anfordern
- [ ] Apple Developer Program, iPad mit Mobilfunk-Modul (für GNSS) bereitlegen
- [ ] Verkehrsempfänger organisieren, z.B. Stratux Europe Edition
- [ ] Claude-Projekt mit Konzept und diesem Plan anlegen, AGENTS.md als erste Aufgabe

### Offene Punkte

- **Testpiloten:** Wer fliegt die 20 Testflüge, solange deine eigene Lizenz noch im Aufbau ist? Verein oder Bekannte früh ansprechen.
- **Enroute-Team:** Kontakt aufnehmen, Abgrenzung und mögliche Zusammenarbeit bei Datenaufbereitung klären.
- **NOTAMs:** Quelle für Phase 2 klären (FAA NMS-Zugang anfragen, DFS ansprechen).
- **UL-Regeln:** Kartenmitführungspflicht und Besonderheiten für UL mit DULV oder DAeC klären.
- **Hosting:** Server und Bucket auswählen, sobald M3 ansteht.
