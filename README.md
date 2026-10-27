# MestyOS 1.0

**MestyOS** is a lightweight, 64-bit toy operating system built from scratch in Rust (`no_std`). It serves as a practical sandbox for learning low-level system programming, kernel architecture, memory management, and custom framebuffer graphics.

---

##  Key Features

* **Bare-Metal Rust (`no_std`):** Runs directly on computer hardware without relying on an underlying OS.
* **64-bit Long Mode:** Fully boots into 64-bit execution mode using a custom bootloader pipeline.
* **Interrupt Handling:** Configured Interrupt Descriptor Table (IDT) with support for CPU exceptions and hardware interrupts (keyboard input, timer).
* **Memory Management:** Basic physical frame allocation and heap memory allocator for dynamic data structures.
* **Graphical Framebuffer UI:** Direct pixel rendering interface with basic graphical desktop concepts.

