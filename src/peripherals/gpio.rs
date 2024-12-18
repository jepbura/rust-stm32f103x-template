use stm32f1::stm32f103::{GPIOC, RCC};

pub struct Led {
    port: GPIOC,
}

impl Led {
    pub fn toggle(&mut self) {
        // Read current state
        let is_set = self.port.odr.read().odr13().bit_is_set();

        // Toggle the state
        if is_set {
            self.port.bsrr.write(|w| w.br13().set_bit()); // Reset pin
        } else {
            self.port.bsrr.write(|w| w.bs13().set_bit()); // Set pin
        }
    }
}

pub fn init(gpio: GPIOC) -> Led {
    // Enable GPIOC clock
    unsafe {
        (*RCC::ptr()).apb2enr.modify(|_, w| w.iopcen().set_bit());
    }

    // Configure PC13 as output
    gpio.crh.modify(|_, w| w.mode13().output2().cnf13().push_pull());

    Led { port: gpio }
}
