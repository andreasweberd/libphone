use std::env;
use std::fs;
use std::path::{absolute, PathBuf};

fn main() {
    // Sucht das richtige Verzeichnis, in dem die Bibliothek liegt
    // `libphone.so.0` sollte entweder auf dem Systempfad liegen oder lokal bereitgestellt werden.
    let lib_dir = env::var("LIBPHONE_LIB_DIR").unwrap_or_else(|_| "lib".to_string()); // Standard: "./lib"

    // Fügt das Verzeichnis, das `libphone.so.0` enthält, dem Linker hinzu
    println!("cargo:rustc-link-search=native={}", lib_dir);

    // Verweist den Linker darauf, 'phone' mit der Bibliothek zu verlinken
    // (verlinkt libphone.so.0 --> `phone` angeben, ohne Präfix und Suffix).
    println!("cargo:rustc-link-lib=phone");

    // Optionale Überprüfung, ob die Bibliothek existiert
    let lib_path = PathBuf::from(&lib_dir).join("libphone.so.0");
    if !lib_path.exists() {
        panic!(
            "Library libphone.so.0 not found in {}. Ensure it exists or set LIBPHONE_LIB_DIR.",
            absolute(&lib_path).unwrap().display()
        );
    }
}