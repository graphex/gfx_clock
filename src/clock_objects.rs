use std::error::Error;

use std::thread;
use std::time::Duration;

extern crate typenum;
extern crate easer;

use bit_array::{BitArray, BitsIn};
use bit_vec::BitVec;
use typenum::{Unsigned, U96, U10, U2};
use rppal::spi::{Bus, Mode, Segment, SlaveSelect, Spi};
use rppal::gpio::{Gpio, OutputPin, Level};
use rppal::system::DeviceInfo;
use std::mem::replace;
use self::typenum::{NonZero, Bit};
use std::ops::{Add, Sub, Div};
use std::iter::FromIterator;
use std::borrow::BorrowMut;
use chrono::prelude::*;
use easer::functions::*;
use rand::Rng;

//The latch enable pin GPIO number. Should be low during writes.
const LE_PIN: u8 = 22;
//RGB Pins
const R_PIN: u8 = 20;
const G_PIN: u8 = 16;
const B_PIN: u8 = 21;

//TODO: use as generic type parameter for DisplayMessages and ClockDisplay
pub enum ClockType {
    _9Tubes,
    _6Tubes,
}

pub enum DisplayElement {
    NumericTube(NumericTube),
    Separator(Separator),
    IN19ATube(IN19ATube),
}

pub struct DisplayMessage {
    pub t0: NumericTube,
    pub t1: NumericTube,
    pub s0: Separator,
    pub t2: NumericTube,
    pub t3: NumericTube,
    pub s1: Separator,
    pub t4: NumericTube,
    pub t5: NumericTube,
    pub s2: Separator,
    pub t6: NumericTube,
    pub t7: NumericTube,
    pub t8: IN19ATube,
}

impl DisplayMessage {
    // 0-1 s2
    // 2-11 t8
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
    fn to_raw(&self) -> BitArray::<u8, U96> {
        // let mut combined_vec:BitVec<u8> = BitVec::<u8>::new();
        // combined_vec.reserve(96);
        // combined_vec.append(&mut self.s2.get_bits().iter().copied().collect());
        vec![
            self.s2.get_bits().iter(),
            self.t8.get_bits().iter(),
            self.t7.get_bits().iter(),
            self.t6.get_bits().iter(),
            self.s1.get_bits().iter(),
            self.t5.get_bits().iter(),
            self.t4.get_bits().iter(),
            self.t3.get_bits().iter(),
            self.s0.get_bits().iter(),
            self.t2.get_bits().iter(),
            self.t1.get_bits().iter(),
            self.t0.get_bits().iter(),
        ].into_iter().flatten().collect()
    }
    pub fn for_now() -> DisplayMessage {
        let local: DateTime<Local> = Local::now();
        let mut msgString = local.format("%T%.3f").to_string();
        if !DisplayMessage::rnd_pfm(local.timestamp_subsec_micros()) {
            msgString = msgString.replace(":", " ");
            msgString = msgString.replace(".", " ");
        }
        DisplayMessage::from_string(msgString)
        // DisplayMessage::from_string(("22:33:44.55").to_string())
    }
    //super mega unoptimised pulse frequency modulation
    pub fn rnd_pfm(micros:u32) -> bool {
        let mut rng = rand::thread_rng();
        let p:f32;
        if micros < 750_000 {
            p = Bounce::ease_in(micros as f32, 255f32, -255f32, 750_000f32);
        } else {
            return false;
        }
        let r = rng.gen_range(0..255) < p as isize;
        // println!("{},{},{}", micros, p, r);
        r
    }
    // HH:MM:SS.cc
    pub fn from_string(time_string: String) -> DisplayMessage {
        // println!("Showing time: {}", time_string);
        let cs:Vec<char> = time_string.chars().collect::<Vec<_>>();
        DisplayMessage {
            t0: NumericTube::from_char(cs[0]),
            t1: NumericTube::from_char(cs[1]),
            s0: Separator::from_char(cs[2]),
            t2: NumericTube::from_char(cs[3]),
            t3: NumericTube::from_char(cs[4]),
            s1: Separator::from_char(cs[5]),
            t4: NumericTube::from_char(cs[6]),
            t5: NumericTube::from_char(cs[7]),
            s2: Separator::from_char(cs[8]),
            t6: NumericTube::from_char(cs[9]),
            t7: NumericTube::from_char(cs[10]),
            t8: IN19ATube::new(IN19ABitsIndex::CELSIUS),
        }
    }
}

#[derive(Debug)]
pub struct ClockDisplay {
    raw_message: BitArray::<u8, U96>,
    le_pin: OutputPin,
    spi: Spi,
    r_pin: OutputPin,
    g_pin: OutputPin,
    b_pin: OutputPin,
}

impl ClockDisplay {
    pub fn new() -> Result<ClockDisplay, Box<dyn Error>> {
        println!("Running clock on a {}.", DeviceInfo::new()?.model());
        let mut cd = ClockDisplay {
            le_pin: Gpio::new()?.get(LE_PIN)?.into_output(),
            raw_message: BitArray::<u8, U96>::from_elem(false),
            spi: Spi::new(Bus::Spi0, SlaveSelect::Ss0, 8_000_000, Mode::Mode2)?,
            r_pin: Gpio::new()?.get(R_PIN)?.into_output(),
            g_pin: Gpio::new()?.get(G_PIN)?.into_output(),
            b_pin: Gpio::new()?.get(B_PIN)?.into_output(),
        };
        cd.le_pin.set_high();

        Ok(cd)
    }
    fn write_frame(&mut self) {
        if self.le_pin.is_set_low() {
            println!("Latch already set low by another process, aborting write!")
        } else {
            self.le_pin.set_low();
            self.spi.write(&*self.raw_message.to_bytes());
            // thread::sleep(Duration::from_millis(5));
            self.le_pin.set_high();
        }
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
    pub fn show(&mut self, dm: DisplayMessage) {
        self.raw_message = dm.to_raw();
        self.write_frame();
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

pub trait Tube {
    fn get_bits(&self) -> BitVec<u8>;
}

pub struct NumericTube {
    bits: BitArray::<u8, U10>,
}

impl Tube for NumericTube {
    fn get_bits(&self) -> BitVec<u8> {
        BitVec::<u8>::from_iter(self.bits.iter())
    }
}

impl NumericTube {
    pub fn new(bit_index: NumericBitsIndex) -> NumericTube {
        let mut tube: NumericTube = NumericTube {
            bits: BitArray::<u8, U10>::from_elem(false)
        };
        tube.set_cathode(bit_index);
        tube
    }
    pub fn set_cathode(&mut self, bit_index: NumericBitsIndex) {
        self.bits.clear();
        match bit_index {
            NumericBitsIndex::BLANK => (),
            _ => self.bits.set(bit_index as usize, true),
        }
    }
    pub fn from_char(c: char) -> NumericTube {
        let mut tube: NumericTube = NumericTube {
            bits: BitArray::<u8, U10>::from_elem(false)
        };
        let bit_index = match c {
            '0' => NumericBitsIndex::_0,
            '1' => NumericBitsIndex::_1,
            '2' => NumericBitsIndex::_2,
            '3' => NumericBitsIndex::_3,
            '4' => NumericBitsIndex::_4,
            '5' => NumericBitsIndex::_5,
            '6' => NumericBitsIndex::_6,
            '7' => NumericBitsIndex::_7,
            '8' => NumericBitsIndex::_8,
            '9' => NumericBitsIndex::_9,
            _ => NumericBitsIndex::BLANK,
        };
        tube.set_cathode(bit_index);
        tube
    }
}

pub enum NumericBitsIndex {
    _0 = 9,
    _1 = 8,
    _2 = 7,
    _3 = 6,
    _4 = 5,
    _5 = 4,
    _6 = 3,
    _7 = 2,
    _8 = 1,
    _9 = 0,
    BLANK = 10,
}

pub struct IN19ATube {
    bits: BitArray::<u8, U10>,
}

impl Tube for IN19ATube {
    fn get_bits(&self) -> BitVec<u8> {
        BitVec::<u8>::from_iter(self.bits.iter())
    }
}

impl IN19ATube {
    pub fn new(bit_index: IN19ABitsIndex) -> IN19ATube {
        let mut tube: IN19ATube = IN19ATube {
            bits: BitArray::<u8, U10>::from_elem(false)
        };
        tube.set_cathode(bit_index);
        tube
    }

    fn set_cathode(&mut self, bit_index: IN19ABitsIndex) {
        self.bits.clear();
        match bit_index {
            IN19ABitsIndex::BLANK => (),
            _ => self.bits.set(bit_index as usize, true),
        }
    }
}

pub enum IN19ABitsIndex {
    //_ _ ℃ μ η κ ₘ Ρ Μ % (IN-19A)
    CELSIUS = 2,
    MICRO = 3,
    NANO = 4,
    KELVIN = 5,
    M_SMALL = 6,
    P = 7,
    M = 8,
    PERCENT = 9,
    BLANK = 10,
}

pub struct Separator {
    bits: BitArray::<u8, U2>,
}

impl Tube for Separator {
    fn get_bits(&self) -> BitVec<u8> {
        BitVec::<u8>::from_iter(self.bits.iter())
    }
}

impl Separator {
    pub fn new(bit_index: SeparatorBitsIndex) -> Separator {
        let mut tube: Separator = Separator {
            bits: BitArray::<u8, U2>::from_elem(false)
        };
        tube.set_indicators(bit_index);
        tube
    }

    pub fn from_char(c: char) -> Separator {
        let mut tube: Separator = Separator {
            bits: BitArray::<u8, U2>::from_elem(false)
        };
        let bit_index = match c {
            ' ' => SeparatorBitsIndex::BLANK,
            ':' => SeparatorBitsIndex::BOTH,
            '.' => SeparatorBitsIndex::BOTTOM,
            '\'' => SeparatorBitsIndex::TOP,
            _ => SeparatorBitsIndex::BLANK,
        };
        tube.set_indicators(bit_index);
        tube
    }

    fn set_indicators(&mut self, bit_index: SeparatorBitsIndex) {
        self.bits.clear();
        match bit_index {
            SeparatorBitsIndex::BLANK => (),
            SeparatorBitsIndex::BOTH => self.bits.negate(),
            _ => self.bits.set(bit_index as usize, true),
        }
    }
    //nobody uses centiseconds and hundredths of a second is such a mouthful...
    fn from_hundos_string(hundos: String) -> Separator {
        let cs = hundos.parse::<u16>();
        match cs {
            Ok(secs) if secs < 50 => Separator::new(SeparatorBitsIndex::BOTH),
            _ => Separator::new(SeparatorBitsIndex::BLANK),
        }
    }
}

pub enum SeparatorBitsIndex {
    TOP = 0,
    BOTTOM = 1,
    BOTH,
    BLANK,
}