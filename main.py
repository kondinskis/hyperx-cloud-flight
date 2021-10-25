import logging

from PyQt5.QtGui import *
from PyQt5.QtWidgets import *

from cloud_flight import trigger_battery_level_response, CloudFlightThread

logging.basicConfig(filename="cloud_flight.log", encoding="utf-8", level=logging.DEBUG)


def get_icon(battery_level):
    if battery_level == 101:
        return icons["charging"]
    elif battery_level > 95:
        return icons["full"]
    elif battery_level < 95 and battery_level >= 60:
        return icons["good"]
    elif battery_level < 60 and battery_level >= 40:
        return icons["medium"]
    elif battery_level < 40 and battery_level >= 20:
        return icons["low"]
    elif battery_level < 20:
        return icons["caution"]
    else:
        return icons["empty"]


app = QApplication([])
app.setQuitOnLastWindowClosed(False)

icons = {
    "charging": QIcon("icons/headphones-battery-charging.svg"),
    "full": QIcon("icons/headphones-battery-full.svg"),
    "good": QIcon("icons/headphones-battery-good.svg"),
    "medium": QIcon("icons/headphones-battery-medium.svg"),
    "low": QIcon("icons/headphones-battery-low.svg"),
    "caution": QIcon("icons/headphones-battery-caution.svg"),
    "empty": QIcon("icons/headphones-battery-empty.svg"),
    "muted": QIcon("icons/headphones-muted"),
}

# Adding an icon
icon = icons["full"]

# Adding item on the menu bar
tray = QSystemTrayIcon()
tray.setIcon(icon)
tray.setVisible(True)

# Creating the options
menu = QMenu()
header = QAction("HyperX Cloud Flight")
battery = QAction("Battery level: 100")
battery.triggered.connect(trigger_battery_level_response)

menu.addAction(header)
menu.addAction(battery)

# To quit the app
quit = QAction("Quit")
quit.triggered.connect(app.quit)
menu.addAction(quit)

# Adding options to the System Tray
tray.setContextMenu(menu)


def on_power(value):
    logging.debug("Power: {}".format(value))


def on_mute(value):
    if value:
        tray.setIcon(icons["muted"])
    else:
        trigger_battery_level_response()
    logging.debug("Muted: {}".format(value))


def on_battery(value):
    logging.debug("Battery level: {}".format(value))
    battery.setText("Battery level: {}".format(value))
    tray.setIcon(get_icon(value))


def on_volume(value):
    logging.debug("Volume direction: {}".format(value))


cloud_flight_thread = CloudFlightThread(
    on_power=on_power, on_mute=on_mute, on_battery=on_battery, on_volume=on_volume
)
cloud_flight_thread.start()

app.exec_()
