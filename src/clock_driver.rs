use std::error::Error;
use std::thread;

use bit_array::BitArray;
use chrono::prelude::*;
use easer::functions::*;
use rand::Rng;
use rppal::gpio::{Gpio, OutputPin};
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
use rppal::system::DeviceInfo;
use typenum::U96;

use crate::clock_objects::{DisplayMessage, DisplayMessageStringUtils, NCS3148CMessage, NCS3186Message};
use std::time::Duration;

//The latch enable pin GPIO number. Should be low during writes. Also tied to strobe on chips.
const LE_PIN: u8 = 22;

#[derive(Debug)]
#[allow(dead_code)]
pub struct ClockDisplay {
    raw_message: BitArray::<u8, U96>,
    le_pin: OutputPin,
    spi: Spi,
}

#[allow(dead_code)]
impl ClockDisplay {
    pub fn new() -> Result<ClockDisplay, Box<dyn Error>> {
        println!("Running clock tubes from a {}.", DeviceInfo::new()?.model());
        let mut cd = ClockDisplay {
            le_pin: Gpio::new()?.get(LE_PIN)?.into_output(),
            raw_message: BitArray::<u8, U96>::from_elem(false),
            spi: Spi::new(Bus::Spi0, SlaveSelect::Ss0, 8_000_000, Mode::Mode2)?,
        };
        cd.le_pin.set_high();

        Ok(cd)
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
    pub fn sweep(&mut self) -> Result<(), Box<dyn Error>> {
        for i in 0..96 {
            self.raw_message.clear();
            self.raw_message.set(i, true);
            println!("i: {}, raw:{:?}", i, self.raw_message);
            self.write_frame(None, Some(Duration::from_millis(50)))?;
        }
        Ok(())
    }
    pub fn show<T:DisplayMessage>(&mut self, dm: T) -> Result<(), Box<dyn Error>> {
        self.raw_message = dm.to_raw();
        self.write_frame(dm.get_off_linger(), dm.get_on_linger())
    }

    pub fn show_next_frame(&mut self, frame_interval_us: u64) -> Result<(), Box<dyn Error>> {
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

//RGB Pins
#[allow(dead_code)]
const R_PIN: u8 = 20;
#[allow(dead_code)]
const G_PIN: u8 = 16;
#[allow(dead_code)]
const B_PIN: u8 = 21;

#[derive(Debug)]
pub struct LedDisplay {
    r_pin: OutputPin,
    g_pin: OutputPin,
    b_pin: OutputPin,
}

#[allow(dead_code)]
impl LedDisplay {
    pub fn new() -> Result<LedDisplay, Box<dyn Error>> {
        println!("Running LEDs from a {}.", DeviceInfo::new()?.model());
        let cd = LedDisplay {
            r_pin: Gpio::new()?.get(R_PIN)?.into_output(),
            g_pin: Gpio::new()?.get(G_PIN)?.into_output(),
            b_pin: Gpio::new()?.get(B_PIN)?.into_output(),
        };

        Ok(cd)
    }
}