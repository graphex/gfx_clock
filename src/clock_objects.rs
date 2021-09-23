extern crate easer;
extern crate typenum;

use std::iter::FromIterator;
use std::time::Duration;

use crate::tube_objects::{IN19ATube, NumericTube, Separator, Tube};
use bit_array::BitArray;
use bit_vec::BitVec;
use chrono::prelude::*;
use typenum::{U10, U2, U64, U96};

#[derive(Debug)]
pub enum ClockType {
    NCS3148C,
    NCS3186,
}

// pub enum RegisterSizes {
//     U64(typenum::U64),
//     U96(typenum::U64),
// }

pub trait DisplayMessage {
    type L;
    fn to_raw(&self) -> BitArray<u8, U96>;
    fn from_string(time_string: String, frame_lingers: LingerDurations) -> Self
    where
        Self: Sized;
    fn get_off_linger(&self) -> Option<Duration>;
    fn get_on_linger(&self) -> Option<Duration>;
}

pub struct LingerDurations {
    pub off: Option<Duration>,
    pub on: Option<Duration>,
}
pub struct NCS3186Message {
    pub t0: Option<NumericTube>,
    pub t1: Option<NumericTube>,
    pub s0: Option<Separator>,
    pub t2: Option<NumericTube>,
    pub t3: Option<NumericTube>,
    pub s1: Option<Separator>,
    pub t4: Option<NumericTube>,
    pub t5: Option<NumericTube>,
    pub lingers: LingerDurations,
}

//TODO: `impl DisplayMessage for NCS3196Message`

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
    pub lingers: LingerDurations,
}

impl DisplayMessage for NCS3148CMessage {
    type L = U96;

    // Bit Offsets:
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
    fn to_raw(&self) -> BitArray<u8, U96> {
        // let mut combined_vec:BitVec<u8> = BitVec::<u8>::new();
        // combined_vec.reserve(96);
        // combined_vec.append(&mut self.s2.get_bits().iter().copied().collect());
        let sep_default = BitVec::<u8>::from_iter(BitArray::<u8, U2>::from_elem(false).iter());
        let tube_default = BitVec::<u8>::from_iter(BitArray::<u8, U10>::from_elem(false).iter());
        vec![
            self.s2
                .as_ref()
                .map(|v| v.get_bits())
                .unwrap_or(sep_default.clone())
                .iter(),
            self.t8
                .as_ref()
                .map(|v| v.get_bits())
                .unwrap_or(tube_default.clone())
                .iter(),
            self.t7
                .as_ref()
                .map(|v| v.get_bits())
                .unwrap_or(tube_default.clone())
                .iter(),
            self.t6
                .as_ref()
                .map(|v| v.get_bits())
                .unwrap_or(tube_default.clone())
                .iter(),
            self.s1
                .as_ref()
                .map(|v| v.get_bits())
                .unwrap_or(sep_default.clone())
                .iter(),
            self.t5
                .as_ref()
                .map(|v| v.get_bits())
                .unwrap_or(tube_default.clone())
                .iter(),
            self.t4
                .as_ref()
                .map(|v| v.get_bits())
                .unwrap_or(tube_default.clone())
                .iter(),
            self.t3
                .as_ref()
                .map(|v| v.get_bits())
                .unwrap_or(tube_default.clone())
                .iter(),
            self.s0
                .as_ref()
                .map(|v| v.get_bits())
                .unwrap_or(sep_default.clone())
                .iter(),
            self.t2
                .as_ref()
                .map(|v| v.get_bits())
                .unwrap_or(tube_default.clone())
                .iter(),
            self.t1
                .as_ref()
                .map(|v| v.get_bits())
                .unwrap_or(tube_default.clone())
                .iter(),
            self.t0
                .as_ref()
                .map(|v| v.get_bits())
                .unwrap_or(tube_default.clone())
                .iter(),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    // HH:MM:SS.ccS
    fn from_string(time_string: String, lingers: LingerDurations) -> NCS3148CMessage {
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
            lingers: lingers,
        }
    }
    fn get_off_linger(&self) -> Option<Duration> {
        self.lingers.off
    }
    fn get_on_linger(&self) -> Option<Duration> {
        self.lingers.on
    }
}
