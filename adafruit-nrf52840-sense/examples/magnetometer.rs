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
    hal::{
        self,
        twim::{self, Twim},
        uarte::{Baudrate, Parity},
    },
    Pins,
    sensors::lis3mdl::{Lis3mdl, Address},
};
use core::fmt::Write;

#[entry]
fn main() -> ! {
    let cp = bsp::core::Peripherals::take().unwrap();
    let dp = hal::pac::Peripherals::take().unwrap();

    let mut delay = hal::Delay::new(cp.SYST);

    let pins = Pins::new(dp.P0, dp.P1);
    let mut led = pins.d13.into_push_pull_output(hal::gpio::Level::Low);

    let mut uart = serial::init(
        dp.UARTE0,
        pins.rx,
        pins.tx,
        Baudrate::BAUD115200,
        Parity::EXCLUDED
    );

    let twim_pins = twim::Pins{
        scl: pins.scl.into_floating_input().degrade(),
        sda: pins.sda.into_floating_input().degrade()
    };
    let i2c = Twim::new(dp.TWIM0, twim_pins, twim::Frequency::K400);

    let magnetometer = Lis3mdl::new(i2c, Address::Addr1C);

    if let Err(e) = magnetometer {
        write!(uart, "Error: {:?}", e).unwrap();
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

        write!(uart, "x: {}, y: {}, z: {}\r\n", data.x, data.y, data.z).unwrap();
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
