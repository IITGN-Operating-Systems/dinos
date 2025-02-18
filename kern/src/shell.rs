use core::str;
use stack_vec::StackVec;
use crate::console::{kprint, kprintln, CONSOLE};

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
    /// Parse a command from a string `s` using `buf` as storage for the arguments.
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
        // We assume that the command is non-empty (see `parse`).
        self.args[0]
    }
}

/// Starts a shell using `prefix` as the prefix for each line. This function
/// returns if the `exit` command is called.
pub fn shell(prefix: &str) -> ! {
    // A fixed-size buffer for input (adjust the size as needed).
    let mut input_buffer = [0u8; 128];

    loop {
        {
            // Print the prompt.
            kprint!("{}", prefix);
            kprint!("> ");
        }

        // Read a line into `input_buffer`.
        let mut i = 0;
        {
            // Lock the console for reading input.
            let mut console = CONSOLE.lock();
            loop {
                let byte = console.read_byte();
                // Echo the received byte.
                console.write_byte(byte);
                if byte == b'\r' || byte == b'\n' {
                    break;
                }
                if i < input_buffer.len() - 1 {
                    input_buffer[i] = byte;
                    i += 1;
                }
            }
        }
        // Convert the bytes read into a &str.
        let input_line = match str::from_utf8(&input_buffer[..i]) {
            Ok(s) => s,
            Err(_) => {
                kprintln!("\nError: invalid UTF-8");
                continue;
            }
        };

        // Prepare a buffer for splitting the input into arguments.
        let mut args_storage: [&str; 8] = [""; 8];
        match Command::parse(input_line, &mut args_storage) {
            Ok(cmd) => {
                if cmd.path() == "exit" {
                    break;
                } else {
                    // For demonstration, just print the command path and arguments.
                    kprintln!("\nCommand: {}", cmd.path());
                    for (idx, arg) in cmd.args.iter().enumerate() {
                        kprintln!("Arg {}: {}", idx, arg);
                    }
                }
            }
            Err(Error::Empty) => {
                // Do nothing for an empty command.
            }
            Err(Error::TooManyArgs) => {
                kprintln!("\nError: too many arguments");
            }
        }
    }

    // After the exit command, halt by looping indefinitely.
    loop {}
}