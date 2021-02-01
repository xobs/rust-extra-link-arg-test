#![no_std]

pub fn linker_script_enabled() -> bool {
    cfg!(feature = "linker-script")
}
