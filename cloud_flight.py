import logging

import threading
import hid
import schedule

vendor_id = 0x0951
product_id = 0x16C4


def trigger_battery_level_response():
    with hid.Device(vendor_id, product_id) as device:
        device.write(
            bytes(
                [
                    0x21,
                    0xFF,
                    0x05,
                    0x00,
                    0x00,
                    0x00,
                    0x00,
                    0x00,
                    0x00,
                    0x00,
                    0x00,
                    0x00,
                    0x00,
                    0x00,
                    0x00,
                    0x00,
                    0x00,
                    0x00,
                    0x00,
                    0x00,
                ]
            )
        )


def battery_level(charge_state, value):
    logging.debug("Charge state: {} # value: {}".format(charge_state, value))
    # charging
    if charge_state == 0x10 or charge_state == 0x11:
        return 101

    if charge_state == 0xF:
        if value >= 130:
            return 100
        if value < 130 and value >= 120:
            return 95
        if value < 120 and value >= 100:
            return 90
        if value < 100 and value >= 70:
            return 85
        if value < 70 and value >= 50:
            return 80
        if value < 50 and value >= 20:
            return 75
        if value < 20 and value > 0:
            return 70

    if charge_state == 0xE:
        if value > 240:
            return 65
        if value < 240 and value >= 220:
            return 60
        if value < 220 and value >= 208:
            return 55
        if value < 208 and value >= 200:
            return 50
        if value < 200 and value >= 190:
            return 45
        if value < 190 and value >= 180:
            return 40
        if value < 179 and value >= 169:
            return 35
        if value < 169 and value >= 159:
            return 30
        if value < 159 and value >= 148:
            return 25
        if value < 148 and value >= 119:
            return 20
        if value < 119 and value >= 90:
            return 15
        if value < 90:
            return 10
    
    return 0


def volume_direction(value):
    if value == 0x01:
        return "+"
    elif value == 0x02:
        return "-"
    return None


class CloudFlightThread(threading.Thread):
    def __init__(self, on_power, on_mute, on_battery, on_volume):
        super(CloudFlightThread, self).__init__(
            target=self._collect,
            args=[on_power, on_mute, on_battery, on_volume],
            daemon=True,
        )
        trigger_battery_level_response()
        schedule.every(5).minutes.do(trigger_battery_level_response)

    def _collect(self, on_power, on_mute, on_battery, on_volume):
        while True:
            for hid_device in hid.enumerate(vendor_id, product_id):
                with hid.Device(path=hid_device["path"]) as device:
                    data = device.read(2048, 100)

                    # power on/off/muted
                    if len(data) == 2:
                        if data[0] == 0x64:
                            if data[1] == 0x1:
                                trigger_battery_level_response()
                                on_power(True)
                            elif data[1] == 0x3:
                                on_power(False)

                        if data[0] == 0x65:
                            on_mute(data[1] == 0x04)
                    # volume direction
                    elif len(data) == 5:
                        vol_dir = volume_direction(data[1])
                        if vol_dir is not None:
                            on_volume(vol_dir)
                    # battery status
                    elif len(data) == 20:
                        on_battery(battery_level(data[3], data[4]))
            schedule.run_pending()
