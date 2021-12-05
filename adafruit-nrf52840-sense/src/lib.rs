//
// lib.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Nov 10 2021
//
#![no_std]
mod board;

// Re-exports
pub use nrf52840_hal as hal;

pub use cortex_m as core;
pub use cortex_m_rt as core_rt;

pub use core_rt::entry;

pub use embedded_hal as ehal;

pub use nb;

pub mod prelude {
    pub use super::hal::prelude::*;
}

pub use board::Board;

#[derive(Debug)]
pub enum Error {
    HardwareInitializationFailed,
}

pub mod sensors {
    pub use bmp280_driver as bmp280;
    pub use apds9960;
    pub use lis3mdl_driver as lis3mdl;

    use crate::hal::{self, Twim};
    use shared_bus::{I2cProxy, NullMutex};

    type I2C<'a> = I2cProxy<'a, NullMutex<Twim<hal::pac::TWIM0>>>;

    pub mod imu {
        use super::*;
        use crate::Error;

        use lsm6ds33::Lsm6ds33;

        pub type Imu<'a> = Lsm6ds33<I2C<'a>>;
        pub type ImuError = lsm6ds33::Error<crate::hal::twim::Error>;

        pub use lsm6ds33::{
            AccelerometerBandwidth, AccelerometerOutput, AccelerometerScale,
            GyroscopeFullScale, GyroscopeOutput
        };

        pub fn init<'a>(i2c: I2C<'a>) -> Result<Imu<'a>, Error> {
            Lsm6ds33::new(i2c, 0x6A).map_err(|_| Error::HardwareInitializationFailed)
        }
    }

    pub mod magnetometer {
        use super::*;
        use crate::Error;

        use lis3mdl::{Lis3mdl, Address};

        pub type Magnetometer<'a> = Lis3mdl<I2C<'a>>;
        pub use lis3mdl::Error as MagnetometerError;

        pub fn init<'a>(i2c: I2C<'a>) -> Result<Magnetometer<'a>, Error> {
            Lis3mdl::new(i2c, Address::Addr1C).map_err(|_| Error::HardwareInitializationFailed)
        }
    }

    pub mod barometer {
        use super::*;
        use crate::Error;

        use bmp280::BMP280;

        pub type Barometer<'a> = BMP280<I2C<'a>>;

        pub fn init<'a>(i2c: I2C<'a>) -> Result<Barometer<'a>, Error> {
            BMP280::new(i2c, 0x77).map_err(|_| Error::HardwareInitializationFailed)
        }
    }
}
