use cortex_m::peripheral::NVIC;
use stm32f1::stm32f103;
use stm32f1::stm32f103::{TIM2, RCC, interrupt};

pub fn init(tim: TIM2) {
    // Enable TIM2 clock
    unsafe {
        (*RCC::ptr()).apb1enr.modify(|_, w| w.tim2en().set_bit());
    }

    // Configure TIM2 for 250ms interval
    tim.psc.write(|w| w.psc().bits(7200 - 1)); // Prescaler
    tim.arr.write(|w| w.arr().bits(2500 - 1)); // Auto-reload value

    // Enable update interrupt
    tim.dier.write(|w| w.uie().set_bit());
    tim.cr1.write(|w| w.cen().set_bit());

    // Enable TIM2 interrupt
    unsafe {
        NVIC::unmask(stm32f103::Interrupt::TIM2);
    }
}

#[interrupt]
fn TIM2() {
    // Clear interrupt flag
    unsafe {
        (*TIM2::ptr()).sr.modify(|_, w| w.uif().clear_bit());
    }

    // Print message
    // defmt::info!("Timer interrupt at 250ms!");
}
