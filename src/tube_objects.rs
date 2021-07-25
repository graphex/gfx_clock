use bit_array::BitArray;
use bit_vec::BitVec;
use core::marker::Sized;
use core::option::Option;
use core::option::Option::{None, Some};
use std::iter::FromIterator;
use typenum::{U10, U2, U96};

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
