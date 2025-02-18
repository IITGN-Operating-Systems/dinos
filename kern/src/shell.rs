use stack_vec::StackVec;

use crate::console::{kprint, kprintln, CONSOLE};
use core::arch::asm;

/// Error type for `Command` parse failures.
#[derive(Debug)]
enum Error {
    Empty,
    TooManyArgs,
}

/// A structure representing a single shell command.
struct Command<'a> {
    args: StackVec<'a, &'a str>,
}

impl<'a> Command<'a> {
    /// Parse a command from a string `s` using `buf` as storage for the
    /// arguments.
    ///
    /// # Errors
    ///
    /// If `s` contains no arguments, returns `Error::Empty`. If there are more
    /// arguments than `buf` can hold, returns `Error::TooManyArgs`.
    fn parse(s: &'a str, buf: &'a mut [&'a str]) -> Result<Command<'a>, Error> {
        let mut args = StackVec::new(buf);
        for arg in s.split(' ').filter(|a| !a.is_empty()) {
            args.push(arg).map_err(|_| Error::TooManyArgs)?;
        }

        if args.is_empty() {
            return Err(Error::Empty);
        }

        Ok(Command { args })
    }

    /// Returns this command's path. This is equivalent to the first argument.
    fn path(&self) -> &str {
        self.args[0]
    }
}

/// Starts a shell using `prefix` as the prefix for each line. This function
/// returns if the `exit` command is called.
pub fn shell(prefix: &str) -> !{
    // A 512-byte buffer for command input.
    let mut line: [u8; 512] = [0; 512];

    loop {
        // Print the prompt.
        kprint!("{}", prefix);
        let mut pos = 0;

        // Read input one byte at a time.
        loop {
            let byte = CONSOLE.lock().read_byte();
            match byte {
                // Accept both '\r' and '\n' as Enter.
                b'\r' | b'\n' => {
                    kprint!("\r\n");
                    break;
                }
                // Handle backspace (8) and delete (127).
                8 | 127 => {
                    if pos > 0 {
                        pos -= 1;
                        // Erase the last character on the terminal.
                        kprint!("\x08 \x08");
                    } else {
                        // Cannot erase past the prompt.
                        kprint!("\x07");
                    }
                }
                // Ring the bell for other non-visible characters.
                b if b < 32 => {
                    kprint!("\x07");
                }
                // Normal character.
                _ => {
                    if pos < line.len() {
                        line[pos] = byte;
                        pos += 1;
                        // Echo the character.
                        kprint!("{}", byte as char);
                    } else {
                        // Command is too long.
                        kprint!("\x07");
                    }
                }
            }
        }

        // Convert the input to a UTF-8 string.
        let input = core::str::from_utf8(&line[..pos]).unwrap_or("");

        // If the input is empty, show a new prompt.
        if input.is_empty() {
            continue;
        }

        // Create a buffer for at most 64 arguments.
        let mut args_buf: [&str; 64] = [""; 64];
        match Command::parse(input, &mut args_buf) {
            Err(Error::Empty) => continue,
            Err(Error::TooManyArgs) => {
                kprintln!("error: too many arguments");
                continue;
            }
            Ok(cmd) => {
                let cmd_name = cmd.path();
                if cmd_name == "echo" {
                    // Built-in echo: print all arguments after "echo".
                    let mut first = true;
                    for arg in cmd.args.iter().skip(1) {
                        if !first {
                            kprint!(" ");
                        }
                        first = false;
                        kprint!("{}", arg);
                    }
                    kprintln!("");
                } else if cmd_name == "exit" {
                    // Built-in exit: leave the shell.
                    kprintln!("exited.");
                    break;
                } else {
                    // Unknown command.
                    kprintln!("unknown command: {}", cmd_name);
                }
            }
        }
    }
    loop{};
}
