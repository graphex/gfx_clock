use std::error::Error;

use crate::clock_objects::*;
mod clock_objects;

use rppal::spi::{Bus, Mode, Segment, SlaveSelect, Spi};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let mut clock = ClockDisplay::new()?;

    loop {
        clock.show(DisplayMessage::for_now());
        thread::sleep(Duration::from_micros(200));
    }

    Ok(())
}

