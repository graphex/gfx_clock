use std::error::Error;


use crate::clock_objects::*;
use bit_array::BitArray;

mod clock_objects;

use rppal::spi::{Bus, Mode, Segment, SlaveSelect, Spi};

fn main() -> Result<(), Box<dyn Error>> {
    let mut clock = ClockDisplay::new()?;
    loop {
        clock.sweep();
    }
    Ok(())
}

