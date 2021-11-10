//
// blink.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Nov 09 2021
//
#![no_std]
#![no_main]

use panic_halt as _;

use adafruit_nrf52840_express as bsp;

use bsp::{
    entry,
    hal::{self, prelude::*, gpio, uarte},
    Pins,
};
use core::fmt::Write;

#[entry]
fn main() -> ! {
    let cp = bsp::core::Peripherals::take().unwrap();
    let dp = bsp::hal::pac::Peripherals::take().unwrap();

    let mut delay = hal::Delay::new(cp.SYST);

    let pins = Pins::new(dp.P0, dp.P1);

    let (rx, tx) = (
        pins.rx.into_floating_input().degrade(),
        pins.tx.into_push_pull_output(gpio::Level::High).degrade()
    );

    let uart_pins = uarte::Pins {
        rxd: rx,
        txd: tx,
        cts: None,
        rts: None,
    };

    let mut uart = uarte::Uarte::new(
        dp.UARTE0,
        uart_pins,
        uarte::Parity::EXCLUDED,
        uarte::Baudrate::BAUD115200
    );

    loop {
        write!(uart, "Hello World!\r\n").unwrap();
        delay.delay_ms(1000u16);
    }
}
