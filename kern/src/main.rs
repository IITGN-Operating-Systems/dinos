#![feature(alloc_error_handler)]
#![feature(decl_macro)]
#![feature(asm)]
#![feature(global_asm)]
#![feature(auto_traits)]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![feature(negative_impls)]
#[cfg(not(test))]
mod init;

pub mod console;
pub mod mutex;
pub mod shell;

use console::kprintln;
use shell::shell;

/// The kernel entry point.
#[no_mangle]
pub extern "C" fn kmain() -> ! {
    // Print a welcome message.
    kprintln!("Welcome to the Rust shell!");

    // Start the shell with the prompt "> ".
    shell("> ");
}