//
// magnetometer.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Nov 10 2021
//
#![no_std]
#![no_main]

use adafruit_nrf52840_sense as bsp;

use bsp::{
    entry,
    prelude::*,
    hal::gpio,
    Board,
    sensors::lis3mdl::{Lis3mdl, Address},
};
use core::fmt::Write;

#[entry]
fn main() -> ! {
    let board = Board::new().unwrap();

    let mut delay = board.delay;
    let mut led = board.d13.into_push_pull_output(gpio::Level::Low);
    let mut serial = board.serial;

    let i2c = board.i2c;

    let magnetometer = Lis3mdl::new(i2c.acquire_i2c(), Address::Addr1C);

    if let Err(e) = magnetometer {
        write!(serial, "Error: {:?}", e).unwrap();
        loop {}
    }

    let mut magnetometer = magnetometer.unwrap();

    loop {
        // Blink light to show activity
        led.set_high().unwrap();
        delay.delay_ms(50u16);
        led.set_low().unwrap();
        delay.delay_ms(50u16);

        let data = magnetometer.get_mag_axes_mgauss().unwrap();

        write!(serial, "x: {}, y: {}, z: {}\r\n", data.x, data.y, data.z).unwrap();
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
