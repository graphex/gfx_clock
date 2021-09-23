use std::error::Error;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

use bit_array::BitArray;
use chrono::prelude::*;
use rppal::gpio::{Gpio, OutputPin};
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
use rppal::system::DeviceInfo;
use typenum::{U64, U96};

use crate::animation_utils::*;
use crate::clock_objects::{
    ClockType, DisplayMessage, LingerDurations, NCS3148CMessage, NCS3186Message,
};
use crate::temperature_sensor::TemperatureSensor;

//The latch enable pin GPIO number. Should be low during writes. Also tied to strobe on chips.
const LE_PIN: u8 = 22;

pub trait ClockDriver {
    fn show_next_frame(&mut self, frame_interval_us: u64) -> Result<(), Box<dyn Error>>;
    // fn show<M: DisplayMessage>(&mut self, dm: M) -> Result<(), Box<dyn Error>>;
    fn write_frame(
        &mut self,
        off_linger: Option<Duration>,
        on_linger: Option<Duration>,
    ) -> Result<(), Box<dyn Error>>;
}

#[derive(Debug)]
pub struct NCS3148CDriver {
    le_pin: OutputPin,
    spi: Spi,
    raw_message: BitArray<u8, U96>,
    last_frame_time: DateTime<Local>,
    temperature_lock: Arc<RwLock<Option<f32>>>,
    overlays: Vec<Overlay>,
}

impl NCS3148CDriver {
    const CLOCK_TYPE: ClockType = ClockType::NCS3148C;

    pub fn new(temperature_lk: Arc<RwLock<Option<f32>>>) -> Result<NCS3148CDriver, Box<dyn Error>> {
        println!(
            "Running a {:?} clock from a {}.",
            NCS3148CDriver::CLOCK_TYPE,
            DeviceInfo::new()?.model()
        );
        let mut cd = NCS3148CDriver {
            le_pin: Gpio::new()?.get(LE_PIN)?.into_output(),
            spi: Spi::new(Bus::Spi0, SlaveSelect::Ss0, 8_000_000, Mode::Mode2)?,
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
    fn show_next_frame(&mut self, frame_interval_us: u64) -> Result<(), Box<dyn Error>> {
        let local: DateTime<Local> = Local::now();
        let micros = local.timestamp_subsec_micros();
        let secs = local.second();
        let mut msg_string: String;
        let frame_lingers: LingerDurations;
        //TODO: abstract this into the Overlay
        if secs >= 10 && secs < 15 && self.temperature_lock.read().unwrap().is_some() {
            let temp = self.temperature_lock.read().unwrap().unwrap();
            frame_lingers = LingerDurations {
                off: Some(Duration::from_micros(0)),
                on: Some(Duration::from_micros(frame_interval_us)),
            };
            msg_string = format!("{}           ", temp).to_string();
        } else {
            msg_string = DisplayMessageStringUtils::for_local(local);
            if !TimeSeparators::time_separators_animation(micros) {
                msg_string = msg_string.replace(":", " ");
                msg_string = msg_string.replace(".", " ");
                // msg_string = "            ".to_string();
            }
            frame_lingers = PwmAnimation::pwm_seconds_animation(
                local.timestamp_subsec_micros(),
                frame_interval_us,
            );
        }

        let res = self.show(NCS3148CMessage::from_string(
            msg_string,
            frame_lingers,
        ));

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
            off_linger.map(|off| thread::sleep(off));
            self.le_pin.set_high();
            on_linger.map(|on| thread::sleep(on));
        }
        Ok(())
    }
}
