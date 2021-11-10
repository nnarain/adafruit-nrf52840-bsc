//
// lib.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Nov 09 2021
//
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

pub struct Pins {
    pub aref: gpio::p0::P0_31<gpio::Disconnected>,
    pub a0: gpio::p0::P0_04<gpio::Disconnected>,
    pub a1: gpio::p0::P0_05<gpio::Disconnected>,
    pub a2: gpio::p0::P0_30<gpio::Disconnected>,
    pub a3: gpio::p0::P0_23<gpio::Disconnected>,
    pub a4: gpio::p0::P0_02<gpio::Disconnected>,
    pub a5: gpio::p0::P0_03<gpio::Disconnected>,

    pub sck: gpio::p0::P0_14<gpio::Disconnected>,
    pub mosi: gpio::p0::P0_13<gpio::Disconnected>,
    pub miso: gpio::p0::P0_15<gpio::Disconnected>,

    pub rx: gpio::p0::P0_24<gpio::Disconnected>,
    pub tx: gpio::p0::P0_25<gpio::Disconnected>,

    pub d2: gpio::p0::P0_10<gpio::Disconnected>,
    pub d5: gpio::p1::P1_08<gpio::Disconnected>,
    pub d6: gpio::p0::P0_07<gpio::Disconnected>,
    pub d9: gpio::p0::P0_26<gpio::Disconnected>,
    pub d10: gpio::p0::P0_27<gpio::Disconnected>,
    pub d11: gpio::p0::P0_06<gpio::Disconnected>,
    pub d12: gpio::p0::P0_08<gpio::Disconnected>,
    pub d13: gpio::p1::P1_09<gpio::Disconnected>,

    pub scl: gpio::p0::P0_11<gpio::Disconnected>,
    pub sda: gpio::p0::P0_12<gpio::Disconnected>,

    pub switch: gpio::p1::P1_02<gpio::Input<gpio::PullUp>>,
    pub led: gpio::p1::P1_15<gpio::Output<gpio::PushPull>>,
    pub blue_led: gpio::p1::P1_10<gpio::Output<gpio::PushPull>>,
    pub neopixel: gpio::p0::P0_16<gpio::Disconnected>,
    pub nfc1: gpio::p0::P0_09<gpio::Disconnected>,
    pub vlt_mnt: gpio::p0::P0_29<gpio::Disconnected>,
}

impl Pins {
    pub fn new(p0: hal::pac::P0, p1: hal::pac::P1) -> Self {
        let p0 = gpio::p0::Parts::new(p0);
        let p1 = gpio::p1::Parts::new(p1);

        Pins {
            aref: p0.p0_31,
            a0: p0.p0_04,
            a1: p0.p0_05,
            a2: p0.p0_30,
            a3: p0.p0_23,
            a4: p0.p0_02,
            a5: p0.p0_03,
        
            sck: p0.p0_14,
            mosi: p0.p0_13,
            miso: p0.p0_15,
        
            rx: p0.p0_24,
            tx: p0.p0_25,
        
            d2: p0.p0_10,
            d5: p1.p1_08,
            d6: p0.p0_07,
            d9: p0.p0_26,
            d10: p0.p0_27,
            d11: p0.p0_06,
            d12: p0.p0_08,
            d13: p1.p1_09,
        
            scl: p0.p0_11,
            sda: p0.p0_12,

            switch: p1.p1_02.into_pullup_input(),
            led: p1.p1_15.into_push_pull_output(gpio::Level::Low),
            blue_led: p1.p1_10.into_push_pull_output(gpio::Level::Low),
            neopixel: p0.p0_16,
            nfc1: p0.p0_09,
            vlt_mnt: p0.p0_29,
        }
    }
}
