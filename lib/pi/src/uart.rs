use core::fmt;
use core::time::Duration;

use shim::io;
use shim::const_assert_size;

use volatile::prelude::*;
use volatile::{Volatile, ReadVolatile, Reserved, Readable};

use crate::timer;
use crate::common::IO_BASE;
use crate::gpio::{Gpio, Function};

/// The base address for the `MU` registers.
const MU_REG_BASE: usize = IO_BASE + 0x215040;
/// The `AUXENB` register from page 9 of the BCM2837 documentation.
const AUX_ENABLES: *mut Volatile<u8> = (IO_BASE + 0x215004) as *mut Volatile<u8>;

/// Enum representing bit fields of the mini UART Line Status Register.
#[repr(u8)]
enum LsrStatus {
    DataReady   = 1,      // At least one byte in the receive FIFO.
    TxAvailable = 1 << 5, // There is space in the transmit FIFO.
}

#[repr(C)]
#[allow(non_snake_case)]
struct Registers {
    /// 0x00: I/O Data – read/write
    io: Volatile<u32>,
    /// 0x04: Interrupt Enable – read/write
    ier: Volatile<u32>,
    /// 0x08: Interrupt Identify – read/write (used for clearing interrupts)
    iir: Volatile<u32>,
    /// 0x0C: Line Control – read/write
    lcr: Volatile<u32>,
    /// 0x10: Modem Control – read/write
    mcr: Volatile<u32>,
    /// 0x14: Line Status – read-only
    lsr: ReadVolatile<u32>,
    /// 0x18: Modem Status – read-only
    msr: ReadVolatile<u32>,
    /// 0x1C: Scratch – read/write
    scratch: Volatile<u32>,
    /// 0x20: Extra Control (CNTL) – read/write
    cntl: Volatile<u32>,
    /// 0x24: Extra Status (STAT) – read-only
    stat: ReadVolatile<u32>,
    /// 0x28: Baudrate – read/write
    baud: Volatile<u32>,
}

// Optionally ensure the Registers struct is the expected size.
// const_assert_size!(Registers, 44);

/// The Raspberry Pi's "mini UART".
pub struct MiniUart {
    registers: &'static mut Registers,
    timeout: Option<Duration>,
}

impl MiniUart {
    /// Initializes the mini UART by enabling it as an auxiliary peripheral,
    /// setting the data size to 8 bits, setting the BAUD rate to ~115200 (baud
    /// divider of 270), setting GPIO pins 14 and 15 to alternative function 5
    /// (TXD1/RXD1), and finally enabling the UART transmitter and receiver.
    pub fn new() -> MiniUart {
        let registers = unsafe {
            // Enable the mini UART as an auxiliary device.
            (*AUX_ENABLES).or_mask(1);
            &mut *(MU_REG_BASE as *mut Registers)
        };

        let mut uart = MiniUart {
            registers,
            timeout: None,
        };

        // Disable TX/RX during configuration.
        uart.registers.cntl.write(0);
        // Disable interrupts.
        uart.registers.ier.write(0);
        // Set data size to 8 bits (writing 3 to LCR configures 8-bit mode).
        uart.registers.lcr.write(3);
        // Set baud rate to 115200 (divider = 270).
        uart.registers.baud.write(270);
        // Enable transmitter and receiver.
        uart.registers.cntl.write(3);

        // Configure GPIO pins 14 and 15 for alternate function 5 (UART TX/RX).
        // Create separate GPIO handles for each pin.
        let mut gpio14 = Gpio::new(14);
        gpio14.into_alt(Function::Alt5);
        let mut gpio15 = Gpio::new(15);
        gpio15.into_alt(Function::Alt5);

        uart
    }

    /// Set the read timeout to `t` duration.
    pub fn set_read_timeout(&mut self, t: Duration) {
        self.timeout = Some(t);
    }

    /// Write the byte `byte`. This method blocks until there is space available
    /// in the transmit FIFO.
    pub fn write_byte(&mut self, byte: u8) {
        // Wait until TX FIFO has space.
        while (self.registers.lsr.read() & (LsrStatus::TxAvailable as u32)) == 0 {}
        self.registers.io.write(byte as u32);
    }

    /// Returns `true` if there is at least one byte ready to be read.
    pub fn has_byte(&self) -> bool {
        (self.registers.lsr.read() & (LsrStatus::DataReady as u32)) != 0
    }

    /// Blocks until there is a byte ready to read. If a timeout is set, waits
    /// at most that duration.
    pub fn wait_for_byte(&self) -> Result<(), ()> {
        if let Some(timeout) = self.timeout {
            let start = timer::current_time();
            loop {
                if self.has_byte() {
                    return Ok(());
                }
                if timer::current_time().saturating_sub(start) >= timeout {
                    return Err(());
                }
            }
        } else {
            while !self.has_byte() {}
            Ok(())
        }
    }

    /// Reads a byte. Blocks indefinitely until a byte is ready.
    pub fn read_byte(&mut self) -> u8 {
        while self.wait_for_byte().is_err() {}
        self.registers.io.read() as u8
    }
}

impl fmt::Write for MiniUart {
    /// Writes a string to the UART. Inserts a carriage return (`\r`)
    /// before every newline (`\n`).
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            if byte == b'\n' {
                self.write_byte(b'\r');
            }
            self.write_byte(byte);
        }
        Ok(())
    }
}

mod uart_io {
    use super::MiniUart;
    use volatile::Readable;
    use shim::io;
    use shim::io::{Error, ErrorKind};

    /// Implement `io::Read` for MiniUart.
    impl io::Read for MiniUart {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            if buf.is_empty() {
                return Ok(0);
            }
            // Wait for the first byte (with timeout if configured).
            if self.wait_for_byte().is_err() {
                return Err(Error::new(ErrorKind::TimedOut, "UART read timed out"));
            }
            let mut count = 0;
            buf[count] = self.registers.io.read() as u8;
            count += 1;
            // Read any additional bytes that are immediately available.
            while count < buf.len() && self.has_byte() {
                buf[count] = self.registers.io.read() as u8;
                count += 1;
            }
            Ok(count)
        }
    }

    /// Implement `io::Write` for MiniUart.
    impl io::Write for MiniUart {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            for &byte in buf {
                self.write_byte(byte);
            }
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
}
