//! Links the host's JS library into the emscripten build, see `src/bin/emscripten_io.rs`.

const JS_LIBRARY: &str = "WGSL_ANALYZER_JS_LIBRARY";

fn main() {
    println!("cargo::rerun-if-env-changed={JS_LIBRARY}");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("emscripten") {
        return;
    }
    match std::env::var(JS_LIBRARY) {
        Ok(library) => {
            println!("cargo::rerun-if-changed={library}");
            println!("cargo::rustc-link-arg-bin=wgsl-analyzer=--js-library={library}");
        },
        Err(_) => println!("cargo::warning={JS_LIBRARY} is not set."),
    }
}
