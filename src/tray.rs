use ksni::ToolTip;
use std::sync::Arc;
use std::vec::Vec;

use crate::cloud_flight;

const HEADPHONES_MUTED: &str = "audio-input-microphone-muted";
const HEADPHONES_BATTERY_CHARGING: &str = "battery-060-charging";
const HEADPHONES_BATTERY_FULL: &str = "audio-headphones";
const HEADPHONES_BATTERY_GOOD: &str = "audio-headphones";
const HEADPHONES_BATTERY_MEDIUM: &str = "audio-headphones";
const HEADPHONES_BATTERY_LOW: &str = "audio-headphones";
const HEADPHONES_BATTERY_CAUTION: &str = "battery-010";
const HEADPHONES_BATTERY_EMPTY: &str = "battery-empty.svg";

pub struct Tray {
    pub muted: bool,
    pub battery: u8,
    cf: Arc<cloud_flight::CloudFlight>,
}

impl ksni::Tray for Tray {
    fn icon_name(&self) -> String {
        if self.muted {
            HEADPHONES_MUTED.to_string()
        } else {
            battery_icon(self.battery).into()
        }
    }
    fn tool_tip(&self) -> ToolTip {
        ToolTip {
            title: "HyperX Cloud Flight".into(),
            description: format!("Battery: {}%", self.battery),
            icon_name: "".into(),
            icon_pixmap: Vec::new(),
        }
    }
    fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
        use ksni::menu::*;
        vec![
            StandardItem {
                label: "HyperX Cloud Flight".into(),
                ..Default::default()
            }
            .into(),
            StandardItem {
                label: format!("Muted: {}", self.muted),
                ..Default::default()
            }
            .into(),
            StandardItem {
                label: format!("Battery level: {}", self.battery),
                activate: {
                    let cf = self.cf.clone();
                    Box::new(move |_| cf.clone().battery())
                },
                ..Default::default()
            }
            .into(),
            StandardItem {
                label: "Quit".into(),
                activate: Box::new(|_| std::process::exit(0)),
                ..Default::default()
            }
            .into(),
        ]
    }
}

pub struct TrayService {
    handle: ksni::Handle<Tray>,
}

impl TrayService {
    pub fn new(cf: Arc<cloud_flight::CloudFlight>) -> Self {
        let svc = ksni::TrayService::new(Tray {
            muted: false,
            battery: 100,
            cf: cf,
        });
        let handle = svc.handle();
        svc.spawn();
        TrayService { handle: handle }
    }
    pub fn update<F: Fn(&mut Tray)>(&self, f: F) {
        self.handle.update(f);
    }
}

fn battery_icon(battery: u8) -> String {
    match battery {
        0..=19 => HEADPHONES_BATTERY_CAUTION,
        20..=39 => HEADPHONES_BATTERY_LOW,
        40..=59 => HEADPHONES_BATTERY_MEDIUM,
        60..=89 => HEADPHONES_BATTERY_GOOD,
        90..=100 => HEADPHONES_BATTERY_FULL,
        101 => HEADPHONES_BATTERY_CHARGING,
        _ => HEADPHONES_BATTERY_EMPTY,
    }
    .to_string()
}
