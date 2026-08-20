// crates/gooney-tui/build.rs
fn main() {
    let obj_dir = "/home/ideacentre/gooneymart_xzxt_hdl/sim/obj_dir";
    
    // Tell Cargo where to search for the compiled Verilator archives
    println!("cargo:rustc-link-search=native={}", obj_dir);
    
    // Link the compiled Verilator design and runtime statically
    println!("cargo:rustc-link-lib=static=Vgooneymart_xzxt_hdl_xzxt_tb_top");
    println!("cargo:rustc-link-lib=static=verilated");
    
    // Link C++ standard library (required for Verilator C++ runtime)
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=c++");
    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-lib=stdc++");

    // Re-run build if the obj_dir changes
    println!("cargo:rerun-if-changed={}", obj_dir);
}
