fn main() {
    // Tell Cargo to recompile if theme files change
    println!("cargo:rerun-if-changed=assets/themes/default.json");
    println!("cargo:rerun-if-changed=assets/themes/original.json");
    println!("cargo:rerun-if-changed=assets/themes/ascii.json");
    println!("cargo:rerun-if-changed=assets/themes/neon_abyss.json");
    println!("cargo:rerun-if-changed=assets/themes/inferno.json");
    println!("cargo:rerun-if-changed=assets/themes/eclipse.json");
    println!("cargo:rerun-if-changed=assets/themes/glacier.json");
    println!("cargo:rerun-if-changed=assets/themes/blood_oath.json");
    println!("cargo:rerun-if-changed=assets/themes/oblivion.json");
    println!("cargo:rerun-if-changed=assets/themes/spectral.json");
    println!("cargo:rerun-if-changed=assets/themes/venom.json");
    println!("cargo:rerun-if-changed=assets/themes/aurora.json");
    println!("cargo:rerun-if-changed=assets/themes/cyber_void.json");

    // Tell Cargo to recompile if language color files change
    println!("cargo:rerun-if-changed=assets/languages/lang_dark.json");
    println!("cargo:rerun-if-changed=assets/languages/lang_light.json");
    println!("cargo:rerun-if-changed=assets/languages/lang_ascii.json");
}