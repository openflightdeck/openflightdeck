# iOS

Derzeit nur der Spike für M1 (`OFDSpike`): eine iPad-App, die eine lokale PMTiles-Datei mit MapLibre Native anzeigt und ein Overlay aus dem Rust-Kern mit 10, 20 oder 60 Hz aktualisiert. Messplan und Ergebnisse stehen in [docs/spike-m1.md](../../docs/spike-m1.md).

## Bauen

Voraussetzungen: macOS mit Apple Silicon, Xcode, XcodeGen (`brew install xcodegen`), tippecanoe (`brew install tippecanoe`) und die Rust-Targets für iOS:

```bash
rustup target add aarch64-apple-ios aarch64-apple-ios-sim
```

Dann aus der Wurzel des Repos:

```bash
cargo xtask ios
```

```bash
cargo xtask spike-tiles
```

```bash
cd apps/ios && xcodegen && open OFDSpike.xcodeproj
```

`cargo xtask ios` baut `ofd-ffi` für Gerät und Simulator und legt das XCFramework samt Swift-Bindings in `OFDCore/` ab. `cargo xtask spike-tiles` erzeugt eine synthetische Testkarte (Gitter und Ringe um Schönhagen, keine echten Daten) nach `Spike/Resources/spike.pmtiles`. Alle drei Ergebnisse sind nicht eingecheckt und nach Änderungen neu zu erzeugen.

Ohne Apple-Developer-Konto lässt sich die App für den Simulator ad hoc signiert bauen und installieren:

```bash
xcodebuild -project apps/ios/OFDSpike.xcodeproj -scheme OFDSpike -sdk iphonesimulator -configuration Release -derivedDataPath apps/ios/build CODE_SIGN_IDENTITY=- CODE_SIGN_STYLE=Manual build
```

Die Messwerte erscheinen in der App und alle 2 s im Log (`log stream --predicate 'subsystem == "org.openflightdeck.spike"'`).

## Auf ein echtes iPad bringen

Einmalig:

1. **Lizenzvereinbarung bestätigen:** Der Kontoinhaber des Apple-Developer-Teams meldet sich auf developer.apple.com an und stimmt der aktuellen Program License Agreement zu. Solange das fehlt, bricht die Signierung mit „PLA Update available“ ab.
2. **Team eintragen:** `apps/ios/Local.xcconfig` anlegen (wird nicht eingecheckt) mit `DEVELOPMENT_TEAM = <Team-ID>`. Ist die Bundle-ID `org.openflightdeck.spike` im Team schon vergeben, zusätzlich `OFD_BUNDLE_ID = <eigene ID>`.
3. **Entwicklermodus am iPad:** iPad per Kabel an den Mac, „Diesem Computer vertrauen“ bestätigen. Dann am iPad unter Einstellungen › Datenschutz & Sicherheit › Entwicklermodus einschalten und neu starten.

Bauen und starten:

```bash
cargo xtask ios && cargo xtask spike-tiles && cd apps/ios && xcodegen && open OFDSpike.xcodeproj
```

In Xcode oben das iPad als Ziel wählen, Schema `OFDSpike`, Product › Scheme › Edit Scheme › Run › Build Configuration auf **Release** stellen und ▶︎ drücken. Xcode registriert das iPad beim ersten Mal im Team und legt das Profil automatisch an.

Messen: App etwa 30 s je Rate laufen lassen (10, 20, 60 Hz, Track-up an) und die eingeblendeten Werte abfotografieren oder in Xcode unter Debug › Console die Logzeilen `rate=…` kopieren.
