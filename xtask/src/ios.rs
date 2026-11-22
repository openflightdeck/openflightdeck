//! iOS-Build: `ofd-ffi` als XCFramework mit Swift-Bindings, plus die
//! synthetische Testkarte für den Spike.
//!
//! Ergebnis liegt in `apps/ios/OFDCore/` (Swift Package) und ist nicht
//! eingecheckt; nach jeder Änderung an `ofd-ffi` neu bauen.

use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::{Result, cargo};

const TARGETS: &[&str] = &["aarch64-apple-ios", "aarch64-apple-ios-sim"];
const LIB: &str = "libofd_ffi.a";
const FFI_MODULE: &str = "ofd_ffiFFI";

fn root() -> Result<PathBuf> {
    let manifest = env!("CARGO_MANIFEST_DIR");
    Path::new(manifest)
        .parent()
        .map(Path::to_path_buf)
        .ok_or_else(|| "Workspace-Wurzel nicht gefunden".into())
}

fn run(cmd: &mut Command) -> Result<()> {
    let status = cmd.status()?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("{cmd:?} endete mit {status}").into())
    }
}

/// Baut `ofd-ffi` für Gerät und Simulator, erzeugt die Swift-Bindings und
/// packt alles als XCFramework in das Swift Package `apps/ios/OFDCore`.
pub(crate) fn build() -> Result<()> {
    let root = root()?;
    for target in TARGETS {
        cargo(&[
            "build",
            "--package",
            "ofd-ffi",
            "--release",
            "--target",
            target,
        ])?;
    }

    let device_lib = root.join("target/aarch64-apple-ios/release").join(LIB);
    let gen_dir = root.join("target/uniffi-swift");
    if gen_dir.exists() {
        fs::remove_dir_all(&gen_dir)?;
    }
    fs::create_dir_all(&gen_dir)?;

    let device_lib_str = device_lib.to_str().ok_or("Pfad ist kein UTF-8")?;
    let gen_dir_str = gen_dir.to_str().ok_or("Pfad ist kein UTF-8")?;
    cargo(&[
        "run",
        "--quiet",
        "--package",
        "xtask",
        "--bin",
        "uniffi-bindgen",
        "--",
        "generate",
        "--library",
        device_lib_str,
        "--language",
        "swift",
        "--out-dir",
        gen_dir_str,
    ])?;

    // Header und modulemap für das XCFramework.
    let headers = gen_dir.join("headers");
    fs::create_dir_all(&headers)?;
    fs::copy(
        gen_dir.join(format!("{FFI_MODULE}.h")),
        headers.join(format!("{FFI_MODULE}.h")),
    )?;
    fs::copy(
        gen_dir.join(format!("{FFI_MODULE}.modulemap")),
        headers.join("module.modulemap"),
    )?;

    let package = root.join("apps/ios/OFDCore");
    let xcframework = package.join(format!("{FFI_MODULE}.xcframework"));
    if xcframework.exists() {
        fs::remove_dir_all(&xcframework)?;
    }
    let mut cmd = Command::new("xcodebuild");
    cmd.arg("-create-xcframework");
    for target in TARGETS {
        cmd.arg("-library")
            .arg(root.join("target").join(target).join("release").join(LIB))
            .arg("-headers")
            .arg(&headers);
    }
    cmd.arg("-output").arg(&xcframework);
    run(&mut cmd)?;

    let sources = package.join("Sources/OFDCore");
    fs::create_dir_all(&sources)?;
    fs::copy(gen_dir.join("ofd_ffi.swift"), sources.join("ofd_ffi.swift"))?;

    println!("Swift Package bereit: {}", package.display());
    Ok(())
}

/// Erzeugt `apps/ios/Spike/Resources/spike.pmtiles`: ein synthetisches
/// Gitter mit Beschriftungen und Kreisen rund um Schönhagen (EDAZ), ganz ohne
/// externe Daten. Prüft nur, ob MapLibre lokale PMTiles lädt und Labels beim
/// Rotieren lesbar bleiben; es ist keine Karte.
pub(crate) fn spike_tiles() -> Result<()> {
    let root = root()?;
    let out_dir = root.join("apps/ios/Spike/Resources");
    fs::create_dir_all(&out_dir)?;
    let geojson_path = root.join("target/spike-grid.geojson");
    fs::create_dir_all(root.join("target"))?;

    let mut features = Vec::new();
    // Gitter alle 0,1° über Mitteldeutschland.
    for i in 0..=60 {
        let lon = 10.0 + f64::from(i) * 0.1;
        features.push(format!(
            r#"{{"type":"Feature","properties":{{"kind":"grid","major":{}}},"geometry":{{"type":"LineString","coordinates":[[{lon:.1},50.5],[{lon:.1},54.0]]}}}}"#,
            i % 10 == 0
        ));
    }
    for j in 0..=35 {
        let lat = 50.5 + f64::from(j) * 0.1;
        features.push(format!(
            r#"{{"type":"Feature","properties":{{"kind":"grid","major":{}}},"geometry":{{"type":"LineString","coordinates":[[10.0,{lat:.1}],[16.0,{lat:.1}]]}}}}"#,
            j % 10 == 0
        ));
    }
    // Beschriftete Punkte an jedem Gitterknoten im Abstand 0,5°.
    for i in 0..=12 {
        for j in 0..=7 {
            let lon = 10.0 + f64::from(i) * 0.5;
            let lat = 50.5 + f64::from(j) * 0.5;
            features.push(format!(
                r#"{{"type":"Feature","properties":{{"kind":"label","name":"N{lat:.1} E{lon:.1}"}},"geometry":{{"type":"Point","coordinates":[{lon:.1},{lat:.1}]}}}}"#
            ));
        }
    }
    // Konzentrische Ringe um EDAZ als Orientierung.
    for (k, radius_km) in [5.0_f64, 10.0, 20.0, 40.0].iter().enumerate() {
        let mut coords = String::new();
        for step in 0..=72 {
            let angle = f64::from(step) * 5.0_f64.to_radians();
            let lat = 52.2036 + radius_km / 111.2 * angle.cos();
            let lon = 13.1586 + radius_km / (111.2 * 52.2036_f64.to_radians().cos()) * angle.sin();
            if step > 0 {
                coords.push(',');
            }
            let _ = write!(coords, "[{lon:.5},{lat:.5}]");
        }
        features.push(format!(
            r#"{{"type":"Feature","properties":{{"kind":"ring","name":"{radius_km} km","index":{k}}},"geometry":{{"type":"LineString","coordinates":[{coords}]}}}}"#
        ));
    }
    let geojson = format!(
        r#"{{"type":"FeatureCollection","features":[{}]}}"#,
        features.join(",")
    );
    fs::write(&geojson_path, geojson)?;

    let out = out_dir.join("spike.pmtiles");
    run(Command::new("tippecanoe")
        .arg("--force")
        .arg("--output")
        .arg(&out)
        .args([
            "--layer",
            "spike",
            "--minimum-zoom",
            "4",
            "--maximum-zoom",
            "12",
        ])
        .args(["--no-feature-limit", "--no-tile-size-limit", "--quiet"])
        .arg(&geojson_path))?;
    println!("Testkarte: {}", out.display());
    Ok(())
}
