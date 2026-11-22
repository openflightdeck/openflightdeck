//! UniFFI-Bindgen als Werkzeug im Workspace, damit die Version zur
//! UniFFI-Version von `ofd-ffi` passt. Aufruf über `cargo xtask ios`.

fn main() {
    uniffi::uniffi_bindgen_main()
}
