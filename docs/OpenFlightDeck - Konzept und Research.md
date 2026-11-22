# OpenFlightDeck – Konzept & Research

Nov 21, 2026 · @Rene Bartkowiak


> **Entscheidungen seit der Recherche (Stand 22.11.2026)**
> - Name: **OpenFlightDeck** (kurz OFD); openflightdeck.com, .org, .app und github.com/openflightdeck waren frei.
> - Keine Förderung (auch kein Prototype Fund); Nebenprojekt, nicht-kommerziell.
> - Positionierung: Gewinnen über UX, Übersichtlichkeit und Features; Ziel langfristig weltweit und ForeFlight-Niveau.
> - Start mit VFR Deutschland, **iPad zuerst**; Architektur: **Rust-Kern + SwiftUI + MapLibre Native** (Ferrostar-Muster).
> - Details zur Umsetzung: siehe „OpenFlightDeck – Implementierungsplan Phase 0 und 1“.

## Zusammenfassung und Empfehlung

Baue das EFB als nicht-kommerzielles Open-Source-Projekt mit Rust-Kern und nativen Oberflächen, starte mit VFR Deutschland auf dem iPad und plane IFR erst ab Phase 3. Der Engpass ist nicht der Code, sondern die Daten: DFS-Karten und -Daten sind urheberrechtlich geschützt, und für NOTAMs und IFR-Verfahren in Europa gibt es keine offene Quelle. Ein VFR-MVP lässt sich dagegen heute vollständig aus offenen Daten bauen.

**Zeitkritisch: Die Bewerbungsfrist des Prototype Fund (Klasse 03) endet am 30.11.2026, also in neun Tagen.** Gefördert werden Einzelpersonen mit bis zu 47.500 € für sechs Monate, Teams mit bis zu 95.000 € ([Prototype Fund](https://prototypefund.de/en/apply)). Voraussetzung sind Wohnsitz und Steuerpflicht in Deutschland, ein Umzug nach Zypern würde die Förderfähigkeit beenden.

Die Kernempfehlungen in einem Satz pro Bereich:

- **Markt:** ForeFlight ist Apple-only, gehört seit November 2025 Thoma Bravo und kostet 130 bis 390 $ pro Jahr. SkyDemon dominiert VFR in Deutschland, Enroute ist die einzige freie Alternative.
- **Positionierung:** Gegen Enroute gewinnst du nur über deutlich bessere UX, iPad-Politur und einen glaubwürdigen IFR-Pfad. Sprich vor dem Start mit dem Enroute-Team.
- **Daten (MVP):** openAIP, open flightmaps, OpenStreetMap, Copernicus DEM, NOAA aviationweather, DWD Open Data und ECMWF Open Data. Eine eigene ICAO-artige Vektorkarte statt DFS-Rasterkarten.
- **Regulatorik:** Für Privatpiloten unter Part-NCO braucht die App keine Zulassung. Die neue Produkthaftung ab 9.12.2026 nimmt FOSS nur außerhalb einer Geschäftstätigkeit aus.
- **Organisation:** Ein gemeinnütziger Träger (e.V. oder Stiftung) hält Marke und Spenden. Bezahldienste, falls je gewünscht, kommen in eine getrennte Gesellschaft.
- **Architektur:** Rust-Kern über UniFFI (Ferrostar-Muster), SwiftUI- und Compose-Shells, MapLibre Native für die Karte, Web-Planer über WASM.
- **Lizenz:** MPL-2.0 für die App, MIT/Apache-2.0 für wiederverwendbare Crates, DCO statt CLA.
- **Name:** „OpenFlightDeck“ (kurz OFD) ist gewählt; .com, .org, .app und der GitHub-Name waren am 22.11.2026 frei.

Ehrliche Einordnung: ForeFlight hat allein in Europa über 100 Mitarbeiter und lieferte 2026 acht Major-Releases ([ForeFlight Blog](https://foreflight.com/blog/foreflight-partners-with-iaopa-europe-to-offer-discounts-to-general-aviation-pilots), [App Store](https://apps.apple.com/us/app/-/id333252638)). Feature-Parität ist für ein kleines Team kein realistisches Ziel. Realistisch ist ein exzellentes, freies VFR-EFB für Europa mit sauberem Pfad zu IFR-Referenzfunktionen.

## Was ich von dir brauche

Die wichtigste Entscheidung fällt bis zum 30.11.2026: ob du dich beim Prototype Fund bewirbst, obwohl du einen Umzug nach Zypern planst. Alle anderen Punkte können in den nächsten vier Wochen folgen.

### Entscheidungen

- [ ] **Prototype Fund bis 30.11.2026: ja oder nein.** Bedingungen: Wohnsitz in Deutschland, selbstständig oder freiberuflich mit Steuerpflicht hier; Firmen und Vereine sind ausgeschlossen ([Prototype Fund](https://prototypefund.de/en/apply)). Zu klären: Ist ein Antrag als Einzelperson neben deiner GmbH zulässig? Bleibst du während des Förderzeitraums in Deutschland? Die Second Stage muss bereits im Erstantrag angefragt werden.
- [ ] **Kommerziell oder nicht-kommerziell.** Empfehlung: nicht-kommerziell, siehe Regulatorik und Haftung.
- [ ] **Rechtsform.** Gemeinnütziger e.V. (Gemeinnützigkeit für Piloten-Software ist ungeklärt, Steuerberater fragen) oder vorerst ein Fiscal Host wie Open Collective Europe.
- [ ] **Lizenz.** MPL-2.0 für die App, MIT/Apache-2.0 für Crates, DCO.
- [ ] **Name.** „OpenFlightDeck“ ist gewählt: Domains (.com, .org, .app) und GitHub-Organisation sichern, Markenrecherche bei EUIPO und DPMA.
- [ ] **Zielgerät.** iPad zuerst (empfohlen) oder Android zuerst.
- [ ] **Neubau oder Kooperation.** Gespräch mit Enroute (Akaflieg Freiburg) und mit dem Maintainer der Rust-Bibliothek libefb, bevor du Code schreibst.
- [ ] **MVP-Zielgruppe.** Motorflug VFR plus UL (empfohlen); Segelflug später.

### Hardware und Konten

- [ ] iPad mit Mobilfunk-Modul, weil nur diese Modelle GNSS haben (allgemein bekannt, nicht verifiziert).
- [ ] Android-Tablet mit GNSS für Phase 2.
- [ ] Mac mit Apple Silicon und Xcode.
- [ ] Apple Developer Program und Google Play Console, idealerweise auf den späteren Träger angemeldet.
- [ ] Verkehrsempfänger mit GDL90- und FLARM-NMEA-Ausgang, z.B. Stratux Europe Edition (Fertiggerät 439 bis 489 € bei [Dross:Aviation](https://static.dross.net/aviation/stratux/) oder Eigenbau nach [b3nn0/stratux](https://github.com/b3nn0/stratux)).
- [ ] Flugsimulator mit Positions- bzw. GDL90-Ausgabe für Bodentests (konkrete Simulator-Unterstützung nicht recherchiert).
- [ ] Kniebrett- oder Yoke-Halterung für Tests unter realen Bedingungen.

### Zugänge und Kontakte

- [ ] **openAIP:** API-Zugang beantragen und schriftlich bestätigen lassen, dass das geplante Modell (Spenden, App Stores) zur Lizenz passt ([openAIP](https://www.openaip.net)).
- [ ] **open flightmaps (OFMA, Salzburg):** vollständigen Lizenztext der „OFMA General Users License“ anfordern ([OFM FAQ](https://openflightmaps.org/faq/)).
- [ ] **FAA NMS-API (NOTAMs):** Zugang per E-Mail an 7-AWA-NAIMES@faa.gov beantragen und klären, ob internationale NOTAMs enthalten sind ([FAA NMS FAQ](https://www.faa.gov/about/initiatives/notam/faqs)).
- [ ] **autorouter:** API-Bedingungen und Preise für Drittanbieter-Apps klären ([autorouter API](https://www.autorouter.aero/wiki/api/)).
- [ ] **DFS:** Anfrage an data@dfs.de zu AIXM-Datensätzen und Lizenz für nicht-kommerzielle Open-Source-Weitergabe ([DFS Geodaten](https://dfs.de/homepage/en/services/geo-data/)).
- [ ] **FLARM:** formlose Info an info@flarm.com, dass ihr den Data Port implementiert ([FLARM FTD-012](https://www.flarm.com/wp-content/uploads/2024/04/FTD-012-Data-Port-Interface-Control-Document-ICD.pdf)).
- [ ] **Community:** Enroute-Team, libefb-Maintainer, die [AeroRust-Community](https://www.Github.com/AeroRust).
- [ ] **Beratung:** Steuerberater (Gemeinnützigkeit), Anwalt (Produkthaftung, Marke).

### Informationen

- [ ] Realistisches Zeitbudget pro Woche und ein Geldbudget für Hardware, Server und Markenanmeldung.
- [ ] Dein Ziel: Hobby, Community-Projekt, Reputation oder später ein Geschäft. Das entscheidet über Rechtsform und Lizenz.
- [ ] Fliegst du aktuell? Testflüge brauchen einen PIC mit gültiger Lizenz. Solange deine CPL abgelaufen und die PPL im Aufbau ist, brauchst du Testpiloten aus Verein oder Community.
- [ ] Zeitplan für Zypern, weil er Prototype Fund und Sitz des Trägers bestimmt.
- [ ] Gibt es Mitstreiter? Ein Team bis vier Personen kann als GbR beim Prototype Fund antreten; Mitglieder dürfen in anderen EU-Ländern wohnen, die GbR muss in Deutschland sitzen.

## Markt: ForeFlight und Wettbewerber

Der Markt hat eine klare Lücke: ein freies, plattformübergreifendes, hochwertig gestaltetes EFB mit Pfad zu IFR. ForeFlight ist teuer und Apple-only, SkyDemon ist VFR-only, und die einzige freie App (Enroute) ist ein Vereinsprojekt mit kleiner Basis.

### Wettbewerber im Überblick

| App | Plattformen | Preis (Liste, Nov 2026) | VFR/IFR | Daten Europa | Bewertung |
| --- | --- | --- | --- | --- | --- |
| [ForeFlight](https://foreflight.com/pricing/) | iPad, iPhone, Watch, Vision, Web | Starter 130 $, Essential 260 $, Premium 390 $ pro Jahr; DFS-Karten extra | VFR + IFR | Jeppesen, DFS als Add-on | 4,4 (3,3k, US) |
| [SkyDemon](https://apps.apple.com/de/app/skydemon/id497184081?l=en) | iOS, Android, Windows | 229,99 € (iOS), Webshop 139 $ | nur VFR | eigene Vektorkarten (Lizenzbasis unbekannt) | 4,9 (6k, DE) |
| [Garmin Pilot](https://apps.apple.com/de/app/garmin-pilot/id340917615?l=en) | iOS (Android nicht verifiziert) | Europe 119,99 € / 199,99 €, Worldwide 699,99 € | VFR + IFR | Jeppesen NavData | 4,4 (326) |
| [EasyVFR 4](https://apps.apple.com/de/app/easyvfr-4/id1286986764?l=en) | iOS | Live 69,99 €, Pro 99,99 € pro Jahr | VFR | PocketFMS, DFS „endorsing“ | 4,3 (25) |
| [Air Navigation Pro](https://apps.apple.com/DE/app/id301046057) | iOS | 214,99 € / 269,99 € (Deutschland) | VFR, IFR-Karten | DFS-IAPs ca. 69,99 € | 4,3 (1.222) |
| [AirMate](https://apps.apple.com/de/app/airmate/id998181102?l=en) | iOS | Basis frei, Pro 24,99 €, DFS-Karten 59,99 bis 69,99 € | VFR | AIP 200 Länder (Pro) | 4,5 (394) |
| [Enroute](https://akaflieg-freiburg.github.io/enroute/) | iOS, Android, Linux | kostenlos, ohne Werbung | VFR | openAIP, open flightmaps | 5,0 (6, US) |

### ForeFlight: stark, aber angreifbar

ForeFlight wurde am 3.11.2025 als „Jeppesen ForeFlight“ eigenständig, nach dem Verkauf an Thoma Bravo für 10,55 Mrd. $ ([Thoma Bravo](https://www.thomabravo.com/press-releases/jeppesen-foreflight-launches-as-a-standalone-company-to-redefine-the-future-of-aviation-software)). Im Dezember 2025 sank der SAFE/NAFI-Rabatt von 33 auf 15 Prozent, was Nutzer mit Private Equity verbanden ([AvBrief](https://avbrief.com/foreflight-reduces-subscription-discounts/)). Die Roadmap setzt auf KI: ClearNOTAMs, ATIS-Transkription, ChatGPT-Connector ([ForeFlight Releases](https://foreflight.com/releases)).

Ein deutscher Pilot braucht für georeferenzierte Plates und Synthetic Vision mindestens Essential (260 $) plus DFS-Add-on. Für reine VFR-Piloten ist das teuer, genau dort sitzt SkyDemon. Wiederkehrende Kritik betrifft wachsende Komplexität und inkonsistentes Tap-Verhalten ([Pilots of America](https://pilotsofamerica.com/community/threads/why-are-they-messing-up-foreflight.126416/unread?new=1)).

### Open-Source-Landschaft: Partner statt Gegner

Enroute (GPL-3.0, C++/Qt, rund 4.900 Commits) löst bereits openAIP- und open-flightmaps-Import, MBTiles-Erzeugung, FLARM/OGN/GDL90-Verkehr und NOTAM-Proxy ([GitHub Enroute](https://github.com/Akaflieg-Freiburg/enroute)). In Rust existiert libefb (Crate `efb` 0.8.0, 13.11.2026) mit ARINC-424-, AIXM- und OpenAir-Parsing, noch „early development“ ([docs.rs efb](https://docs.rs/crate/efb/latest)). OpenEFB ist ein frühes SwiftUI-iPad-EFB unter MPL-2.0 mit MapLibre ([GitHub openefb](https://github.com/quartermint/openefb)).

Weitere Referenzen sind Avare/AvareX (US, FAA-Daten), XCSoar (GPL, Segelflug) und Stratux (BSD-3, Empfänger) ([Avare](https://github.com/apps4av/avare), [XCSoar](https://github.com/XCSoar/XCSoar), [Stratux](https://github.com/stratux/stratux)). Muster aller erfolgreichen OSS-Luftfahrt-Apps: nicht-kommerziell, vereins- oder freiwilligengetragen, eng auf VFR fokussiert. Ein OSS-EFB mit IFR wurde nicht gefunden.

### Marktgröße Deutschland

Der DAeC verwaltete Ende 2024 17.055 UL-Lizenzen, 840 davon neu im Jahr 2024 ([DAeC Jahresbericht 2024](https://www.daec.de/media/files/Dateien/Luftsportgeraete_Buero/Statistik/Jahresbericht_LSG-B_2024.pdf)). Das LBA zählte 2018 rund 22.000 lizenzierte Piloten und Flugpersonal ([Statista](https://de.statista.com/statistik/daten/studie/183097/umfrage/anzahl-der-piloten-in-deutschland)). Der DULV hat 6.136 Mitglieder, IAOPA Europe über 19.000 ([Lobbyregister](https://www.lobbyregister.bundestag.de/media/94/2c/809209/Lobbyregister-Registereintraege-Detailansicht-R005469-2026-07-08_16-46-59.pdf), [ForeFlight Blog](https://foreflight.com/blog/foreflight-partners-with-iaopa-europe-to-offer-discounts-to-general-aviation-pilots)).

Die adressierbare Gruppe liegt damit im Bereich zehntausender aktiver Piloten (Inferenz). Eine freie App ist schon mit wenigen tausend aktiven Nutzern relevant. Lücke: Es gibt keine Marktanteilsstudie; die SkyDemon-Dominanz ist aus Bewertungszahlen abgeleitet.

## Funktionsumfang

Das MVP muss die SkyDemon-Kernfunktionen für VFR abdecken und sonst nichts. IFR-Funktionen sind eine Datenfrage und kommen erst, wenn Lizenzen gesichert sind.

### Funktionen nach Phase

| Funktion | Phase | Vorbild / Begründung |
| --- | --- | --- |
| Moving Map mit ICAO-artiger Vektorkarte, Track-up/North-up, offline | 1 | Alle Wettbewerber |
| Luftraumwarnung mit Vorausschau, Frequenz und „wen rufen“ | 1 | [SkyDemon In-flight](https://www.SkyDemon.aero/inflight/) |
| Vertikalprofil („Virtual Radar“) entlang des Tracks | 1 | SkyDemon, Enroute 3.0.5 |
| Tap auf Karte „Was ist hier“ mit Distanz, Peilung, Zeit | 1 | SkyDemon |
| Routenplanung (Rubber-Band am Boden), Navlog mit Zeit/Kraftstoff | 1 | ForeFlight, SkyDemon |
| METAR/TAF, Flugkategorien auf der Karte, DWD-Radar | 1 | NOAA, DWD Open Data |
| Verkehr über GDL90 (UDP 4000) und FLARM-NMEA (TCP 2000) | 1 | [Enroute Manual](https://akaflieg-freiburg.github.io/enrouteManual/07-referenceTechnology/01-trafficData.html) |
| Direct-To / Nearest mit großem Button, Gleitring | 1 | SkyDemon |
| Track-Aufzeichnung, GPX-Export | 1 | ForeFlight Debrief |
| Datenaktualität und Quelle sichtbar (AIRAC, Datum) | 1 | PIC-Pflichten, Haftung |
| Android, VFR Europa, Länderdaten | 2 | Plattformlücke von ForeFlight |
| Grafische NOTAMs, Aktivierung von ED-R/TRA (AUP/UUP) | 2 | [EasyVFR Live](https://apps.apple.com/de/app/easyvfr-4/id1286986764?l=en) |
| Mass & Balance, Performance-Profile (inkl. UL 600 kg) | 2 | ForeFlight, UL-Bedarf |
| Winde/Temperaturen in der Höhe, Wolkenprognose | 2 | ECMWF, ICON |
| Checklisten, Mehrstreckenplanung, Logbuch | 2 | SkyDemon 4.2 |
| VFR-Flugplanaufgabe | 2 | SkyDemon, EasyVFR |
| Web-Planer | 2 | ForeFlight Web |
| IFR-Enroute-Karte, Airways | 3 | Lizenzierte Daten nötig |
| SID/STAR/Anflugverfahren, Plates | 3 | Lizenzierte Daten nötig |
| IFR-Routing und Filing über autorouter | 3 | [autorouter API](https://www.autorouter.aero/wiki/api/) |
| USA-Datenregion (FAA, gemeinfrei) | 4 | [FAA AeroNav FAQ](https://www.faa.gov/air_traffic/flight_info/aeronav/faq) |
| Synthetic Vision (benötigt AHRS) | 4 | ForeFlight Essential |

### Was das MVP bewusst nicht verspricht

Over-the-air-FLARM von SkyEcho 2 oder Sentry ist tabu. Die Dekodierung braucht eine kostenpflichtige, geschlossene FLARM-Lizenzbibliothek, die Enroute als unvereinbar mit FOSS ablehnt ([Enroute Manual](https://akaflieg-freiburg.github.io/enrouteManual/07-referenceTechnology/01-trafficData.html)). ADS-B-Verkehr dieser Geräte über GDL90 funktioniert trotzdem.

GAFOR, Low-Level-SWC und DWD-Vereisungsprognosen liegen hinter dem kostenpflichtigen pc\_met ([DWD FlugWetter](https://apps.apple.com/de/app/dwd-flugwetter/id633263810?l=en)). Sie fehlen im MVP, solange es keine Vereinbarung mit dem DWD gibt. Wetter und NOTAMs gelten in der App als ergänzend, nicht als offizielles Briefing.

### Differenzierung mit geringem Aufwand

Drei Funktionen heben das Projekt ab und sind billig. Erstens eine „Take 2“-Pufferanzeige (2 NM lateral, 200 ft vertikal) und ein Routensimulator, beide von der UK CAA empfohlen ([UK CAA](https://infringements.caa.co.uk/tutorial/using-a-vfr-moving-map/)). Zweitens On-Device-Klartext für NOTAMs als Gegenstück zu ForeFlights ClearNOTAMs.

Drittens Datenschutz: keine Registrierung, kein Tracking, wie bei Enroute. Für UL-Piloten zählen zusätzlich UL-Plätze aus Community-Daten und W&B-Profile für die 600-kg-Klasse; der DAeC listet Ende 2024 bereits 22 zugelassene 600-kg-Typen ([DAeC 2024](https://www.daec.de/media/files/Dateien/Luftsportgeraete_Buero/Statistik/Jahresbericht_LSG-B_2024.pdf)).

## Datenquellen und Lizenzen

Das MVP kommt komplett mit offenen Daten aus, DFS-Daten gibt es nur mit schriftlicher Lizenz. NOTAMs und IFR-Verfahren sind die zwei härtesten Datenlücken, die USA die einfachste zweite Region.

### Datenquellen im Vergleich

| Quelle | Inhalt | Lizenz | Einsatz | Vorbehalt |
| --- | --- | --- | --- | --- |
| [openAIP](https://www.openaip.net) | Flugplätze, Lufträume, Navaids, Meldepunkte, Hindernisse, weltweit | CC BY-NC 4.0 + Freigabe für Apps | MVP-Kern | Freigabe ist openAIP-Aussage, nicht Teil von CC; schriftlich bestätigen lassen |
| [open flightmaps](https://openflightmaps.org/faq/) | AIXM-modelliert, detaillierter, u.a. Deutschland | OFMA General Users License | Verfahren, Meldepunkte, Naturschutzgebiete | Lizenztext nicht gelesen; nie primäre Navigationsquelle |
| OpenStreetMap | Basiskarte | ODbL | Basiskarte | Attribution; Share-alike für abgeleitete Datenbanken (nicht verifiziert) |
| [Copernicus DEM GLO-30](https://dataspace.copernicus.eu/explore-data/data-collections/copernicus-contributing-missions/collections-description/COP-DEM) | Gelände, unter 4 m vertikal (LE90), EGM2008 | frei mit Pflichtvermerk DLR/Airbus | Terrain, Profil, SVS | EEA-10 nur für Behörden |
| [NOAA aviationweather](https://aviationweather.gov/data/api/) | METAR, TAF, SIGMET weltweit | US-Regierung, keine Lizenzangabe | Wetter MVP | 100 Anfragen/min; Bulk-Dateien minütlich |
| [DWD Open Data](https://opendata.dwd.de/weather/) | Radar, ICON-Modelle, SYNOP | CC BY 4.0 | Radar, Höhenwind | kein METAR/TAF/GAFOR |
| [ECMWF Open Data](https://ecmwf.int/en/forecasts/datasets/open-data) | IFS/AIFS, 14 Druckflächen | CC BY 4.0, kommerziell erlaubt | Höhenwind weltweit | 0,1° seit 20.10.2026 |
| [OGN](https://www.glidernet.org/ogn-data-usage) | Internet-Verkehr (Segler, UL) | ODbL | Verkehrs-Overlay | Privatsphäre-Regeln, keine Weitergabe nach 24 h |
| [DFS AIS-Portal](https://ais.dfs.de/pilotservice/service/information/disclaimer/disclaimer.jsp) | AIP IFR/VFR, Karten, NfL | Urheberrecht | nur Deep-Link | kommerzielle Nutzung und Bearbeitungen nur mit Genehmigung |
| [DFS AIXM 5.1.1](https://ext.eurocontrol.int/aixm_confluence/display/AIX/Germany) | Navaids, Lufträume, Verfahren, Hindernisse | nicht angegeben | später, mit Lizenz | bei data@dfs.de klären |
| [France SIA](https://www.sia.aviation-civile.gouv.fr/pub/media/news/file//l/i/licenceopendata-vf_2.pdf) | französische AIP-Daten | Licence Ouverte | Phase 2 | Attribution, keine Verfälschung |
| [FAA NASR/CIFP/d-TPP](https://www.faa.gov/air_traffic/flight_info/aeronav/faq) | US-Daten inkl. IFR-Verfahren | gemeinfrei | Phase 4 | DOF darf nicht verändert werden |
| [FAA NMS-API](https://www.faa.gov/about/initiatives/notam/faqs) | NOTAMs | kostenlos auf Antrag | NOTAM-Kandidat | internationale Abdeckung unklar |
| [Open-Meteo](https://open-meteo.com/en/terms) | Wetter-API | Free Tier nur nicht-kommerziell | optional | Self-Hosting möglich |
| [Navigraph](https://developers.navigraph.com/docs/general/restrictions) | Navdaten, Charts | nur Simulation | ausgeschlossen | reale Nutzung verboten |

### Deutschland ist der schwierigste Startmarkt für Daten

Die DFS stellt AIP IFR, AIP VFR und seit 31.03.2026 die NfL kostenlos zum Lesen bereit ([DFS Presse](https://dfs.de/homepage/en/media/press/2026/31-03-2026-dfs-the-german-air-navigation-service-provider-is-expanding-its-range-of-digital-information-services-available-online-free-of-charge/)). Kostenlos heißt aber nicht offen lizenziert. Die Portalbedingungen verbieten kommerzielle Nutzung von Kartenmaterial, Bearbeitungen und automatisierten Abruf ohne schriftliche Genehmigung, mit Ausnahme der AIXM-Downloads ([DFS AIS-Bedingungen](https://ais.dfs.de/pilotservice/service/information/disclaimer/disclaimer.jsp)).

„LuftraumDFS“ über das BKG ist nur für Planungszwecke freigegeben und wird nur halbjährlich aktualisiert ([BKG Nutzungsbedingungen](https://sg.geodatenzentrum.de/public/gdz/lizenz/deu/nutzungsbedingungen_luftraumdfs.pdf)). Kommerzielle Apps verkaufen DFS-Karten als Raster-Add-on für rund 60 bis 70 € pro Ausgabe, offenbar mit bilateralen Lizenzen (Inferenz). Eine öffentliche DFS-Preisliste wurde nicht gefunden.

Daraus folgt: Rendere eine eigene ICAO-artige Vektorkarte aus openAIP und open flightmaps. Verlinke AIP-PDFs nur oder lade sie auf dem Gerät des Nutzers. Beantrage parallel eine DFS-Lizenz für AIXM-Daten.

### Härteste Lücke 1: NOTAMs

Für Deutschland und Europa wurde keine freie, weiterverteilbare NOTAM-Schnittstelle gefunden. EAD Basic ist ausdrücklich ein nicht-operationelles Demo-Werkzeug, EAD Pro und SWIM sind vertraglich ([EAD Basic](https://www.ead.eurocontrol.int/cms-eadbasic/opencms/en/ead-solutions/ead-basic)). Enroute bezog NOTAMs bisher von der FAA ([Enroute Datenquellen](https://akaflieg-freiburg.github.io/enrouteManual/08-appendix/01-dataSources.html)).

Die FAA hat am 18.04.2026 auf das NOTAM Management Service (NMS) umgestellt ([EAA](https://www.eaa.org/eaa/news-and-publications/eaa-news-and-aviation-news/2026-04-16-faas-new-notam-system-online-april-18)). Der Zugang ist kostenlos, aber nur per E-Mail-Antrag ([FAA NMS FAQ](https://www.faa.gov/about/initiatives/notam/faqs)). Ob NMS europäische NOTAMs liefert, ist offen und der erste Klärungspunkt für Phase 2.

### Härteste Lücke 2: IFR-Verfahren und Plates

Codierte europäische Verfahren (ARINC 424) gibt es nicht offen. Optionen sind kommerzielle Lizenzen von Jeppesen, Lido oder Navblue, eine Lizenz für den DFS-AIXM-Datensatz „Instrument Flight Procedures“ oder Teildaten aus open flightmaps. Preise für kleine oder OSS-Entwickler wurden nicht gefunden.

Deutsche Anflugkarten sind urheberrechtlich geschützt; Georeferenzierung gilt als Bearbeitung und braucht DFS-Genehmigung ([DFS AIS-Bedingungen](https://ais.dfs.de/pilotservice/service/information/disclaimer/disclaimer.jsp)). Ob man nur Georeferenzierungs-Parameter verteilen darf, ist rechtlich ungeklärt. Frankreichs Licence Ouverte erlaubt dagegen Wiederverwendung mit Attribution.

### USA als zweite Datenregion

Alle digitalen FAA-Produkte sind gemeinfrei, inklusive NASR, CIFP (ARINC 424) und Plates ([FAA AeroNav FAQ](https://www.faa.gov/air_traffic/flight_info/aeronav/faq)). Das ist der einzige große, offene IFR-Verfahrensdatensatz. Die USA sind deshalb datenseitig einfacher als jedes europäische Land.

### Datenpipeline

Der Server baut pro Quelle versionierte, signierte Datenpakete und liefert sie offline aus. Dazu gehören ein AIRAC-Kalender (28 Tage), das Vorladen des nächsten Zyklus und das Umschalten am Gültigkeitsdatum auf dem Gerät. Die DFS bietet AIXM als NOW/NEXT/AFTER-Baselines mit Prüfsummen an ([Eurocontrol AIXM](https://ext.eurocontrol.int/aixm_confluence/display/AIX/Germany)).

Wetter wird vom eigenen Backend aus den NOAA-Bulk-Dateien gespiegelt, um das Limit von 100 Anfragen pro Minute einzuhalten ([NOAA API](https://aviationweather.gov/data/api/)). Höhenwinde werden serverseitig zu kompakten Gittern verarbeitet. Alle Attributionen (DLR/Airbus, DWD, ECMWF, OSM, openAIP) gehören in einen Credits-Bildschirm.

## Regulatorik und Haftung

Die App braucht für Privatpiloten keine Zulassung, aber das Geschäftsmodell entscheidet über die Haftung. Nicht-kommerziell bleibt das Projekt sehr wahrscheinlich außerhalb der neuen Produkthaftung und des Cyber Resilience Act. Das ist Recherche, keine Rechtsberatung.

### Keine Zulassung für Part-NCO

Unter Part-NCO trägt der verantwortliche Pilot die Pflichten, nicht der App-Hersteller. NCO.GEN.125 verlangt nur, dass das Gerät Systeme und Pilot nicht beeinträchtigt ([UK CAA NCO.GEN.125](https://regulatorylibrary.caa.co.uk/965-2012/Content/Document%20Structure/07%20NCO/2%20Regs/18540_NCOGEN125_Portable_electronic_devices.htm)). Formale EFB-Genehmigungen (SPA.EFB) und Risikobewertungen betreffen CAT- und NCC-Betreiber ([EASA ED 2019/008/R](https://www.easa.europa.eu/en/downloads/71952/en)).

In den USA erlaubt AC 91-78A seit Februar 2024 Part-91-Piloten EFBs statt Papier, wenn die Inhalte funktional gleichwertig und aktuell sind ([AOPA](https://www.aopa.org/news-and-media/all-news/2024/march/14/updated-faa-efb-guidance-supports-paperless-flight)). EASA verlangt laut AMC zu NCO.GEN.125 eine Rückfallebene für Kartenanwendungen ([pilotundflugzeug.de](https://www.pilotundflugzeug.de/forum/luftrecht/2023,09,20,11,1016393)). Der Wortlaut von NCO.GEN.135 zu elektronischen Karten wurde nicht verifiziert.

### Keine DAT-Zertifizierung für VFR-Software

Laut EASA-FAQ 96126 fällt VFR-Planungs- und Navigationssoftware nicht unter die zertifizierte DAT-Bereitstellung nach VO (EU) 2017/373 ([EASA FAQ 96126](https://www.easa.europa.eu/en/faq/96126)). Zertifizierte Datenbanken sind nur Pflicht, wenn GPS primäre Navigation ist, etwa bei SID/STAR, RNAV 1 oder LPV ([EASA FAQ 96128](https://www.easa.europa.eu/en/faq/96128)). Die FAQs stammen von 2019; neuere Aussagen wurden nicht gefunden.

Für IFR heißt das: Plates und Profile sind Referenzmaterial, Navigation nach der App-Position ist unzulässig. Die App darf nie als primäres Navigationsmittel vermarktet werden. Enroute formuliert das drastisch: als Primärnavigation sei die Nutzung „most likely illegal, most certainly stupid“ ([Enroute](https://akaflieg-freiburg.github.io/enroute)).

### Produkthaftung ab 9.12.2026

Nach der neuen Produkthaftungsrichtlinie (EU) 2024/2853 ist Software ein Produkt, unabhängig vom Vertriebsweg ([Sidley](https://www.sidley.com/en/insights/newsupdates/2026/10/revised-eu-product-liability-directive)). FOSS außerhalb einer Geschäftstätigkeit ist ausgenommen, nicht aber bei Bezahlung oder Daten als Gegenleistung. Haftung lässt sich vertraglich nicht ausschließen, und für Tod und Körperverletzung gibt es keine Obergrenze mehr.

Der Bundestag hat das deutsche Umsetzungsgesetz am 8.10.2026 beschlossen, mit FOSS-Ausnahme „außerhalb einer Geschäftstätigkeit“ ([Bundestag](https://www.bundestag.de/dokumente/textarchiv/2026/kw41-de-produkthaftungsrecht-1222456)). Wer ein Produkt wesentlich verändert und in Verkehr bringt, haftet als Hersteller, also auch Fork-Betreiber. Der genaue Wortlaut im Bundesgesetzblatt lag nicht vor.

Haftungsausschlüsse wie „not for primary navigation“ beseitigen die verschuldensunabhängige Haftung nicht. Sie definieren aber den bestimmungsgemäßen Gebrauch und stützen ein Mitverschulden (Inferenz). Deshalb: Bestätigung beim ersten Start, dauerhafter Hinweis, sichtbares Datenalter.

### Cyber Resilience Act

Der CRA erfasst nur FOSS, die im Rahmen einer Geschäftstätigkeit bereitgestellt wird; nicht monetarisierte Produkte zählen nicht dazu ([EU-Kommission](https://digital-strategy.ec.europa.eu/en/policies/cra-open-source)). Spenden ohne Gewinnabsicht sind keine kommerzielle Tätigkeit ([GitHub Blog](https://github.blog/open-source/maintainers/what-the-eus-new-software-legislation-means-for-developers/)). Meldepflichten gelten seit September 2026, alle Pflichten ab etwa Dezember 2027.

Ein gemeinnütziger Träger kann als „Open-Source-Software-Steward“ gelten, mit leichten Pflichten und ohne Bußgelder. Trotzdem lohnt eine Basis ab Tag eins: SECURITY.md, signierte Releases, SBOM über cargo-cyclonedx und Abhängigkeitsprüfung mit cargo-deny. Damit wird ein späterer Wechsel in den Anwendungsbereich billig.

### Geschäftsmodell und Rechtsfolgen

| Modell | Produkthaftung (PLD) | CRA | openAIP | Prototype Fund |
| --- | --- | --- | --- | --- |
| Kostenlos, nur Spenden an e.V. | sehr wahrscheinlich ausgenommen (Spenden in PLD-Quellen nicht behandelt) | außerhalb, ggf. Steward | unproblematisch | Antrag als Person möglich |
| Kostenlose App + Bezahldienst in separater Firma | Firma haftet für Bezahlprodukt; Einfluss auf freien Kern unklar | Firma ist Hersteller | Daten nicht separat verkaufen | Firma nicht förderfähig |
| Bezahl-App oder Abo | Hersteller mit voller Haftung | Hersteller, CE-Selbstbewertung | Freigabe schriftlich klären | nicht förderfähig |
| Werbung oder Datenverwertung | Ausnahme entfällt | Hersteller | kritisch | nicht förderfähig |

### Sonderfälle

Ultraleichtflugzeuge fallen unter nationales Recht (LuftVO, LuftPersV); eine Kartenmitführungspflicht für UL wurde nicht gefunden. Diese Lücke solltest du mit DULV oder DAeC klären. Bei FLARM-Daten müssen NoTrack-Ziele unpersistiert bleiben, auch in der Track-Aufzeichnung ([FLARM FTD-012](https://www.flarm.com/wp-content/uploads/2024/04/FTD-012-Data-Port-Interface-Control-Document-ICD.pdf)).

Ein Umzug nach Zypern ändert an PLD und CRA wenig, weil beide EU-weit gelten. Es ändern sich nationale Umsetzung, Behörden und die Förderfähigkeit beim Prototype Fund.

## Architektur in Rust

Empfohlen ist „Rust-Gehirn, nativer Körper“: Rust besitzt Logik, Daten, Warnungen und Offline-Speicher, SwiftUI und Compose besitzen die Bildschirme, MapLibre Native die Karte. „Alles in Rust“ heißt dann geschätzt 80 bis 90 Prozent des Codes; die Zahl ist geschätzt, nicht gemessen.

**Architektur (Diagramm als Text):**

- Oberfläche: iPad/iPhone (SwiftUI, MapLibre Native/Metal), Android (Jetpack Compose, MapLibre Native/Vulkan, Phase 2), Web-Planer (MapLibre GL JS + WASM-Kern, Phase 2)
- Brücke: UniFFI-Bindings (iOS, Android) · WASM (Web)
- Rust-Kern (geschätzt 80 bis 90 % des Codes): efb-core (App-Zustand, Warnlogik), efb-flight (Route, Navlog, Beladung), efb-geo (Geodäsie, Luftraumgeometrie), efb-traffic (GDL90, FLARM-NMEA, Discovery), efb-wx (METAR, TAF, SIGMET, NOTAM), efb-navdata · efb-tiles (SQLite R*Tree, PMTiles, Style)
- Externe Quellen: Verkehrsempfänger (Stratux Europe Edition, FLARM, SkyEcho; GDL90 über UDP 4000, FLARM-NMEA über TCP 2000) → Position, Verkehr; Datenpipeline auf dem Server (openAIP, open flightmaps, OSM, Copernicus DEM; Wetter von NOAA, DWD, ECMWF; signierte Offline-Pakete im AIRAC-Takt) → Datenpakete

Die Shells zeichnen nur Bildschirme und lesen Sensoren; alle Entscheidungen, inklusive der Luftraumwarnungen, fallen im Rust-Kern und laufen dadurch auf allen Plattformen identisch.

### Schichten

| Schicht | Technik | Begründung |
| --- | --- | --- |
| Kernlogik | Rust: geo, geographiclib-rs, rstar, uom, jiff | Geodäsie, Lufträume, Flugzustand, Warnungen |
| Speicher | rusqlite (SQLite mit R\*Tree, FTS5), PMTiles/MBTiles | Offline-Navdaten und Karten; [pmtiles 0.24.1](https://docs.rs/crate/pmtiles/latest) |
| FFI | UniFFI (Ferrostar-Muster) oder Crux | [Ferrostar](https://stadiamaps.com/news/ferrostar-building-a-cross-platform-navigation-sdk-in-rust-part-1), [crux\_core 0.20](https://docs.rs/crate/crux_core/0.20.0) |
| Karte | MapLibre Native iOS (Metal) und Android (Vulkan) | einzige produktionsreife Option mit Labels, Kollision, Gesten |
| UI | SwiftUI (iPad zuerst), Jetpack Compose | Split View, VoiceOver, Hintergrundortung |
| Sensoren | CoreLocation/CoreMotion, Android Foreground Service; GDL90/NMEA in Rust | [Ferrostar Doku](https://stadiamaps.github.io/ferrostar/print.html) |
| Web-Planer | MapLibre GL JS + Rust-Kern als WASM | gleiche Logik im Browser |
| Synthetic Vision | eigener wgpu-View (Phase 4) | [wgpu 30.0.2](https://docs.rs/crate/wgpu/latest), Metal/Vulkan first-class |

Ferrostar von Stadia Maps ist der direkte Präzedenzfall: Rust-Kern, UniFFI, MapLibre in SwiftUI und Compose, produktiv auf iOS und Android ([Ferrostar Doku](https://stadiamaps.github.io/ferrostar/print.html)). Lehren daraus: iOS braucht `staticlib`, UniFFI-Versionen pinnen, Objekte müssen Send+Sync sein. UniFFI passt besser für hochfrequente Datenströme (10 Hz Position, Verkehr), Crux für ein strenges Event/Effect-Modell.

### Workspace

```
crates/efb-geo        Geodäsie, Luftraumgeometrie
crates/efb-navdata    openAIP/AIXM/ARINC-424-Import nach SQLite+R*Tree
crates/efb-wx         METAR/TAF/SIGMET, später NOTAM
crates/efb-traffic    GDL90, FLARM-NMEA, Discovery (UDP 63093)
crates/efb-flight     Route, Navlog, Kraftstoff, W&B, Flugzustand
crates/efb-tiles      PMTiles/MBTiles, Style-JSON-Generator
crates/efb-core       App-Zustand, Warnlogik, Effekte
crates/efb-ffi        UniFFI-Bindings
crates/efb-cli        Headless-Planer, Testläufer, Steuerkanal/MCP
crates/efb-svs        wgpu-Synthetic-Vision (später)
apps/ios  apps/android  apps/web  xtask
```

Die Luftfahrt-Crates sind jung: arinc424 0.4.1 (Docs-Abdeckung 38 Prozent), metar-taf-parser 0.4.9, gdl90 1.0.0 ohne ForeFlight-Erweiterungen ([docs.rs arinc424](https://docs.rs/arinc424), [metar-taf-parser](https://docs.rs/crate/metar-taf-parser/0.4.9), [lib.rs gdl90](https://lib.rs/crates/gdl90)). Ein Rust-AIXM- oder NOTAM-Parser wurde nicht gefunden. Rechne damit, diese Parser selbst zu besitzen, zu fuzzen und als sicherheitsrelevant zu behandeln.

### Verkehr und Position

Unterstütze GDL90 auf UDP 4000 (zusätzlich 49002), FLARM-NMEA als TCP-Client auf Port 2000 und die ForeFlight-Discovery auf UDP 63093 ([Enroute Manual](https://akaflieg-freiburg.github.io/enrouteManual/07-referenceTechnology/01-trafficData.html)). Damit sind Stratux, SkyEcho und FLARM-Installationen abgedeckt. Die Positionsquelle folgt einer Priorität: externes Gerät, internes GNSS, manueller Fallback.

Auf iOS braucht UDP-Empfang die Local-Network-Freigabe; Bluetooth Classic erfordert MFi, also ist BLE der realistische Weg (nicht verifiziert). Druckhöhe kommt aus PGRMZ oder GDL90-Ownship, nicht vom Tablet-Barometer.

### Offene technische Risiken

Ungeprüft ist, ob UniFFI-Aufrufe bei 10 bis 20 Hz mit großen GeoJSON-Updates in MapLibre auf dem iPad flüssig bleiben. Ebenso offen ist, ob MapLibre direkt aus einem Rust-eigenen PMTiles-Speicher lesen kann, ohne localhost-Server. Beides gehört in einen Prototyp-Spike in Phase 0.

### Alternative: alles in Rust

Ein reiner Rust-Stack hieße Slint 1.17 für die UI und eine Rust-Karte. Slint hat die beste Mobile-Doku mit Safe-Area-Insets und virtueller Tastatur, iOS über winit und Skia ([Slint 1.17](https://slint.dev/blog/slint-1.17-released), [Slint iOS](https://docs.slint.dev/latest/docs/slint/guide/platforms/mobile/ios/)). Die Royalty-free-Lizenz ist mit offener App-Software vereinbar und verlangt nur einen About-Hinweis ([Slint Lizenz](https://unpkg.com/slint-ui@1.17.1/LICENSES/LicenseRef-Slint-Royalty-free-2.0.md)).

Das Problem ist die Karte. maplibre-rs bezeichnet sich als Proof of Concept ohne Labels, Kollision und Raster ([maplibre-rs](https://raw.githubusercontent.com/maplibre/maplibre-rs/main/README.md)). Galileo hatte seit Juli 2025 kein Release, Touch ist „WIP“ ([docs.rs galileo](https://docs.rs/crate/galileo/latest)).

Ein eigener Renderer wäre ein Aufwand von mehreren Personenjahren (Inferenz). Dazu kommen VoiceOver, Split View und Hintergrundortung, die man nachbauen oder überbrücken müsste. Beobachten lohnt sich die experimentelle MapLibre-C-API mit Rust-Bindings (maplibre-native-ffi) von April 2026, die später eine Rust-gesteuerte Karte auf allen Plattformen ermöglichen könnte ([MapLibre Newsletter April 2026](https://maplibre.org/news/2026-05-02-maplibre-newsletter-april-2026/)).

### Agentisch bauen im ArtCraft-Stil

Das Projekt eignet sich gut für Coding-Agenten, wenn die Spezifikation maschinenprüfbar ist. Vorbild sind die AGENTS.md-Regeln der „Craft“-Familie (LightCraft, VectorCraft); das PhotoCraft-Repo selbst wurde in der Recherche nicht gefunden ([LightCraft AGENTS.md](https://raw.githubusercontent.com/storytold/lightcraft/master/AGENTS.md), [VectorCraft AGENTS.md](https://raw.githubusercontent.com/storytold/vectorcraft/main/AGENTS.md)).

- **Paritätskatalog:** `docs/parity.md` mit einer Zeile pro Funktion aus der ForeFlight-, SkyDemon- und Garmin-Featureliste, jeweils mit Akzeptanzkriterien und Test-IDs. Dazu eine priorisierte „Top gaps“-Liste nach Nutzerwirkung; `cargo xtask parity` prüft, dass zitierte IDs existieren.
- **AGENTS.md:** Session-Start mit `plan/STATUS.md`, eine Task-ID pro Commit, ehrlicher Status („ein grüner Haken ist kein Paritätsbeweis“).
- **xtask-Gates:** `cargo xtask ci` vor jedem Commit (fmt, clippy -D warnings, Tests, `xtask layers`, WASM-Build), `xtask bench` für Performance-Budgets.
- **Never crash:** kein Panic im Produktivcode, `unsafe` nur in einem Crate, Eingabezahlen gelten als feindlich, jeder Crash-Fix bekommt einen Regressionstest.
- **Headless-Steuerkanal:** „Alles ist ein Command“, ansprechbar über UI, CLI, JSON-Lines-Kanal und MCP in `efb-cli`. Agenten steuern die App nie über synthetische OS-Eingaben.
- **Golden-Image-Tests:** Kartenrendering über MapLibre Native headless auf Linux mit festen Styles und Fixtures, plus Screenshot-Diffs aus XCUITest und Compose-Tests.
- **Simulator-Tests:** aufgezeichnete GDL90- und NMEA-Ströme als Fixtures, ein Flugsimulator als Live-Quelle, Routensimulation im Kern.
- **Parallele Agenten:** getrennte git-Worktrees mit eigenem CARGO\_TARGET\_DIR.

Agenten arbeiten primär im reinen Rust-Kern, wo Tests schnell und deterministisch sind. Luftfahrtkorrektheit (vertikale Luftraumgrenzen, Missweisung) braucht von Menschen geprüfte Fixtures.

**Vorsicht beim Paritätskatalog:** Er katalogisiert Funktionen, nicht Aussehen. Kopiere weder UI pixelgenau noch die proprietäre Kartengestaltung von ForeFlight oder SkyDemon. Folge stattdessen ICAO Annex 4 und den öffentlichen DFS-Konventionen; eine Rechtsquelle zum Designschutz von Kartenstilen wurde nicht gefunden.

## UI/UX und Designsystem

Ein einheitliches Erscheinungsbild entsteht aus einem gemeinsamen Designsystem, nicht aus einem gemeinsamen UI-Framework. Tokens, Symbolik-Spezifikation und Komponentenkatalog leben im Repo und werden für SwiftUI, Compose, Web und den MapLibre-Style generiert.

### Design-Tokens aus Human-Factors-Forschung

| Token | Wert | Quelle |
| --- | --- | --- |
| Minimale Touch-Fläche | 15 mm physisch (ca. 85 pt auf dem iPad) | [Avsar et al. 2015](https://cendekia.unisza.edu.my/neuaxis-e/Record/nottingham-31399/Details) |
| Primäraktionen im Flug | 22 mm (Direct-To, Nearest, Quittieren) | [FAA/Volpe §6.4.6](https://hfcc.dot.gov/publications/docs/GeneralGuidance/zz_FAA_GeneralGuidanceDoc_Chapter_06_Section_04.html) |
| Feedback-Latenz | unter 100 ms | FAA/Volpe |
| Touch-Entprellung | 200 ms nach letztem Kontakt ignorieren | [Cockburn et al. 2017](https://www.lri.fr/~anab/teaching/evaluation/papers/turbulent-touch.pdf) |
| Kodierfarben | maximal 6, immer mit zweitem Merkmal | [Volpe 2014](https://ntlrepository.blob.core.windows.net/lib/51000/51900/51993/Use_of_Color_on_Airport_Moving_Maps_and_CDTIs.pdf) |
| Kontrast | 8:1 für Daten, 3:1 für Kartenelemente | Volpe 2014 |
| Helligkeit | unabhängig, in feinen Stufen | [FAA AC 120-76E](https://www.faa.gov/documentLibrary/media/Advisory_Circular/AC_120-76E_FAA_Web.pdf) |

Apples 44-pt-Richtlinie entspricht etwa 7,7 mm und ist für den Flug viel zu klein. Die Tokens müssen deshalb in physischen Millimetern pro Gerät umgerechnet werden. Unter Vibration stieg die Auswahlzeit von 1.163 ms auf 2.360 ms; 16-mm-Ziele waren deutlich schneller als 8-mm-Ziele ([Cockburn et al.](https://www.lri.fr/~anab/teaching/evaluation/papers/turbulent-touch.pdf)).

### Interaktionsregeln im Flugmodus

Im Flugmodus gilt: nur Tippen und Wischen, keine Schieberegler, kein Ziehen. Schieberegler und Drag-Auswahl waren in Studien unter Vibration am fehleranfälligsten ([Coutts et al.](https://eprints.soton.ac.uk/426464/1/TouchscreensVibrationEnvironmentPaperThird.pdf)). Auslösen beim Loslassen, Randberührungen durch aufgestützte Daumen verwerfen, kritische Eingaben bestätigen.

Rubber-Band-Routing ist am Boden in Ordnung, im Flug nur nach modaler Bestätigung. Komplexe Eingaben während Start, Steigflug und Sinkflug vermeidet die App ([FAA AC 120-76E](https://www.faa.gov/documentLibrary/media/Advisory_Circular/AC_120-76E_FAA_Web.pdf)).

### Layout und Warnungen

Das Fluglayout folgt dem SkyDemon-Muster: vollflächige Karte, oben eine konfigurierbare Leiste mit 4 bis 6 Feldern (GS, TRK, ALT, nächster Wegpunkt mit ETE, Wind). Unten ein einklappbares Vertikalprofil, dazu ein dauerhafter großer Direct-To/Nearest-Button. Tippen auf die Karte öffnet immer denselben Standardinhalt, um ForeFlights kontextabhängige Tabs zu vermeiden ([Pilots of America](https://pilotsofamerica.com/community/threads/why-are-they-messing-up-foreflight.126416/unread?new=1)).

Warnungen haben drei Stufen: Hinweis, Vorsicht (Amber), Warnung (Rot), Sprachausgabe nur für Rot. Die UK CAA dokumentiert, dass Piloten Warnungen bei Überflutung ungelesen wegwischen ([UK CAA](https://infringements.caa.co.uk/tutorial/using-a-vfr-moving-map/)). Deshalb: Ratenbegrenzung, Deduplizierung, Quittieren mit einem großen Tipp.

### Kartensymbolik nach DFS-Konvention

Deutsche Piloten erwarten die DFS-Kodierung der ICAO-Karte 1:500 000 ([DFS AIC VFR 6/2009](https://service.dhv.de/dbfiles/gl/gutachter/Vorlagen_Dateien/Luftraum/aic_vfr_6_2009_ICAO_Darstellung_neu.pdf)):

- Klasse C und D: blaue Linie mit blauem Band.
- Klasse E: einzelne, etwas dickere blaue Linie ohne Band.
- ED-R und ED-D: rote Linie mit schraffiertem rotem Band.
- FIS-Gebiete: grün; Klasse und Grenzen in blauen Kästen entlang der Grenze.

Die DFS-Ausgabe 2026 hat Topografie und Farbschema erneuert und an andere V500-Karten angeglichen ([DFS Presse 27.01.2026](https://dfs.de/homepage/en/media/press/2026/27-01-2026-icao-chart-2026-with-optimised-layout-and-more-information/)). Die Legende für TMZ und RMZ wurde in der Recherche nicht gefunden und muss nachgeholt werden. Für andere Länder dient ICAO Annex 4 als Basis ([ICAO Annex 4](https://www.bazl.admin.ch/dam/bazl/en/dokumente/Fachleute/Regulationen_und_Grundlagen/icao-annex/icao_annex_4_aeronauticalcharts.pdf.download.pdf/an04_cons.pdf)).

Der Konflikt zwischen rotem ED-R und roter Warnfarbe löst sich so: statisches ED-R als entsättigte, schraffierte Linie, kräftiges Rot mit Banner und Ton nur für aktive Warnungen. Das ist eine Inferenz, kein getestetes Ergebnis.

### Nachtmodus, Farbsehschwäche, Sprache

Der Nachtmodus ist ein eigenes, sehr dunkles Theme, kein Rotfilter. Ein Rotfilter würde die rote und magentafarbene Kartenkodierung zerstören (Inferenz). Volpe verlangt, dass der Nachtmodus auf allen Zoomstufen die Dunkeladaption nicht stört ([Volpe 2014](https://ntlrepository.blob.core.windows.net/lib/51000/51900/51993/Use_of_Color_on_Airport_Moving_Maps_and_CDTIs.pdf)).

Ein CVD-Modus verstärkt Muster statt Farben: Kreuzschraffur für ED-R, Diagonale für ED-D, gepunktet für TMZ, gestrichelt für RMZ. Jedes Symbol muss in Graustufen über zwei Merkmale unterscheidbar sein. Eine empirische Studie zu farbsehschwachen Piloten und Luftraumpaletten wurde nicht gefunden.

Texte entstehen deutsch zuerst mit DFS-Terminologie, ICAO-Abkürzungen (TMZ, RMZ, CTR, QNH) bleiben unübersetzt. Datenfelder brauchen etwa 30 Prozent mehr Breite für deutsche Labels (Inferenz).

### Aufbau des Designsystems

- **Tokens:** Farben, Abstände, Touch-Größen, Typografie als JSON; generiert nach Swift, Kotlin, CSS.
- **Symbolik-Spezifikation:** Luftraum-, Flugplatz- und Navaid-Darstellung pro Land; daraus erzeugt Rust das MapLibre-Style-JSON für alle Plattformen.
- **Komponentenkatalog:** Datenleiste, Warnbanner, Was-ist-hier-Sheet, Profilansicht, mit Golden-Screenshots pro Plattform und Theme.

## Open-Source-Modell, Lizenz und Finanzierung

Empfehlung: MPL-2.0 für die App, MIT/Apache-2.0 für wiederverwendbare Crates, DCO statt CLA, und ein gemeinnütziger Träger hält Marke und Spenden. Das hält die FOSS-Ausnahmen von PLD und CRA und passt zu openAIP und zu Förderprogrammen.

### Lizenzwahl

MPL-2.0 ist File-Level-Copyleft: Änderungen an Projektdateien bleiben offen, Kombination mit proprietärem Code ist erlaubt. Die Lizenz gilt als App-Store-tauglich, OpenEFB nutzt sie bereits ([GitHub openefb](https://github.com/quartermint/openefb)). GPL-Projekte im App Store hängen dagegen an der Zustimmung aller Beitragenden; VLC flog 2011 nach einer einzigen Beschwerde aus dem Store ([The Next Web](https://thenextweb.com/news/apple-officially-pulls-vlc-from-the-app-store), [Michel Fortin](https://michelf.ca/blog/2011/gpl-ios-app-store)).

Parser für ARINC 424, AIXM, GDL90 und METAR sollten MIT/Apache-2.0 sein, wie im Rust-Ökosystem üblich. So können libefb, Avionik-Hersteller und andere Apps sie nutzen, und das Projekt wird Infrastruktur statt Insel. Das Backend (Wetterspiegel, NOTAM-Proxy) kann optional unter AGPL-3.0 stehen, um SaaS-Trittbrettfahrer zu verhindern.

Trade-off: Unter MPL kann ein Wettbewerber den Code in einer geschlossenen App nutzen, solange er Änderungen an MPL-Dateien offenlegt. GPL-3.0 schützt stärker, bräuchte aber eine App-Store-Ausnahme nach §7 ab Tag eins. Achtung: Enroute-Code (GPL) lässt sich nicht in ein MPL-Projekt übernehmen; nutzbar sind Konzepte und Kooperation auf Datenebene (Lizenzlogik, nicht juristisch geprüft).

DCO (Signed-off-by) reicht, weil MPL kein App-Store-Problem hat und kein Relizenzierungsrecht braucht. Ein CLA wäre nur für ein späteres Dual-Licensing nötig und schreckt Beitragende ab.

### Governance: die Lehre aus Organic Maps

Organic Maps wurde über eine estnische OÜ zweier Entwickler geführt, die Marke und Finanzen hielt. Nach Affiliate-Links, einer versteckten Server-Komponente und fehlender Finanztransparenz spaltete sich 2025 CoMaps ab ([LWN](https://lwn.net/Articles/1024387), [CoMaps](https://www.comaps.app/news/2025-04-16/1/)). CoMaps ist seitdem strikt gemeinnützig und nimmt Spenden über Open Collective an.

Daraus folgt: Marke, Spenden und App-Store-Konten gehören einem gemeinnützigen e.V. oder einer Stiftung mit öffentlichen Finanzen. Eine Markenrichtlinie erlaubt unveränderte Weitergabe und verlangt Umbenennung bei Forks, wie bei Firefox oder Rust. Das trennt auch die Haftung, weil Fork-Betreiber als Hersteller gelten.

Bezahldienste wie Sync oder Premium-Wetter gehören, falls je gewünscht, in eine getrennte Gesellschaft mit Markenlizenz und Produkthaftpflicht. Ob ein bezahlter Dienst den freien Kern „kommerziell“ macht, ist rechtlich ungeklärt. Die Gemeinnützigkeit von Piloten-Software in Deutschland muss ein Steuerberater prüfen.

### Finanzierung

| Programm | Betrag | Frist | Eignung | Haken |
| --- | --- | --- | --- | --- |
| [Prototype Fund](https://prototypefund.de/en/apply) | bis 47.500 € (Person), 95.000 € (Team), 6 Monate; Second Stage bis 15 Projekte | **30.11.2026** | sehr gut für Phase 0/1 | deutscher Wohnsitz, keine Firmen/Vereine |
| [NLnet / NGI Zero](https://nlnet.nl/funding.html) | 5.000 bis 50.000 € | 3. jedes ungeraden Monats, nächste 3.1.2027 | für Bibliotheken und offene Datenformate | Luftfahrt-App kein offensichtlicher Themen-Fit |
| [Sovereign Tech Fund](https://sovereign.tech/programs/fund) | ab 50.000 € Umfang | laufend | nur extrahierte Basis-Bibliotheken | keine Endnutzer-Apps, keine Prototypen |
| Open Collective, GitHub Sponsors | laufend | keine | Server, Daten, Hardware | Spendenbasis muss wachsen |

Die Reihenfolge: Prototype Fund für Phase 0 und 1, solange du in Deutschland wohnst. NLnet für Infrastrukturstücke, die auch ohne deutschen Wohnsitz offen bleiben. Spendenplattformen für laufende Kosten; die Termine für 2027 und die Fortführung von NGI nach 2026 sind nicht bestätigt.

## Name und Branding

Der Name ist „OpenFlightDeck“, kurz OFD. Flight Deck heißt Cockpit, der Name sagt also direkt, wofür die App da ist; „Open“ stellt sie neben openAIP und open flightmaps. Am 22.11.2026 waren openflightdeck.com, .org, .app und github.com/openflightdeck frei (Registry-Abfrage).

### Entscheidung und geprüfte Alternativen

| Name | Ergebnis |
| --- | --- |
| **OpenFlightDeck** | gewählt; im Blick behalten: die Kneeboard-App „[FlightDeck EFB](https://apps.apple.com/us/app/id1592153352)“ und das abgeschlossene GE-Programm „[Open Flight Deck](https://www.ainonline.com/aviation-news/aerospace/2017-06-19/ge-launches-new-partnership-open-flight-deck)“ (2017 bis 2021) |
| OpenAirborne | Alternative, .com frei |
| OpenWing | verworfen: .com, .org und GitHub vergeben, Nähe zu Wings „OpenSky“ |
| OpenEFB | verworfen: .com vergeben, mehrere Projekte gleichen Namens |
| OpenFlug | verworfen: zu deutsch für IFR und internationale Expansion |
| Oxwing, Ferrowing, Ironwing, Lotse | verworfen |

Verworfen wurden Kompass (KOMPASS-Kartenverlag), Kestrel (mehrere Luftfahrtmarken), Squawk (Skyguide-App), Rudder (RudderStack), QNH (beschreibend), Rhumb und Aileron (bestehende Apps) ([Justia Kestrel](https://trademarks.justia.com/870/80/kestrel-87080686.html), [Skyguide Squawk](https://apps.apple.com/ie/app/skyguide-squawk/id6444333848), [KOMPASS](https://apps.apple.com/DE/app/id459387986)). AeroRust ist eine bestehende Community für Rust in der Luft- und Raumfahrt, ein Kooperationspartner, kein Name ([GitHub AeroRust](https://www.Github.com/AeroRust)).

### Rust im Namen und beschreibende Namen

Die Rust Foundation verlangt eine schriftliche Erlaubnis, wenn „Rust“ Teil einer anderen Marke wird ([Rust Foundation](https://rustfoundation.org/policy/rust-trademark-policy)). Crate- oder Repo-Namen dürfen „Rust“ zur Beschreibung der Sprachkompatibilität nutzen. Deshalb sind Skyrust und Aerorust als Produktnamen ausgeschlossen.

Beschreibende Wörter wie Lotse, Wegpunkt oder Seitenruder riskieren eine Zurückweisung beim DPMA mangels Unterscheidungskraft. „Lotse“ eignet sich besser als Funktionsname, etwa für die Warn-Engine.

### Nächste Schritte

1. openflightdeck.com, .org und .app registrieren.
2. GitHub-Organisation „openflightdeck“ und passende crates.io-Namen sichern.
3. Kurze Markenrecherche bei EUIPO und DPMA in den Klassen 9, 39 und 42 vor dem öffentlichen Start.
4. Wortmarke später auf den Träger anmelden, nicht auf dich persönlich.

## Roadmap

Die Roadmap hat fünf Phasen mit harten Gates; jede Phase beginnt erst, wenn das Gate der vorherigen erfüllt ist. Phase 1 ist das MVP und der eigentliche Prüfstein.

**Roadmap (Diagramm als Text): Jede Phase startet erst, wenn das Gate der vorherigen erfüllt ist. Dauern sind grobe Schätzungen.**

1. **Phase 0 · Fundament · ca. 2 bis 3 Monate:** Workspace, xtask-Gates, AGENTS.md, Paritätskatalog; Träger oder Fiscal Host. Datenpipeline aus openAIP, open flightmaps, OSM und Copernicus DEM; Navdaten in SQLite mit R*Tree. GDL90- und NMEA-Parser mit Fuzzing; Spike zu UniFFI-Durchsatz und PMTiles in MapLibre.
   - *Gate:* Headless-CLI plant eine Route durch Deutschland mit korrekten Luftraumschnitten · Golden-Image-Tests grün · schriftliche Datenfreigaben von openAIP und open flightmaps liegen vor
2. **Phase 1 · MVP VFR Deutschland auf dem iPad · ca. 4 bis 6 Monate:** Moving Map offline, Luftraumwarnung mit Vertikalprofil, Routenplanung und Navlog. METAR/TAF, DWD-Radar, Verkehr über GDL90 und FLARM-NMEA, Track-Aufzeichnung. Direct-To und Nearest, Designsystem mit Tag-, Nacht- und CVD-Theme, Datenalter immer sichtbar.
   - *Gate:* 20 Testflüge mit gültigem PIC ohne falsche Luftraumwarnung · TestFlight-Beta mit 50 Piloten · über 99,5 % absturzfreie Sessions (Zielwerte als Vorschlag)
3. **Phase 2 · VFR Europa, Android, NOTAM · ca. 6 bis 9 Monate:** Compose-App für Android; Länderdaten zuerst Frankreich (Licence Ouverte), Österreich, Schweiz. Grafische NOTAMs, ED-R-Aktivierung, Masse und Schwerpunkt, Performance inkl. UL, Höhenwind. VFR-Flugplan, Web-Planer mit WASM-Kern.
   - *Gate:* rechtssichere NOTAM-Quelle mindestens für Deutschland gesichert · beide Stores live · SkyDemon-Kernfunktionen laut Paritätskatalog erreicht
4. **Phase 3 · IFR · ab ca. Monat 9 bis 12, abhängig von Lizenzen:** IFR-Enroute-Karte und Airways; SID/STAR und Anflüge aus lizenzierten Daten (DFS-AIXM oder Anbieter). Plates per Nutzer-Download oder Lizenz; IFR-Routing und Filing über autorouter.
   - *Gate:* unterschriebene Datenlizenz für mindestens ein europäisches Land · Verfahren von Menschen gegen die AIP geprüft · klar als Referenz, nicht als Navigation gekennzeichnet
5. **Phase 4 · Weltweit und Synthetic Vision · offen:** USA mit gemeinfreien FAA-Daten (NASR, CIFP, d-TPP) als erste Region außerhalb Europas. Synthetic Vision als wgpu-View mit externem AHRS; Desktop über Web-Planer oder Mac-App.
   - *Ziel:* FAA-Pipeline folgt automatisch dem 28/56-Tage-Zyklus · SVS hält das Framebudget auf älteren iPads

Drei Abhängigkeiten können den Plan kippen. Ohne NOTAM-Quelle bleibt das Produkt nach Phase 2 ein reines Kartenwerkzeug. Ohne Datenlizenz beschränkt sich IFR auf die USA, und ohne fliegenden Tester erreicht Phase 1 ihr Gate nicht.

## Quellen

**Markt und Wettbewerber:** [Thoma Bravo: Jeppesen ForeFlight standalone](https://www.thomabravo.com/press-releases/jeppesen-foreflight-launches-as-a-standalone-company-to-redefine-the-future-of-aviation-software) · [ForeFlight Pricing](https://foreflight.com/pricing/) · [ForeFlight Releases](https://foreflight.com/releases) · [ForeFlight Blog: IAOPA Europe](https://foreflight.com/blog/foreflight-partners-with-iaopa-europe-to-offer-discounts-to-general-aviation-pilots) · [ForeFlight App Store (US)](https://apps.apple.com/us/app/-/id333252638) · [AvBrief: Rabattkürzung](https://avbrief.com/foreflight-reduces-subscription-discounts/) · [Pilots of America](https://pilotsofamerica.com/community/threads/why-are-they-messing-up-foreflight.126416/unread?new=1) · [SkyDemon App Store](https://apps.apple.com/de/app/skydemon/id497184081?l=en) · [SkyDemon In-flight](https://www.SkyDemon.aero/inflight/) · [Garmin Pilot](https://apps.apple.com/de/app/garmin-pilot/id340917615?l=en) · [EasyVFR 4](https://apps.apple.com/de/app/easyvfr-4/id1286986764?l=en) · [Air Navigation Pro](https://apps.apple.com/DE/app/id301046057) · [AirMate](https://apps.apple.com/de/app/airmate/id998181102?l=en) · [Enroute Website](https://akaflieg-freiburg.github.io/enroute/) · [Enroute GitHub](https://github.com/Akaflieg-Freiburg/enroute) · [Avare](https://github.com/apps4av/avare) · [XCSoar](https://github.com/XCSoar/XCSoar) · [Stratux](https://github.com/stratux/stratux) · [OpenEFB](https://github.com/quartermint/openefb) · [DAeC LSG-B Jahresbericht 2024](https://www.daec.de/media/files/Dateien/Luftsportgeraete_Buero/Statistik/Jahresbericht_LSG-B_2024.pdf) · [Statista: Piloten in Deutschland](https://de.statista.com/statistik/daten/studie/183097/umfrage/anzahl-der-piloten-in-deutschland) · [Lobbyregister DULV](https://www.lobbyregister.bundestag.de/media/94/2c/809209/Lobbyregister-Registereintraege-Detailansicht-R005469-2026-07-08_16-46-59.pdf) · [pilotundflugzeug.de: EFB-Backup](https://www.pilotundflugzeug.de/forum/luftrecht/2023,09,20,11,1016393)

**Daten und Lizenzen:** [DFS AIS-Portal Bedingungen](https://ais.dfs.de/pilotservice/service/information/disclaimer/disclaimer.jsp) · [DFS Presse 31.03.2026](https://dfs.de/homepage/en/media/press/2026/31-03-2026-dfs-the-german-air-navigation-service-provider-is-expanding-its-range-of-digital-information-services-available-online-free-of-charge/) · [DFS Geodaten](https://dfs.de/homepage/en/services/geo-data/) · [BKG LuftraumDFS](https://sg.geodatenzentrum.de/public/gdz/lizenz/deu/nutzungsbedingungen_luftraumdfs.pdf) · [Eurocontrol AIXM: Germany](https://ext.eurocontrol.int/aixm_confluence/display/AIX/Germany) · [EAD Basic](https://www.ead.eurocontrol.int/cms-eadbasic/opencms/en/ead-solutions/ead-basic) · [SIA Licence Ouverte](https://www.sia.aviation-civile.gouv.fr/pub/media/news/file//l/i/licenceopendata-vf_2.pdf) · [openAIP](https://www.openaip.net) · [open flightmaps FAQ](https://openflightmaps.org/faq/) · [Enroute Datenquellen](https://akaflieg-freiburg.github.io/enrouteManual/08-appendix/01-dataSources.html) · [FAA AeroNav FAQ](https://www.faa.gov/air_traffic/flight_info/aeronav/faq) · [FAA NMS FAQs](https://www.faa.gov/about/initiatives/notam/faqs) · [EAA: neues NOTAM-System](https://www.eaa.org/eaa/news-and-publications/eaa-news-and-aviation-news/2026-04-16-faas-new-notam-system-online-april-18) · [Navigraph Restrictions](https://developers.navigraph.com/docs/general/restrictions) · [autorouter API](https://www.autorouter.aero/wiki/api/) · [NOAA Data API](https://aviationweather.gov/data/api/) · [DWD Open Data](https://opendata.dwd.de/weather/) · [DWD FlugWetter](https://apps.apple.com/de/app/dwd-flugwetter/id633263810?l=en) · [ECMWF Open Data](https://ecmwf.int/en/forecasts/datasets/open-data) · [Open-Meteo Terms](https://open-meteo.com/en/terms) · [Copernicus DEM](https://dataspace.copernicus.eu/explore-data/data-collections/copernicus-contributing-missions/collections-description/COP-DEM) · [OGN Data Usage](https://www.glidernet.org/ogn-data-usage) · [FLARM FTD-012](https://www.flarm.com/wp-content/uploads/2024/04/FTD-012-Data-Port-Interface-Control-Document-ICD.pdf) · [Enroute: Traffic Data](https://akaflieg-freiburg.github.io/enrouteManual/07-referenceTechnology/01-trafficData.html) · [Stratux Europe Edition](https://github.com/b3nn0/stratux) · [Dross:Aviation Stratux](https://static.dross.net/aviation/stratux/)

**Regulatorik, Haftung, Open Source, Förderung:** [NCO.GEN.125](https://regulatorylibrary.caa.co.uk/965-2012/Content/Document%20Structure/07%20NCO/2%20Regs/18540_NCOGEN125_Portable_electronic_devices.htm) · [EASA ED 2019/008/R](https://www.easa.europa.eu/en/downloads/71952/en) · [AOPA: FAA EFB guidance](https://www.aopa.org/news-and-media/all-news/2024/march/14/updated-faa-efb-guidance-supports-paperless-flight) · [EASA FAQ 96126](https://www.easa.europa.eu/en/faq/96126) · [EASA FAQ 96128](https://www.easa.europa.eu/en/faq/96128) · [Sidley: Produkthaftungsrichtlinie](https://www.sidley.com/en/insights/newsupdates/2026/10/revised-eu-product-liability-directive) · [Bundestag: Produkthaftungsrecht](https://www.bundestag.de/dokumente/textarchiv/2026/kw41-de-produkthaftungsrecht-1222456) · [EU-Kommission: CRA und Open Source](https://digital-strategy.ec.europa.eu/en/policies/cra-open-source) · [GitHub Blog: EU-Softwaregesetze](https://github.blog/open-source/maintainers/what-the-eus-new-software-legislation-means-for-developers/) · [Michel Fortin: GPL und App Store](https://michelf.ca/blog/2011/gpl-ios-app-store) · [The Next Web: VLC](https://thenextweb.com/news/apple-officially-pulls-vlc-from-the-app-store) · [LWN: Organic Maps](https://lwn.net/Articles/1024387) · [CoMaps Open Letter](https://www.comaps.app/news/2025-04-16/1/) · [Prototype Fund](https://prototypefund.de/en/apply) · [NLnet Funding](https://nlnet.nl/funding.html) · [Sovereign Tech Fund](https://sovereign.tech/programs/fund)

**Technik:** [Ferrostar Doku](https://stadiamaps.github.io/ferrostar/print.html) · [Ferrostar Teil 1](https://stadiamaps.com/news/ferrostar-building-a-cross-platform-navigation-sdk-in-rust-part-1) · [crux\_core 0.20.0](https://docs.rs/crate/crux_core/0.20.0) · [maplibre-rs](https://raw.githubusercontent.com/maplibre/maplibre-rs/main/README.md) · [Galileo](https://docs.rs/crate/galileo/latest) · [MapLibre Newsletter April 2026](https://maplibre.org/news/2026-05-02-maplibre-newsletter-april-2026/) · [wgpu](https://docs.rs/crate/wgpu/latest) · [pmtiles](https://docs.rs/crate/pmtiles/latest) · [Slint 1.17](https://slint.dev/blog/slint-1.17-released) · [Slint iOS](https://docs.slint.dev/latest/docs/slint/guide/platforms/mobile/ios/) · [Slint Royalty-free License](https://unpkg.com/slint-ui@1.17.1/LICENSES/LicenseRef-Slint-Royalty-free-2.0.md) · [efb (libefb)](https://docs.rs/crate/efb/latest) · [arinc424](https://docs.rs/arinc424) · [metar-taf-parser](https://docs.rs/crate/metar-taf-parser/0.4.9) · [gdl90](https://lib.rs/crates/gdl90) · [LightCraft AGENTS.md](https://raw.githubusercontent.com/storytold/lightcraft/master/AGENTS.md) · [VectorCraft AGENTS.md](https://raw.githubusercontent.com/storytold/vectorcraft/main/AGENTS.md)

**UI/UX und Name:** [FAA AC 120-76E](https://www.faa.gov/documentLibrary/media/Advisory_Circular/AC_120-76E_FAA_Web.pdf) · [FAA/Volpe §6.4.6 Touch Screens](https://hfcc.dot.gov/publications/docs/GeneralGuidance/zz_FAA_GeneralGuidanceDoc_Chapter_06_Section_04.html) · [Avsar et al. 2015](https://cendekia.unisza.edu.my/neuaxis-e/Record/nottingham-31399/Details) · [Cockburn et al. 2017](https://www.lri.fr/~anab/teaching/evaluation/papers/turbulent-touch.pdf) · [Coutts et al.](https://eprints.soton.ac.uk/426464/1/TouchscreensVibrationEnvironmentPaperThird.pdf) · [Volpe 2014: Farben auf Moving Maps](https://ntlrepository.blob.core.windows.net/lib/51000/51900/51993/Use_of_Color_on_Airport_Moving_Maps_and_CDTIs.pdf) · [DFS AIC VFR 6/2009](https://service.dhv.de/dbfiles/gl/gutachter/Vorlagen_Dateien/Luftraum/aic_vfr_6_2009_ICAO_Darstellung_neu.pdf) · [DFS Presse: ICAO-Karte 2026](https://dfs.de/homepage/en/media/press/2026/27-01-2026-icao-chart-2026-with-optimised-layout-and-more-information/) · [ICAO Annex 4](https://www.bazl.admin.ch/dam/bazl/en/dokumente/Fachleute/Regulationen_und_Grundlagen/icao-annex/icao_annex_4_aeronauticalcharts.pdf.download.pdf/an04_cons.pdf) · [UK CAA: VFR Moving Map](https://infringements.caa.co.uk/tutorial/using-a-vfr-moving-map/) · [Rust Foundation Trademark Policy](https://rustfoundation.org/policy/rust-trademark-policy) · [AeroRust](https://www.Github.com/AeroRust) · [Justia: Kestrel Aviation](https://trademarks.justia.com/870/80/kestrel-87080686.html) · [Skyguide Squawk](https://apps.apple.com/ie/app/skyguide-squawk/id6444333848) · [KOMPASS](https://apps.apple.com/DE/app/id459387986)
