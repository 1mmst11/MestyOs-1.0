#![no_std]
#![no_main]

mod loader;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loader::BootLoader::show_splash_screen();

    let vga_buffer = 0xb8000 as *mut u8;
    let message = b"Welcome to MestyOS!";

    for (i, &byte) in message.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0x0F;
        }
    }

    loop {}
}
