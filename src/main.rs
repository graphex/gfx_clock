#![allow(dead_code, unused_imports)]

use crate::{clock_driver::*, clock_objects::ClockType};

mod clock_driver;
mod clock_objects;
mod spin_delay; //will be unnecessary once new version of rppal is released
mod temperature_sensor;
mod tube_objects;
mod animation_utils;

use crate::clock_objects::{DisplayMessage, NCS3148CMessage};
use crate::temperature_sensor::TemperatureSensor;
use std::env::temp_dir;
use std::error::Error;
use std::fmt::{Debug, Write};
use std::thread::sleep;
use std::time::Duration;
use std::{fmt, thread};
use std::sync::Arc;
use tokio::runtime::Builder;
use typenum::U96;

const FPS_HZ: f32 = 5000f32; //Approximate Max is 5kHz

pub type Result<T, E = Box<dyn Error>> = std::result::Result<T, E>;

#[derive(Debug)]
enum ArgumentError {
    ClockTypeNeeded,
}
impl Error for ArgumentError {}
impl fmt::Display for ArgumentError {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Specify clock type as the first arg, NCS3148C | NCS3186")
    }
}

fn main() -> Result<()> {
    let clock_type = match std::env::args().nth(1).as_deref() {
        Some("NCS3148C") => ClockType::NCS3148C,
        Some("NCS3186") => ClockType::NCS3186,
        _ => {
            println!("Specify clock type as the first arg, NCS3148C | NCS3186");
            return Result::Err(Box::new(ArgumentError::ClockTypeNeeded));
        }
    };
    let runtime = Builder::new_multi_thread()
        .enable_all()
        .worker_threads(2)
        .thread_name("clockworker")
        .thread_stack_size(2 * 1024 * 1024)
        .build()?;

    let mut temp_sensor =  temperature_sensor::TemperatureSensor::new();
    let temperature_lk = temp_sensor.raw_degrees_c.clone();
    thread::spawn(move || temp_sensor.run_temp_sensor());

    let clock_driver = match clock_type {
        ClockType::NCS3148C => NCS3148CDriver::new(temperature_lk),
        ClockType::NCS3186 => NCS3148CDriver::new(temperature_lk),
    }
    .expect("Clock Initialization Failed");

    runtime.block_on(async {
        // runtime.spawn_blocking(|| temp_sensor());
        runtime.spawn_blocking(|| timeloop(clock_driver));
        wait_for_signal().await;
        println!("Exiting clock");
    });
    println!("Shutting down clock");
    runtime.shutdown_background();
    Ok(())
}

async fn wait_for_signal() {
    use tokio::signal::unix::{signal, SignalKind};
    let mut term = signal(SignalKind::terminate()).expect("failed to register signal handler");
    let mut int = signal(SignalKind::interrupt()).expect("failed to register signal handler");
    println!("Watching for signals");
    tokio::select! {
        _ = term.recv() => println!("Received SIGTERM"),
        _ = int.recv() => println!("Received SIGINT"),
    }
}

/// This has to be a pretty hot loop, looking for 200μs or higher precision for 5kHz
/// and async isn't cutting it, with around 1ms being the min delay
fn timeloop<T: ClockDriver>(mut clock: T) -> ! {
    let mut frame_interval_us = (1f32 / FPS_HZ * 1000f32 * 1000f32) as u64;
    if frame_interval_us > 100 {
        frame_interval_us = frame_interval_us - 100;
    }
    println!("Clock Interval {:?}us", frame_interval_us);
    loop {
        clock
            .show_next_frame(frame_interval_us)
            .expect("Clock Display Failed");
    }
}
