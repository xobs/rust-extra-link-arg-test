#![no_std]
#![no_main]

#[panic_handler]
fn handle_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

pub fn not_main() -> bool {
    lib_maybe_link::linker_script_enabled()
}
