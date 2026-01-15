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

use crate::graphics_utils::*;

extern crate alloc;
use alloc::format;

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
    use crate::config::{BACKGROUND_COLOR, MAX_DRAW_BUF, DRAW_BUF_WIDTH, DRAW_BUF_HEIGHT, LCD_WIDTH, LCD_HEIGHT};
    use embedded_graphics_framebuf::FrameBuf;

    use embedded_graphics::
        primitives::{Rectangle, RoundedRectangle, PrimitiveStyle, PrimitiveStyleBuilder};

    let mut counter: u32 = 0;

    let mut stopwatch_ui = ClockStopwatchUi::new(
        "hello ",
        Point::new(100, 100),
        2
    );

    // let mut draw_buff = DRAW_BUFF.init( DrawBuffer { framebuffer: [0u16; (LCD_WIDTH * LCD_HEIGHT) as usize] });
    // stopwatch_ui.draw(&mut draw_buff);
  
    let _ = display.clear(BACKGROUND_COLOR);

    let mut data = [BACKGROUND_COLOR; MAX_DRAW_BUF];

    loop {
        Timer::after_secs(1).await;
        counter += 1;
        info!("[lcd] time seconds: {}", counter);
        stopwatch_ui.set_text(format!("hello : {}", counter));


        if stopwatch_ui.updated {
            // let draw_dest = stopwatch_ui.draw(&mut data);
            // if let Some(draw_dest) = stopwatch_ui.draw(&mut data) {
            //     let _ = display.fill_contiguous(&draw_dest, data);
            // }
            // else {
            //     error!("[lcd] out of bounds draw");
            // }
            stopwatch_ui.updated = false;
            // let (text, back_g, occ_bounds) = stopwatch_ui.get_to_draw(Point::new(0, 0));
            // let step = MAX_DRAW_BUFF / occ_bounds.size.height;
            for i in (0..LCD_WIDTH).step_by(DRAW_BUF_WIDTH) {
                let mut fbuf = FrameBuf::new(&mut data, DRAW_BUF_WIDTH, DRAW_BUF_HEIGHT);
                // let (text, back_g) =  stopwatch_ui.get_to_draw(&mut fbuf, Point::new(-(i as i32), 0));
                let (text, back_g, occ_bounds) = stopwatch_ui.get_to_draw(Point::new(-(i as i32), 0));
                stopwatch_ui.clear(&mut fbuf, occ_bounds);
                let _ = back_g.draw(&mut fbuf);
                let _ = text.draw(&mut fbuf);

                
                let draw_dest = Rectangle::new(Point::new(i as i32, 0), fbuf.size());
                let _ = display.fill_contiguous(&draw_dest, data);
            }
        }

        // let bounds = Rectangle::new(Point::new(100, 100), fbuf.size());

        // info!("bounds: w: {} h: {}", bounds.size.width, bounds.size.height);
    }
}
