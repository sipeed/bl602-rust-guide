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
    dp.GLB.gpio_cfgctl2.modify(|_, w| unsafe { w
        .reg_gpio_5_func_sel().bits(11) // GPIO_FUN_SWGPIO
        .reg_gpio_5_ie().clear_bit() // output
        .reg_gpio_5_pu().clear_bit()
        .reg_gpio_5_pd().clear_bit()
        .reg_gpio_5_drv().bits(0) // disabled
        .reg_gpio_5_smt().clear_bit()
    });
    dp.GLB.gpio_cfgctl32.modify(|_, w| w.reg_gpio_5_o().set_bit());
    loop {}
}
