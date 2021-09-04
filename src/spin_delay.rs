//! Miscellaneous `embedded-hal` trait implementations.
//!
//! The `hal` module consists of a collection of `embedded-hal` trait
//! implementations for traits that aren't tied to a specific peripheral.
//!
//! This module is only included when either the `hal` or `hal-unproven` feature
//! flag is enabled.

use std::thread;
use std::time::{Duration, Instant};

use embedded_hal::blocking::delay::{DelayMs, DelayUs};
use embedded_hal::timer::CountDown;
use spin_sleep;

/// Implements the `embedded-hal` `DelayMs` and `DelayUs` traits.
#[derive(Debug, Default)]
pub struct Delay {}

impl Delay {
    /// Constructs a new `Delay`.
    pub fn new() -> Delay {
        Delay {}
    }
}

impl DelayMs<u8> for Delay {
    fn delay_ms(&mut self, ms: u8) {
        spin_sleep::sleep(Duration::from_millis(u64::from(ms)));
    }
}

impl DelayMs<u16> for Delay {
    fn delay_ms(&mut self, ms: u16) {
        spin_sleep::sleep(Duration::from_millis(u64::from(ms)));
    }
}

impl DelayMs<u32> for Delay {
    fn delay_ms(&mut self, ms: u32) {
        spin_sleep::sleep(Duration::from_millis(u64::from(ms)));
    }
}

impl DelayMs<u64> for Delay {
    fn delay_ms(&mut self, ms: u64) {
        spin_sleep::sleep(Duration::from_millis(ms));
    }
}

impl DelayUs<u8> for Delay {
    fn delay_us(&mut self, us: u8) {
        spin_sleep::sleep(Duration::from_micros(u64::from(us)));
    }
}

impl DelayUs<u16> for Delay {
    fn delay_us(&mut self, us: u16) {
        spin_sleep::sleep(Duration::from_micros(u64::from(us)));
    }
}

impl DelayUs<u32> for Delay {
    fn delay_us(&mut self, us: u32) {
        spin_sleep::sleep(Duration::from_micros(u64::from(us)));
    }
}

impl DelayUs<u64> for Delay {
    fn delay_us(&mut self, us: u64) {
        spin_sleep::sleep(Duration::from_micros(us));
    }
}
