use std::error::Error;

use std::thread;
use std::time::Duration;

extern crate typenum;

use bit_array::{BitArray, BitsIn};
use typenum::{Unsigned, U96};
use rppal::spi::{Bus, Mode, Segment, SlaveSelect, Spi};
use rppal::gpio::{Gpio, OutputPin};
use rppal::system::DeviceInfo;
use std::mem::replace;
use self::typenum::NonZero;
use std::ops::{Add, Sub, Div};

const LE_PIN: u8 = 22;

#[derive(Debug)]
pub struct ClockDisplay {
    raw_message: BitArray::<u8, U96>,
    le_pin: OutputPin,
    spi: Spi,
}

impl ClockDisplay {
    pub fn new() -> Result<ClockDisplay, Box<dyn Error>> {
        println!("Running clock on a {}.", DeviceInfo::new()?.model());
        let mut cd = ClockDisplay {
            le_pin: Gpio::new()?.get(LE_PIN)?.into_output(),
            raw_message: BitArray::<u8, U96>::from_elem(false),
            spi: Spi::new(Bus::Spi0, SlaveSelect::Ss0, 2_000_000, Mode::Mode2)?,
        };
        cd.le_pin.set_high();
        Ok(cd)
    }
    fn write_frame(&mut self) {
        self.le_pin.set_low();
        self.spi.write(&*self.raw_message.to_bytes());
        self.le_pin.set_high();
    }
    pub fn sweep(&mut self) {
        for i in 0..96 {
            self.raw_message.clear();
            self.raw_message.set(i, true);
            println!("i: {}, raw:{:?}", i, self.raw_message);
            self.write_frame();
            thread::sleep(Duration::from_millis(50));
        }
    }
}

//Initial attempt at making ClockDisplay a singleton
// pub struct Clock {
//     clock_display: Option<ClockDisplay>,
// }
//
// impl Clock {
//     pub fn take_clock(&mut self) -> ClockDisplay {
//         let c = replace(&mut self.clock_display, None);
//         c.unwrap()
//     }
// }
//
// pub static mut CLOCK: Clock = Clock {
//     clock_display: None,
// };


// 0-1 s2
// 2-11 t8 _ _ ℃ μ η κ ₘ Ρ Μ % (IN-19A)
// 12-21 t7
// 22-31 t6
// 32-33 s1
// 34-43 t5
// 44-53 t4
// 54-63 t3
// 64-65 s0
// 66-75 t2
// 76-85 t1
// 86-95 t0

// pub trait Tube {
//     fn get_bits(&self) -> impl BitArray;
//     // fn set_bits(&mut self, BitArray<u8, U10>);
// }
//
// pub struct NumericTube {
//     bits: BitArray<u8, U10>,
// }
//
// impl Tube for NumericTube {
//     fn get_bits(&self) -> impl BitArray {
//         self.bits.clone()
//     }
// }
//
// pub enum NumericBitsIndex {
//     C0 = 9,
//     C1 = 8,
//     C2 = 7,
//     C3 = 6,
//     C4 = 5,
//     C5 = 4,
//     C6 = 3,
//     C7 = 2,
//     C8 = 1,
//     C9 = 0,
// }