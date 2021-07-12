extern crate easer;
extern crate typenum;

use std::iter::FromIterator;

use bit_array::BitArray;
use bit_vec::BitVec;
use chrono::prelude::*;
use typenum::{U10, U2, U96};
use std::time::Duration;

//TODO: use as generic type parameter for DisplayMessages and ClockDisplay
// #[allow(dead_code)]
// pub enum ClockType {
//     NCS3148C(NCS3148C),
//     NCS3186(NCS3186),
// }

#[allow(dead_code)]
pub struct NCS3186Message {
    pub t0: Option<NumericTube>,
    pub t1: Option<NumericTube>,
    pub s0: Option<Separator>,
    pub t2: Option<NumericTube>,
    pub t3: Option<NumericTube>,
    pub s1: Option<Separator>,
    pub t4: Option<NumericTube>,
    pub t5: Option<NumericTube>,
    pub off_linger: Option<Duration>,
    pub on_linger: Option<Duration>,
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
pub trait DisplayMessage {
    fn to_raw(&self) -> BitArray::<u8, U96>;
    fn from_string(time_string: String, off_linger: Option<Duration>, on_linger: Option<Duration>) -> Self
        where Self: Sized;
    fn get_off_linger(&self) -> Option<Duration>;
    fn get_on_linger(&self) -> Option<Duration>;
}

pub struct NCS3148CMessage {
    pub t0: Option<NumericTube>,
    pub t1: Option<NumericTube>,
    pub s0: Option<Separator>,
    pub t2: Option<NumericTube>,
    pub t3: Option<NumericTube>,
    pub s1: Option<Separator>,
    pub t4: Option<NumericTube>,
    pub t5: Option<NumericTube>,
    pub s2: Option<Separator>,
    pub t6: Option<NumericTube>,
    pub t7: Option<NumericTube>,
    pub t8: Option<IN19ATube>,
    pub off_linger: Option<Duration>,
    pub on_linger: Option<Duration>,
}

impl DisplayMessage for NCS3148CMessage {
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
        let sep_default = BitVec::<u8>::from_iter(BitArray::<u8, U2>::from_elem(false).iter());
        let tube_default = BitVec::<u8>::from_iter(BitArray::<u8, U10>::from_elem(false).iter());
        vec![
            self.s2.as_ref().map(|v| v.get_bits()).unwrap_or(sep_default.clone()).iter(),
            self.t8.as_ref().map(|v| v.get_bits()).unwrap_or(tube_default.clone()).iter(),
            self.t7.as_ref().map(|v| v.get_bits()).unwrap_or(tube_default.clone()).iter(),
            self.t6.as_ref().map(|v| v.get_bits()).unwrap_or(tube_default.clone()).iter(),
            self.s1.as_ref().map(|v| v.get_bits()).unwrap_or(sep_default.clone()).iter(),
            self.t5.as_ref().map(|v| v.get_bits()).unwrap_or(tube_default.clone()).iter(),
            self.t4.as_ref().map(|v| v.get_bits()).unwrap_or(tube_default.clone()).iter(),
            self.t3.as_ref().map(|v| v.get_bits()).unwrap_or(tube_default.clone()).iter(),
            self.s0.as_ref().map(|v| v.get_bits()).unwrap_or(sep_default.clone()).iter(),
            self.t2.as_ref().map(|v| v.get_bits()).unwrap_or(tube_default.clone()).iter(),
            self.t1.as_ref().map(|v| v.get_bits()).unwrap_or(tube_default.clone()).iter(),
            self.t0.as_ref().map(|v| v.get_bits()).unwrap_or(tube_default.clone()).iter(),
        ].into_iter().flatten().collect()
    }

    // HH:MM:SS.ccS
    fn from_string(time_string: String, off_linger: Option<Duration>, on_linger: Option<Duration>) -> NCS3148CMessage {
        // println!("Showing time: {}", time_string);
        let cs: Vec<char> = time_string.chars().collect::<Vec<_>>();
        NCS3148CMessage {
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
            off_linger: off_linger,
            on_linger: on_linger,
        }
    }
    fn get_off_linger(&self) -> Option<Duration> {
        self.off_linger
    }
    fn get_on_linger(&self) -> Option<Duration> {
        self.on_linger
    }
}

#[allow(dead_code)]
pub struct AntiPoisonAnimation {
    pub start_time: DateTime<Local>,
    pub duration: Duration,
}
impl AntiPoisonAnimation {

}

pub trait Tube {
    fn get_bits(&self) -> BitVec<u8>;
    fn from_char(c:char) -> Option<Self>
        where Self: Sized;
}

pub struct NumericTube {
    bits: BitArray::<u8, U10>,
}

impl Tube for NumericTube {
    fn get_bits(&self) -> BitVec<u8> {
        BitVec::<u8>::from_iter(self.bits.iter())
    }
    fn from_char(c: char) -> Option<NumericTube> {
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
            _ => return None,//NumericBitsIndex::BLANK,
        };
        tube.set_cathode(bit_index);
        Some(tube)
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

    fn from_char(c: char) -> Option<IN19ATube> {
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
            ' ' => return None, //IN19ABitsIndex::Blank,
            _ => return None, //IN19ABitsIndex::Blank,
        };
        tube.set_cathode(bit_index);
        Some(tube)
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

    fn from_char(c: char) -> Option<Separator> {
        let mut tube: Separator = Separator {
            bits: BitArray::<u8, U2>::from_elem(false)
        };
        let bit_index = match c {
            ' ' => SeparatorBitsIndex::BLANK,
            ':' => SeparatorBitsIndex::BOTH,
            '.' => SeparatorBitsIndex::BOTTOM,
            '\'' => SeparatorBitsIndex::TOP,
            _ => return None, //SeparatorBitsIndex::BLANK,
        };
        tube.set_indicators(bit_index);
        Some(tube)
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

    fn set_indicators(&mut self, bit_index: SeparatorBitsIndex) {
        self.bits.clear();
        match bit_index {
            SeparatorBitsIndex::BLANK => (),
            SeparatorBitsIndex::BOTH => self.bits.negate(),
            _ => self.bits.set(bit_index as usize, true),
        }
    }
}

pub enum SeparatorBitsIndex {
    TOP = 0,
    BOTTOM = 1,
    BOTH,
    BLANK,
}