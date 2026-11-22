//! Build- und Prüfaufgaben für OpenFlightDeck.
//!
//! `cargo xtask ci` ist das einzige Gate: Formatierung, Clippy mit Warnungen als
//! Fehler, Tests und die Schichtenprüfung. Lokal und in GitHub Actions läuft
//! exakt dasselbe.

// xtask ist ein Entwicklerwerkzeug und meldet sich über die Konsole.
#![allow(clippy::print_stdout)]

use std::env;
use std::error::Error;
use std::process::{Command, ExitCode};

mod ios;

pub(crate) type Result<T> = std::result::Result<T, Box<dyn Error>>;

const MIT_APACHE: &str = "MIT OR Apache-2.0";
const MPL: &str = "MPL-2.0";

/// Eine Schicht im Rust-Kern: welche internen Crates sie nutzen darf und unter
/// welcher Lizenz sie steht.
struct Layer {
    name: &'static str,
    license: &'static str,
    allowed: &'static [&'static str],
}

/// Erlaubte Abhängigkeiten zwischen den Crates, von unten nach oben.
///
/// Eine neue Crate muss hier eingetragen werden, sonst schlägt `cargo xtask ci`
/// fehl. Bibliotheken (MIT/Apache-2.0) dürfen nie von MPL-Crates abhängen.
const LAYERS: &[Layer] = &[
    Layer {
        name: "ofd-units",
        license: MIT_APACHE,
        allowed: &[],
    },
    Layer {
        name: "ofd-geo",
        license: MIT_APACHE,
        allowed: &["ofd-units"],
    },
    Layer {
        name: "ofd-navdata",
        license: MIT_APACHE,
        allowed: &["ofd-units", "ofd-geo"],
    },
    Layer {
        name: "ofd-wx",
        license: MIT_APACHE,
        allowed: &["ofd-units"],
    },
    Layer {
        name: "ofd-traffic",
        license: MIT_APACHE,
        allowed: &["ofd-units", "ofd-geo"],
    },
    Layer {
        name: "ofd-flight",
        license: MIT_APACHE,
        allowed: &["ofd-units", "ofd-geo"],
    },
    Layer {
        name: "ofd-alerts",
        license: MIT_APACHE,
        allowed: &["ofd-units", "ofd-geo", "ofd-flight", "ofd-navdata"],
    },
    Layer {
        name: "ofd-core",
        license: MPL,
        allowed: &[
            "ofd-units",
            "ofd-geo",
            "ofd-navdata",
            "ofd-wx",
            "ofd-traffic",
            "ofd-flight",
            "ofd-alerts",
        ],
    },
    Layer {
        name: "ofd-ffi",
        license: MPL,
        allowed: &["ofd-core", "ofd-units"],
    },
    Layer {
        name: "ofd-cli",
        license: MPL,
        allowed: &[
            "ofd-core",
            "ofd-units",
            "ofd-geo",
            "ofd-navdata",
            "ofd-wx",
            "ofd-traffic",
            "ofd-flight",
            "ofd-alerts",
        ],
    },
    Layer {
        name: "xtask",
        license: MIT_APACHE,
        allowed: &[],
    },
];

fn main() -> ExitCode {
    let task = env::args().nth(1).unwrap_or_default();
    let result = match task.as_str() {
        "ci" => ci(),
        "fmt" => cargo(&["fmt", "--all"]),
        "lint" => lint(),
        "test" => test(),
        "layers" => layers(),
        "ios" => ios::build(),
        "spike-tiles" => ios::spike_tiles(),
        _ => {
            print_help();
            return if task.is_empty() || task == "help" {
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            };
        }
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("xtask {task}: {err}");
            ExitCode::FAILURE
        }
    }
}

fn print_help() {
    println!(
        "cargo xtask <aufgabe>

Aufgaben:
  ci       alle Prüfungen wie in CI (fmt-Prüfung, clippy, Tests, Schichten)
  fmt      Code formatieren
  lint     clippy mit Warnungen als Fehler
  test     alle Tests im Workspace
  layers   Abhängigkeiten und Lizenzen der Crates prüfen
  ios      ofd-ffi für iOS bauen: XCFramework und Swift-Bindings (nur macOS)
  spike-tiles  synthetische PMTiles-Testkarte für den Spike erzeugen (tippecanoe)"
    );
}

fn ci() -> Result<()> {
    step("Formatierung", || cargo(&["fmt", "--all", "--check"]))?;
    step("Clippy", lint)?;
    step("Tests", test)?;
    step("Schichten und Lizenzen", layers)?;
    println!("\n==> cargo xtask ci: alles grün");
    Ok(())
}

fn step(name: &str, f: impl FnOnce() -> Result<()>) -> Result<()> {
    println!("\n==> {name}");
    f().map_err(|err| format!("{name} fehlgeschlagen: {err}").into())
}

fn lint() -> Result<()> {
    cargo(&[
        "clippy",
        "--workspace",
        "--all-targets",
        "--locked",
        "--",
        "-D",
        "warnings",
    ])
}

fn test() -> Result<()> {
    cargo(&["test", "--workspace", "--all-targets", "--locked"])
}

pub(crate) fn cargo(args: &[&str]) -> Result<()> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned());
    let status = Command::new(cargo).args(args).status()?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("`cargo {}` endete mit {status}", args.join(" ")).into())
    }
}

/// Prüft jede Workspace-Crate gegen [`LAYERS`]: Sie muss eingetragen sein, die
/// richtige Lizenz tragen und darf nur erlaubte interne Crates nutzen.
fn layers() -> Result<()> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned());
    let output = Command::new(cargo)
        .args(["metadata", "--format-version", "1", "--no-deps", "--locked"])
        .output()?;
    if !output.status.success() {
        return Err(format!(
            "cargo metadata fehlgeschlagen: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }
    let metadata: serde_json::Value = serde_json::from_slice(&output.stdout)?;
    let (checked, problems) = check_layers(&metadata)?;
    if problems.is_empty() {
        println!("{checked} Crates geprüft, keine Verstöße");
        Ok(())
    } else {
        for problem in &problems {
            eprintln!("  - {problem}");
        }
        Err(format!("{} Verstöße gegen die Schichtenregeln", problems.len()).into())
    }
}

/// Wendet [`LAYERS`] auf die Ausgabe von `cargo metadata` an und liefert die
/// Zahl der geprüften Crates und alle Verstöße.
fn check_layers(metadata: &serde_json::Value) -> Result<(usize, Vec<String>)> {
    let packages = metadata
        .get("packages")
        .and_then(|p| p.as_array())
        .ok_or("cargo metadata ohne packages")?;

    let mut problems = Vec::new();

    for package in packages {
        let name = str_field(package, "name")?;
        let Some(layer) = LAYERS.iter().find(|l| l.name == name) else {
            problems.push(format!(
                "{name}: nicht in LAYERS (xtask/src/main.rs) eingetragen"
            ));
            continue;
        };

        let license = package
            .get("license")
            .and_then(|l| l.as_str())
            .unwrap_or("");
        if license != layer.license {
            problems.push(format!(
                "{name}: Lizenz ist „{license}“, erwartet „{}“",
                layer.license
            ));
        }

        let deps = package
            .get("dependencies")
            .and_then(|d| d.as_array())
            .ok_or("Paket ohne dependencies")?;
        for dep in deps {
            let dep_name = str_field(dep, "name")?;
            let is_internal =
                LAYERS.iter().any(|l| l.name == dep_name) || dep_name.starts_with("ofd-");
            if is_internal && !layer.allowed.contains(&dep_name) {
                problems.push(format!(
                    "{name} darf nicht von {dep_name} abhängen (erlaubt: {})",
                    if layer.allowed.is_empty() {
                        "keine internen Crates".to_owned()
                    } else {
                        layer.allowed.join(", ")
                    }
                ));
            }
        }
    }

    Ok((packages.len(), problems))
}

fn str_field<'a>(value: &'a serde_json::Value, key: &str) -> Result<&'a str> {
    value
        .get(key)
        .and_then(|v| v.as_str())
        .ok_or_else(|| format!("Feld {key} fehlt in cargo metadata").into())
}

#[cfg(test)]
mod tests {
    use super::{LAYERS, check_layers};
    use serde_json::json;

    fn package(name: &str, license: &str, deps: &[&str]) -> serde_json::Value {
        let deps: Vec<_> = deps.iter().map(|d| json!({ "name": d })).collect();
        json!({ "name": name, "license": license, "dependencies": deps })
    }

    /// INFRA-001: erlaubte Abhängigkeiten gehen durch.
    #[test]
    fn infra_001_allowed_dependencies_pass() {
        let metadata = json!({ "packages": [
            package("ofd-units", "MIT OR Apache-2.0", &[]),
            package("ofd-geo", "MIT OR Apache-2.0", &["ofd-units", "serde"]),
            package("ofd-core", "MPL-2.0", &["ofd-geo", "ofd-units"]),
        ]});
        let (checked, problems) = check_layers(&metadata).unwrap();
        assert_eq!(checked, 3);
        assert!(problems.is_empty(), "{problems:?}");
    }

    /// INFRA-001: eine Abhängigkeit gegen die Schichtung wird gemeldet.
    #[test]
    fn infra_001_forbidden_dependency_fails() {
        let metadata = json!({ "packages": [
            package("ofd-units", "MIT OR Apache-2.0", &["ofd-geo"]),
        ]});
        let (_, problems) = check_layers(&metadata).unwrap();
        assert_eq!(problems.len(), 1);
        assert!(problems[0].contains("ofd-units darf nicht von ofd-geo"));
    }

    /// INFRA-001: eine nicht eingetragene Crate wird gemeldet.
    #[test]
    fn infra_001_unknown_crate_fails() {
        let metadata = json!({ "packages": [ package("ofd-neu", "MPL-2.0", &[]) ]});
        let (_, problems) = check_layers(&metadata).unwrap();
        assert_eq!(problems.len(), 1);
        assert!(problems[0].contains("nicht in LAYERS"));
    }

    /// INFRA-002: falsche Lizenz einer Crate wird gemeldet.
    #[test]
    fn infra_002_wrong_license_fails() {
        let metadata = json!({ "packages": [ package("ofd-geo", "MPL-2.0", &[]) ]});
        let (_, problems) = check_layers(&metadata).unwrap();
        assert_eq!(problems.len(), 1);
        assert!(problems[0].contains("Lizenz"));
    }

    /// INFRA-002: keine Bibliothek darf laut LAYERS eine MPL-Crate nutzen.
    #[test]
    fn infra_002_libraries_never_depend_on_mpl_crates() {
        for layer in LAYERS.iter().filter(|l| l.license == super::MIT_APACHE) {
            for dep in layer.allowed {
                let target = LAYERS.iter().find(|l| l.name == *dep).unwrap();
                assert_eq!(
                    target.license,
                    super::MIT_APACHE,
                    "{} (MIT/Apache) darf {} (MPL) nicht nutzen",
                    layer.name,
                    dep
                );
            }
        }
    }

    /// INFRA-001: LAYERS erlaubt nur Crates, die selbst eingetragen sind.
    #[test]
    fn infra_001_allowed_dependencies_are_known_layers() {
        for layer in LAYERS {
            for dep in layer.allowed {
                assert!(
                    LAYERS.iter().any(|l| l.name == *dep),
                    "{} erlaubt unbekannte Crate {}",
                    layer.name,
                    dep
                );
            }
        }
    }
}
