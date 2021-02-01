fn main() {
    if cfg!(feature = "linker-script") {
        println!("cargo:rustc-link-arg=-Tnonexistent-script.ld");
    }
}
