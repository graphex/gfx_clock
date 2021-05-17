use std::error::Error;

use crate::clock_objects::*;
mod clock_objects;

use rppal::spi::{Bus, Mode, Segment, SlaveSelect, Spi};

fn main() -> Result<(), Box<dyn Error>> {
    let mut clock = ClockDisplay::new()?;
    // loop {
    //     clock.sweep();
    // }
    clock.show(DisplayMessage{
        t0: NumericTube::new(NumericBitsIndex::_0),
        t1: NumericTube::new(NumericBitsIndex::_1),
        s0: Separator::new(SeparatorBitsIndex::BOTH),
        t2: NumericTube::new(NumericBitsIndex::_2),
        t3: NumericTube::new(NumericBitsIndex::_3),
        s1: Separator::new(SeparatorBitsIndex::BOTH),
        t4: NumericTube::new(NumericBitsIndex::_4),
        t5: NumericTube::new(NumericBitsIndex::_5),
        s2: Separator::new(SeparatorBitsIndex::BOTH),
        t6: NumericTube::new(NumericBitsIndex::_6),
        t7: NumericTube::new(NumericBitsIndex::_7),
        t8: IN19ATube::new(IN19ABitsIndex::CELSIUS),
    });
    Ok(())
}

