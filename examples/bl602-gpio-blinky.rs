#![no_std]
#![no_main]

use panic_halt as _;
use bl602_hal as hal;
use hal::{
    clock::{self, SysclkFreq},
    pac,
    prelude::*,
};

#[riscv_rt::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut parts = dp.GLB.split();
    let _clocks = clock::Strict::new()
        .use_pll(40_000_000u32.Hz())
        .sys_clk(SysclkFreq::Pll160Mhz)
        .freeze(&mut parts.clk_cfg);
    let mut gpio5 = parts.pin5.into_pull_down_output();
    gpio5.try_set_high().unwrap();
    loop {
        use riscv::register::mcycle;
        let t0 = mcycle::read64();
        while mcycle::read64().wrapping_sub(t0) <= 50_000_000 { }
        gpio5.try_toggle().unwrap();
    }
}
