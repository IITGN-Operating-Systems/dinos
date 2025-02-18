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
    fn parse(s: &'a str, buf: &'a mut [&'a str]) -> Result<Command<'a>, Error> {
        let mut args = StackVec::new(buf);
        for arg in s.split_whitespace() {
            args.push(arg).map_err(|_| Error::TooManyArgs)?;
        }
        if args.is_empty() {
            return Err(Error::Empty);
        }
        Ok(Command { args })
    }

    /// Returns this command's path, equivalent to the first argument.
    fn path(&self) -> &str {
        self.args[0]
    }
}

/// Starts a shell using `prefix` as the prefix for each line. This function returns if `exit` is called.
pub fn shell(prefix: &str) -> ! {
    let mut buffer = [0u8; 512];
    let mut input_buf = [""; 64];
    
    loop {
        kprint!("{}", prefix);
        let mut buf_index = 0;

        loop {
            let byte = CONSOLE.lock().read_byte();

            match byte {
                b'\n' | b'\r' => {  // Enter key
                    kprintln!("");
                    break;
                }
                8 | 127 => {  // Backspace or delete
                    if buf_index > 0 {
                        buf_index -= 1;
                        kprint!("\u{8} \u{8}"); // Backspace, space, backspace
                    }
                }
                32..=126 => {  // Printable characters
                    if buf_index < buffer.len() {
                        buffer[buf_index] = byte;
                        buf_index += 1;
                        kprint!("{}", byte as char);
                    }
                }
                7 => kprint!("\u{7}"), // Bell character
                _ => {} // Ignore unknown characters
            }
        }

        let input = core::str::from_utf8(&buffer[..buf_index]).unwrap_or("");
        match Command::parse(input, &mut input_buf) {
            Ok(command) => {
                match command.path() {
                    "echo" => {
                        let args = &command.args[1..];
                        kprintln!("{}", args.join(" "));
                    }
                    "exit" => return,
                    _ => kprintln!("unknown command: {}", command.path()),
                }
            }
            Err(Error::TooManyArgs) => kprintln!("error: too many arguments"),
            Err(Error::Empty) => continue,
        }
    }
}
