//
// serial.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Nov 11 2021
//
#![no_std]
#![no_main]

use panic_halt as _;

#[cfg(feature = "express")]
use adafruit_nrf52840_express as bsp;

#[cfg(feature = "sense")]
use adafruit_nrf52840_sense as bsp;

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
        #[cfg(feature = "express")]
        write!(uart, "Hello Express!\r\n").unwrap();
        #[cfg(feature = "sense")]
        write!(uart, "Hello Sense!\r\n").unwrap();
        delay.delay_ms(1000u16);
    }
}
