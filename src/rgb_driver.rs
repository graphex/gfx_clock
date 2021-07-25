use alloc::boxed::Box;
use core::result::Result;
use core::result::Result::Ok;
use rppal::gpio::Gpio;
use rppal::gpio::pin::OutputPin;
use rppal::system::DeviceInfo;
use std::error::Error;

//RGB Pins
#[allow(dead_code)]
const R_PIN: u8 = 20;
#[allow(dead_code)]
const G_PIN: u8 = 16;
#[allow(dead_code)]
const B_PIN: u8 = 21;

#[derive(Debug)]
pub struct LedDisplay {
    r_pin: OutputPin,
    g_pin: OutputPin,
    b_pin: OutputPin,
}

#[allow(dead_code)]
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
