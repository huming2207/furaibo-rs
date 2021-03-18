#![no_std]
#![macro_use]
mod furaibo;

use furaibo::ui;



#[no_mangle]
pub extern fn start() {
    ui::set_info(f_str!("Hello Rust"), f_str!("WASM on ESP32"));
}

#[panic_handler]
fn handle_panic(_: &core::panic::PanicInfo) -> ! {
    unsafe { core::arch::wasm32::unreachable() }
}