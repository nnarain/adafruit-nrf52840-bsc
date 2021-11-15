//
// blink.rs
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

    let mut delay = board.delay;

    #[cfg(feature = "express")]
    let mut led1 = board.led;
    #[cfg(feature = "sense")]
    let mut led1 = board.d13.into_push_pull_output(gpio::Level::Low);

    let mut led2 = board.blue_led;

    led2.set_high().unwrap();

    loop {
        led1.set_high().unwrap();
        led2.set_low().unwrap();

        delay.delay_ms(500u16);

        led1.set_low().unwrap();
        led2.set_high().unwrap();

        delay.delay_ms(500u16);
    }
}
