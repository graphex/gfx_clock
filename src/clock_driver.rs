use std::error::Error;
use std::thread;
use std::time::Duration;

use bit_array::BitArray;
use chrono::prelude::*;
use easer::functions::*;
use rand::Rng;
use rppal::gpio::{Gpio, OutputPin};
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
use rppal::system::DeviceInfo;
use typenum::{U64, U96};

use crate::clock_objects::{ClockType, DisplayMessage, DisplayMessageStringUtils, NCS3148CMessage, NCS3186Message};

//The latch enable pin GPIO number. Should be low during writes. Also tied to strobe on chips.
const LE_PIN: u8 = 22;

pub trait ClockDriver {
    fn show_next_frame(&mut self, frame_interval_us: u64) -> Result<(), Box<dyn Error>>;
    fn show<M:DisplayMessage>(&mut self, dm: M) -> Result<(), Box<dyn Error>>;
    fn write_frame(&mut self, off_linger: Option<Duration>, on_linger: Option<Duration>) -> Result<(), Box<dyn Error>> ;
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct ClockDisplay {
    le_pin: OutputPin,
    spi: Spi,
    raw_message: BitArray::<u8, U96>,
}

// impl<M:NCS3148CMessage,L:U96> ClockDriver for ClockDisplay<M,L> {
// }
impl ClockDriver for ClockDisplay {

    fn show_next_frame(&mut self, frame_interval_us: u64) -> Result<(), Box<dyn Error>> {
        let local: DateTime<Local> = Local::now();
        let mut msg_string = DisplayMessageStringUtils::for_local(local);
        if !self.time_separators_animation(local.timestamp_subsec_micros()) {
            msg_string = msg_string.replace(":", " ");
            msg_string = msg_string.replace(".", " ");
            // msg_string = "            ".to_string();
        }
        let (off_linger, on_linger) = self.pwm_seconds_animation(local.timestamp_subsec_micros(), frame_interval_us);
        self.show(NCS3148CMessage::from_string(msg_string, Some(off_linger), Some(on_linger)))
    }
    fn show<M:DisplayMessage>(&mut self, dm: M) -> Result<(), Box<dyn Error>> {
        self.raw_message = dm.to_raw();
        self.write_frame(dm.get_off_linger(), dm.get_on_linger())
    }
    fn write_frame(&mut self, off_linger: Option<Duration>, on_linger: Option<Duration>) -> Result<(), Box<dyn Error>> {
        if self.le_pin.is_set_low() {
            println!("Latch already set low by another process, aborting write!")
        } else {
            self.le_pin.set_low();
            self.spi.write(&*self.raw_message.to_bytes())?;
            off_linger.map(|off| thread::sleep(off));
            self.le_pin.set_high();
            on_linger.map(|on| thread::sleep(on));
        }
        Ok(())
    }

}

#[allow(dead_code)]
impl ClockDisplay {
    pub fn new(clock_type:ClockType) -> Result<ClockDisplay, Box<dyn Error>> {
        println!("Running a {:?} clock from a {}.", clock_type, DeviceInfo::new()?.model());
        let mut cd = match clock_type {
            ClockType::NCS3148C => ClockDisplay {
                le_pin: Gpio::new()?.get(LE_PIN)?.into_output(),
                spi: Spi::new(Bus::Spi0, SlaveSelect::Ss0, 8_000_000, Mode::Mode2)?,
                raw_message: BitArray::<u8, U96>::from_elem(false),
            },
            ClockType::NCS3186 => ClockDisplay {
                le_pin: Gpio::new()?.get(LE_PIN)?.into_output(),
                spi: Spi::new(Bus::Spi0, SlaveSelect::Ss0, 8_000_000, Mode::Mode2)?,
                // raw_message: BitArray::<u8, U64>::from_elem(false),
                raw_message: BitArray::<u8, U96>::from_elem(false),
            },
        };
        cd.le_pin.set_high();

        Ok(cd)
    }

    //a pulse frequency modulation-based animation
    fn time_separators_animation(&self, micros: u32) -> bool {
        if micros < 750_000 {
            let p: f32;
            p = Bounce::ease_in(micros as f32, 255f32, -255f32, 750_000f32);
            rand::thread_rng().gen_range(0..255) < p as isize
        } else {
            false
        }
    }
    fn pwm_seconds_animation(&self, micros: u32, frame_interval_us: u64) -> (Duration, Duration) {
        if micros < 750_000 {
            let p: f32;
            p = Sine::ease_in(micros as f32, 1f32, -1f32, 750_000f32);
            (Duration::from_micros(((1f32 - p) * frame_interval_us as f32) as u64), Duration::from_micros((p * frame_interval_us as f32) as u64))
        } else if micros < 900_000 {
            (Duration::from_micros(frame_interval_us), Duration::from_micros(0))
        } else {
            let p: f32;
            p = Quint::ease_in((micros-900_000u32) as f32, 0f32, 1f32, 100_000f32);
            (Duration::from_micros(((1f32 - p) * frame_interval_us as f32) as u64), Duration::from_micros((p * frame_interval_us as f32) as u64))
        }
    }
}