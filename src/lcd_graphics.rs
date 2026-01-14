use embassy_time::{Duration, Timer};
use esp_hal::{
    timer::timg::TimerGroup,
    time::Rate,
    spi::{
        Mode,
        master::{Config, ConfigError, Spi}
    },
    gpio::{Level, Output, OutputConfig},
    delay::Delay,
    peripherals::*,
    Blocking,
};
use embedded_graphics::{
    mono_font::{ascii::FONT_9X18, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    text::{Alignment, LineHeight, Text, TextStyleBuilder}
};
use log::{warn, info, error};
use display_interface_spi::SPIInterface;
use embedded_hal_bus::spi::{ExclusiveDevice, NoDelay};
use ili9341::{DisplaySize240x320, Ili9341, Orientation};

// channel
use embassy_sync::{
    channel::Channel,
    blocking_mutex::raw::NoopRawMutex
};
use static_cell::StaticCell;

use crate::graphics_utils::*;
use crate::config::*;

extern crate alloc;
use alloc::format;

static DRAW_BUFF: StaticCell<DrawBuffer> = StaticCell::new();

pub async fn display_init<'a>(
    lcd_host: SPI2<'a>,
    sck: GPIO18<'a>,
    mosi: GPIO19<'a>,
    miso: GPIO21<'a>,
    rst_pin: GPIO22<'a>,
    cs_pin: GPIO4<'a>,
    dc_pin: GPIO5<'a>
) -> Result<Display<'a>, ConfigError> {
    // let lcd_host = peripherals.SPI2;
    // let sck = peripherals.GPIO18;
    // let mosi: GPIO19 = peripherals.GPIO19;
    // let miso = peripherals.GPIO21;
    let rst = Output::new(rst_pin, Level::Low, OutputConfig::default());
    let cs = Output::new(cs_pin, Level::Low, OutputConfig::default());
    let dc = Output::new(dc_pin, Level::Low, OutputConfig::default());
    
    let spi = Spi::new(lcd_host, Config::default()
        .with_frequency(Rate::from_mhz(10))
    )?
        .with_sck(sck)
        .with_miso(miso)
        .with_mosi(mosi);
        // .with_cs(cs);


    let spi_dev = ExclusiveDevice::new_no_delay(spi, cs).unwrap();
    let interface = SPIInterface::new(spi_dev, dc);


    let display: Display = Ili9341::new(
        interface,
        rst,
        &mut Delay::new(),
        Orientation::Landscape,
        DisplaySize240x320
    ).unwrap();


    Ok(display)
}

#[embassy_executor::task]
pub async fn update_task(mut display: Display<'static>) {
    use crate::config::BACKGROUND_COLOR;

    let mut counter: u32 = 0;

    let mut stopwatch_ui = ClockStopwatchUi::new(
        "hello world",
        Point::new(50, 50)
    );

    // let mut draw_buff = DRAW_BUFF.init( DrawBuffer { framebuffer: [0u16; (LCD_WIDTH * LCD_HEIGHT) as usize] });
    // stopwatch_ui.draw(&mut draw_buff);
  
    let _ = display.clear(BACKGROUND_COLOR);

    loop {
        Timer::after_secs(1).await;
        counter += 1;
        info!("[lcd] time seconds: {}", counter);
        stopwatch_ui.set_text(&mut display, format!("hello world: {}", counter));
        stopwatch_ui.draw(&mut display);
    }
}
