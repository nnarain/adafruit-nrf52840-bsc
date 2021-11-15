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
    prelude::*,
    hal::gpio,
    Board,
};

#[entry]
fn main() -> ! {
    let board = Board::new().unwrap();

    let button = board.switch;

    #[cfg(feature = "express")]
    let mut led = board.led;
    #[cfg(feature = "sense")]
    let mut led = board.d13.into_push_pull_output(gpio::Level::Low);

    loop {
        if button.is_low().unwrap() {
            led.set_high().unwrap();
        }
        else {
            led.set_low().unwrap();
        }
    }
}
