use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changes=build.rs");

    let target = env::var("TARGET").expect("valid TARGET defined");
    if target.contains("pc-windows-msvc") {
        let manifest_dir = PathBuf::from(
            env::var("CARGO_MANIFEST_DIR").expect("valid CARGO_MANIFEST_DIR defined"),
        );
        let out_dir = PathBuf::from(env::var("OUT_DIR").expect("valid CARGO_OUT_DIR defined"));

        let architecture = if target.contains("x86_64") {
            "64"
        } else {
            "32"
        };
        let lib_dir = manifest_dir
            .join("lib")
            .join("msvc")
            .join("lib")
            .join(architecture);
        let dll_dir = manifest_dir
            .join("lib")
            .join("msvc")
            .join("dll")
            .join(architecture);

        println!("cargo:rustc-link-search=native={}", out_dir.display());

        for dir in [&lib_dir, &dll_dir] {
            copy_files_from_dir_to(dir, &out_dir);
        }
    }
}

fn copy_files_from_dir_to(src: &PathBuf, dest: &PathBuf) {
    for entry in fs::read_dir(src)
        .unwrap_or_else(|err| panic!("can't read directory: {}, {}", src.display(), err))
    {
        let entry_path = entry.expect("invalid fs entry").path();
        if let Some(file_name) = entry_path.file_name().and_then(|name| name.to_str()) {
            let dest_path = dest.join(file_name);
            fs::copy(&entry_path, &dest_path).unwrap_or_else(|err| {
                panic!(
                    "can't copy from: {} to {}, {}",
                    entry_path.display(),
                    dest_path.display(),
                    err
                )
            });
        }
    }
}
