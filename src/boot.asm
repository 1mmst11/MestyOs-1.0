MODULEALIGN equ 1<<0
MEMINFO     equ 1<<1
GRAPHMODE   equ 1<<2
FLAGS       equ MODULEALIGN | MEMINFO | GRAPHMODE
MAGIC       equ 0x1BADB002
CHECKSUM    equ -(MAGIC + FLAGS)

section .multiboot
align 4
    dd MAGIC
    dd FLAGS
    dd CHECKSUM

    ; Заглушки адресов для GRUB
    dd 0, 0, 0, 0, 0

    ; Передаем наши жесткие графические настройки для разрешения 800x600
    dd 0
    dd 800
    dd 600
    dd 32

section .text
bits 32
global start
extern _start

start:
    cli

    ; Сохраняем указатель Multiboot Info в регистр EDI для передачи в Rust
    mov edi, ebx

    mov eax, 0x80000000
    cpuid
    cmp eax, 0x80000001
    jb .no_long_mode
    mov eax, 0x80000001
    cpuid
    test edx, 1 << 29
    jz .no_long_mode

    ; Настраиваем таблицы страниц памяти (Маппим 4 Гигабайта!)
    mov eax, p3_table
    or eax, 0b11
    mov [p4_table], eax

    ; Связываем 4 записи таблицы PDPT, чтобы покрыть 4 ГБ адресного пространства
    mov eax, p2_table
    or eax, 0b11
    mov [p3_table], eax

    mov eax, p2_table + 4096
    or eax, 0b11
    mov [p3_table + 8], eax

    mov eax, p2_table + 8192
    or eax, 0b11
    mov [p3_table + 16], eax

    mov eax, p2_table + 12288
    or eax, 0b11
    mov [p3_table + 24], eax

    ; Цикл маппирования: заполняем 2048 страниц по 2 МБ (итого 4 ГБ памяти)
    mov ecx, 0
.map_p2_table:
    mov eax, 0x200000
    mul ecx
    or eax, 0b10000011
    mov [p2_table + ecx * 8], eax
    inc ecx
    cmp ecx, 2048
    jne .map_p2_table

    mov eax, p4_table
    mov cr3, eax
    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax

    mov ecx, 0xC0000080
    rdmsr
    or eax, 1 << 8
    wrmsr

    mov eax, cr0
    or eax, 1 << 31
    mov cr0, eax

    lgdt [gdt64.pointer]
    jmp gdt64.code:long_mode_start

.no_long_mode:
    hlt

bits 64
long_mode_start:
    mov ax, 0
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    mov rsp, stack_space

    call _start
    hlt

section .rodata
gdt64:
    dq 0
.code: equ $ - gdt64
    dq (1<<43) | (1<<44) | (1<<47) | (1<<53)
.pointer:
    dw $ - gdt64 - 1
    dq gdt64

section .bss
align 4096
p4_table:
    resb 4096
p3_table:
    resb 4096
p2_table:
    resb 4096 * 4 ; Выделили место под 4 таблицы страниц для покрытия 4 ГБ памяти
resb 8192
stack_space:
