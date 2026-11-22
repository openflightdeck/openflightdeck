//! App-Zustand, Commands und Ereignisse an die Oberfläche.
//!
//! Jede Nutzeraktion ist ein Command mit ID, Parametern und Test. SwiftUI,
//! CLI und ein späterer MCP-Kanal rufen dieselben Commands auf.

/// Version des Kerns, wie sie in der App und im CLI angezeigt wird.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::VERSION;

    #[test]
    fn version_is_set() {
        assert!(!VERSION.is_empty());
    }
}
