use clokwerk::{Scheduler, TimeUnits};
use log::info;
use simple_logger::SimpleLogger;
use std::sync::Arc;
use std::thread;

const BATTERY_REFRESH_ON_MUTE: bool = true;

mod cloud_flight;
mod tray;

fn main() {
    SimpleLogger::new().init().unwrap();

    let cf = Arc::new(cloud_flight::CloudFlight::new());
    let svc = tray::TrayService::new(cf.clone());

    let mut scheduler = Scheduler::new();

    let handle = thread::spawn(move || {
        let s_cf = cf.clone();
        scheduler.every(3.minutes()).run(move || {
            s_cf.battery();
        });

        cf.battery();

        loop {
            scheduler.run_pending();
            let event = cf.read();
            svc.update(|tray: &mut tray::Tray| {
                match event {
                    cloud_flight::EventType::BatteryCharging => {
                        info!("Battery charging")
                    }
                    cloud_flight::EventType::Battery { value } => {
                        tray.battery = value;
                        info!("Battery {}", value)
                    }
                    cloud_flight::EventType::VolumeUp => {
                        info!("Volume up")
                    }
                    cloud_flight::EventType::VolumeDown => {
                        info!("Volume down")
                    }
                    cloud_flight::EventType::Muted => {
                        tray.muted = true;
                        if BATTERY_REFRESH_ON_MUTE {
                            cf.battery();
                        }
                        info!("Muted")
                    }
                    cloud_flight::EventType::Unmuted => {
                        tray.muted = false;
                        info!("Unmuted")
                    }
                    cloud_flight::EventType::PowerOff => info!("Power off"),
                    cloud_flight::EventType::PowerOn => info!("Power on"),
                    cloud_flight::EventType::Ignored => (),
                };
            });
        }
    });
    handle.join().unwrap();
}
