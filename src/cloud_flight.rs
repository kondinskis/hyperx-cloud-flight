use hidapi::{HidApi, HidDevice};
use log::{debug, info};
use std::cell::Cell;

const VENDOR_ID: u16 = 0x0951;
const PRODUCT_IDS: [u16; 2] = [0x1723, 0x16c4];

const BATTERY_TRIGGER_PACKET: [u8; 20] = {
    let mut buf = [0; 20];
    buf[0] = 0x21;
    buf[1] = 0xff;
    buf[2] = 0x05;
    buf
};

fn battery_percent(charge_state: u8, value: u8) -> u8 {
    match charge_state {
        0x0e => match value {
            0..=89 => 10,
            90..=119 => 15,
            120..=148 => 20,
            149..=159 => 25,
            160..=169 => 30,
            170..=179 => 35,
            180..=189 => 40,
            190..=199 => 45,
            200..=209 => 50,
            210..=219 => 55,
            220..=239 => 60,
            240..=255 => 65,
        },
        0x0f => match value {
            0..=19 => 70,
            20..=49 => 75,
            50..=69 => 80,
            70..=99 => 85,
            100..=119 => 90,
            120..=129 => 95,
            130..=255 => 100,
        },
        _ => 0,
    }
}

pub enum Event {
    Battery { value: u8 },
    BatteryCharging,
    VolumeUp,
    VolumeDown,
    Muted,
    Unmuted,
    PowerOff,
    PowerOn,
    Ignored,
}
pub struct CloudFlight {
    device: HidDevice,
    pub powered: Cell<bool>,
    pub muted: Cell<bool>,
    pub charging: Cell<bool>,
    pub battery: Cell<u8>,
}
impl CloudFlight {
    pub fn new() -> Self {
        let api = HidApi::new().unwrap();

        let device = PRODUCT_IDS
            .iter()
            .map(|pid| api.open(VENDOR_ID, *pid))
            .filter(|device| device.is_ok())
            .map(|device| device.unwrap())
            .last();

        if device.is_none() {
            panic!("Not found any compatible device");
        }

        CloudFlight {
            device: device.unwrap(),
            powered: Cell::new(true),
            muted: Cell::new(false),
            charging: Cell::new(false),
            battery: Cell::new(100),
        }
    }
    pub fn read(&self) -> Event {
        let mut buf = [0u8; 32];
        let bytes = self.device.read_timeout(&mut buf, 500).unwrap();
        debug!("Read: {}, {:02x?}", bytes, buf);
        match bytes {
            2 => {
                if buf[0] == 0x64 {
                    if buf[1] == 0x01 {
                        self.battery();
                        self.powered.set(true);
                        info!("Power on");
                        return Event::PowerOn;
                    } else if buf[1] == 0x03 {
                        self.powered.set(false);
                        info!("Power off");
                        return Event::PowerOff;
                    }
                }
                if buf[0] == 0x65 {
                    if buf[1] == 0x04 {
                        self.muted.set(true);
                        info!("Muted");
                        return Event::Muted;
                    } else {
                        self.muted.set(false);
                        info!("Unmuted");
                        return Event::Unmuted;
                    }
                }
                return Event::Ignored;
            }
            5 => {
                if buf[1] == 0x01 {
                    info!("Volume up");
                    return Event::VolumeUp;
                } else if buf[1] == 0x02 {
                    info!("Volume down");
                    return Event::VolumeDown;
                }
                return Event::Ignored;
            }
            20 => {
                if buf[3] == 0x10 || buf[3] == 0x11 {
                    info!("Battery charging");
                    self.charging.set(true);
                    if buf[4] >= 20 {
                        return Event::BatteryCharging;
                    }
                    return Event::Battery { value: 100 };
                }
                let b_percent = battery_percent(buf[3], buf[4]);
                info!("Battery {}", b_percent);
                self.charging.set(false);
                self.battery.set(b_percent);
                return Event::Battery { value: b_percent };
            }
            _ => return Event::Ignored,
        }
    }
    pub fn battery(&self) {
        self.device.write(&BATTERY_TRIGGER_PACKET).unwrap();
    }
}

unsafe impl Sync for CloudFlight {}
