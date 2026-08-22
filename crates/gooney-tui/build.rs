// crates/gooney-tui/build.rs
use std::env;
use std::path::PathBuf;

fn main() {
    // 1. Allow overriding via an environment variable, 
    // or fallback to looking relative to the home directory.
    let obj_dir = if let Ok(custom_path) = env::var("GOONEY_HDL_OBJ_DIR") {
        PathBuf::from(custom_path)
    } else {
        let mut path = home::home_dir().expect("Failed to find home directory");
        path.push("gooneymart_xzxt_hdl");
        path.push("sim");
        path.push("obj_dir");
        path
    };

    let obj_dir_str = obj_dir.to_str().expect("Invalid path string");

    // Tell Cargo where to search for the compiled Verilator archives
    println!("cargo:rustc-link-search=native={}", obj_dir_str);

    // Link the compiled Verilator design and runtime statically
    println!("cargo:rustc-link-lib=static=Vgooneymart_xzxt_hdl_xzxt_tb_top");
    println!("cargo:rustc-link-lib=static=verilated");

    // Link C++ standard library (required for Verilator C++ runtime)
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=c++");
    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-lib=stdc++");

    // Re-run build if the obj_dir changes
    println!("cargo:rerun-if-changed={}", obj_dir_str);
}
