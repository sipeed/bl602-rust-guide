#![no_std]
#![no_main]

use panic_halt as _;
use bl602_hal as hal;
use hal::{
    clock::{self, SysclkFreq, UART_PLL_FREQ},
    pac,
    prelude::*,
};

#[riscv_rt::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut parts = dp.GLB.split();
    let clocks = clock::Strict::new()
        .use_pll(40_000_000u32.Hz())
        .sys_clk(SysclkFreq::Pll160Mhz)
        .uart_clk(UART_PLL_FREQ.Hz())
        .freeze(&mut parts.clk_cfg);
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
    // 8N1
    /* 4->5b 5->6b 6->7b 7->8b */
    let data_bits_cfg = 7; // 8 bits
    /* 0->0.5b 1->1b 2->1.5b 3->2b */
    let stop_bits_cfg = 1; // todo: check this parameter
    dp.UART.utx_config.write(|w| unsafe { w
        .cr_utx_prt_en().clear_bit() // parity: none
        .cr_utx_bit_cnt_d().bits(data_bits_cfg)
        .cr_utx_bit_cnt_p().bits(stop_bits_cfg) 
        .cr_utx_frm_en().set_bit() // freerun on
        // freerun off
        .cr_utx_cts_en().clear_bit() // no CTS
        .cr_utx_en().set_bit() // enable TX
    });
    dp.UART.urx_config.write(|w| unsafe { w
        .cr_urx_prt_en().clear_bit() // parity: none
        .cr_urx_bit_cnt_d().bits(data_bits_cfg)
        .cr_urx_deg_en().clear_bit() // no rx input de-glitch
        .cr_urx_rts_sw_mode().clear_bit() // no RTS
        .cr_urx_en().set_bit() // enable RX
    });
    let pin16 = parts.pin16.into_uart_sig0();
    let pin7 = parts.pin7.into_uart_sig7();
    let mux0 = parts.uart_mux0.into_uart0_tx();
    let mux7 = parts.uart_mux7.into_uart0_rx();
    loop {
        // write data
        while dp.UART.uart_fifo_config_1.read().tx_fifo_cnt().bits() < 1 {}
        dp.UART.uart_fifo_wdata.write(|w| unsafe {
            w.bits(b'R' as u32)
        });
        while dp.UART.uart_fifo_config_1.read().tx_fifo_cnt().bits() < 1 {}
        dp.UART.uart_fifo_wdata.write(|w| unsafe {
            w.bits(b'U' as u32)
        });
        while dp.UART.uart_fifo_config_1.read().tx_fifo_cnt().bits() < 1 {}
        dp.UART.uart_fifo_wdata.write(|w| unsafe {
            w.bits(b'S' as u32)
        });
        while dp.UART.uart_fifo_config_1.read().tx_fifo_cnt().bits() < 1 {}
        dp.UART.uart_fifo_wdata.write(|w| unsafe {
            w.bits(b'T' as u32)
        });
    }
}
