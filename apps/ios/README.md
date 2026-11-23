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
