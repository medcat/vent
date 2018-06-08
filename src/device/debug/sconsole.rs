use super::Device;
use kvm::core::IoAddress::Port;
use kvm::core::{IoAction, IoAddress};
use std::io::Write;
use std::sync::Mutex;

#[derive(Debug)]
pub struct SerialConsole(u64, Mutex<Serial>);

#[derive(Debug)]
struct Serial {
    /// - bit 0: Data Ready
    /// - bit 1: Overrun Error
    /// - bit 2: Parity Error
    /// - bit 3: Framing Error
    /// - bit 4: Break Interrupt
    /// - bit 5: Empty Transmitter Holding Register
    /// - bit 6: Empty Data Holding Registers
    /// - bit 7: Error in Received FIFO
    status: u8,
    divisor: (u8, u8),
    /// This has a few bits that are important in it.
    ///
    /// - bit 0, bit 1: character length - 0, 0 is 5, 0, 1 is 6, 1, 0 is 7, 1, 1 is 8.
    /// - bit 2: number of stop bits. 0 is 1, 1 is 1.5/2.
    /// - bit 3: parity enable bit.
    /// - bit 4, 5: parity type bit.
    /// - bit 7: DLAB bit; sets port 0/1 to accept DLAB.
    control: u8,
    /// - bit 0: data available interrupt
    /// - bit 1: transmitter empty interrupt
    /// - bit 2: break/error interrupt
    /// - bit 3: status change interrupt
    interrupts: u8,
}

impl Serial {
    fn dlab(&self) -> bool {
        (self.control & (1 << 7)) != 0
    }
}

impl SerialConsole {
    pub fn new(port: u64) -> SerialConsole {
        SerialConsole(
            port,
            Mutex::new(Serial {
                status: 0,
                divisor: (0, 1),
                control: 0b00000011,
                interrupts: 0,
            }),
        )
    }
}

impl Device for SerialConsole {
    fn request(&self) -> Vec<IoAddress> {
        (0..8)
            .map(|v| IoAddress::Port(self.0 + v))
            .collect::<Vec<_>>()
    }

    fn handle(&self, io: IoAction, memory: &mut [u8]) -> Option<()> {
        let base = Port(self.0);
        let mut serial = self.1.lock().unwrap();

        if io == base.inb() && serial.dlab() {
            memory[0] = serial.divisor.1;
            Some(())
        } else if io == base.outb() && serial.dlab() {
            serial.divisor.1 = memory[0];
            Some(())
        } else if io == base.outb() {
            let stderr_basic = ::std::io::stderr();
            let mut stderr = stderr_basic.lock();
            let _ = stderr.write_all(&mut memory[0..1]);
            let _ = stderr.flush();
            Some(())
        } else if io == (base + 1u64).inb() && serial.dlab() {
            memory[0] = serial.divisor.1;
            Some(())
        } else if io == (base + 1u64).outb() && serial.dlab() {
            serial.divisor.0 = memory[0];
            Some(())
        } else if io == (base + 3u64).inb() {
            memory[0] = serial.control;
            Some(())
        } else if io == (base + 3u64).outb() {
            warn!("setting control to {:b}", memory[0]);
            serial.control = memory[0];
            Some(())
        } else if io == (base + 5u64).inb() {
            memory[0] = 0b01100000;
            Some(())
        } else {
            None
        }
    }
}
