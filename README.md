<h1 align="center">Serial Port Pass-Through</h1>

[![Top Language](https://img.shields.io/github/languages/top/arctic-hen7/perseus?style=for-the-badge)]()
[![Lisence](https://img.shields.io/github/license/mcqueen256/serial-port-passthrough?style=for-the-badge)]()
[![Status](https://img.shields.io/badge/Status-Awesome%20%F0%9F%98%8E-blue?style=for-the-badge)]()

Expose a devices serial port to the machines local network through TCP.

## Get Started

Make sure you have a device pluged in and that you know which serial
device file it is being represented by. Start the serial pass-through:

```shell
git clone https://github.com/mcqueen256/serial-port-passthrough.git
cd serial-port-passthrough
#            serial port            buadrate  TCP port
cargo run -- /dev/tty.usbmodemACME0 115200    8080
```

Then in another terminal, try it out with:
```shell
nc localhost 8080
```

Voila! Now you can connect your network tools to the serial device!
