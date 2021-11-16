# adafruit-nrf52840-bsc

Board support crates for Adafruit's nRF52840 based boards.

| Board   | Info Page                                                            | crates.io  |
| ------- | -------------------------------------------------------------------- | ---------- |
| Sense   | https://learn.adafruit.com/adafruit-feather-sense                    | [![express crate](https://img.shields.io/crates/v/adafruit-nrf52840-express.svg)](https://crates.io/crates/adafruit-nrf52840-express) |
| Express | https://learn.adafruit.com/introducing-the-adafruit-nrf52840-feather | [![sense crate](https://img.shields.io/crates/v/adafruit-nrf52840-sense.svg)](https://crates.io/crates/adafruit-nrf52840-sense) |

Simple example:

```rust
use adafruit_nrf52840_sense as bsp;

use bsp::{
    entry,
    prelude::*,
    hal::gpio,
    Board,
};

#[entry]
fn main() -> ! {
    let board = Board::new().unwrap();

    let mut delay = board.delay;

    let mut led1 = board.d13.into_push_pull_output(gpio::Level::Low);
    let mut led2 = board.blue_led;

    led2.set_high().unwrap();

    loop {
        led1.set_high().unwrap();
        led2.set_low().unwrap();

        delay.delay_ms(500u16);

        led1.set_low().unwrap();
        led2.set_high().unwrap();

        delay.delay_ms(500u16);
    }
}
```

**Examples**

Install cargo-just

```
cargo install just
```

Run example specifying both the board and the example name:

```
just run-example sense serial
```
