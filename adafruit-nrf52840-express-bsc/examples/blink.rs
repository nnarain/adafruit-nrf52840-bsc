#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m::asm;

use adafruit_nrf52840_express_bsc as bsc;

#[cortex_m_rt::entry]
fn main() -> ! {
    let dp = bsc::hal::pac::Peripherals::take().unwrap();

    loop {
        asm::nop();
    }
}
