extern crate easer;

use chrono::prelude::*;
use chrono::Duration;
use easer::functions::*;
use rand::Rng;

use crate::clock_objects::DisplayMessage;
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
pub enum AntiPoisonAnimationStyle {
    Random,
    Sequential,
}
#[derive(Debug)]
pub struct AntiPoisonAnimation {
    pub tube_position: u8,
    pub style: AntiPoisonAnimationStyle,
    pub start_time: DateTime<Local>,
    pub duration: Duration,
}

impl AntiPoisonAnimation {
    pub fn has_ended(&self) -> bool {
        self.start_time + self.duration < Local::now()
    }
    pub fn is_visible(&self) -> bool {
        self.start_time < Local::now() &&
        self.start_time + self.duration > Local::now()
    }
}
#[derive(Debug)]
pub struct TempOverlayAnimation {
    pub start_time: DateTime<Local>,
    pub duration: Duration,
}
impl TempOverlayAnimation {
    pub fn has_ended(&self) -> bool {
        self.start_time + self.duration < Local::now()
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

pub struct PwmFadeOut{
    pub start_time: DateTime<Local>,
    pub duration: Duration,
    pub frame_interval_us: i64,
}
impl PwmFadeOut{
    pub fn get_linger_durations(&self) -> LingerDurations {
        LingerDurations {
            off: Some(Duration::microseconds(self.frame_interval_us)),
            on: Some(Duration::microseconds(0)),
        }
    } 
}
pub struct PwmFadeIn{
    pub start_time: DateTime<Local>,
    pub duration: Duration,
    pub frame_interval_us: i64,
}
impl PwmFadeIn{
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
        if micros < 750_000 {
            let p: f32;
            p = Sine::ease_in(micros as f32, 1f32, -1f32, 750_000f32);
            LingerDurations {
                off: Some(Duration::microseconds(((1f32 - p) * self.frame_interval_us as f32) as i64)),
                on: Some(Duration::microseconds((p * self.frame_interval_us as f32) as i64)),
            }
        } else if micros < 900_000 {
            LingerDurations {
                off: Some(Duration::microseconds(self.frame_interval_us as i64)),
                on: Some(Duration::microseconds(0)),
            }
        } else {
            let p: f32;
            p = Quint::ease_in((micros - 900_000u32) as f32, 0f32, 1f32, 100_000f32);
            LingerDurations {
                off: Some(Duration::microseconds(((1f32 - p) * self.frame_interval_us as f32) as i64)),
                on: Some(Duration::microseconds((p * self.frame_interval_us as f32) as i64)),
            }
        }
    }
}
