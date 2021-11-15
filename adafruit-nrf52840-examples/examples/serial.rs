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
    Board,
};
use core::fmt::Write;

#[entry]
fn main() -> ! {
    let board = Board::new().unwrap();

    let mut delay = board.delay;
    let mut serial = board.serial;

    loop {
        #[cfg(feature = "express")]
        write!(serial, "Hello Express!\r\n").unwrap();
        #[cfg(feature = "sense")]
        write!(serial, "Hello Sense!\r\n").unwrap();
        delay.delay_ms(1000u16);
    }
}
