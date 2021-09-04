use alloc::boxed::Box;
use core::result::Result;
use core::result::Result::Ok;
use rppal::gpio::Gpio;
use rppal::gpio::pin::OutputPin;
use rppal::system::DeviceInfo;
use std::error::Error;

//RGB Pins
const R_PIN: u8 = 20;
const G_PIN: u8 = 16;
const B_PIN: u8 = 21;

#[derive(Debug)]
pub struct LedDisplay {
    r_pin: OutputPin,
    g_pin: OutputPin,
    b_pin: OutputPin,
}

impl LedDisplay {
    pub fn new() -> Result<LedDisplay, Box<dyn Error>> {
        println!("Running LEDs from a {}.", DeviceInfo::new()?.model());
        let cd = LedDisplay {
            r_pin: Gpio::new()?.get(R_PIN)?.into_output(),
            g_pin: Gpio::new()?.get(G_PIN)?.into_output(),
            b_pin: Gpio::new()?.get(B_PIN)?.into_output(),
        };

        Ok(cd)
    }
}
