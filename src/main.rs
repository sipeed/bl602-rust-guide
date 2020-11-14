#![no_std]
#![no_main]

use bl602_hal::{serial::*, pac, prelude::*};

use panic_halt as _;

#[riscv_rt::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    // enable clock
    // let clock = 160_000_000 as u32;
    let uart_clk_div = 3; // reset
    dp.GLB.clk_cfg2.write(|w| unsafe { w
        .uart_clk_div().bits(uart_clk_div)
        .uart_clk_en().set_bit()
    });
    // calculate baudrate
    let baudrate_divisor = 2000;  // 160M / 4 / 2000 = 20K baud
    dp.UART.uart_bit_prd.write(|w| unsafe { w
        .cr_urx_bit_prd().bits(baudrate_divisor - 1)
        .cr_utx_bit_prd().bits(baudrate_divisor - 1)
    });
    // no bit inverse
    dp.UART.data_config.write(|w| w
        .cr_uart_bit_inv().clear_bit()
    );
    
    let parts = dp.GLB.split();
    let pin16 = parts.pin16.into_uart_sig0();
    let pin7 = parts.pin7.into_uart_sig7();
    let mux0 = parts.uart_mux0.into_uart0_tx();
    let mux7 = parts.uart_mux7.into_uart0_rx();
    let mut serial = Serial::uart0(
        dp.UART,
        Config::default().baudrate(20000.Bd()),
        ((pin16, mux0), (pin7, mux7))
    );
    loop {
        serial.try_write(b'R').ok();
        serial.try_flush().ok();
        serial.try_write(b'U').ok();
        serial.try_flush().ok();
        serial.try_write(b'S').ok();
        serial.try_flush().ok();
        serial.try_write(b'T').ok();
        serial.try_flush().ok();
    }
}
