use crate::clock_driver::*;

mod clock_objects;
mod clock_driver;

use tokio::runtime::{Builder};
use std::error::Error;
use crate::clock_objects::{NCS3148C, DisplayMessage};

const FPS_HZ: f32 = 5000f32; //Approximate Max is 5kHz

pub type Result<T, E = Box<dyn Error>> = std::result::Result<T, E>;

fn main() -> Result<()> {
    let runtime = Builder::new_multi_thread()
        .enable_all()
        .worker_threads(2)
        .thread_name("clockworker")
        .thread_stack_size(2 * 1024 * 1024)
        .build()?;
    runtime.block_on(async {
        runtime.spawn_blocking(|| { timeloop(ClockDisplay::new().expect("Clock Initialization Failed")) });
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
fn timeloop(mut clock: ClockDisplay) {
    let mut frame_interval_us = (1f32 / FPS_HZ * 1000f32 * 1000f32) as u64;
    if frame_interval_us > 100 {
        frame_interval_us = frame_interval_us - 100;
    }
    println!("Clock Interval {:?}us", frame_interval_us);
    loop {
        clock.show_next_frame(frame_interval_us).expect("Clock Display Failed");
    }
}