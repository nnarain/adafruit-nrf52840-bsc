//
// blink-button.rs
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
    hal::prelude::*,
    Pins,
};

#[entry]
fn main() -> ! {
    let dp = bsp::hal::pac::Peripherals::take().unwrap();

    let pins = Pins::new(dp.P0, dp.P1);

    let button = pins.switch;
    let mut led = pins.led;

    loop {
        if button.is_low().unwrap() {
            led.set_high().unwrap();
        }
        else {
            led.set_low().unwrap();
        }
    }
}
