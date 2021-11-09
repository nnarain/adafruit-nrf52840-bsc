#![no_std]
#![no_main]

use panic_halt as _;

use adafruit_nrf52840_sense_bsc as bsc;

use bsc::{
    entry,
    hal::{
        self,
        prelude::*,
    },
    Pins,
};

#[entry]
fn main() -> ! {
    let cp = bsc::core::Peripherals::take().unwrap();
    let dp = hal::pac::Peripherals::take().unwrap();

    let mut delay = hal::Delay::new(cp.SYST);

    let pins = Pins::new(dp.P0, dp.P1);
    let mut led = pins.d13.into_push_pull_output(hal::gpio::Level::Low);

    loop {
        led.set_high().unwrap();
        delay.delay_ms(1000u16);
        led.set_low().unwrap();
        delay.delay_ms(1000u16);
    }
}
