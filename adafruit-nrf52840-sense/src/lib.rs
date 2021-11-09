#![no_std]

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

use hal::gpio;

// Pin abstraction
pub struct Pins {
    pub d2: gpio::p0::P0_10<gpio::Disconnected>,
    pub d5: gpio::p1::P1_05<gpio::Disconnected>,
    pub d6: gpio::p0::P0_07<gpio::Disconnected>,
    pub d7: gpio::p1::P1_02<gpio::Disconnected>,
    pub d9: gpio::p0::P0_26<gpio::Disconnected>,
    pub d10: gpio::p0::P0_27<gpio::Disconnected>,
    pub d11: gpio::p0::P0_06<gpio::Disconnected>,
    pub d12: gpio::p0::P0_08<gpio::Disconnected>,
    pub d13: gpio::p1::P1_09<gpio::Disconnected>,

    pub rx: gpio::p0::P0_24<gpio::Disconnected>,
    pub tx: gpio::p0::P0_25<gpio::Disconnected>,

    pub scl: gpio::p0::P0_11<gpio::Disconnected>,
    pub sda: gpio::p0::P0_12<gpio::Disconnected>,
}

impl Pins {
    pub fn new(p0: hal::pac::P0, p1: hal::pac::P1) -> Self {
        let p0 = gpio::p0::Parts::new(p0);
        let p1 = gpio::p1::Parts::new(p1);

        Pins {
            d2: p0.p0_10,
            d5: p1.p1_05,
            d6: p0.p0_07,
            d7: p1.p1_02,
            d9: p0.p0_26,
            d10: p0.p0_27,
            d11: p0.p0_06,
            d12: p0.p0_08,
            d13: p1.p1_09,

            rx: p0.p0_24,
            tx: p0.p0_25,

            scl: p0.p0_11,
            sda: p0.p0_12,
        }
    }
}
