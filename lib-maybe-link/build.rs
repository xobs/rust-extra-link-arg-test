fn main() {
    if cfg!(feature = "linker-script") {
        println!("cargo:rustc-link-arg=--passing-args-does-work");
    }
}
