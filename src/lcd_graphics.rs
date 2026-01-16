extern crate alloc;
use alloc::{format, string::String};

// Third-party crates (alphabetically, grouped by crate)
use display_interface_spi::SPIInterface;

use embassy_futures::{join::join, select::{select, Either}};
use embassy_sync::{
    blocking_mutex::raw::{CriticalSectionRawMutex, NoopRawMutex},
    channel::{Channel, Receiver},
};
use embassy_time::{Duration, Timer};

use embedded_graphics::{
    mono_font::{ascii::FONT_9X18, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    text::{Alignment, LineHeight, Text, TextStyleBuilder},
};

use embedded_hal_bus::spi::{ExclusiveDevice, NoDelay};

use esp_hal::{
    delay::Delay,
    gpio::{Level, Output, OutputConfig},
    peripherals::*,
    spi::master::{Config, ConfigError, Spi},
    spi::Mode,
    time::Rate,
    timer::timg::TimerGroup,
    Blocking,
};

use ili9341::{DisplaySize240x320, Ili9341, Orientation};

use log::{error, info, warn};

// Local/crate imports (last)
use crate::stopwatch_ui::*;
use crate::config::{
    BACKGROUND_COLOR, MAX_DRAW_BUF, DRAW_BUF_WIDTH, DRAW_BUF_HEIGHT, LCD_WIDTH, LCD_HEIGHT
};

pub type Display<'a> 
    = Ili9341<SPIInterface<ExclusiveDevice<
            Spi<'a, Blocking>, Output<'a>, NoDelay
        >, Output<'a>>, Output<'a>>;

pub async fn display_init<'a>(
    lcd_host: SPI2<'a>,
    sck: GPIO18<'a>,
    mosi: GPIO19<'a>,
    miso: GPIO21<'a>,
    rst_pin: GPIO22<'a>,
    cs_pin: GPIO4<'a>,
    dc_pin: GPIO5<'a>
) -> Result<Display<'a>, ConfigError> {
    let rst = Output::new(rst_pin, Level::Low, OutputConfig::default());
    let cs = Output::new(cs_pin, Level::Low, OutputConfig::default());
    let dc = Output::new(dc_pin, Level::Low, OutputConfig::default());
    
    let spi = Spi::new(lcd_host, Config::default()
        .with_frequency(Rate::from_mhz(10))
    )?
        .with_sck(sck)
        .with_miso(miso)
        .with_mosi(mosi);

    let spi_dev = ExclusiveDevice::new_no_delay(spi, cs).unwrap();
    let interface = SPIInterface::new(spi_dev, dc);

    let display: Display = Ili9341::new(
        interface,
        rst,
        &mut Delay::new(),
        Orientation::Landscape,
        DisplaySize240x320
    ).unwrap();


    let display_bounds = display.bounding_box();
    let display_width = display_bounds.size.width as usize;
    let display_height = display_bounds.size.height as usize;

    assert!(display_width == LCD_WIDTH as usize,
        "[lcd_graphics] lcd width is not equal to display width");
    assert!(display_height == LCD_HEIGHT as usize,
        "[lcd_graphics] lcd height is not equal to display height");


    Ok(display)
}

#[derive(Debug)]
pub enum ClockUiUpdate {
    Stopwatch(String)
}

#[embassy_executor::task]
pub async fn update_task(
    mut display: Display<'static>,
    ui_update_rec: Receiver<'static, CriticalSectionRawMutex, ClockUiUpdate, 10>
    
) {
    use embedded_graphics_framebuf::FrameBuf;
    use embedded_graphics:: primitives::Rectangle;
  
    let _ = display.clear(BACKGROUND_COLOR);
    
    let mut data = [BACKGROUND_COLOR; MAX_DRAW_BUF];

    // let mut counter = 0;
    let mut stopwatch_ui = ClockStopwatchUi::new(
        format!("hello: {}", 0).as_str(),
        Point::new(140, 100),
        5
    );

    loop {
        ui_update_rec.ready_to_receive().await;

        // poll until all updates are cleared
        while let Ok(ui_update) = ui_update_rec.try_receive() {
            match ui_update {
                ClockUiUpdate::Stopwatch(time_str) => stopwatch_ui.set_text(time_str),
            }
        }

        // draw in vertical strips
        for i in (0..LCD_WIDTH).step_by(DRAW_BUF_WIDTH) {

            // init draw vertical strip buffer
            let mut fbuf = FrameBuf::new(&mut data, DRAW_BUF_WIDTH, DRAW_BUF_HEIGHT);
            let _ = fbuf.clear(BACKGROUND_COLOR);

            // start draw to buff
            let _ = stopwatch_ui.draw(&mut fbuf, Point::new(-(i as i32), 0));
            // end draw

            // flush buffer into display
            let draw_strip = Rectangle::new(Point::new(i as i32, 0), fbuf.size());
            let _ = display.fill_contiguous(&draw_strip, data);
        }
    }
}
