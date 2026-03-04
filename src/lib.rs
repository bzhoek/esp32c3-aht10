#![no_std]

use esp_hal::Blocking;
use esp_hal::clock::CpuClock;
use esp_hal::gpio::interconnect::PeripheralOutput;
use esp_hal::i2c::master::{Config, I2c, Instance};
use esp_hal::peripherals::Peripherals;

pub fn take_peripherals() -> Peripherals {
  let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
  esp_hal::init(config)
}

pub fn create_i2c<'a>(i2c: impl Instance + 'a, sda: impl PeripheralOutput<'a>, sdl: impl PeripheralOutput<'a>) -> I2c<'a, Blocking> {
  I2c::new(i2c, Config::default()).unwrap()
    .with_sda(sda)
    .with_scl(sdl)
}
