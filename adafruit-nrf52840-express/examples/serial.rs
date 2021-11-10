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
    prelude::*,
    hal::{
        Delay,
        uarte::{Parity, Baudrate}
    },
    Pins,
};
use core::fmt::Write;

#[entry]
fn main() -> ! {
    let cp = bsp::core::Peripherals::take().unwrap();
    let dp = bsp::hal::pac::Peripherals::take().unwrap();

    let mut delay = Delay::new(cp.SYST);

    let pins = Pins::new(dp.P0, dp.P1);

    let mut uart = serial::init(
        dp.UARTE0,
        pins.rx,
        pins.tx,
        Baudrate::BAUD115200,
        Parity::EXCLUDED
    );

    loop {
        write!(uart, "Hello World!\r\n").unwrap();

        delay.delay_ms(1000u16);
    }
}
