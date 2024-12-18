use crate::peripherals::{gpio, timer};
use stm32f1::stm32f103;

pub fn run() -> ! {
    let peripherals = stm32f103::Peripherals::take().unwrap();

    // Setup GPIO
    let mut led = gpio::init(peripherals.GPIOC);

    // Setup Timer
    timer::init(peripherals.TIM2);

    loop {
        led.toggle();
    }
}
