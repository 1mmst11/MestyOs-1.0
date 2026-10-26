#![no_std]
#![no_main]

mod desktop;
mod loader;

use crate::loader::BootLoader;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn _start(multiboot_info_ptr: *const u32) -> ! {
    BootLoader::show_splash_screen();

    let mut fb_address = 0xA000_0000 as *mut u32;

    if !multiboot_info_ptr.is_null() {
        let flags = *multiboot_info_ptr;
        if (flags & (1 << 11)) != 0 {
            let fb_addr_ptr = *multiboot_info_ptr.offset(22) as *mut u32;
            if !fb_addr_ptr.is_null() {
                fb_address = fb_addr_ptr;
            }
        }
    }

    desktop::init_desktop_800x600(fb_address);

    loop {}
}
