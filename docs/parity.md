# Funktionsparität und Nachweise

Jede Funktion des MVP steht hier mit einem prüfbaren Akzeptanzkriterium und einer Test-ID. Eine Zeile ist erst **erledigt**, wenn ein Test mit genau dieser ID existiert und in `cargo xtask ci` grün ist; der grüne Test ist der Nachweis, nicht die Behauptung.

## Regeln

- **Test-ID:** `BEREICH-NNN`, z.B. `UNITS-001`. Die ID steht im Namen oder im Doku-Kommentar des Tests, damit `grep -r UNITS-001` ihn findet.
- **Status:** `offen`, `in Arbeit`, `erledigt`. Eine Zeile wird nie ohne grünen Test auf `erledigt` gesetzt.
- **Fixture-geprüft** (Spalte „Mensch“): `ja` heißt, die zugrunde liegenden Fixtures hat ein Mensch gegen die ICAO-Karte geprüft. Agenten setzen diese Spalte nie selbst.
- Neue Funktion: zuerst Zeile anlegen, dann Test schreiben, dann implementieren.
- Tests auf dem iPad (UI, Golden-Screenshots, Testflüge) bekommen ebenfalls eine ID; der Nachweis ist dann ein Testprotokoll oder ein XCTest.

## M0 Fundament

| ID | Funktion | Akzeptanzkriterium | Status | Mensch |
| --- | --- | --- | --- | --- |
| INFRA-001 | Schichtenprüfung | `cargo xtask layers` schlägt fehl, wenn eine Crate eine nicht erlaubte interne Crate nutzt oder nicht in `LAYERS` steht | erledigt | – |
| INFRA-002 | Lizenztrennung | Bibliotheks-Crates (MIT/Apache-2.0) dürfen keine MPL-Crate nutzen; das `license`-Feld jeder Crate passt zu `LAYERS` | erledigt | – |
| INFRA-003 | CI-Gate | `cargo xtask ci` läuft in GitHub Actions bei jedem Push auf `main` und jedem Pull Request | erledigt | – |

## M1 Spike

| ID | Funktion | Akzeptanzkriterium | Status | Mensch |
| --- | --- | --- | --- | --- |
| SPIKE-001 | Overlay aus Rust | `frameAt` plus Overlay-Update p95 unter 8 ms bei 20 Hz auf dem iPad, Tick-Rate innerhalb 5 % | in Arbeit (Simulator erfüllt, iPad offen) | – |
| SPIKE-002 | Lokale PMTiles | MapLibre Native zeigt `spike.pmtiles` aus dem App-Bundle ohne Netz | in Arbeit (Simulator erfüllt, iPad offen) | – |

## M2 Einheiten und Geometrie

| ID | Funktion | Akzeptanzkriterium | Status | Mensch |
| --- | --- | --- | --- | --- |
| UNITS-001 | Höhe mit Bezug | AMSL, AGL und FL sind getrennte Typen; Addition verschiedener Bezüge kompiliert nicht | offen | – |
| UNITS-002 | Flugfläche umrechnen | FL ↔ AMSL nur mit explizitem QNH; FL065 bei QNH 1013,25 hPa ergibt 6500 ft AMSL | offen | – |
| UNITS-003 | Kurs mit Bezug | Rechtweisend und missweisend sind getrennte Typen; Umrechnung nur mit Missweisung | offen | – |
| UNITS-004 | Distanz und Geschwindigkeit | NM, km, kt, km/h mit verlustfreier Umrechnung innerhalb 1e-9 relativ | offen | – |
| GEO-001 | Großkreisdistanz und Peilung | Ergebnis stimmt mit GeographicLib auf 1 m bzw. 0,01° überein | offen | – |
| GEO-002 | Missweisung | WMM-Wert für Deutschland weicht höchstens 0,1° vom NOAA-Rechner ab | offen | – |
| GEO-003 | Punkt in Luftraum | Lateral und vertikal korrekt, inklusive Kreisbögen und Grenzen „GND“ und „FL“ | offen | – |
| GEO-004 | Schnitt Route–Luftraum | 20 geprüfte Fixture-Fälle liefern Eintritts- und Austrittspunkte auf 50 m genau | offen | nein |
| GEO-005 | Vertikalprofil entlang Track | Lufträume und Gelände entlang einer Route mit korrekten Unter- und Obergrenzen | offen | nein |

## M3 Datenpipeline Deutschland

| ID | Funktion | Akzeptanzkriterium | Status | Mensch |
| --- | --- | --- | --- | --- |
| NAV-001 | openAIP-Import | Lufträume, Flugplätze, Navaids und Hindernisse für DE ins gemeinsame Modell, mit Quelle und Gültigkeit | offen | – |
| NAV-002 | open-flightmaps-Import | Meldepunkte und Platzrunden ins gemeinsame Modell | offen | – |
| NAV-003 | Quellenkonflikte | Widersprüche zwischen Quellen landen im Protokoll und werden nicht stillschweigend aufgelöst | offen | – |
| NAV-004 | Datenprüfungen | Offene Polygone, unplausible Höhen und Duplikate stoppen die Veröffentlichung | offen | – |
| NAV-005 | Räumliche Abfrage | „Was ist hier“ über R*Tree liefert alle Lufträume an einem Punkt in unter 5 ms | offen | – |
| NAV-006 | Signiertes Paket | Manifest mit Version, Quellen, Datenstand und Prüfsummen; ungültige Signatur wird abgelehnt | offen | – |

## M4 Headless-Planer

| ID | Funktion | Akzeptanzkriterium | Status | Mensch |
| --- | --- | --- | --- | --- |
| CLI-001 | Route planen | `ofd` plant eine Route durch DE und listet alle durchflogenen Lufträume mit Grenzen | offen | nein |
| CLI-002 | Commands | Jede Aktion des CLI läuft über einen Command in `ofd-core`, nicht über eigene Logik | offen | – |

## M5 bis M6 Moving Map, Planen und Navlog

| ID | Funktion | Akzeptanzkriterium | Status | Mensch |
| --- | --- | --- | --- | --- |
| MAP-001 | Offline-Karte | Karte aus lokalen PMTiles ohne Netzverbindung | offen | – |
| MAP-002 | Track-up und North-up | Umschalten mit einem Tipp, Beschriftungen bleiben lesbar | offen | – |
| MAP-003 | Was ist hier | Tipp auf die Karte zeigt Lufträume mit Grenzen, Flugplatz, Distanz, Peilung, Zeit | offen | – |
| MAP-004 | Positionsverlust | Flugzeugsymbol verschwindet und Banner erscheint innerhalb von 2 s | offen | – |
| MAP-005 | Datenalter sichtbar | Datenstand und Quelle sind auf der Karte immer sichtbar | offen | – |
| FLT-001 | Navlog | Kurs, Zeit und Kraftstoff je Leg mit Wind, gegen Handrechnung auf 1° und 1 min | offen | – |
| FLT-002 | Direct-To und Nearest | Großer Button, Direct-To zum nächsten Flugplatz in höchstens zwei Tipps | offen | – |
| FLT-003 | Track-Aufzeichnung | Aufzeichnung im Hintergrund, GPX-Export, der sich in gängigen Viewern öffnet | offen | – |
| APT-001 | Flugplatzinfos | Pisten, Frequenzen, Höhe und Platzrunde, wo vorhanden | offen | – |

## M7 Warnungen und Profil

| ID | Funktion | Akzeptanzkriterium | Status | Mensch |
| --- | --- | --- | --- | --- |
| ALR-001 | Luftraum-Vorausschau | Zeit bis Eintritt aus Track und Groundspeed, auf 5 s genau gegenüber Simulation | offen | – |
| ALR-002 | Drei Stufen | Hinweis, Vorsicht, Warnung mit konfigurierbaren Schwellen; Warnung mit Ton | offen | – |
| ALR-003 | Entprellung | Kein Flackern zwischen Stufen bei Flug entlang einer Grenze | offen | – |
| ALR-004 | Kein Fehlalarm | Simulierte Flüge aus `fixtures/` lösen keine falsche Warnung aus | offen | nein |
| ALR-005 | Quittieren | Eine Warnung wird mit einem Tipp quittiert und kommt erst bei neuem Luftraum wieder | offen | – |

## M8 Wetter und Verkehr

| ID | Funktion | Akzeptanzkriterium | Status | Mensch |
| --- | --- | --- | --- | --- |
| WX-001 | METAR/TAF | Parser übersteht Fuzzing und liest die aviationweather.gov-Sammeldatei für DE ohne Fehler | offen | – |
| WX-002 | Flugkategorie | VFR, MVFR, IFR, LIFR nach FAA-Definition aus Sicht und Wolkenuntergrenze | offen | – |
| WX-003 | Alter der Meldung | Alter sichtbar, Meldungen älter als 90 min markiert | offen | – |
| TRF-001 | GDL90 | Ownship und Traffic aus aufgezeichneten Stratux-Mitschnitten korrekt dekodiert | offen | – |
| TRF-002 | FLARM-NMEA | PFLAA/PFLAU aus Mitschnitten korrekt, relative Höhe mit Vorzeichen | offen | – |
| TRF-003 | NoTrack | Ziele mit NoTrack-Kennung werden nicht angezeigt | offen | – |

## M9 Beta

| ID | Funktion | Akzeptanzkriterium | Status | Mensch |
| --- | --- | --- | --- | --- |
| APP-001 | Credits | Alle Datenquellen mit Attribution in einem Credits-Bildschirm | offen | – |
| APP-002 | Disclaimer | Beim ersten Start sichtbar, muss bestätigt werden | offen | – |
| APP-003 | Themes | Tag, Nacht und Farbsehschwäche-Theme gegen Golden-Screenshots | offen | – |
| APP-004 | Testflüge | 20 dokumentierte Testflüge ohne Absturz und ohne falsche Luftraumwarnung | offen | nein |
