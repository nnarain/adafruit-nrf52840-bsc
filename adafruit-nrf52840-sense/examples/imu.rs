//
// imu.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Nov 12 2021
//
#![no_std]
#![no_main]

use adafruit_nrf52840_sense as bsp;

use bsp::{
    entry,
    prelude::*,
    hal::gpio,
    Board,
    sensors::lsm6ds33::{Lsm6ds33, AccelerometerOutput, GyroscopeOutput, GyroscopeFullScale},
};
use core::fmt::Write;

#[entry]
fn main() -> ! {
    let board = Board::new().unwrap();

    let mut delay = board.delay;
    let mut led = board.d13.into_push_pull_output(gpio::Level::Low);
    let mut serial = board.serial;
    let i2c = board.i2c;

    let mut imu = Lsm6ds33::new(i2c.acquire_i2c(), 0x6A).unwrap();
    imu.set_accelerometer_output(AccelerometerOutput::Rate6_66k).unwrap();
    imu.set_gyroscope_output(GyroscopeOutput::Rate6_66k).unwrap();
    imu.set_gyroscope_scale(GyroscopeFullScale::Dps2000).unwrap();

    loop {
        // Blink light to show activity
        led.set_high().unwrap();
        delay.delay_ms(50u16);
        led.set_low().unwrap();
        delay.delay_ms(50u16);

        if let Ok(available) = imu.accel_data_available() {
            if available {
                let (x, y, z) = imu.read_accelerometer().unwrap();
                write!(serial, "Accel[{}, {}, {}]\r\n", x, y, z).unwrap();
            }
        }

        if let Ok(available) = imu.gyro_data_available() {
            if available {
                let (x, y, z) = imu.read_gyro().unwrap();
                write!(serial, "Gyro[{}, {}, {}]\r\n", x, y, z).unwrap();
            }
        }
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
