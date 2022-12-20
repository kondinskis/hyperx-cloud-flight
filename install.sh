#!/bin/bash
# Author: Evandro Begati
# Tested on Linux Mint 21.1 (Ubuntu 22.04 LTS)

# Install dependency
sudo apt install -y libhidapi-hidraw0

# Download app
sudo mkdir /opt/hyperx-headset
sudo chown $USER:$USER /opt/hyperx-headset
wget https://github.com/begati/hyperx-cloud-flight-tray-icon/releases/download/stable/cloud-flight-tray-icon_amd64 -O /opt/hyperx-headset/cloud-flight-tray-icon_amd64
chmod +x /opt/hyperx-headset/cloud-flight-tray-icon_amd64

# Create auto start entry
echo $'[Desktop Entry]
Exec=/opt/hyperx-headset/cloud-flight-tray-icon_amd64
Terminal=false
Type=Application
Name=Cloud Flight Tray Icon
'>/home/$USER/.config/autostart/cloud-flight-tray-icon.desktop

# Allow non root user to control the headset from the app
sudo echo $'KERNEL=="hidraw*", ATTRS{idVendor}=="0951", ATTRS{idProduct}=="16c4", MODE="0666"
KERNEL=="hidraw*", ATTRS{idVendor}=="0951", ATTRS{idProduct}=="1723", MODE="0666"
KERNEL=="hidraw*", ATTRS{idVendor}=="0951", ATTRS{idProduct}=="1749", MODE="0666"'>/etc/udev/rules.d/99-hyperx-cloud-flight.rules

# Alert user to unplug and replug the usb dongle
clear
read -p "Please, unplug the Headset USB dongle, plug it again and press any key to finish the installation." temp </dev/tty

# Execute the app and exit
nohup /opt/hyperx-headset/cloud-flight-tray-icon_amd64 >/dev/null 2>&1 &