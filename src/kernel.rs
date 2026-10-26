static mut FB_ADDR: *mut u32 = 0 as *mut u32;
static mut SCREEN_W: u32 = 800;
static mut SCREEN_H: u32 = 600;

pub unsafe fn draw_pixel(x: u32, y: u32, color: u32) {
    if FB_ADDR.is_null() || x >= SCREEN_W || y >= SCREEN_H {
        return;
    }
    let offset = (y * SCREEN_W + x) as isize;
    FB_ADDR.offset(offset).write_volatile(color);
}

pub unsafe fn clear_screen(color: u32) {
    for y in 0..SCREEN_H {
        for x in 0..SCREEN_W {
            draw_pixel(x, y, color);
        }
    }
}

pub unsafe fn init(fb: *mut u32, width: u32, height: u32) {
    FB_ADDR = fb;
    SCREEN_W = width;
    SCREEN_H = height;

    clear_screen(0x1A1A1A);
}
