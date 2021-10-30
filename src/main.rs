use clokwerk::{Scheduler, TimeUnits};
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
            match event {
                cloud_flight::Event::BatteryCharging => (),
                cloud_flight::Event::Battery { value: _ } => (),
                cloud_flight::Event::VolumeUp => (),
                cloud_flight::Event::VolumeDown => (),
                cloud_flight::Event::Muted => {
                    if BATTERY_REFRESH_ON_MUTE {
                        cf.battery();
                    }
                }
                cloud_flight::Event::Unmuted => (),
                cloud_flight::Event::PowerOff => (),
                cloud_flight::Event::PowerOn => (),
                cloud_flight::Event::Ignored => (),
            };
            if !matches!(event, cloud_flight::Event::Ignored) {
                svc.update();
            }
        }
    });
    handle.join().unwrap();
}
