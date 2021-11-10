//
// serial.rs
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
    hal,
    Pins,
};

#[entry]
fn main() -> ! {
    let cp = bsp::core::Peripherals::take().unwrap();
    let dp = bsp::hal::pac::Peripherals::take().unwrap();

    let mut delay = hal::Delay::new(cp.SYST);

    let pins = Pins::new(dp.P0, dp.P1);

    let mut led1 = pins.led;
    let mut led2 = pins.blue_led;

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
