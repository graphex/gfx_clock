use std::fmt::{Debug, Write};
use std::sync::{Arc, RwLock};
use std::thread;
use chrono::prelude::*;
use chrono::Duration;
use ds18b20::{Ds18b20, Resolution};
use embedded_hal::blocking::delay::{DelayMs, DelayUs};
use embedded_hal::digital::v2::{InputPin, OutputPin};
use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayUs;
use one_wire_bus::{OneWire, OneWireError, OneWireResult};
use rppal::gpio::{Gpio, InputPin as RppalInputPin, OutputPin as RppalOutputPin};
use spin_sleep;
//will be unnecessary once new version of rppal is released
// use rppal::hal::Delay;
use crate::spin_delay::Delay;

#[derive(Debug)]
pub struct TemperatureSensor {
    raw_degrees_c: Arc<RwLock<Option<f32>>>,
    temperature_updated_at: Option<DateTime<Local>>,
}

impl TemperatureSensor {
    const TMP_PIN: u8 = 5;

    //NB: this is blocking and should only be run in a separate thread
    pub fn run(temperature_lock: Arc<RwLock<Option<f32>>>) -> ! {
        let mut sensor = TemperatureSensor {
            raw_degrees_c: temperature_lock,
            temperature_updated_at: None,
        };
        sensor.run_temp_sensor();
    }

    fn run_temp_sensor(&mut self) -> ! {
        loop {
            let one_wire_pin = Gpio::new()
                .unwrap()
                .get(TemperatureSensor::TMP_PIN)
                .unwrap()
                .into_output();
            let mut one_wire_bus = OneWire::new(one_wire_pin).unwrap();
            let res = self.get_temperature(&mut one_wire_bus);
            match res {
                Ok(cur_reading) => {
                    let mut temp_lock = self.raw_degrees_c.write().unwrap();
                    *temp_lock = Some(cur_reading);
                    drop(temp_lock);
                    self.temperature_updated_at = Some(Local::now());
                    // println!("Temperature succeeded");
                }
                Err(e) => {
                    if let Some(last_update) = self.temperature_updated_at {
                        if last_update + Duration::minutes(2) < Local::now() {
                            //temperature is stale, so don't keep showing it
                            let mut temp_lock = self.raw_degrees_c.write().unwrap();
                            *temp_lock = None;
                            drop(temp_lock);
                            self.temperature_updated_at = None;
                        }
                    }
                    println!("Temperature Failed: {:?}", e);
                }
            }
            thread::yield_now();
            thread::sleep(Duration::seconds(5).to_std().unwrap());
        }
    }

    fn get_temperature<P, E>(&mut self, one_wire_bus: &mut OneWire<P>) -> OneWireResult<f32, E>
        where
            P: OutputPin<Error=E> + InputPin<Error=E>,
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
                    device_address,
                    sensor_data.temperature,
                );
                return Ok(sensor_data.temperature);
            } else {
                // println!("No device found");
                break;
            }
        }
        Err(OneWireError::Timeout)
    }
}
