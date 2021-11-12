//
// blink-button.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Nov 09 2021
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
    hal::{self, prelude::*},
    Pins,
};

#[entry]
fn main() -> ! {
    let dp = bsp::hal::pac::Peripherals::take().unwrap();

    let pins = Pins::new(dp.P0, dp.P1);

    let button = pins.switch;

    #[cfg(feature = "express")]
    let mut led = pins.led;
    #[cfg(feature = "sense")]
    let mut led = pins.d13.into_push_pull_output(hal::gpio::Level::Low);

    loop {
        if button.is_low().unwrap() {
            led.set_high().unwrap();
        }
        else {
            led.set_low().unwrap();
        }
    }
}
