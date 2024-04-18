fn main() {
    println!("cargo:rustc-cfg=dev_mode");
    tauri_build::build()
}
