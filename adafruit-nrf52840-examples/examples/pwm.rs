//
// pwm.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Nov 14 2021
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
    hal::{
        self,
        gpio,
        pwm::{Pwm, Channel},
        timer::{self, Timer},
    },
    Pins,
    nb::block,
};

#[entry]
fn main() -> ! {
    let dp = hal::pac::Peripherals::take().unwrap();

    let pins = Pins::new(dp.P0, dp.P1);

    let mut timer = Timer::new(dp.TIMER0);

    #[cfg(feature = "express")]
    let led1 = pins.led;
    #[cfg(feature = "sense")]
    let led1 = pins.d13.into_push_pull_output(gpio::Level::Low);

    let led2 = pins.blue_led;

    let pwm = Pwm::new(dp.PWM0);
    pwm.set_period(500u32.hz());
    pwm.set_output_pin(Channel::C0, led1.degrade());
    pwm.set_output_pin(Channel::C1, led2.degrade());

    let max_duty = pwm.get_max_duty() as u32;
    let wait_time = 1_000_000u32 / max_duty;
    loop {
        for duty in 0..pwm.get_max_duty() {
            pwm.set_duty_on(Channel::C0, duty);
            pwm.set_duty_on(Channel::C1, (max_duty as u16) - duty);
            delay(&mut timer, wait_time);
        }
    }
}

fn delay<T>(timer: &mut Timer<T>, cycles: u32)
where
    T: timer::Instance,
{
    timer.start(cycles);
    let _ = block!(timer.wait());
}
