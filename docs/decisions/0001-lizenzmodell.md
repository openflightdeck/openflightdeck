# 0001 Lizenzmodell: MPL-2.0 für die App, MIT/Apache-2.0 für Bibliotheken

Stand: 22.11.2026 · Status: angenommen

## Kontext

OpenFlightDeck soll frei bleiben, im App Store verteilbar sein und Bausteine liefern, die andere Projekte (z.B. andere EFBs, Simulatoren, Datenpipelines) ohne Hürden nutzen können. GPL ist mit den App-Store-Bedingungen schwer vereinbar.

## Entscheidung

- **MIT OR Apache-2.0** für die fachlichen Bibliotheken: `ofd-units`, `ofd-geo`, `ofd-navdata`, `ofd-wx`, `ofd-traffic`, `ofd-flight`, `ofd-alerts` sowie das Werkzeug `xtask`. Das ist der Rust-Standard und erlaubt jede Weiterverwendung.
- **MPL-2.0** für die App und alles, was sie zusammensetzt: `ofd-core`, `ofd-ffi`, `ofd-cli`, iOS-App, Pipeline, Design, Website und Dokumentation. Änderungen an diesen Dateien bleiben offen, die Einbindung in größere Werke und die App-Store-Verteilung sind möglich.
- Beiträge mit **DCO-Sign-off** statt CLA.

## Folgen

- Bibliotheken dürfen nie von MPL-Crates abhängen; `cargo xtask layers` prüft das zusammen mit dem `license`-Feld jeder Crate.
- Fremdabhängigkeiten nur mit Lizenzen aus `deny.toml`, geprüft durch cargo-deny in CI.
- Daten (openAIP, open flightmaps, OSM, Copernicus, DWD, NOAA) unterliegen eigenen Lizenzen und werden in der App attribuiert.
