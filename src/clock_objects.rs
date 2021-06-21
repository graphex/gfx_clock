extern crate easer;
extern crate typenum;

use std::iter::FromIterator;

use bit_array::BitArray;
use bit_vec::BitVec;
use chrono::prelude::*;
use typenum::{U10, U2, U96};
use std::time::Duration;

//TODO: use as generic type parameter for DisplayMessages and ClockDisplay
#[allow(dead_code)]
pub enum ClockType {
    _9Tubes,
    _6Tubes,
}

#[allow(dead_code)]
pub enum DisplayElement {
    NumericTube(NumericTube),
    Separator(Separator),
    IN19ATube(IN19ATube),
}

pub struct DisplayMessageStringUtils {}

impl DisplayMessageStringUtils {
    pub fn for_local(local: DateTime<Local>) -> String {
        let mut s = local.format("%T%.3f").to_string();
        s.push_str(" ");
        s
    }
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
    pub off_linger: Duration,
    pub on_linger: Duration,
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
    pub(crate) fn to_raw(&self) -> BitArray::<u8, U96> {
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

    // HH:MM:SS.ccS
    pub fn from_string(time_string: String, off_linger: Option<Duration>, on_linger: Option<Duration>) -> DisplayMessage {
        // println!("Showing time: {}", time_string);
        let cs: Vec<char> = time_string.chars().collect::<Vec<_>>();
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
            t8: IN19ATube::from_char(cs[11]),
            off_linger: off_linger.unwrap_or(Duration::from_micros(0)),
            on_linger: on_linger.unwrap_or(Duration::from_micros(0)),
        }
    }
}

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

#[allow(dead_code)]
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

#[allow(dead_code)]
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
            IN19ABitsIndex::Blank => (),
            _ => self.bits.set(bit_index as usize, true),
        }
    }

    pub fn from_char(c: char) -> IN19ATube {
        let mut tube: IN19ATube = IN19ATube {
            bits: BitArray::<u8, U10>::from_elem(false)
        };
        let bit_index = match c {
            '℃' => IN19ABitsIndex::Celsius,
            'c' => IN19ABitsIndex::Celsius,
            'C' => IN19ABitsIndex::Celsius,
            'μ' => IN19ABitsIndex::Micro,
            'u' => IN19ABitsIndex::Micro,
            'η' => IN19ABitsIndex::Nano,
            'n' => IN19ABitsIndex::Nano,
            'κ' => IN19ABitsIndex::Kelvin,
            'k' => IN19ABitsIndex::Kelvin,
            'K' => IN19ABitsIndex::Kelvin,
            'ₘ' => IN19ABitsIndex::MSmall,
            'm' => IN19ABitsIndex::MSmall,
            'P' => IN19ABitsIndex::P,
            'Μ' => IN19ABitsIndex::M,
            '%' => IN19ABitsIndex::Percent,
            ' ' => IN19ABitsIndex::Blank,
            _ => IN19ABitsIndex::Blank,
        };
        tube.set_cathode(bit_index);
        tube
    }
}

#[allow(dead_code)]
pub enum IN19ABitsIndex {
    //_ _ ℃ μ η κ ₘ Ρ Μ % (IN-19A)
    Celsius = 2,
    Micro = 3,
    Nano = 4,
    Kelvin = 5,
    MSmall = 6,
    P = 7,
    M = 8,
    Percent = 9,
    Blank = 10,
}

pub struct Separator {
    bits: BitArray::<u8, U2>,
}

impl Tube for Separator {
    fn get_bits(&self) -> BitVec<u8> {
        BitVec::<u8>::from_iter(self.bits.iter())
    }
}

#[allow(dead_code)]
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