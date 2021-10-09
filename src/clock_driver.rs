use std::borrow::Borrow;
use bit_array::BitArray;
use chrono::prelude::*;
use chrono::Duration;
use rand::prelude::SliceRandom;
use rand::Rng;
use rppal::gpio::{Gpio, OutputPin};
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
use rppal::system::DeviceInfo;
use std::error::Error;
use std::sync::{Arc, RwLock};
use std::thread;
use typenum::{U64, U96};

use crate::animation_utils::*;
use crate::animation_utils::Overlay::TempOverlay;
use crate::clock_objects::{
    ClockType, DisplayMessage, LingerDurations, NCS3148CMessage, NCS3186Message,
};
use crate::temperature_sensor::TemperatureSensor;

//The latch enable pin GPIO number. Should be low during writes. Also tied to strobe on chips.
const LE_PIN: u8 = 22;

pub trait ClockDriver {
    fn show_next_frame(&mut self) -> Result<(), Box<dyn Error>>;
    // fn show<M: DisplayMessage>(&mut self, dm: M) -> Result<(), Box<dyn Error>>;
    fn write_frame(
        &mut self,
        off_linger: Option<Duration>,
        on_linger: Option<Duration>,
    ) -> Result<(), Box<dyn Error>>;
    fn setup_overlays_for_minute(&mut self) -> ();
}

#[derive(Debug)]
pub struct NCS3148CDriver {
    le_pin: OutputPin,
    spi: Spi,
    frame_interval_us: i64,
    raw_message: BitArray<u8, U96>,
    last_frame_time: DateTime<Local>,
    temperature_lock: Arc<RwLock<Option<f32>>>,
    overlays: Vec<Overlay>,
}

impl NCS3148CDriver {
    const CLOCK_TYPE: ClockType = ClockType::NCS3148C;

    pub fn new(
        temperature_lk: Arc<RwLock<Option<f32>>>,
        frame_interval_us: i64,
    ) -> Result<NCS3148CDriver, Box<dyn Error>> {
        println!(
            "Running a {:?} clock from a {}.",
            NCS3148CDriver::CLOCK_TYPE,
            DeviceInfo::new()?.model()
        );
        let mut cd = NCS3148CDriver {
            le_pin: Gpio::new()?.get(LE_PIN)?.into_output(),
            spi: Spi::new(Bus::Spi0, SlaveSelect::Ss0, 8_000_000, Mode::Mode2)?,
            frame_interval_us: frame_interval_us,
            raw_message: BitArray::<u8, U96>::from_elem(false),
            last_frame_time: Local::now(),
            temperature_lock: temperature_lk,
            overlays: vec![],
        };
        cd.le_pin.set_high();

        Ok(cd)
    }

    fn show(&mut self, dm: NCS3148CMessage) -> Result<(), Box<dyn Error>> {
        self.raw_message = dm.to_raw();
        self.write_frame(dm.get_off_linger(), dm.get_on_linger())
    }
}

impl ClockDriver for NCS3148CDriver {
    fn show_next_frame(&mut self) -> Result<(), Box<dyn Error>> {
        let seconds_pulse = PwmAnimation {
            frame_interval_us: self.frame_interval_us,
        };
        let local: DateTime<Local> = Local::now();
        let micros = local.timestamp_subsec_micros();
        // let secs = local.second();
        let minute = local.minute();
        let mut msg_string: String;
        let frame_lingers: LingerDurations;
        if self.last_frame_time.minute() < minute {
            self.setup_overlays_for_minute();
        }

        msg_string = DisplayMessageStringUtils::for_local(local);
        if !TimeSeparators::time_separators_animation(micros) {
            msg_string = msg_string.replace(":", " ");
            msg_string = msg_string.replace(".", " ");
            // msg_string = "            ".to_string();
        }
        frame_lingers = seconds_pulse.pwm_seconds_animation(local.timestamp_subsec_micros());
        let mut cur_message = NCS3148CMessage::from_string(msg_string, frame_lingers);

        for cur_overlay in &mut self.overlays {
            match cur_overlay {
                Overlay::TempOverlay(t) => t.apply_to_message(local, &mut cur_message),
                Overlay::AntiPoison(o) => o.apply_to_message(local, &mut cur_message),
            }
        }

        let res = self.show(cur_message);

        self.last_frame_time = local;
        res
    }
    fn write_frame(
        &mut self,
        off_linger: Option<Duration>,
        on_linger: Option<Duration>,
    ) -> Result<(), Box<dyn Error>> {
        if self.le_pin.is_set_low() {
            println!("Latch already set low by another process, aborting write!")
        } else {
            self.le_pin.set_low();
            self.spi.write(&*self.raw_message.to_bytes())?;
            off_linger.map(|off| thread::sleep(off.to_std().unwrap()));
            self.le_pin.set_high();
            on_linger.map(|on| thread::sleep(on.to_std().unwrap()));
        }
        Ok(())
    }
    fn setup_overlays_for_minute(&mut self) -> () {
        //clear out expired overlays
        let local: DateTime<Local> = Local::now();
        self.overlays.retain(|cur_overlay| match cur_overlay {
            Overlay::AntiPoison(ap) => !ap.has_ended(local),
            Overlay::TempOverlay(t) => !t.has_ended(local),
        });

        //adds a number of random anti-poison overlays to individual numeric tubes
        self.overlays.append(&mut AntiPoisonAnimation::matrix_style_set());
        let temperature_overlay = TempOverlay(
            TempOverlayAnimation::new(
                self.temperature_lock.clone(),
                ClockType::NCS3148C,
            )
        );
        self.overlays.push(temperature_overlay);

        for o in &self.overlays {
            println!("{:?}", o)
        }
    }
}
