use std::{
    env,
    error::Error,
    path::{Path, PathBuf},
};

const SOURCES: &[&str] = &[
    "source/tables.c",
    "source/types.c",
    "source/uacpi.c",
    "source/utilities.c",
    "source/interpreter.c",
    "source/opcodes.c",
    "source/namespace.c",
    "source/stdlib.c",
    "source/shareable.c",
    "source/opregion.c",
    "source/default_handlers.c",
    "source/io.c",
    "source/notify.c",
    "source/sleep.c",
    "source/registers.c",
    "source/resources.c",
    "source/event.c",
];

fn main() -> Result<(), Box<dyn Error>> {
    let project_dir = std::env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let uacpi_path = Path::new(&project_dir).join("vendor");
    let uacpi_path_str = uacpi_path.to_string_lossy();

    let sources = SOURCES
        .iter()
        .map(|file| format!("{uacpi_path_str}/{file}"))
        .collect::<Vec<_>>();

    let mut cc = cc::Build::new();

    cc.files(sources)
        .include(format!("{uacpi_path_str}/include"))
        .define("UACPI_SIZED_FREES", "1")
        .flag("-fno-stack-protector")
        .flag("-mno-sse")
        .flag("-mno-mmx")
        .flag("-msoft-float")
        .flag("-mno-red-zone")
        .flag("-fno-builtin")
        .flag("-nostdlib")
        .flag("-ffreestanding");

    if cfg!(feature = "reduced-hardware") {
        cc.define("UACPI_REDUCED_HARDWARE", "1");
    }

    cc.compile("uacpi");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .clang_args(&[
            "-Ivendor/include",
            "-DUACPI_SIZED_FREES=1",
            #[cfg(feature = "reduced-hardware")]
            "-DUACPI_REDUCED_HARDWARE=1",
            "-ffreestanding",
        ])
        .use_core()
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    Ok(())
}
