ENTRY(start)
OUTPUT_FORMAT("elf64-x86-64") /* Жестко говорим линкеру выдавать 64-битный формат */

SECTIONS {
    . = 1M; /* Загружаем ядро с 1 Мегабайта оперативной памяти */

    .text : {
        *(.multiboot) /* GRUB должен найти эту метку в первых 8 КБ файла */
        *(.text)
    }

    .rodata : { *(.rodata) }
    .data   : { *(.data) }
    .bss    : { *(.bss) }
}
