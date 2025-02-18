<<<<<<< HEAD
dinos
```
.
|-- README.md
|-- bin
|   |-- aarch64-objdump
|   |-- aarch64-readelf
|   |-- build-openocd.sh
|   |-- build-qemu.sh
|   |-- gen-rpi3-config.py
|   |-- get-dist
|   |-- get-host-target.sh
|   |-- install-kernel.py
|   |-- ncpus
|   |-- qemu-system-aarch64
|   `-- setup.sh
|-- boot
|   |-- Cargo.toml
|   |-- Makefile
|   |-- build.rs
|   |-- qemu.sh
|   `-- src
|       |-- init
|       |   |-- init.s
|       |   `-- panic.rs
|       |-- init.rs
|       `-- main.rs
|-- ext
|   |-- firmware
|   |   |-- bootcode.bin
|   |   |-- fixup.dat
|   |   `-- start.elf
|   |-- rpi3-debug
|   |   `-- rpi3.cfg
|   |-- rpi3-gpio16
|   |   |-- bootcode.bin
|   |   |-- config.txt
|   |   |-- fixup.dat -> ../firmware/fixup.dat
|   |   |-- kernel8.img
|   |   `-- start.elf -> ../firmware/start.elf
|   |-- rpi3-led
|   |   |-- bootcode.bin
|   |   |-- config.txt
|   |   |-- fixup.dat -> ../firmware/fixup.dat
|   |   |-- kernel8.img
|   |   `-- start.elf -> ../firmware/start.elf
|   `-- rpi3-uart
|       |-- config.txt
|       |-- fixup.dat -> ../firmware/fixup.dat
|       |-- kernel8.img
|       `-- start.elf -> ../firmware/start.elf
|-- kern
|   |-- Cargo.lock
|   |-- Cargo.toml
|   |-- Makefile
|   |-- build.rs
|   |-- qemu.sh
|   `-- src
|       |-- console.rs
|       |-- init
|       |   |-- init.s
|       |   |-- oom.rs
|       |   `-- panic.rs
|       |-- init.rs
|       |-- main.rs
|       |-- mutex.rs
|       `-- shell.rs
`-- lib
    |-- pi
    |   |-- Cargo.lock
    |   |-- Cargo.toml
    |   `-- src
    |       |-- common.rs
    |       |-- gpio.rs
    |       |-- lib.rs
    |       |-- timer.rs
    |       `-- uart.rs
    |-- shim
    |   |-- Cargo.toml
    |   `-- src
    |       |-- lib.rs
    |       |-- macros.rs
    |       |-- no_std
    |       |   |-- ffi
    |       |   |   `-- os_str_bytes.rs
    |       |   |-- ffi.rs
    |       |   `-- path.rs
    |       |-- no_std.rs
    |       |-- std.rs
    |       `-- tests.rs
    |-- stack-vec
    |   |-- Cargo.lock
    |   |-- Cargo.toml
    |   `-- src
    |       |-- lib.rs
    |       `-- tests.rs
    |-- ttywrite
    |   |-- Cargo.lock
    |   |-- Cargo.toml
    |   |-- input -> /dev/pts/9
    |   |-- output -> /dev/pts/10
    |   |-- src
    |   |   |-- main.rs
    |   |   `-- parsers.rs
    |   |-- target
    |   |   `-- debug
    |   |       |-- build
    |   |       |-- deps
    |   |       |-- native
    |   |       |-- ttywrite
    |   |       `-- ttywrite.d
    |   `-- test.sh
    |-- volatile
    |   |-- Cargo.toml
    |   `-- src
    |       |-- lib.rs
    |       |-- macros.rs
    |       `-- traits.rs
    `-- xmodem
        |-- Cargo.lock
        |-- Cargo.toml
        `-- src
            |-- lib.rs
            |-- progress.rs
            |-- read_ext.rs
            `-- tests.rs
```
