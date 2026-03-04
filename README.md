# AHT10 sensor with Rust

### Next

- [ ] Show temperature on 0.91" SSD1306 OLED display
- [ ] Post the temperature and humidity to an endpoint using Wi-Fi

This is a simple Rust example of how to read the AHT10 sensor using the ESP32C3 Super Mini. The
AHT10 is a very accurate temperature and humidity sensor with an I²C interface.

The ESP32 I²C interface is abstracted through `embedded-hal` traits, so you can use any
AHT10 crate that implements these traits. This example uses a fork of the
[`aht10-embedded`](https://github.com/bzhoek/rs-aht10/tree/embedded-hal-1-0) crate that was
updated to the 1.0 version of `embedded-hal`.

### ESP32C3 Super Mini pinout

```
VIN ----- 5V           GPIO5
GND ---- GND           GPIO6
        3.3V           GPIO7
SDL -- GPIO4           GPIO8
SDA -- GPIO3           GPIO9
       GPIO2           GPIO10
       GPIO1           GPIO20
       GPIO0           GPIO21
```