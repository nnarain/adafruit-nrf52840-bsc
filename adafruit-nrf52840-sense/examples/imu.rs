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
    hal::{
        self,
        twim::{self, Twim},
        uarte::{Baudrate, Parity},
    },
    Pins,
    sensors::lsm6ds33::{Lsm6ds33, AccelerometerOutput, GyroscopeOutput},
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

    let mut imu = Lsm6ds33::new(i2c, 0x6A).unwrap();
    imu.set_accelerometer_output(AccelerometerOutput::Rate6_66k).unwrap();
    imu.set_gyroscope_output(GyroscopeOutput::Rate6_66k).unwrap();

    loop {
        // Blink light to show activity
        led.set_high().unwrap();
        delay.delay_ms(50u16);
        led.set_low().unwrap();
        delay.delay_ms(50u16);

        if let Ok(available) = imu.accel_data_available() {
            if available {
                let (x, y, z) = imu.read_accel().unwrap();
                write!(uart, "Accel[{}, {}, {}]\r\n", x, y, z).unwrap();
            }
        }

        if let Ok(available) = imu.gyro_data_available() {
            if available {
                let (x, y, z) = imu.read_gyro().unwrap();
                write!(uart, "Gyro[{}, {}, {}]\r\n", x, y, z).unwrap();
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
