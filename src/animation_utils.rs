extern crate easer;

use std::ops::Add;
use std::sync::{Arc, RwLock};
use chrono::prelude::*;
use chrono::Duration;
use easer::functions::*;
use rand::Rng;
use rand::prelude::SliceRandom;

use crate::clock_objects::{ClockType, DisplayMessage};
use crate::clock_objects::LingerDurations;

pub struct DisplayMessageStringUtils {}

impl DisplayMessageStringUtils {
    pub fn for_local(local: DateTime<Local>, clock_type: ClockType) -> String {
        match clock_type {
            ClockType::NCS3148C => {
                let mut s = local.format("%I:%M:%S%.3f").to_string();
                s.push_str(" ");
                s
            },
            ClockType::NCS3186 => {
                local.format("%I:%M:%S").to_string()
            }
        }

    }
}

pub trait Overlayable {
    fn has_ended(&self, current_time: DateTime<Local>) -> bool;
    fn is_visible(&self, current_time: DateTime<Local>) -> bool;
}

#[derive(Debug)]
pub enum Overlay {
    AntiPoison(AntiPoisonAnimation),
    TempOverlay(TempOverlayAnimation),
}

#[derive(Debug, PartialEq, Eq)]
pub enum AntiPoisonAnimationStyle {
    Random,
    Sequential,
}

#[derive(Debug)]
pub struct AntiPoisonAnimation {
    pub tube_idx: usize,
    pub style: AntiPoisonAnimationStyle,
    pub start_time: DateTime<Local>,
    pub duration: Duration,
    cycle: Vec<char>,
    fade_pct: f32,
}

impl AntiPoisonAnimation {
    pub fn new(
        tube_idx: usize,
        style: AntiPoisonAnimationStyle,
        start_time: DateTime<Local>,
        duration: Duration,
        cycle: Vec<char>,
        fade_pct: f32,
    ) -> AntiPoisonAnimation {
        let mut retval = AntiPoisonAnimation {
            tube_idx: tube_idx,
            style: style,
            start_time: start_time,
            duration: duration,
            cycle: cycle,
            fade_pct: fade_pct,
        };
        if retval.style == AntiPoisonAnimationStyle::Random {
            retval.cycle.shuffle(&mut rand::thread_rng());
        }
        retval
    }

    pub fn matrix_style_set() -> Vec<Overlay> {
        //numeric tube anti-poisions, skipping sub-second tubes which aren't at risk of poisioning
        let mut rng = rand::thread_rng();
        let mut numeric_tube_positions = vec![0, 1, 3, 4, 6, 7];
        numeric_tube_positions.shuffle(&mut rng);
        let num_antipoisons = rng.gen_range(0..9);
        let mut set_for_minute: Vec<Overlay> = vec![];
        for i in 0..num_antipoisons {
            let cur_anti_poison = AntiPoisonAnimation::new(
                numeric_tube_positions[i % numeric_tube_positions.len()],
                AntiPoisonAnimationStyle::Random,
                Local::now().with_second(rng.gen_range(5..55)).unwrap(),
                Duration::seconds(2),
                vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'],
                0.3f32,
            );
            set_for_minute.push(Overlay::AntiPoison(cur_anti_poison));
        }
        set_for_minute
    }

    pub fn apply_to_message(
        &self,
        current_time: DateTime<Local>,
        cur_message: &mut impl DisplayMessage,
    ) -> () {
        if self.is_visible(current_time) {
            if let Some(v) = self.get_current_value(current_time) {
                cur_message.set_tube(self.tube_idx, v).unwrap()
            }
        }
    }
    fn get_current_value(&self, current_time: DateTime<Local>) -> Option<char> {
        if let Some(t) = (current_time - self.start_time)
            .num_microseconds()
            .map(|t| t as f32)
        {
            if let Some(d) = self.duration.num_microseconds().map(|d| d as f32) {
                let p = Linear::ease_in(t, 0f32, 70f32, d as f32) as usize;
                let cur_char = Some(self.cycle[p % self.cycle.len()]);
                //use PFM to fade between this char and the underlying time value
                if t < (d * self.fade_pct) {
                    let in_p = Linear::ease_in(t, 0f32, 255f32, d * self.fade_pct);
                    if rand::thread_rng().gen_range(0..255) > in_p as isize {
                        None
                    } else {
                        cur_char
                    }
                } else if t > (d - (d * self.fade_pct)) {
                    let out_p =
                        Linear::ease_out(t - (d - (d * self.fade_pct)), 0f32, 255f32, d * self.fade_pct);
                    if rand::thread_rng().gen_range(0..255) > out_p as isize {
                        cur_char
                    } else {
                        None
                    }
                } else {
                    //overlay only
                    cur_char
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Overlayable for AntiPoisonAnimation {
    fn has_ended(&self, current_time: DateTime<Local>) -> bool {
        self.start_time + self.duration < current_time
    }
    fn is_visible(&self, current_time: DateTime<Local>) -> bool {
        self.start_time < current_time && self.start_time + self.duration > current_time
    }
}

#[derive(Debug)]
pub struct TempOverlayAnimation {
    pub start_time: DateTime<Local>,
    pub duration: Duration,
    //a Some(None) means to stop trying to read the temp, it doesn't exist
    temperature_celsius: Option<Option<f32>>,
    temperature_lock: Arc<RwLock<Option<f32>>>,
    clock_type: ClockType,
}

impl TempOverlayAnimation {
    pub fn new(
        temperature_lock: Arc<RwLock<Option<f32>>>,
        clock_type: ClockType,
    ) -> TempOverlayAnimation {
        TempOverlayAnimation {
            start_time: Local::now().with_second(16).unwrap(),
            duration: Duration::seconds(3),
            temperature_celsius: None,
            temperature_lock: temperature_lock,
            clock_type: clock_type,
        }
    }
    pub fn apply_to_message(
        &mut self,
        current_time: DateTime<Local>,
        cur_message: &mut impl DisplayMessage,
    ) -> () {
        if self.is_visible(current_time) {
            if let Some(temp_string) = self.get_temperature_string() {
                cur_message.set_from_string(temp_string).unwrap();
                cur_message.set_lingers(LingerDurations {
                    off: Some(Duration::microseconds(0)),
                    on: Some(Duration::microseconds(99)),
                }).expect("Attempt to set bad message linger");
            }
        }
    }
    fn get_temperature_string(&mut self) -> Option<String> {
        if self.temperature_celsius.is_none() {
            let t_lock = self.temperature_lock.read().unwrap();
            if t_lock.is_some() {
                let temp = t_lock.unwrap();
                self.temperature_celsius = Some(Some(temp));
            } else {
                self.temperature_celsius = Some(None);//This says we've checked the temp lock but there was no reported temp
            }
        }
        if let Some(cur_temp) = self.temperature_celsius.flatten() {
            // Some("12.34'56.78℃".to_string())
            match self.clock_type {
                ClockType::NCS3148C => Some(format!("{:2.2}'{:2.2}℃", ((cur_temp*(9f32/5f32))+32f32)%100f32, cur_temp%100f32)),
                ClockType::NCS3186 => Some(format!("{:2.2}'  ", ((cur_temp*(9f32/5f32))+32f32)%100f32)),
            }
        } else {
            None
        }
    }
}

impl Overlayable for TempOverlayAnimation {
    fn has_ended(&self, current_time: DateTime<Local>) -> bool {
        self.start_time + self.duration < current_time
    }
    fn is_visible(&self, current_time: DateTime<Local>) -> bool {
        self.start_time < current_time && self.start_time + self.duration > current_time
    }
}

pub struct TimeSeparators {}

impl TimeSeparators {
    //a pulse frequency modulation-based animation
    pub fn time_separators_animation(micros: u32) -> bool {
        if micros < 750_000 {
            let p: f32;
            p = Bounce::ease_in(micros as f32, 255f32, -255f32, 750_000f32);
            rand::thread_rng().gen_range(0..255) < p as isize
        } else {
            false
        }
    }
}

pub struct PwmFadeOut {
    pub start_time: DateTime<Local>,
    pub duration: Duration,
    pub frame_interval_us: i64,
}

impl PwmFadeOut {
    pub fn get_linger_durations(&self) -> LingerDurations {
        LingerDurations {
            off: Some(Duration::microseconds(self.frame_interval_us)),
            on: Some(Duration::microseconds(0)),
        }
    }
}

pub struct PwmFadeIn {
    pub start_time: DateTime<Local>,
    pub duration: Duration,
    pub frame_interval_us: i64,
}

impl PwmFadeIn {
    pub fn get_linger_durations(&self) -> LingerDurations {
        LingerDurations {
            off: Some(Duration::microseconds(0)),
            on: Some(Duration::microseconds(self.frame_interval_us)),
        }
    }
}

pub struct PwmAnimation {
    pub frame_interval_us: i64,
}

impl PwmAnimation {
    pub fn pwm_seconds_animation(&self, micros: u32) -> LingerDurations {
        // if micros < 750_000 {
        //     let p: f32;
        //     p = Sine::ease_in(micros as f32, 1f32, -1f32, 750_000f32);
        //     LingerDurations {
        //         off: Some(Duration::microseconds(
        //             ((1f32 - p) * self.frame_interval_us as f32) as i64,
        //         )),
        //         on: Some(Duration::microseconds(
        //             (p * self.frame_interval_us as f32) as i64,
        //         )),
        //     }
        // } else if micros < 900_000 {
        //     LingerDurations {
        //         off: Some(Duration::microseconds(self.frame_interval_us as i64)),
        //         on: Some(Duration::microseconds(0)),
        //     }
        // } else {
        //     let p: f32;
        //     p = Quint::ease_in((micros - 900_000u32) as f32, 0f32, 1f32, 100_000f32);
        //     LingerDurations {
        //         off: Some(Duration::microseconds(
        //             ((1f32 - p) * self.frame_interval_us as f32) as i64,
        //         )),
        //         on: Some(Duration::microseconds(
        //             (p * self.frame_interval_us as f32) as i64,
        //         )),
        //     }
        // }
        LingerDurations {
            off: Some(Duration::microseconds(0)),
            on: Some(Duration::microseconds(99)),
        }
    }
}
