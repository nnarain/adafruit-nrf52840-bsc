//
// proximity.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Nov 10 2021
//
#![no_std]
#![no_main]

use nb::block;

use adafruit_nrf52840_sense as bsp;

use bsp::{
    entry,
    prelude::*,
    hal::gpio,
    Board,
    sensors::apds9960::Apds9960,
};
use core::fmt::Write;

#[entry]
fn main() -> ! {
    let board = Board::new().unwrap();

    let mut delay = board.delay;
    let mut led = board.d13.into_push_pull_output(gpio::Level::Low);
    let mut serial = board.serial;
    let i2c = board.i2c;

    let mut sensor = Apds9960::new(i2c.acquire_i2c());
    sensor.enable().unwrap();
    sensor.enable_proximity().unwrap();
    sensor.enable_light().unwrap();

    loop {
        // Blink light to show activity
        led.set_high().unwrap();
        delay.delay_ms(50u16);
        led.set_low().unwrap();
        delay.delay_ms(50u16);

        let prox = block!(sensor.read_proximity()).unwrap();
        let light = block!(sensor.read_light()).unwrap();
        write!(serial, "Proximity: {}, Light: {:?}\r\n", prox, light).unwrap();
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
