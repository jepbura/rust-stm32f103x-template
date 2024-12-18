#![no_main]
#![no_std]

use rust_stm32f103x_template::bsp::blink;

use panic_halt as _;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    blink::run();
}
