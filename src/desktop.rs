#![no_std]
#![allow(dead_code)]
#![allow(static_mut_refs)]
#![allow(unsafe_op_in_unsafe_fn)]

// =====================================================================
//  Рабочий стол в стиле KDE Plasma (светлая тема Breeze)
//  - градиентные обои с «лентами» как у Plasma 6
//  - плавающая панель задач со скруглением и тенью
//  - меню Пуск с единственной кнопкой питания
//  - диалог «Power Options»: Shut Down / Restart
//  - клик срабатывает по отпусканию кнопки, есть «нажатое» состояние
//  - сглаженные скругления без f32 (только целые числа)
//  - курсор поверх кадра с сохранением фона
//  - перерисовка только изменившихся областей (clip-rect)
// =====================================================================

// ---------------------------------------------------------------------
//  Шрифт 8x8. Бит 7 = левый пиксель (MSB слева), рабочая ширина 6 колонок
// ---------------------------------------------------------------------
pub const BASIC_LEGACY: [[u8; 8]; 128] = {
    let mut f = [[0u8; 8]; 128];

    // Цифры
    f[b'0' as usize] = [0x3C, 0x42, 0x46, 0x4A, 0x52, 0x62, 0x3C, 0x00];
    f[b'1' as usize] = [0x18, 0x38, 0x18, 0x18, 0x18, 0x18, 0x3C, 0x00];
    f[b'2' as usize] = [0x3C, 0x42, 0x02, 0x0C, 0x30, 0x40, 0x7E, 0x00];
    f[b'3' as usize] = [0x3C, 0x42, 0x02, 0x1C, 0x02, 0x42, 0x3C, 0x00];
    f[b'4' as usize] = [0x0C, 0x14, 0x24, 0x44, 0x7E, 0x04, 0x04, 0x00];
    f[b'5' as usize] = [0x7E, 0x40, 0x7C, 0x02, 0x02, 0x42, 0x3C, 0x00];
    f[b'6' as usize] = [0x1C, 0x20, 0x40, 0x7C, 0x42, 0x42, 0x3C, 0x00];
    f[b'7' as usize] = [0x7E, 0x02, 0x04, 0x08, 0x10, 0x10, 0x10, 0x00];
    f[b'8' as usize] = [0x3C, 0x42, 0x42, 0x3C, 0x42, 0x42, 0x3C, 0x00];
    f[b'9' as usize] = [0x3C, 0x42, 0x42, 0x3E, 0x02, 0x04, 0x38, 0x00];

    // Заглавные
    f[b'A' as usize] = [0x18, 0x24, 0x42, 0x7E, 0x42, 0x42, 0x42, 0x00];
    f[b'B' as usize] = [0x7C, 0x42, 0x42, 0x7C, 0x42, 0x42, 0x7C, 0x00];
    f[b'C' as usize] = [0x3C, 0x42, 0x40, 0x40, 0x40, 0x42, 0x3C, 0x00];
    f[b'D' as usize] = [0x7C, 0x42, 0x42, 0x42, 0x42, 0x42, 0x7C, 0x00];
    f[b'E' as usize] = [0x7E, 0x40, 0x40, 0x7C, 0x40, 0x40, 0x7E, 0x00];
    f[b'F' as usize] = [0x7E, 0x40, 0x40, 0x7C, 0x40, 0x40, 0x40, 0x00];
    f[b'G' as usize] = [0x3C, 0x42, 0x40, 0x4E, 0x42, 0x42, 0x3C, 0x00];
    f[b'H' as usize] = [0x42, 0x42, 0x42, 0x7E, 0x42, 0x42, 0x42, 0x00];
    f[b'I' as usize] = [0x3C, 0x18, 0x18, 0x18, 0x18, 0x18, 0x3C, 0x00];
    f[b'J' as usize] = [0x1E, 0x04, 0x04, 0x04, 0x04, 0x44, 0x38, 0x00];
    f[b'K' as usize] = [0x42, 0x44, 0x48, 0x70, 0x48, 0x44, 0x42, 0x00];
    f[b'L' as usize] = [0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x7E, 0x00];
    f[b'M' as usize] = [0x42, 0x66, 0x5A, 0x5A, 0x42, 0x42, 0x42, 0x00];
    f[b'N' as usize] = [0x42, 0x62, 0x52, 0x4A, 0x46, 0x42, 0x42, 0x00];
    f[b'O' as usize] = [0x3C, 0x42, 0x42, 0x42, 0x42, 0x42, 0x3C, 0x00];
    f[b'P' as usize] = [0x7C, 0x42, 0x42, 0x7C, 0x40, 0x40, 0x40, 0x00];
    f[b'Q' as usize] = [0x3C, 0x42, 0x42, 0x42, 0x4A, 0x44, 0x3A, 0x00];
    f[b'R' as usize] = [0x7C, 0x42, 0x42, 0x7C, 0x48, 0x44, 0x42, 0x00];
    f[b'S' as usize] = [0x3C, 0x42, 0x40, 0x3C, 0x02, 0x42, 0x3C, 0x00];
    f[b'T' as usize] = [0x7E, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x00];
    f[b'U' as usize] = [0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x3C, 0x00];
    f[b'V' as usize] = [0x42, 0x42, 0x42, 0x42, 0x24, 0x24, 0x18, 0x00];
    f[b'W' as usize] = [0x42, 0x42, 0x42, 0x5A, 0x5A, 0x66, 0x42, 0x00];
    f[b'X' as usize] = [0x42, 0x24, 0x18, 0x18, 0x18, 0x24, 0x42, 0x00];
    f[b'Y' as usize] = [0x42, 0x42, 0x24, 0x18, 0x18, 0x18, 0x18, 0x00];
    f[b'Z' as usize] = [0x7E, 0x04, 0x08, 0x10, 0x20, 0x40, 0x7E, 0x00];

    // Строчные
    f[b'a' as usize] = [0x00, 0x00, 0x3C, 0x02, 0x3E, 0x42, 0x3E, 0x00];
    f[b'b' as usize] = [0x40, 0x40, 0x7C, 0x42, 0x42, 0x42, 0x7C, 0x00];
    f[b'c' as usize] = [0x00, 0x00, 0x3C, 0x40, 0x40, 0x40, 0x3C, 0x00];
    f[b'd' as usize] = [0x02, 0x02, 0x3E, 0x42, 0x42, 0x42, 0x3E, 0x00];
    f[b'e' as usize] = [0x00, 0x00, 0x3C, 0x42, 0x7E, 0x40, 0x3C, 0x00];
    f[b'f' as usize] = [0x0C, 0x10, 0x10, 0x7C, 0x10, 0x10, 0x10, 0x00];
    f[b'g' as usize] = [0x00, 0x00, 0x3E, 0x42, 0x42, 0x3E, 0x02, 0x3C];
    f[b'h' as usize] = [0x40, 0x40, 0x7C, 0x42, 0x42, 0x42, 0x42, 0x00];
    f[b'i' as usize] = [0x18, 0x00, 0x38, 0x18, 0x18, 0x18, 0x3C, 0x00];
    f[b'j' as usize] = [0x04, 0x00, 0x0C, 0x04, 0x04, 0x04, 0x44, 0x38];
    f[b'k' as usize] = [0x40, 0x40, 0x44, 0x48, 0x70, 0x48, 0x44, 0x00];
    f[b'l' as usize] = [0x38, 0x18, 0x18, 0x18, 0x18, 0x18, 0x3C, 0x00];
    f[b'm' as usize] = [0x00, 0x00, 0x76, 0x5A, 0x5A, 0x5A, 0x5A, 0x00];
    f[b'n' as usize] = [0x00, 0x00, 0x7C, 0x42, 0x42, 0x42, 0x42, 0x00];
    f[b'o' as usize] = [0x00, 0x00, 0x3C, 0x42, 0x42, 0x42, 0x3C, 0x00];
    f[b'p' as usize] = [0x00, 0x00, 0x7C, 0x42, 0x42, 0x7C, 0x40, 0x40];
    f[b'q' as usize] = [0x00, 0x00, 0x3E, 0x42, 0x42, 0x3E, 0x02, 0x02];
    f[b'r' as usize] = [0x00, 0x00, 0x5C, 0x62, 0x40, 0x40, 0x40, 0x00];
    f[b's' as usize] = [0x00, 0x00, 0x3C, 0x40, 0x3C, 0x02, 0x3C, 0x00];
    f[b't' as usize] = [0x20, 0x20, 0x7C, 0x20, 0x20, 0x22, 0x1C, 0x00];
    f[b'u' as usize] = [0x00, 0x00, 0x42, 0x42, 0x42, 0x42, 0x3E, 0x00];
    f[b'v' as usize] = [0x00, 0x00, 0x42, 0x42, 0x42, 0x24, 0x18, 0x00];
    f[b'w' as usize] = [0x00, 0x00, 0x42, 0x42, 0x5A, 0x5A, 0x24, 0x00];
    f[b'x' as usize] = [0x00, 0x00, 0x42, 0x24, 0x18, 0x24, 0x42, 0x00];
    f[b'y' as usize] = [0x00, 0x00, 0x42, 0x42, 0x42, 0x3E, 0x02, 0x3C];
    f[b'z' as usize] = [0x00, 0x00, 0x7E, 0x04, 0x18, 0x20, 0x7E, 0x00];

    // Знаки
    f[b'.' as usize] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x18, 0x00];
    f[b':' as usize] = [0x00, 0x18, 0x18, 0x00, 0x00, 0x18, 0x18, 0x00];
    f[b'-' as usize] = [0x00, 0x00, 0x00, 0x7E, 0x00, 0x00, 0x00, 0x00];
    f[b'/' as usize] = [0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x00, 0x00];
    f[b' ' as usize] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    f
};

static FONT: [[u8; 8]; 128] = BASIC_LEGACY;

// ---------------------------------------------------------------------
//  Драйвер мыши на C и управление питанием
// ---------------------------------------------------------------------
extern "C" {
    fn init_mouse();
    fn update_mouse_c() -> i32;
    fn get_mouse_x() -> i32;
    fn get_mouse_y() -> i32;
    fn get_mouse_buttons() -> i32;
}

#[inline(always)]
unsafe fn outb(port: u16, value: u8) {
    core::arch::asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack, preserves_flags));
}

#[inline(always)]
unsafe fn outw(port: u16, value: u16) {
    core::arch::asm!("out dx, ax", in("dx") port, in("ax") value, options(nomem, nostack, preserves_flags));
}

#[inline(always)]
unsafe fn inb(port: u16) -> u8 {
    let v: u8;
    core::arch::asm!("in al, dx", out("al") v, in("dx") port, options(nomem, nostack, preserves_flags));
    v
}

/// Пустой дескриптор IDT: загрузка + int3 вызывает triple fault (аппаратный сброс)
static NULL_IDT: [u8; 10] = [0; 10];

/// Перезагрузка: контроллер клавиатуры, при неудаче — triple fault
unsafe fn system_reboot() -> ! {
    core::arch::asm!("cli", options(nomem, nostack));

    // Ждём, пока входной буфер контроллера освободится
    let mut n = 0u32;
    while n < 100_000 {
        if inb(0x64) & 0x02 == 0 {
            break;
        }
        n += 1;
    }
    outb(0x64, 0xFE);

    // Небольшая пауза, чтобы сброс успел сработать
    for _ in 0..2_000_000u32 {
        core::hint::spin_loop();
    }

    // Запасной вариант: triple fault
    core::arch::asm!("lidt [{0}]", "int3", in(reg) NULL_IDT.as_ptr(), options(nostack));

    loop {
        core::arch::asm!("hlt", options(nomem, nostack, preserves_flags));
    }
}

/// Выключение: QEMU, Bochs и VirtualBox
unsafe fn system_shutdown() -> ! {
    outw(0x604, 0x2000);
    outw(0xB004, 0x2000);
    outw(0x4004, 0x3400);
    loop {
        core::arch::asm!("cli; hlt", options(nomem, nostack));
    }
}

// ---------------------------------------------------------------------
//  Геометрия экрана и раскладка интерфейса
// ---------------------------------------------------------------------
const SCREEN_W: i32 = 800;
const SCREEN_H: i32 = 600;
const STRIDE: usize = 800;

#[derive(Clone, Copy)]
struct Rect {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

impl Rect {
    const fn new(x: i32, y: i32, w: i32, h: i32) -> Rect {
        Rect { x, y, w, h }
    }

    fn contains(&self, px: i32, py: i32) -> bool {
        px >= self.x && px < self.x + self.w && py >= self.y && py < self.y + self.h
    }

    fn inflate(&self, d: i32) -> Rect {
        Rect::new(self.x - d, self.y - d, self.w + d * 2, self.h + d * 2)
    }

    fn union(&self, o: &Rect) -> Rect {
        let x0 = self.x.min(o.x);
        let y0 = self.y.min(o.y);
        let x1 = (self.x + self.w).max(o.x + o.w);
        let y1 = (self.y + self.h).max(o.y + o.h);
        Rect::new(x0, y0, x1 - x0, y1 - y0)
    }
}

fn merge(a: Option<Rect>, b: Option<Rect>) -> Option<Rect> {
    match (a, b) {
        (Some(x), Some(y)) => Some(x.union(&y)),
        (Some(x), None) => Some(x),
        (None, y) => y,
    }
}

const SCREEN: Rect = Rect::new(0, 0, SCREEN_W, SCREEN_H);

// Панель задач
const PANEL: Rect = Rect::new(10, SCREEN_H - 54, SCREEN_W - 20, 44);
const START_BTN: Rect = Rect::new(PANEL.x + 8, PANEL.y + 6, 40, 32);

// Меню Пуск (только кнопка питания)
const MENU: Rect = Rect::new(PANEL.x, PANEL.y - 10 - 76, 244, 76);
const POWER_BTN: Rect = Rect::new(MENU.x + 12, MENU.y + 12, MENU.w - 24, MENU.h - 24);

// Область, которую надо обновить при открытии/закрытии меню (меню + тень + панель)
const MENU_REGION: Rect = Rect::new(
    MENU.x - 20,
    MENU.y - 20,
    MENU.w + 40,
    SCREEN_H - (MENU.y - 20),
);

// Диалог выбора: выключить или перезагрузить
const DIALOG: Rect = Rect::new((SCREEN_W - 380) / 2, (SCREEN_H - 210) / 2 - 20, 380, 210);
const SHUT_BTN: Rect = Rect::new(DIALOG.x + 24, DIALOG.y + 82, 158, 92);
const REST_BTN: Rect = Rect::new(DIALOG.x + 198, DIALOG.y + 82, 158, 92);
const CLOSE_BTN: Rect = Rect::new(DIALOG.x + DIALOG.w - 38, DIALOG.y + 14, 26, 26);

// Идентификаторы элементов под курсором
const H_NONE: u8 = 0;
const H_START: u8 = 1;
const H_POWER: u8 = 2;
const H_SHUTDOWN: u8 = 3;
const H_RESTART: u8 = 4;
const H_CLOSE: u8 = 5;

// ---------------------------------------------------------------------
//  Палитра (Breeze Light)
// ---------------------------------------------------------------------
const SKY_TOP: u32 = 0x8DB7E2;
const SKY_MID: u32 = 0xE8C5CC;
const SKY_BOT: u32 = 0xD3E3F4;

const PANEL_BG: u32 = 0xEEF2F7;
const PANEL_BORDER: u32 = 0xCBD3DD;
const MENU_BG: u32 = 0xF6F8FB;
const MENU_BORDER: u32 = 0xCDD5DF;
const CARD_BG: u32 = 0xFFFFFF;
const CARD_BORDER: u32 = 0xDCE2E9;
const DIVIDER: u32 = 0xE0E5EB;

const TEXT_DARK: u32 = 0x1F2933;
const TEXT_MUTED: u32 = 0x66707B;
const ICON_GRAY: u32 = 0x4B5560;

const ACCENT: u32 = 0x3DAEE9;
const ACCENT_SOFT: u32 = 0xDDF0FB;
const DANGER: u32 = 0xDA4453;
const DANGER_SOFT: u32 = 0xFBE3E6;
const HOVER_BG: u32 = 0xDDE6F0;

const SHADOW_COLOR: u32 = 0x0E1A2B;
const DIM_COLOR: u32 = 0x0B1624;
const DIM_ALPHA: i32 = 110;

// ---------------------------------------------------------------------
//  Глобальное состояние
// ---------------------------------------------------------------------
static mut FB_ADDR: *mut u32 = core::ptr::null_mut();

// Прямоугольник отсечения: всё рисование ограничено им
static mut CLIP_X0: i32 = 0;
static mut CLIP_Y0: i32 = 0;
static mut CLIP_X1: i32 = SCREEN_W;
static mut CLIP_Y1: i32 = SCREEN_H;

static mut MENU_OPEN: bool = false;
static mut DIALOG_OPEN: bool = false;
static mut PREV_CLICK: bool = false;
static mut HOVER: u8 = H_NONE;

// Состояние нажатия: элемент, на котором нажали кнопку мыши
static mut PRESS_DOWN: bool = false;
static mut PRESS_ID: u8 = H_NONE;

// Курсор-стрелка 12x18: 1 = контур, 2 = заливка
const CURSOR_W: i32 = 12;
const CURSOR_H: i32 = 18;
static CURSOR_PIXELS: [[u8; 12]; 18] = [
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 2, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 2, 2, 2, 1, 0, 0, 0, 0, 0, 0, 0],
    [1, 2, 2, 2, 2, 1, 0, 0, 0, 0, 0, 0],
    [1, 2, 2, 2, 2, 2, 1, 0, 0, 0, 0, 0],
    [1, 2, 2, 2, 2, 2, 2, 1, 0, 0, 0, 0],
    [1, 2, 2, 2, 2, 2, 2, 2, 1, 0, 0, 0],
    [1, 2, 2, 2, 2, 2, 2, 2, 2, 1, 0, 0],
    [1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 0],
    [1, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1],
    [1, 2, 2, 2, 1, 2, 2, 1, 0, 0, 0, 0],
    [1, 2, 2, 1, 0, 1, 2, 2, 1, 0, 0, 0],
    [1, 2, 1, 0, 0, 1, 2, 2, 1, 0, 0, 0],
    [1, 1, 0, 0, 0, 0, 1, 2, 2, 1, 0, 0],
    [1, 0, 0, 0, 0, 0, 1, 2, 2, 1, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0],
];

static mut CUR_SAVE: [u32; (CURSOR_W * CURSOR_H) as usize] = [0; (CURSOR_W * CURSOR_H) as usize];
static mut CUR_X: i32 = 0;
static mut CUR_Y: i32 = 0;
static mut CUR_VISIBLE: bool = false;

// ---------------------------------------------------------------------
//  Цвет и математика (только целые числа, без f32)
// ---------------------------------------------------------------------
fn isqrt(n: i32) -> i32 {
    if n <= 0 {
        return 0;
    }
    let mut x = n;
    let mut y = (x + 1) / 2;
    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }
    x
}

/// Смешивание двух цветов: t = 0 даёт a, t = 255 даёт b
fn mix(a: u32, b: u32, t: i32) -> u32 {
    let t = t.clamp(0, 255);
    let it = 255 - t;
    let r = (((a >> 16) & 0xFF) as i32 * it + ((b >> 16) & 0xFF) as i32 * t) / 255;
    let g = (((a >> 8) & 0xFF) as i32 * it + ((b >> 8) & 0xFF) as i32 * t) / 255;
    let bl = ((a & 0xFF) as i32 * it + (b & 0xFF) as i32 * t) / 255;
    ((r as u32) << 16) | ((g as u32) << 8) | (bl as u32)
}

/// Приближённый синус: период 512, результат от -256 до 256
fn wave(p: i32) -> i32 {
    let p = p & 511;
    if p < 256 {
        p * (256 - p) / 64
    } else {
        let t = p - 256;
        -(t * (256 - t) / 64)
    }
}

// ---------------------------------------------------------------------
//  Низкоуровневое рисование (с защитой границ экрана)
// ---------------------------------------------------------------------
#[inline(always)]
unsafe fn put(x: i32, y: i32, color: u32) {
    if FB_ADDR.is_null() || x < 0 || y < 0 || x >= SCREEN_W || y >= SCREEN_H {
        return;
    }
    FB_ADDR
        .add(y as usize * STRIDE + x as usize)
        .write_volatile(color);
}

#[inline(always)]
unsafe fn get(x: i32, y: i32) -> u32 {
    if FB_ADDR.is_null() || x < 0 || y < 0 || x >= SCREEN_W || y >= SCREEN_H {
        return 0;
    }
    FB_ADDR
        .add(y as usize * STRIDE + x as usize)
        .read_volatile()
        & 0x00FF_FFFF
}

#[inline(always)]
unsafe fn in_clip(x: i32, y: i32) -> bool {
    x >= CLIP_X0 && x < CLIP_X1 && y >= CLIP_Y0 && y < CLIP_Y1
}

unsafe fn set_clip(r: Rect) {
    CLIP_X0 = r.x.max(0);
    CLIP_Y0 = r.y.max(0);
    CLIP_X1 = (r.x + r.w).min(SCREEN_W);
    CLIP_Y1 = (r.y + r.h).min(SCREEN_H);
}

unsafe fn reset_clip() {
    set_clip(SCREEN);
}

#[inline(always)]
unsafe fn draw_pixel(x: i32, y: i32, color: u32) {
    if in_clip(x, y) {
        put(x, y, color);
    }
}

/// Пиксель с прозрачностью a (0..255)
#[inline(always)]
unsafe fn blend_pixel(x: i32, y: i32, color: u32, a: i32) {
    if a <= 0 || !in_clip(x, y) {
        return;
    }
    if a >= 255 {
        put(x, y, color);
    } else {
        let bg = get(x, y);
        put(x, y, mix(bg, color, a));
    }
}

/// Горизонтальная линия [x0, x1) в строке y
unsafe fn span(x0: i32, x1: i32, y: i32, color: u32, a: i32) {
    if a <= 0 || y < CLIP_Y0 || y >= CLIP_Y1 {
        return;
    }
    let xs = x0.max(CLIP_X0);
    let xe = x1.min(CLIP_X1);
    if xs >= xe {
        return;
    }
    if a >= 255 {
        for x in xs..xe {
            put(x, y, color);
        }
    } else {
        for x in xs..xe {
            let bg = get(x, y);
            put(x, y, mix(bg, color, a));
        }
    }
}

unsafe fn fill_rect(x: i32, y: i32, w: i32, h: i32, color: u32, a: i32) {
    for dy in 0..h {
        span(x, x + w, y + dy, color, a);
    }
}

/// Скруглённый прямоугольник с anti-aliasing на углах
unsafe fn rrect(x: i32, y: i32, w: i32, h: i32, r: i32, color: u32, a: i32) {
    if w <= 0 || h <= 0 || a <= 0 {
        return;
    }
    if x >= CLIP_X1 || x + w <= CLIP_X0 || y >= CLIP_Y1 || y + h <= CLIP_Y0 {
        return;
    }
    let r = r.min(w / 2).min(h / 2).max(0);
    let r16 = r * 16;

    for j in 0..h {
        let py = y + j;
        if py < CLIP_Y0 || py >= CLIP_Y1 {
            continue;
        }
        // Вертикальное смещение центра пикселя от центра угловой окружности (в 1/16 пикселя)
        let dy16 = if j < r {
            r16 - (j * 16 + 8)
        } else if j >= h - r {
            (j - (h - r)) * 16 + 8
        } else {
            0
        };

        if dy16 == 0 {
            span(x, x + w, py, color, a);
            continue;
        }

        for i in 0..r {
            let dx16 = r16 - (i * 16 + 8);
            let d = isqrt(dx16 * dx16 + dy16 * dy16);
            let cov = (r16 - d + 8).clamp(0, 16);
            if cov > 0 {
                let aa = a * cov / 16;
                blend_pixel(x + i, py, color, aa);
                blend_pixel(x + w - 1 - i, py, color, aa);
            }
        }
        span(x + r, x + w - r, py, color, a);
    }
}

/// Прямоугольник с однопиксельной рамкой
unsafe fn border_rrect(r: Rect, radius: i32, fill: u32, border: u32) {
    rrect(r.x, r.y, r.w, r.h, radius, border, 255);
    rrect(r.x + 1, r.y + 1, r.w - 2, r.h - 2, radius - 1, fill, 255);
}

/// Мягкая тень из набора расширяющихся полупрозрачных слоёв
unsafe fn shadow(r: Rect, radius: i32, spread: i32, oy: i32, alpha: i32) {
    let mut i = spread;
    while i >= 1 {
        rrect(
            r.x - i,
            r.y - i + oy,
            r.w + i * 2,
            r.h + i * 2,
            radius + i,
            SHADOW_COLOR,
            alpha,
        );
        i -= 1;
    }
}

/// Линия заданной толщины (алгоритм Брезенхема)
unsafe fn line(mut x0: i32, mut y0: i32, x1: i32, y1: i32, color: u32, thick: i32) {
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    loop {
        fill_rect(x0, y0, thick, thick, color, 255);
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}

// ---------------------------------------------------------------------
//  Текст
// ---------------------------------------------------------------------
unsafe fn draw_text(s: &[u8], x: i32, y: i32, color: u32, scale: i32) {
    if y + 8 * scale <= CLIP_Y0 || y >= CLIP_Y1 {
        return;
    }
    let mut cx = x;
    for &c in s {
        let g = &FONT[(c & 0x7F) as usize];
        for row in 0..8i32 {
            let bits = g[row as usize];
            if bits == 0 {
                continue;
            }
            for col in 1..8i32 {
                if (bits >> (7 - col)) & 1 == 1 {
                    if scale == 1 {
                        draw_pixel(cx + col - 1, y + row, color);
                    } else {
                        fill_rect(
                            cx + (col - 1) * scale,
                            y + row * scale,
                            scale,
                            scale,
                            color,
                            255,
                        );
                    }
                }
            }
        }
        cx += 7 * scale;
    }
}

/// «Полужирный» текст: двойной проход со сдвигом на 1 пиксель
unsafe fn draw_text_bold(s: &[u8], x: i32, y: i32, color: u32) {
    draw_text(s, x, y, color, 1);
    draw_text(s, x + 1, y, color, 1);
}

fn text_width(s: &[u8], scale: i32) -> i32 {
    s.len() as i32 * 7 * scale
}

unsafe fn draw_text_centered_bold(s: &[u8], cx: i32, y: i32, color: u32) {
    draw_text_bold(s, cx - text_width(s, 1) / 2, y, color);
}

unsafe fn draw_text_centered(s: &[u8], cx: i32, y: i32, color: u32) {
    draw_text(s, cx - text_width(s, 1) / 2, y, color, 1);
}

// ---------------------------------------------------------------------
//  Иконки (рисуются примитивами)
// ---------------------------------------------------------------------

/// Кольцо с вырезом.
/// gap: 0 — без выреза, 1 — вырез сверху (power),
/// 2 — вырез в правом верхнем секторе (restart), 3 — только правая сторона (звук)
unsafe fn ring(cx: i32, cy: i32, r: i32, thick: i32, color: u32, gap: u8) {
    let rm16 = r * 16;
    let half = thick * 8;
    let ext = r + thick + 2;
    for py in (cy - ext)..(cy + ext) {
        for px in (cx - ext)..(cx + ext) {
            let dx = (px - cx) * 16 + 8;
            let dy = (py - cy) * 16 + 8;
            let skip = match gap {
                1 => dy < 0 && dx.abs() < r * 16 / 3,
                2 => dx > 0 && dy < 0 && -dy < dx,
                3 => !(dx > 0 && dy.abs() <= dx),
                _ => false,
            };
            if skip {
                continue;
            }
            let d = isqrt(dx * dx + dy * dy);
            let cov = (half - (d - rm16).abs() + 8).clamp(0, 16);
            if cov > 0 {
                blend_pixel(px, py, color, 255 * cov / 16);
            }
        }
    }
}

/// Значок питания: кольцо с вертикальной чертой
unsafe fn draw_power_icon(cx: i32, cy: i32, r: i32, color: u32) {
    let t = (r / 5 + 1).max(2);
    ring(cx, cy, r, t, color, 1);
    rrect(cx - t / 2, cy - r - 1, t, r + 2, t / 2, color, 255);
}

/// Значок перезагрузки: почти замкнутое кольцо со стрелкой
unsafe fn draw_restart_icon(cx: i32, cy: i32, r: i32, color: u32) {
    let t = (r / 5 + 1).max(2);
    ring(cx, cy, r, t, color, 2);

    // Наконечник стрелки смотрит вверх, в разрыв кольца
    let ax = cx + r;
    let apex = cy - r * 6 / 10;
    let base = cy + 1;
    let bw = t + 3;
    for yy in apex..=base {
        let half = (yy - apex) * bw / (base - apex).max(1);
        span(ax - half, ax + half + 1, yy, color, 255);
    }
}

static START_DOTS: [(i32, i32); 4] = [(0, -7), (-7, 0), (7, 0), (0, 7)];

/// Значок кнопки Пуск: ромб из точек
unsafe fn draw_start_icon(cx: i32, cy: i32, color: u32) {
    for k in 0..4 {
        let (dx, dy) = START_DOTS[k];
        rrect(cx + dx - 2, cy + dy - 2, 4, 4, 2, color, 255);
    }
    rrect(cx - 3, cy - 3, 6, 6, 3, color, 255);
}

/// Шеврон «>» для строки меню
unsafe fn draw_chevron(x: i32, cy: i32, color: u32) {
    line(x, cy - 5, x + 5, cy, color, 2);
    line(x + 5, cy, x, cy + 5, color, 2);
}

/// Значок «×» для закрытия
unsafe fn draw_cross(cx: i32, cy: i32, color: u32) {
    line(cx - 4, cy - 4, cx + 3, cy + 3, color, 2);
    line(cx + 3, cy - 4, cx - 4, cy + 3, color, 2);
}

static SIGNAL_H: [i32; 4] = [4, 7, 10, 13];

/// Значок сети: четыре столбика
unsafe fn draw_signal_icon(x: i32, base_y: i32, color: u32) {
    for i in 0..4usize {
        let h = SIGNAL_H[i];
        rrect(x + i as i32 * 5, base_y - h, 3, h, 1, color, 255);
    }
}

/// Значок звука: динамик и две волны
unsafe fn draw_speaker_icon(x: i32, cy: i32, color: u32) {
    rrect(x, cy - 3, 4, 6, 1, color, 255);
    for c in 0..5i32 {
        let hh = 3 + c * 3 / 4;
        fill_rect(x + 4 + c, cy - hh, 1, hh * 2, color, 255);
    }
    ring(x + 9, cy, 3, 2, color, 3);
    ring(x + 9, cy, 6, 2, color, 3);
}

// ---------------------------------------------------------------------
//  Обои: градиент неба, мягкое свечение и «ленты» в стиле Plasma
// ---------------------------------------------------------------------
struct Ribbon {
    base: i32,
    amp: i32,
    period: i32,
    phase: i32,
    thick: i32,
    light: u32,
    dark: u32,
}

static RIBBONS: [Ribbon; 4] = [
    Ribbon {
        base: 190,
        amp: 55,
        period: 900,
        phase: 100,
        thick: 90,
        light: 0x70B2F1,
        dark: 0x2C6FCF,
    },
    Ribbon {
        base: 270,
        amp: 50,
        period: 800,
        phase: 220,
        thick: 80,
        light: 0x7EBCF5,
        dark: 0x2F78D6,
    },
    Ribbon {
        base: 350,
        amp: 45,
        period: 760,
        phase: 330,
        thick: 70,
        light: 0x8DC7F7,
        dark: 0x3A86DE,
    },
    Ribbon {
        base: 430,
        amp: 40,
        period: 700,
        phase: 60,
        thick: 60,
        light: 0x9CD1FA,
        dark: 0x4796E6,
    },
];

static mut RIBBON_Y: [[i32; SCREEN_W as usize]; 4] = [[0; SCREEN_W as usize]; 4];

const RIB_X0: i32 = 360;
const RIB_FADE: i32 = 200;

const GLOW_X: i32 = 170;
const GLOW_Y: i32 = 470;
const GLOW_R2: i32 = 280 * 280;

/// Предрасчёт центральных линий лент (в 1/16 пикселя)
unsafe fn build_ribbons() {
    for k in 0..4usize {
        let rb = &RIBBONS[k];
        for x in 0..SCREEN_W {
            let p = x * 512 / rb.period + rb.phase;
            RIBBON_Y[k][x as usize] = rb.base * 16 + rb.amp * wave(p) / 16;
        }
    }
}

fn sky_color(y: i32) -> u32 {
    let split = SCREEN_H * 45 / 100;
    if y < split {
        mix(SKY_TOP, SKY_MID, y * 255 / split)
    } else {
        mix(SKY_MID, SKY_BOT, (y - split) * 255 / (SCREEN_H - split))
    }
}

/// dim — степень затемнения (0 = без затемнения), используется под диалогом
unsafe fn draw_wallpaper(dim: i32) {
    let (x0, x1, y0, y1) = (CLIP_X0, CLIP_X1, CLIP_Y0, CLIP_Y1);
    for y in y0..y1 {
        let base = sky_color(y);
        for x in x0..x1 {
            let mut c = base;

            // Мягкое белое свечение слева внизу
            let gdx = x - GLOW_X;
            let gdy = y - GLOW_Y;
            let d2 = gdx * gdx + gdy * gdy;
            if d2 < GLOW_R2 {
                let u = (GLOW_R2 - d2) * 256 / GLOW_R2;
                let a = (((u * u) >> 8) * 120) >> 8;
                c = mix(c, 0xFFFFFF, a);
            }

            // Ленты справа, плавно проявляются слева направо
            if x >= RIB_X0 {
                let fade = (x - RIB_X0).min(RIB_FADE) * 255 / RIB_FADE;
                for k in 0..4usize {
                    let rb = &RIBBONS[k];
                    let half16 = rb.thick * 8;
                    let dy16 = y * 16 + 8 - RIBBON_Y[k][x as usize];
                    let ad = dy16.abs();
                    if ad < half16 + 16 {
                        let cov = (half16 - ad + 8).clamp(0, 16);
                        if cov > 0 {
                            let t = ((dy16 + half16) * 255 / (half16 * 2)).clamp(0, 255);
                            let col = mix(rb.light, rb.dark, t);
                            c = mix(c, col, cov * fade / 16);
                        }
                    }
                }
            }

            if dim > 0 {
                c = mix(c, DIM_COLOR, dim);
            }
            put(x, y, c);
        }
    }
}

// ---------------------------------------------------------------------
//  Состояние «нажато»: кнопка мыши зажата на этом элементе и курсор над ним
// ---------------------------------------------------------------------
unsafe fn is_pressed(id: u8) -> bool {
    PRESS_DOWN && PRESS_ID == id && HOVER == id
}

// ---------------------------------------------------------------------
//  Панель задач
// ---------------------------------------------------------------------
unsafe fn draw_taskbar() {
    shadow(PANEL, 14, 10, 2, 5);
    border_rrect(PANEL, 14, PANEL_BG, PANEL_BORDER);

    // Кнопка Пуск
    let s = START_BTN;
    let active = MENU_OPEN;
    let hover = HOVER == H_START;
    let pressed = is_pressed(H_START);

    let bg: Option<u32> = if active {
        Some(if pressed {
            mix(ACCENT_SOFT, ACCENT, 50)
        } else {
            ACCENT_SOFT
        })
    } else if pressed {
        Some(mix(HOVER_BG, ACCENT, 50))
    } else if hover {
        Some(HOVER_BG)
    } else {
        None
    };
    if let Some(c) = bg {
        rrect(s.x, s.y, s.w, s.h, 9, c, 255);
    }
    if active {
        // Синий индикатор активности, как в Plasma
        rrect(s.x + 6, PANEL.y + 1, s.w - 12, 3, 1, ACCENT, 255);
    }
    let icon_color = if active { ACCENT } else { TEXT_DARK };
    draw_start_icon(s.x + s.w / 2, s.y + s.h / 2, icon_color);

    // Системный трей справа: сеть и звук
    let right = PANEL.x + PANEL.w;
    let cy = PANEL.y + PANEL.h / 2;
    let speaker_x = right - 16 - 16;
    let signal_x = speaker_x - 18 - 16;
    draw_signal_icon(signal_x, cy + 7, ICON_GRAY);
    draw_speaker_icon(speaker_x, cy, ICON_GRAY);
}

// ---------------------------------------------------------------------
//  Меню Пуск: только кнопка питания
// ---------------------------------------------------------------------
unsafe fn draw_menu() {
    shadow(MENU, 14, 12, 4, 5);
    border_rrect(MENU, 14, MENU_BG, MENU_BORDER);

    let b = POWER_BTN;
    let hover = HOVER == H_POWER;
    let pressed = is_pressed(H_POWER);
    if hover {
        let fill = if pressed {
            mix(ACCENT_SOFT, ACCENT, 50)
        } else {
            ACCENT_SOFT
        };
        rrect(b.x, b.y, b.w, b.h, 11, ACCENT, 255);
        rrect(b.x + 1, b.y + 1, b.w - 2, b.h - 2, 10, fill, 255);
    }

    let cy = b.y + b.h / 2;
    let icx = b.x + 10 + 18;

    // Круглая иконка питания
    rrect(
        b.x + 10,
        cy - 18,
        36,
        36,
        18,
        if hover { DANGER } else { DANGER_SOFT },
        255,
    );
    draw_power_icon(icx, cy + 1, 8, if hover { 0xFFFFFF } else { DANGER });

    draw_text_bold(b"Power", b.x + 58, cy - 10, TEXT_DARK);
    draw_text(b"Shut down or restart", b.x + 58, cy + 3, TEXT_MUTED, 1);

    draw_chevron(b.x + b.w - 16, cy, if hover { ACCENT } else { TEXT_MUTED });
}

// ---------------------------------------------------------------------
//  Диалог Power Options
// ---------------------------------------------------------------------
unsafe fn action_card(
    b: Rect,
    hover: bool,
    pressed: bool,
    accent: u32,
    soft: u32,
    label: &[u8],
    restart: bool,
) {
    if hover {
        let inner = if pressed {
            mix(0xFFFFFF, accent, 55)
        } else {
            mix(0xFFFFFF, accent, 22)
        };
        rrect(b.x, b.y, b.w, b.h, 14, accent, 255);
        rrect(b.x + 2, b.y + 2, b.w - 4, b.h - 4, 12, inner, 255);
    } else {
        rrect(b.x, b.y, b.w, b.h, 14, CARD_BORDER, 255);
        rrect(b.x + 1, b.y + 1, b.w - 2, b.h - 2, 13, CARD_BG, 255);
    }

    let cx = b.x + b.w / 2;
    let cy = b.y + 14 + 22;
    rrect(
        cx - 22,
        cy - 22,
        44,
        44,
        22,
        if hover { accent } else { soft },
        255,
    );

    let icon_color = if hover { 0xFFFFFF } else { accent };
    if restart {
        draw_restart_icon(cx, cy, 11, icon_color);
    } else {
        draw_power_icon(cx, cy + 1, 11, icon_color);
    }

    let text_color = if hover {
        mix(accent, 0x000000, 60)
    } else {
        TEXT_DARK
    };
    draw_text_centered_bold(label, cx, b.y + 70, text_color);
}

unsafe fn draw_dialog() {
    let d = DIALOG;
    shadow(d, 16, 16, 6, 5);
    border_rrect(d, 16, MENU_BG, MENU_BORDER);

    draw_text(b"Power Options", d.x + 24, d.y + 22, TEXT_DARK, 2);
    draw_text(
        b"Choose what you want to do",
        d.x + 24,
        d.y + 46,
        TEXT_MUTED,
        1,
    );

    // Кнопка закрытия
    let c = CLOSE_BTN;
    let close_hover = HOVER == H_CLOSE;
    let close_bg = if close_hover {
        if is_pressed(H_CLOSE) {
            mix(DANGER, 0x000000, 60)
        } else {
            DANGER
        }
    } else {
        HOVER_BG
    };
    rrect(c.x, c.y, c.w, c.h, 13, close_bg, 255);
    draw_cross(
        c.x + c.w / 2,
        c.y + c.h / 2,
        if close_hover { 0xFFFFFF } else { ICON_GRAY },
    );

    fill_rect(d.x + 24, d.y + 66, d.w - 48, 1, DIVIDER, 255);

    action_card(
        SHUT_BTN,
        HOVER == H_SHUTDOWN,
        is_pressed(H_SHUTDOWN),
        DANGER,
        DANGER_SOFT,
        b"Shut Down",
        false,
    );
    action_card(
        REST_BTN,
        HOVER == H_RESTART,
        is_pressed(H_RESTART),
        ACCENT,
        ACCENT_SOFT,
        b"Restart",
        true,
    );

    draw_text_centered(
        b"Click outside this window to cancel",
        d.x + d.w / 2,
        d.y + d.h - 24,
        TEXT_MUTED,
    );
}

// ---------------------------------------------------------------------
//  Сборка сцены и курсор
// ---------------------------------------------------------------------
unsafe fn render_scene() {
    let dim = if DIALOG_OPEN { DIM_ALPHA } else { 0 };
    draw_wallpaper(dim);
    draw_taskbar();

    if DIALOG_OPEN {
        // Затемнение панели под окном диалога
        rrect(PANEL.x, PANEL.y, PANEL.w, PANEL.h, 14, DIM_COLOR, DIM_ALPHA);
        draw_dialog();
    } else if MENU_OPEN {
        draw_menu();
    }
}

unsafe fn show_cursor(mx: i32, my: i32) {
    for row in 0..CURSOR_H {
        for col in 0..CURSOR_W {
            let t = CURSOR_PIXELS[row as usize][col as usize];
            if t == 0 {
                continue;
            }
            let x = mx + col;
            let y = my + row;
            if x < 0 || y < 0 || x >= SCREEN_W || y >= SCREEN_H {
                continue;
            }
            CUR_SAVE[(row * CURSOR_W + col) as usize] = get(x, y);
            put(x, y, if t == 1 { 0x14181C } else { 0xFFFFFF });
        }
    }
    CUR_X = mx;
    CUR_Y = my;
    CUR_VISIBLE = true;
}

unsafe fn hide_cursor() {
    if !CUR_VISIBLE {
        return;
    }
    for row in 0..CURSOR_H {
        for col in 0..CURSOR_W {
            if CURSOR_PIXELS[row as usize][col as usize] == 0 {
                continue;
            }
            let x = CUR_X + col;
            let y = CUR_Y + row;
            if x < 0 || y < 0 || x >= SCREEN_W || y >= SCREEN_H {
                continue;
            }
            put(x, y, CUR_SAVE[(row * CURSOR_W + col) as usize]);
        }
    }
    CUR_VISIBLE = false;
}

/// Перерисовать область экрана и вернуть курсор на место
unsafe fn refresh(area: Rect, mx: i32, my: i32) {
    hide_cursor();
    set_clip(area);
    render_scene();
    reset_clip();
    show_cursor(mx, my);
}

// ---------------------------------------------------------------------
//  Логика интерфейса
// ---------------------------------------------------------------------
unsafe fn hit_test(mx: i32, my: i32) -> u8 {
    if DIALOG_OPEN {
        if CLOSE_BTN.contains(mx, my) {
            return H_CLOSE;
        }
        if SHUT_BTN.contains(mx, my) {
            return H_SHUTDOWN;
        }
        if REST_BTN.contains(mx, my) {
            return H_RESTART;
        }
        return H_NONE;
    }
    if MENU_OPEN && POWER_BTN.contains(mx, my) {
        return H_POWER;
    }
    if START_BTN.contains(mx, my) {
        return H_START;
    }
    H_NONE
}

fn widget_rect(id: u8) -> Option<Rect> {
    match id {
        H_START => Some(START_BTN.inflate(2)),
        H_POWER => Some(POWER_BTN.inflate(2)),
        H_SHUTDOWN => Some(SHUT_BTN.inflate(2)),
        H_RESTART => Some(REST_BTN.inflate(2)),
        H_CLOSE => Some(CLOSE_BTN.inflate(2)),
        _ => None,
    }
}

/// Область, которую нужно обновить при смене элемента под курсором
fn hover_dirty(old: u8, new: u8) -> Option<Rect> {
    merge(widget_rect(old), widget_rect(new))
}

/// Отпускание левой кнопки. press_id — элемент, на котором кнопку нажали,
/// rel_id — элемент под курсором сейчас. Возвращает область для перерисовки,
/// если состояние интерфейса изменилось.
unsafe fn handle_release(press_id: u8, rel_id: u8, mx: i32, my: i32) -> Option<Rect> {
    if press_id != H_NONE {
        // Клик засчитывается, только если курсор остался на том же элементе
        if press_id != rel_id {
            return None;
        }
        return match press_id {
            H_START => {
                MENU_OPEN = !MENU_OPEN;
                Some(MENU_REGION)
            }
            H_POWER => {
                MENU_OPEN = false;
                DIALOG_OPEN = true;
                Some(SCREEN)
            }
            H_SHUTDOWN => system_shutdown(),
            H_RESTART => system_reboot(),
            H_CLOSE => {
                DIALOG_OPEN = false;
                Some(SCREEN)
            }
            _ => None,
        };
    }

    // Нажатие было не на кнопке: клик мимо закрывает диалог или меню
    if rel_id == H_NONE {
        if DIALOG_OPEN {
            if !DIALOG.contains(mx, my) {
                DIALOG_OPEN = false;
                return Some(SCREEN);
            }
        } else if MENU_OPEN && !MENU.contains(mx, my) {
            MENU_OPEN = false;
            return Some(MENU_REGION);
        }
    }
    None
}

// ---------------------------------------------------------------------
//  Точка входа
// ---------------------------------------------------------------------
pub unsafe fn init_desktop_800x600(base: *mut u32) {
    if base.is_null() {
        return;
    }
    FB_ADDR = base;

    build_ribbons();
    init_mouse();

    let mut mx = get_mouse_x().clamp(0, SCREEN_W - 1);
    let mut my = get_mouse_y().clamp(0, SCREEN_H - 1);

    // Если кнопка уже зажата при старте — не считаем это кликом
    PREV_CLICK = (get_mouse_buttons() & 1) != 0;
    PRESS_DOWN = false;
    PRESS_ID = H_NONE;

    reset_clip();
    HOVER = hit_test(mx, my);
    render_scene();
    show_cursor(mx, my);

    loop {
        // Опрос драйвера; положение берём из геттеров
        let _ = update_mouse_c();
        let left = (get_mouse_buttons() & 1) != 0;

        let nx = get_mouse_x().clamp(0, SCREEN_W - 1);
        let ny = get_mouse_y().clamp(0, SCREEN_H - 1);
        let cursor_moved = nx != mx || ny != my;
        mx = nx;
        my = ny;

        let mut dirty: Option<Rect> = None;

        // Подсветка элементов под курсором
        let h = hit_test(mx, my);
        if h != HOVER {
            dirty = merge(dirty, hover_dirty(HOVER, h));
            HOVER = h;
        }

        if left && !PREV_CLICK {
            // Нажатие: запоминаем элемент, показываем «нажатое» состояние
            PRESS_DOWN = true;
            PRESS_ID = HOVER;
            dirty = merge(dirty, widget_rect(HOVER));
        } else if !left && PREV_CLICK {
            // Отпускание: срабатывает клик
            let pid = PRESS_ID;
            PRESS_DOWN = false;
            PRESS_ID = H_NONE;
            dirty = merge(dirty, widget_rect(pid));

            if let Some(r) = handle_release(pid, HOVER, mx, my) {
                dirty = merge(dirty, Some(r));
                // Интерфейс изменился — пересчитываем элемент под курсором
                let h2 = hit_test(mx, my);
                dirty = merge(dirty, widget_rect(HOVER));
                dirty = merge(dirty, widget_rect(h2));
                HOVER = h2;
            }
        }
        PREV_CLICK = left;

        if let Some(area) = dirty {
            refresh(area, mx, my);
        } else if cursor_moved {
            hide_cursor();
            show_cursor(mx, my);
        }

        core::hint::spin_loop();
    }
}
