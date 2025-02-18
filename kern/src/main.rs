#![feature(alloc_error_handler)]
#![feature(decl_macro)]
#![feature(auto_traits)]
#![feature(negative_impls)]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

#[cfg(not(test))]
mod init;

pub mod console;
pub mod mutex;
pub mod shell;

use console::kprintln;

// FIXME: You need to add dependencies here to
// test your drivers (Phase 2). Add them as needed.

extern crate pi;  
use pi::uart::MiniUart;


fn test_uart() {
    let mut uart = MiniUart::new();

    kprintln!("MiniUart initialized!");

    uart.write_byte(b'H');
    uart.write_byte(b'e');
    uart.write_byte(b'l');
    uart.write_byte(b'l');
    uart.write_byte(b'o');
    uart.write_byte(b'\n');

    // Echo back received characters (basic shell loop)
    loop {
        if uart.has_byte() {
            let byte = uart.read_byte();
            uart.write_byte(byte);
        }
    }
}

fn kmain() -> ! {
    // FIXME: Start the shell.
    test_uart();
    unimplemented!()
}
