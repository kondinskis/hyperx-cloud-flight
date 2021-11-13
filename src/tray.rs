use ksni::menu::StandardItem;
use ksni::ToolTip;
use std::sync::Arc;
use std::vec::Vec;

use crate::cloud_flight;

const HEADPHONES_MUTED: &str = "microphone-sensitivity-muted";
const HEADPHONES_BATTERY_CHARGING: &str = "battery-060-charging";
const HEADPHONES_BATTERY_FULL: &str = "audio-headphones";
const HEADPHONES_BATTERY_GOOD: &str = "audio-headphones";
const HEADPHONES_BATTERY_MEDIUM: &str = "audio-headphones";
const HEADPHONES_BATTERY_LOW: &str = "audio-headphones";
const HEADPHONES_BATTERY_CAUTION: &str = "battery-010";
const HEADPHONES_BATTERY_EMPTY: &str = "battery-empty.svg";

pub struct Tray {
    cf: Arc<cloud_flight::CloudFlight>,
}

impl ksni::Tray for Tray {
    fn icon_name(&self) -> String {
        if self.cf.muted.get() {
            HEADPHONES_MUTED.to_string()
        } else if self.cf.charging.get() {
            HEADPHONES_BATTERY_CHARGING.to_string()
        } else {
            match self.cf.battery.get() {
                0..=19 => HEADPHONES_BATTERY_CAUTION,
                20..=39 => HEADPHONES_BATTERY_LOW,
                40..=59 => HEADPHONES_BATTERY_MEDIUM,
                60..=89 => HEADPHONES_BATTERY_GOOD,
                90..=100 => HEADPHONES_BATTERY_FULL,
                _ => HEADPHONES_BATTERY_EMPTY,
            }
            .to_string()
        }
    }
    fn tool_tip(&self) -> ToolTip {
        let description: String;
        if self.cf.charging.get() {
            description = format!("Charging battery");
        } else {
            description = format!("Battery: {}%", self.cf.battery.get());
        }
        ToolTip {
            title: "HyperX Cloud Flight".into(),
            description: description,
            icon_name: "".into(),
            icon_pixmap: Vec::new(),
        }
    }
    fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
        let muted_text: String;
        if self.cf.muted.get() {
            muted_text = "Yes".into();
        } else {
            muted_text = "No".into();
        }

        let battery_text: String;
        if self.cf.charging.get() {
            battery_text = "Battery charging".into();
        } else {
            battery_text = format!("Battery level: {}", self.cf.battery.get());
        }

        vec![
            StandardItem {
                label: "HyperX Cloud Flight".into(),
                ..Default::default()
            }
            .into(),
            StandardItem {
                label: format!("Muted: {}", muted_text),
                ..Default::default()
            }
            .into(),
            StandardItem {
                label: battery_text,
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
        let svc = ksni::TrayService::new(Tray { cf: cf });
        let handle = svc.handle();
        svc.spawn();
        TrayService { handle: handle }
    }
    pub fn update(&self) {
        self.handle.update(|_: &mut Tray| {});
    }
}
