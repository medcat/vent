use super::Device;
use kvm::core::IoAction;
use kvm::core::IoAddress;
use libc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

#[derive(Debug)]
pub struct Cmos(AtomicUsize, Mutex<Vec<u8>>);

const CMOS_RAM_INDEX: IoAddress = IoAddress::Port(0x70);
const CMOS_RAM_DATA: IoAddress = IoAddress::Port(0x71);

const RTC_SECONDS: u8 = 0x00;
const RTC_MINUTES: u8 = 0x02;
const RTC_HOURS: u8 = 0x04;
const RTC_DAY_OF_WEEK: u8 = 0x06;
const RTC_DAY_OF_MONTH: u8 = 0x07;
const RTC_MONTH: u8 = 0x08;
const RTC_CENTURY: u8 = 0x32;
const RTC_REG_C: u8 = 0x0C;
const RTC_REG_D: u8 = 0x0D;

fn gmtime() -> libc::tm {
    let mut time = unsafe { ::std::mem::zeroed() };
    unsafe { libc::time(&mut time) };
    let mut gmtime = unsafe { ::std::mem::zeroed() };
    unsafe { libc::gmtime_r(&time, &mut gmtime) };
    gmtime
}

/// This takes a binary value and "encodes" the binary value in BCD.
/// Essentially, each decimal digit gets its own set of bits, so e.g.
/// the first 4 bits of the output will be the "ones" column in
/// decimal, and the second 4 bits of the output will be the "tens"
/// column.  We can do this by first dividing the value by 10, then
/// shifting left four, and adding the remainder.  This only works with
/// 8-bit values, though, and limits the output to two decimal places.
fn encode(value: u32) -> u8 {
    (((value / 10) << 4) | (value % 10)) as u8
}

impl Cmos {
    pub fn new() -> Cmos {
        Cmos(AtomicUsize::new(0), Mutex::new(vec![0u8; 128]))
    }
}

impl Device for Cmos {
    fn request(&self) -> Vec<IoAddress> {
        vec![CMOS_RAM_INDEX, CMOS_RAM_DATA]
    }

    fn handle(&self, io: IoAction, memory: &mut [u8]) -> Option<()> {
        if io == CMOS_RAM_INDEX.outb() {
            self.0
                .store((memory[0] & !(1 << 7)) as usize, Ordering::SeqCst);
            Some(())
        } else if io == CMOS_RAM_DATA.inb() {
            let address = self.0.load(Ordering::SeqCst) as u8;
            let time = gmtime();

            match address {
                RTC_SECONDS => memory[0] = encode(time.tm_sec as u32),
                RTC_MINUTES => memory[0] = encode(time.tm_min as u32),
                RTC_HOURS => memory[0] = encode(time.tm_hour as u32),
                RTC_DAY_OF_WEEK => memory[0] = encode((time.tm_wday + 1) as u32),
                RTC_DAY_OF_MONTH => memory[0] = encode(time.tm_mday as u32),
                RTC_MONTH => memory[0] = encode(((time.tm_year + 1900) % 100) as u32),
                RTC_CENTURY => memory[0] = encode(((time.tm_year + 1900) / 100) as u32),
                _ => {
                    let data = self.1.lock().unwrap();
                    memory[0] = data[address as usize];
                }
            }
            Some(())
        } else if io == CMOS_RAM_DATA.outb() {
            let address = self.0.load(Ordering::SeqCst) as u8;

            match address {
                RTC_REG_C | RTC_REG_D => {}
                _ => {
                    let mut data = self.1.lock().unwrap();
                    data[address as usize] = memory[0];
                }
            }

            Some(())
        } else {
            None
        }
    }
}
