//
// lib.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Nov 10 2021
//
#![no_std]

mod pins;

// Re-exports
pub use nrf52840_hal as hal;

pub use cortex_m as core;
pub use cortex_m_rt as core_rt;

pub use core_rt::entry;

pub use embedded_hal as ehal;

pub use nb;

pub mod prelude {
    pub use super::hal::prelude::*;
    pub use adafruit_nrf52840_common::*;
}

pub use pins::Pins;

pub mod sensors {
    pub use lsm6ds33;
    pub use bmp280_ehal as bmp280;
    pub use apds9960;
    pub use lis3mdl;
}
