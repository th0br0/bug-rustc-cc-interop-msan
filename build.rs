use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=src/callee.cc");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let obj_path = out_dir.join("callee.o");
    let lib_path = out_dir.join("libcallee.a");

    let status = Command::new("clang++")
        .args([
            "-c",
            "src/callee.cc",
            "-o",
            obj_path.to_str().unwrap(),
            "-fsanitize=memory",
            "-O2",
            "-g",
        ])
        .status()
        .expect("Failed to execute clang++");
    assert!(status.success(), "clang++ compilation of src/callee.cc failed");

    let status = Command::new("ar")
        .args(["crus", lib_path.to_str().unwrap(), obj_path.to_str().unwrap()])
        .status()
        .expect("Failed to execute ar");
    assert!(status.success(), "ar archiving failed");

    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=callee");
}
