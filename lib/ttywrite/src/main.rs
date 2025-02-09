extern crate serial;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;
extern crate xmodem;
use std::path::PathBuf;
use std::time::Duration;
use structopt::StructOpt;
use serial::core::{BaudRate, CharSize, FlowControl, SerialDevice, SerialPortSettings, StopBits};
use xmodem::{Progress, Xmodem};
use std::fs::File;
use std::io::{self, BufReader};

#[derive(StructOpt, Debug)]
#[structopt(about = "Write to TTY using the XMODEM protocol by default.")]
struct Opt {
    #[structopt(short = "i", help = "Input file (defaults to stdin if not set)")]
    input: Option<PathBuf>,

    #[structopt(short = "b", long = "baud", help = "Set baud rate", default_value = "115200")]
    baud_rate: String,  

    #[structopt(short = "t", long = "timeout", help = "Set timeout in seconds", default_value = "10")]
    timeout: u64,

    #[structopt(short = "w", long = "width", help = "Set data character width in bits", default_value = "8")]
    char_width: String, 

    #[structopt(help = "Path to TTY device")]
    tty_path: PathBuf,

    #[structopt(short = "f", long = "flow-control", help = "Enable flow control", default_value = "none")]
    flow_control: String,  

    #[structopt(short = "s", long = "stop-bits", help = "Set number of stop bits", default_value = "1")]
    stop_bits: String,  

    #[structopt(short = "r", long = "raw", help = "Disable XMODEM")]
    raw: bool,
}

fn progress_fn(progress: Progress) {
    println!("Progress: {:?}", progress);
}


fn main() {

    let opt = Opt::from_args();

    // converting read arguments to respective objects
    let baud_rate = match opt.baud_rate.as_str() {
        "1200" => BaudRate::Baud1200,
        "2400" => BaudRate::Baud2400,
        "4800" => BaudRate::Baud4800,
        "9600" => BaudRate::Baud9600,
        "19200" => BaudRate::Baud19200,
        "38400" => BaudRate::Baud38400,
        "57600" => BaudRate::Baud57600,
        "115200" => BaudRate::Baud115200,
        _ => panic!("Invalid baud rate: {}", opt.baud_rate),
    };

    let char_width = match opt.char_width.as_str() {
        "5" => CharSize::Bits5,
        "6" => CharSize::Bits6,
        "7" => CharSize::Bits7,
        "8" => CharSize::Bits8,
        _ => panic!("Invalid character width: {}", opt.char_width),
    };

    let flow_control = match opt.flow_control.as_str() {
        "none" => FlowControl::FlowNone,
        "software" => FlowControl::FlowSoftware,
        "hardware" => FlowControl::FlowHardware,
        _ => panic!("Invalid flow control: {}", opt.flow_control),
    };

    let stop_bits = match opt.stop_bits.as_str() {
        "1" => StopBits::Stop1,
        "2" => StopBits::Stop2,
        _ => panic!("Invalid stop bits: {}", opt.stop_bits),
    };

    // setting up port for serial communication
    let mut port = serial::open(&opt.tty_path).expect("Invalid path, points to invalid TTY.");

    // read serial settings
    let mut settings = port.read_settings().expect("Invalid serial device.");
    settings.set_baud_rate(baud_rate).expect("Invalid baud rate.");
    settings.set_stop_bits(stop_bits);
    settings.set_flow_control(flow_control);
    settings.set_char_size(char_width);
    // write serial settings
    port.write_settings(&settings).expect("Invalid Settings.");
    port.set_timeout(Duration::from_secs(opt.timeout)).expect("Invalid Timeout.");

    // Raw Mode
    if opt.raw {
        match opt.input {
            // Reading from given file
            Some(ref path) => {
                let mut input = BufReader::new(File::open(path).expect("Invalid file path."));
                io::copy(&mut input, &mut port).expect("Transfer failed.");
            }
            // File not provided, read stdin()
            None => {
                let mut input = io::stdin();
                io::copy(&mut input, &mut port).expect("Transfer failed.");
            }
        };
    
    // XMODEM Mode: 
    } else {
        match opt.input {
            Some(ref path) => {
                let mut input = BufReader::new(File::open(path).expect("Invalid file path."));
                match Xmodem::transmit_with_progress(input, &mut port, progress_fn) {
                    Ok(_) => return,
                    Err(err) => panic!("Error: {:?}", err),
                }
            }
            None => {
                let mut input = io::stdin();
                match Xmodem::transmit_with_progress(input, &mut port, progress_fn) {
                    Ok(_) => return,
                    Err(err) => panic!("Error: {:?}", err),
                }
            }
        }
    }
}
