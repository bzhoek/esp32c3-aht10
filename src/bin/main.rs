#![no_std]
#![no_main]
#![deny(
  clippy::mem_forget,
  reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use aht10_embedded::AHT10;
use defmt::{error, info};
use embedded_hal::delay::DelayNs;
use esp32c3_aht10::{create_i2c, take_peripherals};
use esp_hal::delay::Delay;
use esp_hal::main;
use {esp_backtrace as _, esp_println as _};

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
  clippy::large_stack_frames,
  reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
  let peripherals = take_peripherals();
  let i2c = create_i2c(peripherals.I2C0, peripherals.GPIO3, peripherals.GPIO4);

  let mut aht = AHT10::new(i2c);
  let mut delay = Delay::new();

  loop {
    match aht.read_data(&mut delay) {
      Ok(data) => {
        let temp = ((data.temperature_celsius() * 10.0) as i16) as f32 / 10.0;
        info!("Temperature: {}°C, Humidity: {}%", temp, data.humidity() as u8);
      }
      Err(_) => error!("Failed to read data"),
    }
    delay.delay_ms(2000);
  }

}
