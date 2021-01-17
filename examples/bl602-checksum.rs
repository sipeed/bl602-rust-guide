#![no_std]
#![no_main]

use bl602_hal::checksum::{Checksum, Endianness};
use bl602_hal::pac;

use panic_halt as _;

#[riscv_rt::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let checksum = Checksum::new(dp.CKS, Endianness::Little);

    checksum.write(&[
        0x45, 0x00, 0x00, 0x73, 0x00, 0x00, 0x40, 0x00, 0x40, 0x11, 0x00, 0x00, 0xc0, 0xa8, 0x00,
        0x01, 0xc0, 0xa8, 0x00, 0xc7,
    ]);

    let result = checksum.result();

    assert_eq!(result, u16::from_be_bytes([0xb8, 0x61]));

    loop {}
}
