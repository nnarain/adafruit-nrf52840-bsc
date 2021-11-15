//
// board.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Nov 14 2021
//
use crate::{
    core,
    hal::{
        pac,
        gpio,
        Delay,
        Uarte, uarte::{self, Baudrate, Parity},
        twim, Twim,
        pwm::Pwm,
        timer::{Timer, OneShot}
    }
};

use shared_bus::BusManagerSimple;

#[derive(Debug)]
pub enum Error {
    HardwareInitializationFailed,
}

pub struct Board {
    pub aref: gpio::p0::P0_31<gpio::Disconnected>,
    pub a0: gpio::p0::P0_04<gpio::Disconnected>,
    pub a1: gpio::p0::P0_05<gpio::Disconnected>,
    pub a2: gpio::p0::P0_30<gpio::Disconnected>,
    pub a3: gpio::p0::P0_28<gpio::Disconnected>,
    pub a4: gpio::p0::P0_02<gpio::Disconnected>,
    pub a5: gpio::p0::P0_03<gpio::Disconnected>,
    pub d2: gpio::p0::P0_10<gpio::Disconnected>,
    pub d5: gpio::p1::P1_05<gpio::Disconnected>,
    pub d6: gpio::p0::P0_07<gpio::Disconnected>,
    pub d9: gpio::p0::P0_26<gpio::Disconnected>,
    pub d10: gpio::p0::P0_27<gpio::Disconnected>,
    pub d11: gpio::p0::P0_06<gpio::Disconnected>,
    pub d12: gpio::p0::P0_08<gpio::Disconnected>,
    pub d13: gpio::p1::P1_09<gpio::Disconnected>,

    pub sck: gpio::p0::P0_14<gpio::Disconnected>,
    pub mosi: gpio::p0::P0_13<gpio::Disconnected>,
    pub miso: gpio::p0::P0_15<gpio::Disconnected>,

    pub switch: gpio::p1::P1_02<gpio::Input<gpio::PullUp>>,
    pub blue_led: gpio::p1::P1_10<gpio::Output<gpio::PushPull>>,
    pub neopixel: gpio::p0::P0_16<gpio::Disconnected>,
    pub nfc1: gpio::p0::P0_09<gpio::Disconnected>,
    pub mic_data: gpio::p0::P0_00<gpio::Disconnected>,
    pub mic_clock: gpio::p0::P0_01<gpio::Disconnected>,
    pub prox_int: gpio::p1::P1_00<gpio::Disconnected>,
    pub vlt_mnt: gpio::p0::P0_29<gpio::Disconnected>,
    pub imu_int: gpio::p1::P1_11<gpio::Disconnected>,

    pub delay: Delay,
    pub serial: Uarte<pac::UARTE0>,
    pub i2c: BusManagerSimple<Twim<pac::TWIM0>>,

    pub pwm0: Pwm<pac::PWM0>,
    pub pwm1: Pwm<pac::PWM1>,
    pub pwm2: Pwm<pac::PWM2>,
    pub pwm3: Pwm<pac::PWM3>,

    pub timer0: Timer<pac::TIMER0, OneShot>,
}

impl Board {
    pub fn new() -> Result<Board, Error> {
        if let (Some(cp), Some(dp)) = (core::Peripherals::take(), pac::Peripherals::take()) {
            let delay = Delay::new(cp.SYST);

            // GPIO pins
            let p0 = gpio::p0::Parts::new(dp.P0);
            let p1 = gpio::p1::Parts::new(dp.P1);

            let aref = p0.p0_31;
            let a0 = p0.p0_04;
            let a1 = p0.p0_05;
            let a2 = p0.p0_30;
            let a3 = p0.p0_28;
            let a4 = p0.p0_02;
            let a5 = p0.p0_03;

            let d2 = p0.p0_10;
            let d5 = p1.p1_05;
            let d6 = p0.p0_07;
            let d9 = p0.p0_26;
            let d10 = p0.p0_27;
            let d11 = p0.p0_06;
            let d12 = p0.p0_08;
            let d13 = p1.p1_09;

            let rx = p0.p0_24;
            let tx = p0.p0_25;

            let scl = p0.p0_11;
            let sda = p0.p0_12;

            let sck = p0.p0_14;
            let mosi = p0.p0_13;
            let miso = p0.p0_15;

            let switch = p1.p1_02.into_pullup_input();
            let blue_led = p1.p1_10.into_push_pull_output(gpio::Level::Low);
            let neopixel = p0.p0_16;
            let nfc1 = p0.p0_09;
            let mic_data = p0.p0_00;
            let mic_clock = p0.p0_01;
            let prox_int = p1.p1_00;
            let vlt_mnt = p0.p0_29;
            let imu_int = p1.p1_11;

            // Serial port
            let rx = rx.into_floating_input().degrade();
            let tx = tx.into_push_pull_output(gpio::Level::High).degrade();
            let uart_pins = uarte::Pins {
                rxd: rx,
                txd: tx,
                cts: None,
                rts: None
            };
            let serial = Uarte::new(dp.UARTE0, uart_pins, Parity::EXCLUDED, Baudrate::BAUD115200);

            // I2C
            let twim_pins = twim::Pins{
                scl: scl.into_floating_input().degrade(),
                sda: sda.into_floating_input().degrade()
            };
            let i2c = Twim::new(dp.TWIM0, twim_pins, twim::Frequency::K400);
            let i2c = BusManagerSimple::new(i2c);

            // PWM and Timers
            let pwm0 = Pwm::new(dp.PWM0);
            let pwm1 = Pwm::new(dp.PWM1);
            let pwm2 = Pwm::new(dp.PWM2);
            let pwm3 = Pwm::new(dp.PWM3);

            let timer0 = Timer::new(dp.TIMER0);

            let board = Board {
                aref,
                a0,
                a1,
                a2,
                a3,
                a4,
                a5,
                d2,
                d5,
                d6,
                d9,
                d10,
                d11,
                d12,
                d13,
                sck,
                mosi,
                miso,
                switch,
                blue_led,
                neopixel,
                nfc1,
                mic_data,
                mic_clock,
                prox_int,
                vlt_mnt,
                imu_int,

                delay,
                serial,
                i2c,

                pwm0,
                pwm1,
                pwm2,
                pwm3,

                timer0,
            };

            Ok(board)
        }
        else {
            Err(Error::HardwareInitializationFailed)
        }
    }
}
