use std::{error::Error, path::Path, process::Command};

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

const UACPI_GITHUB_URL: &str = "https://github.com/UltraOS/uACPI";

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let uacpi_path = Path::new(&out_dir).join("bundled").join("uacpi");
    let uacpi_path_str = uacpi_path.to_string_lossy();

    if !uacpi_path.exists() {
        Command::new("git")
            .args(&["clone", UACPI_GITHUB_URL, &uacpi_path_str])
            .status()
            .unwrap();
    }

    let sources = SOURCES
        .iter()
        .map(|file| format!("{uacpi_path_str}/{file}"))
        .collect::<Vec<_>>();

    cc::Build::new()
        .files(sources)
        .include(format!("{uacpi_path_str}/include"))
        .flag("-fno-stack-protector")
        .flag("-mno-sse")
        .flag("-mno-mmx")
        .flag("-msoft-float")
        .flag("-mno-red-zone")
        .flag("-fno-builtin")
        .flag("-nostdlib")
        .flag("-ffreestanding")
        .compile("uacpi");

    Ok(())
}
