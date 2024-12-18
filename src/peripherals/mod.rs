pub mod gpio;
pub mod timer;

use stm32f1::stm32f103;

pub struct GpioOut {
    port: stm32f103::GPIOA,
    pin: u8,
}

pub struct Timer {
    tim2: stm32f103::TIM2,
}

impl GpioOut {
    pub fn set_high(&mut self) {
        self.port.bsrr.write(|w| unsafe { w.bits(1 << self.pin) });
    }

    pub fn set_low(&mut self) {
        self.port.bsrr.write(|w| unsafe { w.bits(1 << (self.pin + 16)) });
    }
}

impl Timer {
    pub fn delay_ms(&mut self, ms: u32) {
        for _ in 0..ms {
            // Assuming 8 MHz system clock, roughly 1 ms delay
            cortex_m::asm::delay(8000);
        }
    }

    pub fn setup_interrupt(&mut self) {
        // Set up timer for 250 ms interrupts
        self.tim2.cr1.write(|w| w.cen().set_bit());
        self.tim2.arr.write(|w| unsafe { w.bits(39999) }); // ARR for 250ms at 8 MHz
        self.tim2.dier.write(|w| w.uie().set_bit());
        self.tim2.cr1.write(|w| w.cen().set_bit());
    }
}