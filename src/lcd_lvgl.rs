use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::clock::CpuClock;
use esp_hal::timer::timg::TimerGroup;
use esp_hal::spi::{
    Mode,
    master::{Config, ConfigError, Spi}
};
// Embedded Grpahics related
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;

use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_radio::ble::controller::BleConnector;
use esp_hal::delay::Delay;
use log::{warn, info, error};
use display_interface_spi::SPIInterface;
use embedded_hal_bus::spi::ExclusiveDevice;
use ili9341::{DisplaySize240x320, Ili9341, Orientation};

use esp_hal::peripherals::*;

pub fn lcd_lvgl_init(
    lcd_host: SPI2,
    sck: GPIO18,
    mosi: GPIO19,
    miso: GPIO21,
    rst_pin: GPIO22,
    cs_pin: GPIO4,
    dc_pin: GPIO5
) -> Result<(), ConfigError> {
    // let lcd_host = peripherals.SPI2;
    // let sck = peripherals.GPIO18;
    // let mosi: GPIO19 = peripherals.GPIO19;
    // let miso = peripherals.GPIO21;
    let rst = Output::new(rst_pin, Level::Low, OutputConfig::default());
    let cs = Output::new(cs_pin, Level::Low, OutputConfig::default());
    let dc = Output::new(dc_pin, Level::Low, OutputConfig::default());
    
    let spi = Spi::new(lcd_host, Config::default())?
        .with_sck(sck)
        .with_miso(miso)
        .with_mosi(mosi);
        // .with_cs(cs);

    let spi_dev = ExclusiveDevice::new_no_delay(spi, cs).unwrap();
    let interface = SPIInterface::new(spi_dev, dc);
    
    let mut display = Ili9341::new(
        interface,
        rst,
        &mut Delay::new(),
        Orientation::Landscape,
        DisplaySize240x320
        
    ).unwrap();

    let _ = display.clear(Rgb565::MAGENTA);

    Ok(())
}

#[embassy_executor::task]
pub async fn lcd_lvgl_task() {
  
    loop {
        Timer::after_secs(1).await;
        info!("WORKING TASK");
    }
}
