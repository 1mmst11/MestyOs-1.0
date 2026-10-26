all: mesty_os.iso

# 1. Компилируем Ассемблер в 64-битный объектный файл elf64
boot.o: src/boot.asm
	nasm -f elf64 src/boot.asm -o boot.o

# 2. Передаем флаг -Z в саму команду cargo
rust_kernel:
	cargo build -Z unstable-options -Zjson-target-spec --target x86_64-mesty_os.json --release

# 3. Склеиваем Ассемблер и Rust по правилам 64-битного линкера elf_x86_64
kernel.bin: boot.o rust_kernel
	ld -m elf_x86_64 -T linker.ld -o kernel.bin boot.o target/x86_64-mesty_os/release/libmesty_os.a

# 4. Копируем ядро в папку GRUB и собираем загрузочный ISO-диск
mesty_os.iso: kernel.bin
	cp kernel.bin isofiles/boot/kernel.bin
	grub-mkrescue -o mesty_os.iso isofiles

clean:
	rm -f *.o kernel.bin mesty_os.iso isofiles/boot/kernel.bin
	cargo clean
