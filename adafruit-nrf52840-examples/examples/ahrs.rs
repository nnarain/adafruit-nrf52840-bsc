//
// ahrs.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Nov 20 2021
//
#![no_std]
#![no_main]

use panic_halt as _;

use adafruit_nrf52840_sense as bsp;

use bsp::{Board, entry, hal::gpio, prelude::*, sensors::lsm6ds33::{AccelerometerOutput, AccelerometerScale, GyroscopeFullScale, GyroscopeOutput, Lsm6ds33}};

use core::fmt::Write;

use ahrs::{Ahrs, Madgwick};
use nalgebra::Vector3;

#[entry]
fn main() -> ! {
    let board = Board::new().unwrap();

    let mut delay = board.delay;
    let mut led = board.d13.into_push_pull_output(gpio::Level::Low);
    let mut serial = board.serial;
    let i2c = board.i2c;

    let mut imu = Lsm6ds33::new(i2c.acquire_i2c(), 0x6A).unwrap();
    imu.set_accelerometer_output(AccelerometerOutput::Rate104).unwrap();
    imu.set_accelerometer_scale(AccelerometerScale::G04).unwrap();
    imu.set_gyroscope_output(GyroscopeOutput::Rate104).unwrap();
    imu.set_gyroscope_scale(GyroscopeFullScale::Dps2000).unwrap();

    let mut ahrs = Madgwick::default();

    loop {
        // Blink light to show activity
        led.set_high().unwrap();
        delay.delay_ms(50u16);
        led.set_low().unwrap();
        delay.delay_ms(50u16);

        let accel_available = imu.accel_data_available();
        let gyro_available = imu.gyro_data_available();

        if let (Ok(accel_available), Ok(gyro_available)) = (accel_available, gyro_available) {
            if accel_available && gyro_available {
                // TODO: Incomplete. Most likely requires IMU calibration step first.

                let (ax, ay, az) = imu.read_accelerometer().unwrap();
                let (gx, gy, gz) = imu.read_gyro().unwrap();

                let av = Vector3::new(ax as f64, ay as f64, az as f64);
                let gv = Vector3::new(gx as f64, gy as f64, gz as f64);

                let quat = ahrs.update_imu(&gv, &av).unwrap();
                let (roll, pitch, yaw) = quat.euler_angles();

                let (roll, pitch, yaw) = (roll.to_degrees(), pitch.to_degrees(), yaw.to_degrees());

                write!(serial, "Accel:       [{:.4}, {:.4}, {:.4}]\r\n", ax, ay, az).unwrap();
                write!(serial, "Gyro:        [{:.4}, {:.4}, {:.4}]\r\n", gx, gy, gz).unwrap();
                write!(serial, "Orientation: [{:.4}, {:.4}, {:.4}]\r\n", yaw, pitch, roll).unwrap();
            }
        }
    }
}

