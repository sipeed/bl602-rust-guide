#![no_std]
#![no_main]

use bl602_hal::pac;

use panic_halt as _;

/*
    bl_gpio_enable_output(DEMO_GPIO_PIN, 0, 0);
    
    log_debug("led on");
    bl_gpio_output_set(DEMO_GPIO_PIN, 1); // 0x40000188
*/

#[riscv_rt::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    dp.GLB.gpio_cfgctl32.modify(|_, w| w.reg_gpio_17_o().set_bit());
    loop {}
}
