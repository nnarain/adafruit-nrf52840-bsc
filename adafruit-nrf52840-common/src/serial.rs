use nrf52840_hal as hal;

use hal::{pac, gpio, uarte};

pub type RxPin = gpio::p0::P0_24<gpio::Disconnected>;
pub type TxPin = gpio::p0::P0_25<gpio::Disconnected>;

/// Helper function to initialize the serial port
pub fn init(uarte: pac::UARTE0, rx: RxPin, tx: TxPin, baudrate: uarte::Baudrate, parity: uarte::Parity) -> uarte::Uarte<pac::UARTE0> {
    let rx = rx.into_floating_input().degrade();
    let tx = tx.into_push_pull_output(gpio::Level::High).degrade();

    let pins = uarte::Pins {
        rxd: rx,
        txd: tx,
        cts: None,
        rts: None
    };

    uarte::Uarte::new(uarte, pins, parity, baudrate)
}
