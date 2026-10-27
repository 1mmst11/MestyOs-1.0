# MestyOS 1.0

**MestyOS** is a lightweight, 64-bit toy operating system built from scratch in Rust (`no_std`). It serves as a practical sandbox for learning low-level system programming, kernel architecture, memory management, and custom framebuffer graphics.

---

## 🚀 Key Features

* **Bare-Metal Rust (`no_std`):** Runs directly on computer hardware without relying on an underlying OS.
* **64-bit Long Mode:** Fully boots into 64-bit execution mode using a custom bootloader pipeline.
* **Interrupt Handling:** Configured Interrupt Descriptor Table (IDT) with support for CPU exceptions and hardware interrupts (keyboard input, timer).
* **Memory Management:** Basic physical frame allocation and heap memory allocator for dynamic data structures.
* **Graphical Framebuffer UI:** Direct pixel rendering interface with basic graphical desktop concepts.

---

## 🛠️ Prerequisites

To build and run **MestyOS**, you will need:

1. **Rust Nightly Toolchain:**
   
   rustup default nightly
   rustup component add rust-src

    Bootimage Tool:
    Bash

    cargo install bootimage

    QEMU Emulator: Download and install QEMU to run the OS in a virtual machine.

💻 Quick Start

    Clone the repository:
    

    git clone [https://github.com/1mmst11/MestyOs-1.0.git](https://github.com/1mmst11/MestyOs-1.0.git)
    cd MestyOs-1.0

    
    

    cargo run

    cargo run automatically builds the kernel image, invokes bootimage, and launches QEMU.

📂 Project Structure
Plaintext

├── src/
│   ├── main.rs          # Kernel entry point (_start) and system initialization
│   ├── interrupts.rs    # IDT setup and hardware interrupt handlers
│   ├── memory.rs        # Paging and physical frame allocator
│   ├── allocator.rs     # Global heap allocator configuration
│   └── vga_buffer.rs    # Text/Graphics buffer rendering logic
├── Cargo.toml           # Project dependencies and configuration
└── x86_64-mestyos.json  # Custom target specification for bare-metal x86_64

🤝 Contributing & Customization

MestyOS is designed as a modular starting template for students and enthusiasts. Feel free to fork the repository, add custom UI components, expand keyboard layout support, or implement custom system commands!
