use hidapi::{HidApi, HidDevice};
use log::debug;

const VENDOR_ID: u16 = 0x0951;
const PRODUCT_ID: u16 = 0x16c4;

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

pub enum EventType {
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
}
impl CloudFlight {
    pub fn new() -> Self {
        let api = HidApi::new().unwrap();

        CloudFlight {
            device: api.open(VENDOR_ID, PRODUCT_ID).unwrap(),
        }
    }
    pub fn read(&self) -> EventType {
        let mut buf = [0u8; 32];
        let bytes = self.device.read_timeout(&mut buf, 500).unwrap();
        debug!("Read: {}, {:02x?}", bytes, buf);
        match bytes {
            2 => {
                if buf[0] == 0x64 {
                    if buf[1] == 0x01 {
                        self.battery();
                        return EventType::PowerOn;
                    } else if buf[1] == 0x03 {
                        return EventType::PowerOff;
                    }
                }
                if buf[0] == 0x65 {
                    if buf[1] == 0x04 {
                        return EventType::Muted;
                    } else {
                        return EventType::Unmuted;
                    }
                }
                return EventType::Ignored;
            }
            5 => {
                if buf[1] == 0x01 {
                    return EventType::VolumeUp;
                } else if buf[1] == 0x02 {
                    return EventType::VolumeDown;
                }
                return EventType::Ignored;
            }
            20 => {
                if (buf[3] == 0x10 || buf[3] == 0x11) && buf[4] >= 20 {
                    return EventType::BatteryCharging;
                }
                return EventType::Battery {
                    value: battery_percent(buf[3], buf[4]),
                };
            }
            _ => return EventType::Ignored,
        }
    }
    pub fn battery(&self) {
        self.device.write(&BATTERY_TRIGGER_PACKET).unwrap();
    }
}

unsafe impl Sync for CloudFlight {}
