# HyperX Cloud Flight Battery Monitoring

## Introduction

Simple tray application which shows battery level for <em>[HyperX Cloud Flight Wireless Headset.](https://www.hyperxgaming.com/unitedstates/us/headsets/cloud-flight-wireless-gaming-headset)</em>

## Screenshots

<p align="center">
    <img width="200" alt="HyperX Cloud Flight" src="./images/screenshot.png">
    <img width="200" alt="HyperX Cloud Flight" src="./images/screenshot_2.png">
</p>

## Getting Started


### Prerequisites

#### hidraw

Make sure you have hidraw installed on your system.

- Debian/Ubuntu

```
sudo apt install libhidapi-hidraw0
```

- Arch

```
sudo pacman -S hidapi
```

#### udev

Create new file in `/etc/udev/rules.d/99-hyperx-cloud-flight.rules` and place the following content:

```
KERNEL=="hidraw*", ATTRS{idVendor}=="0951", ATTRS{idProduct}=="16c4", MODE="0666"
KERNEL=="hidraw*", ATTRS{idVendor}=="0951", ATTRS{idProduct}=="1723", MODE="0666"
```

Once created replug the wireless dongle.

### Installation
#### Direct download

Download the latest binary from the releases and run it.

```console
foo@bar:~$ curl -LO https://github.com/kondinskis/hyperx-cloud-flight/releases/download/0.1.5/cloud-flight_amd64
foo@bar:~$ chmod +x cloud-flight_amd64
foo@bar:~$ ./cloud-flight_amd64
```

#### Building your own Debian package

```bash
version="$(awk 'NR==1 {print $2}' debian/changelog | tr -d '()')"
docker build .
id="$(docker create "$(docker image list | awk 'NR==2 {print $3}')")"
docker cp "${id}:/hyperx-cloud-flight_${version}_$(dpkg --print-architecture).deb" .
docker rm -v "$id"
```

If the build is successful, you will find your binary package in the current directory.

### Supported operating systems

- Linux 

## Help

Feel free to [report any issues](https://github.com/kondinskis/hyperx-cloud-flight/issues) you may have while using this application.

## License

This project is licensed under the MIT License - see the [LICENSE.md](https://github.com/kondinskis/hyperx-cloud-flight/blob/main/LICENSE.md) file for details

## Other Projects

* [hyperx-cloud-flight-wireless](https://github.com/srn/hyperx-cloud-flight-wireless) Module for interfacing with HyperX Cloud Flight Wireless
