//
// blink-button.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Nov 10 2021
//
#![no_std]
#![no_main]

use panic_halt as _;

use adafruit_nrf52840_sense as bsp;

use bsp::{
    entry,
    hal::{
        self,
        prelude::*,
    },
    Pins,
};

#[entry]
fn main() -> ! {
    let dp = hal::pac::Peripherals::take().unwrap();

    let pins = Pins::new(dp.P0, dp.P1);
    let mut led = pins.d13.into_push_pull_output(hal::gpio::Level::Low);
    let switch = pins.switch;

    loop {
        if switch.is_low().unwrap() {
            led.set_high().unwrap();
        }
        else {
            led.set_low().unwrap();
        }
    }
}
