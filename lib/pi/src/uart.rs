use core::fmt;
use core::time::Duration;

use shim::io;
use shim::const_assert_size;

use volatile::prelude::*;
use volatile::{Volatile, ReadVolatile, Reserved};

use crate::timer;
use crate::common::IO_BASE;
use crate::gpio::{Gpio, Function};

/// The base address for the `MU` registers.
const MU_REG_BASE: usize = IO_BASE + 0x215040;

/// The `AUXENB` register from page 9 of the BCM2837 documentation.
const AUX_ENABLES: *mut Volatile<u8> = (IO_BASE + 0x215004) as *mut Volatile<u8>;

/// Enum representing bit fields of the `AUX_MU_LSR_REG` register.
#[repr(u8)]
enum LsrStatus {
    DataReady = 1,
    TxAvailable = 1 << 5,
}

#[repr(C)]
#[allow(non_snake_case)]
struct Registers {
    AUX_MU_IO_REG: Volatile<u8>,
    __r0: [Reserved<u8>; 3],
    AUX_MU_IER_REG: Volatile<u8>,
    __r1: [Reserved<u8>; 3],
    AUX_MU_IIR_REG: Volatile<u8>,
    __r2: [Reserved<u8>; 3],
    AUX_MU_LCR_REG: Volatile<u8>,
    __r3: [Reserved<u8>; 3],
    AUX_MU_MCR_REG: Volatile<u8>,
    __r4: [Reserved<u8>; 3],
    AUX_MU_LSR_REG: ReadVolatile<u8>,
    __r5: [Reserved<u8>; 3],
    AUX_MU_MSR_REG: ReadVolatile<u8>,
    __r6: [Reserved<u8>; 3],
    AUX_MU_SCRATCH: Volatile<u8>,
    __r7: [Reserved<u8>; 3],
    AUX_MU_CNTL_REG: Volatile<u8>,
    __r8: [Reserved<u8>; 3],
    AUX_MU_STAT_REG: ReadVolatile<u32>,
    __r9: [Reserved<u8>; 3],
    AUX_MU_BAUD: Volatile<u16>,
}

/// The Raspberry Pi's "mini UART".
pub struct MiniUart {
    registers: &'static mut Registers,
    timeout: Option<Duration>,
}

impl MiniUart {
    pub fn new() -> MiniUart {
        let registers = unsafe {
            (*AUX_ENABLES).or_mask(1);
            &mut *(MU_REG_BASE as *mut Registers)
        };

        Gpio::new(14).into_alt(Function::Alt5);
        Gpio::new(15).into_alt(Function::Alt5);

        registers.AUX_MU_LCR_REG.write(0b11);
        registers.AUX_MU_BAUD.write(270);
        registers.AUX_MU_CNTL_REG.write(0b11);

        MiniUart {
            registers,
            timeout: None,
        }
    }

    pub fn set_read_timeout(&mut self, t: Duration) {
        self.timeout = Some(t);
    }

    pub fn write_byte(&mut self, byte: u8) {
        while !self.registers.AUX_MU_LSR_REG.has_mask(LsrStatus::TxAvailable as u8) {}
        self.registers.AUX_MU_IO_REG.write(byte);
    }

    pub fn has_byte(&self) -> bool {
        self.registers.AUX_MU_LSR_REG.has_mask(LsrStatus::DataReady as u8)
    }

    pub fn wait_for_byte(&self) -> Result<(), ()> {
        match self.timeout {
            None => loop {
                if self.has_byte() {
                    return Ok(());
                }
            },
            Some(timeout) => {
                let deadline = timer::current_time() + timeout;
                while timer::current_time() < deadline {
                    if self.has_byte() {
                        return Ok(());
                    }
                }
                Err(())
            }
        }
    }

    pub fn read_byte(&mut self) -> u8 {
        while !self.has_byte() {}
        self.registers.AUX_MU_IO_REG.read()
    }
}

impl fmt::Write for MiniUart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            match byte {
                b'\n' => {
                    self.write_byte(b'\r');
                    self.write_byte(b'\n');
                }
                _ => self.write_byte(byte),
            }
        }
        Ok(())
    }
}

impl io::Read for MiniUart {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self.wait_for_byte() {
            Ok(()) => {
                let mut index = 0;
                while self.has_byte() && index < buf.len() {
                    buf[index] = self.read_byte();
                    index += 1;
                }
                Ok(index)
            }
            Err(()) => Err(io::Error::new(io::ErrorKind::TimedOut, "reading UART timed out")),
        }
    }
}

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
