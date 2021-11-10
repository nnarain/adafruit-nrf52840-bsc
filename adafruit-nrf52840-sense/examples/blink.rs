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
    let cp = bsp::core::Peripherals::take().unwrap();
    let dp = hal::pac::Peripherals::take().unwrap();

    let mut delay = hal::Delay::new(cp.SYST);

    let pins = Pins::new(dp.P0, dp.P1);
    let mut led = pins.d13.into_push_pull_output(hal::gpio::Level::Low);

    let mut blue_led = pins.blue_led;
    blue_led.set_high().unwrap();

    loop {
        led.set_high().unwrap();
        blue_led.set_low().unwrap();

        delay.delay_ms(1000u16);
    
        led.set_low().unwrap();
        blue_led.set_high().unwrap();

        delay.delay_ms(1000u16);
    }
}
