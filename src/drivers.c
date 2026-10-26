#define DATA_PORT 0x60
#define STATUS_PORT 0x64
#define COMMAND_PORT 0x64

int mouse_x = 400;
int mouse_y = 300;
int mouse_buttons = 0; // Переменная для хранения состояния кнопок

unsigned char mouse_cycle = 0;
unsigned char mouse_byte[3] = {0, 0, 0};

unsigned char inb(unsigned short port) {
    unsigned char result;
    __asm__ volatile("inb %1, %0" : "=a"(result) : "Nd"(port));
    return result;
}

void outb(unsigned short port, unsigned char value) {
    __asm__ volatile("outb %0, %1" :: "a"(value), "Nd"(port));
}

void mouse_wait(unsigned char type) {
    unsigned int timeout = 100000;
    if (type == 0) {
        while (timeout > 0 && (inb(STATUS_PORT) & 1) == 1) { timeout--; }
    } else {
        while (timeout > 0 && (inb(STATUS_PORT) & 2) == 2) { timeout--; }
    }
}

void mouse_write(unsigned char value) {
    mouse_wait(1);
    outb(COMMAND_PORT, 0xD4);
    mouse_wait(1);
    outb(DATA_PORT, value);
}

unsigned char mouse_read() {
    mouse_wait(0);
    return inb(DATA_PORT);
}

void init_mouse() {
    mouse_wait(1);
    outb(COMMAND_PORT, 0xA8);

    mouse_wait(1);
    outb(COMMAND_PORT, 0x20);
    mouse_wait(0);
    unsigned char status = inb(DATA_PORT) | 2;

    mouse_wait(1);
    outb(COMMAND_PORT, 0x60);
    mouse_wait(1);
    outb(DATA_PORT, status);

    mouse_write(0xF6);
    mouse_read();

    mouse_write(0xF4);
    mouse_read();
}

int update_mouse_c() {
    if ((inb(STATUS_PORT) & 1) != 0) {
        unsigned char data = inb(DATA_PORT);

        if (mouse_cycle == 0) {
            if ((data & 0x08) != 0) {
                mouse_byte[0] = data;
                mouse_cycle = 1;
            }
        } else if (mouse_cycle == 1) {
            mouse_byte[1] = data;
            mouse_cycle = 2;
        } else if (mouse_cycle == 2) {
            mouse_byte[2] = data;
            mouse_cycle = 0;

            int sign_x = (mouse_byte[0] & 0x10) != 0;
            int sign_y = (mouse_byte[0] & 0x20) != 0;

            int move_x = mouse_byte[1];
            int move_y = mouse_byte[2];

            if (sign_x) move_x -= 256;
            if (sign_y) move_y -= 256;

            mouse_x += move_x;
            mouse_y -= move_y;

            if (mouse_x < 0) mouse_x = 0;
            if (mouse_x > 788) mouse_x = 788; // Учитываем ширину стрелки 12 пикселей
            if (mouse_y < 0) mouse_y = 0;
            if (mouse_y > 582) mouse_y = 582; // Учитываем высоту стрелки 18 пикселей

            // Извлекаем состояние кнопок: Бит 0 = ЛКМ, Бит 1 = ПКМ
            mouse_buttons = mouse_byte[0] & 0x07;

            return 1;
        }
    }
    return 0;
}

int get_mouse_x() { return mouse_x; }
int get_mouse_y() { return mouse_y; }
int get_mouse_buttons() { return mouse_buttons; } // Геттер для кнопок готов!
