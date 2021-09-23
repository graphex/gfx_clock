extern crate easer;

use chrono::prelude::*;
use easer::functions::*;
use rand::Rng;
use std::time::Duration;

use crate::clock_objects::LingerDurations;

pub struct DisplayMessageStringUtils {}

impl DisplayMessageStringUtils {
    pub fn for_local(local: DateTime<Local>) -> String {
        let mut s = local.format("%I:%M:%S%.3f").to_string();
        s.push_str(" ");
        s
    }
}

#[derive(Debug)]
pub enum Overlay {
    AntiPoison(AntiPoisonAnimation),
    TempOverlay(TempOverlayAnimation),
}

#[derive(Debug)]
pub enum AntiPoisiontAnimationStyle {
    Random,
    Sequential,
}
#[derive(Debug)]
pub struct AntiPoisonAnimation {
    pub tube_position: u8,
    pub style: AntiPoisiontAnimationStyle,
    pub start_time: DateTime<Local>,
    pub duration: Duration,
}

impl AntiPoisonAnimation {
    fn has_ended(&self) -> bool {
        self.start_time > Local::now()
    }
}
#[derive(Debug)]
pub struct TempOverlayAnimation {
    pub start_time: DateTime<Local>,
    pub duration: Duration,
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
pub struct PwmAnimation {}
impl PwmAnimation {
    pub fn pwm_seconds_animation(micros: u32, frame_interval_us: u64) -> LingerDurations {
        if micros < 750_000 {
            let p: f32;
            p = Sine::ease_in(micros as f32, 1f32, -1f32, 750_000f32);
            LingerDurations {
                off: Some(Duration::from_micros(((1f32 - p) * frame_interval_us as f32) as u64)),
                on: Some(Duration::from_micros((p * frame_interval_us as f32) as u64)),
            }
        } else if micros < 900_000 {
            LingerDurations {
                off: Some(Duration::from_micros(frame_interval_us)),
                on: Some(Duration::from_micros(0)),
            }
        } else {
            let p: f32;
            p = Quint::ease_in((micros - 900_000u32) as f32, 0f32, 1f32, 100_000f32);
            LingerDurations {
                off: Some(Duration::from_micros(((1f32 - p) * frame_interval_us as f32) as u64)),
                on: Some(Duration::from_micros((p * frame_interval_us as f32) as u64)),
            }
        }
    }
}
