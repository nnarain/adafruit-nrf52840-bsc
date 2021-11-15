//
// lib.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Nov 09 2021
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
