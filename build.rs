fn main() {
    // Tell Cargo to recompile if theme files change
    println!("cargo:rerun-if-changed=assets/themes/dark.json");
    println!("cargo:rerun-if-changed=assets/themes/light.json");
    println!("cargo:rerun-if-changed=assets/themes/dark_original.json");
    println!("cargo:rerun-if-changed=assets/themes/light_original.json");
    println!("cargo:rerun-if-changed=assets/themes/ascii.json");
}