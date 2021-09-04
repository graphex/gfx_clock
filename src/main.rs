#![allow(dead_code, unused_imports)]

use crate::{clock_driver::*, clock_objects::ClockType};

mod clock_driver;
mod clock_objects;
mod tube_objects;
mod spin_delay;

use crate::clock_objects::{DisplayMessage, NCS3148CMessage};
use ds18b20::{Ds18b20, Resolution};
use embedded_hal::blocking::delay::{DelayMs, DelayUs};
use embedded_hal::digital::v2::{InputPin, OutputPin};
use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayUs;
use one_wire_bus::{OneWire, OneWireError, OneWireResult};
use rppal::gpio::{Gpio, InputPin as RppalInputPin, OutputPin as RppalOutputPin};
// use rppal::hal::Delay;
use crate::spin_delay::Delay;
use std::error::Error;
use std::fmt::{Debug, Write};
use std::thread::sleep;
use std::time::Duration;
use std::{fmt, thread};
use tokio::runtime::Builder;
use spin_sleep;
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
    let clock_driver = match clock_type {
        ClockType::NCS3148C => NCS3148CDriver::new(),
        ClockType::NCS3186 => NCS3148CDriver::new(),
    }
    .expect("Clock Initialization Failed");

    thread::spawn(|| temp_sensor());
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

fn temp_sensor() -> ! {
    const TMP_PIN: u8 = 5;
    loop {
        println!("Getting Temperature");
        let one_wire_pin = Gpio::new().unwrap().get(TMP_PIN).unwrap().into_output();
        let mut one_wire_bus = OneWire::new(one_wire_pin).unwrap();
        get_temperature(&mut one_wire_bus);
        thread::sleep(Duration::from_millis(2000));
    }
}

fn get_temperature<P, E>(one_wire_bus: &mut OneWire<P>) -> OneWireResult<(), E>
where
    P: OutputPin<Error = E> + InputPin<Error = E>,
    E: Debug,
{
    let mut delay = Delay::new();
    // initiate a temperature measurement for all connected devices
    ds18b20::start_simultaneous_temp_measurement(one_wire_bus, &mut delay)?;

    // wait until the measurement is done. This depends on the resolution you specified
    // If you don't know the resolution, you can obtain it from reading the sensor data,
    // or just wait the longest time, which is the 12-bit resolution (750ms)
    Resolution::Bits12.delay_for_measurement_time(&mut delay);

    // iterate over all the devices, and report their temperature
    let mut search_state = None;
    loop {
        if let Some((device_address, state)) =
            one_wire_bus.device_search(search_state.as_ref(), false, &mut delay)?
        {
            println!("Found device");
            search_state = Some(state);
            if device_address.family_code() != ds18b20::FAMILY_CODE {
                // skip other devices
                continue;
            }
            // You will generally create the sensor once, and save it for later
            let sensor = Ds18b20::new(device_address)?;

            // contains the read temperature, as well as config info such as the resolution used
            let sensor_data = sensor.read_data(one_wire_bus, &mut delay)?;
            println!(
                "Device at {:?} is {}°C",
                device_address, sensor_data.temperature
            );
        } else {
            println!("No device found");
            break;
        }
    }
    Ok(())
}
