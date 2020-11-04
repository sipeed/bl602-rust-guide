#![no_std]
#![no_main]

use bl602_hal::{pac, prelude::*};

use panic_halt as _;

#[riscv_rt::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let parts = dp.GLB.split();
    let mut gpio5 = parts.pin5.into_push_pull_output();
    gpio5.try_set_high().unwrap();
    loop {
        use riscv::register::mcycle;
        let t0 = mcycle::read64();
        while mcycle::read64().wrapping_sub(t0) <= 50_000_000 { }
        gpio5.try_toggle().unwrap();
    }
}
