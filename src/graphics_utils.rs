use esp_hal::{
    timer::timg::TimerGroup,
    spi::{
        Mode,
        master::{Config, ConfigError, Spi}
    },
    gpio::{Level, Output, OutputConfig},
    delay::Delay,
    peripherals::SPI2,
    Blocking,
};
use embedded_graphics::{
    mono_font::{ascii::FONT_9X18, MonoTextStyle},
    pixelcolor::{Rgb565, Gray8},
    prelude::*,
    text::{Alignment, LineHeight, Text, TextStyleBuilder},
    primitives::{Rectangle, RoundedRectangle, PrimitiveStyle, PrimitiveStyleBuilder},
};
use log::{warn, info, error};
use display_interface_spi::SPIInterface;
use embedded_hal_bus::spi::{ExclusiveDevice, NoDelay};
use ili9341::{DisplaySize240x320, Ili9341, Orientation};
extern crate alloc;
use alloc::string::String;

use crate::config::{BACKGROUND_COLOR, LCD_WIDTH, LCD_HEIGHT};

pub type Display<'a> 
    = Ili9341<SPIInterface<ExclusiveDevice<
            Spi<'a, Blocking>, Output<'a>, NoDelay
        >, Output<'a>>, Output<'a>>;


pub struct DrawBuffer {
    pub framebuffer: [u16; (LCD_WIDTH * LCD_HEIGHT) as usize], 
    // pub iface: Spi<'a, Blocking> 
}

impl DrawBuffer {
    /// Updates the display from the framebuffer.
    pub fn flush<'a>(&self, display: &'a mut Display) -> Result<(), ()> {
        // self.iface.send_bytes(&self.framebuffer)
        let x0: u16 = 0;
        let y0: u16 = 0;
        let x1: u16 = LCD_WIDTH as u16;
        let y1: u16 = LCD_HEIGHT as u16;
        let _ = display.draw_raw_iter(x0, y0, x1, y1, self.framebuffer.iter().copied());
        // warn!("[flush] no defined flush yet");
        
        Ok(())
    }
}

impl DrawTarget for DrawBuffer {
    type Color = Rgb565;
    // `ExampleDisplay` uses a framebuffer and doesn't need to communicate with the display
    // controller to draw pixel, which means that drawing operations can never fail. To reflect
    // this the type `Infallible` was chosen as the `Error` type.
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            // Check if the pixel coordinates are out of bounds (negative or greater than
            // (63,63)). `DrawTarget` implementation are required to discard any out of bounds
            // pixels without returning an error or causing a panic.
            // Calculate the index in the framebuffer.
            if let Ok((x @ 0..=319, y @ 0..=239)) = coord.try_into() {
                let index: u32 = x + y * LCD_WIDTH;
                self.framebuffer[index as usize] = color.into_storage();
            }
        }

        Ok(())
    }
}

impl OriginDimensions for DrawBuffer {
    fn size(&self) -> Size {
        Size::new(LCD_WIDTH, LCD_HEIGHT)
    }
}

pub struct ClockStopwatchUi {
    text: String,
    pos: Point,
    text_bounds: Option<Rectangle>,
    updated: bool,
}

impl ClockStopwatchUi {

    pub fn new(text: &str, pos: Point) -> Self {
        Self {
            text: String::from(text),
            pos,
            text_bounds: None,
            updated: true, // allow redraw on init
        }
    }

    pub fn pos(&self) -> Point {
        self.pos
    }

    pub fn set_text<'b> (&mut self, display: &'b mut Display, new_text: String) {
        self.text = new_text;
        self.updated = true;
        self.clear(display);

    }

    pub fn set_pos<'b>(&mut self, display: &'b mut Display, new_pos: Point) {
        self.pos = new_pos;
        self.updated = true;
        self.clear(display);
    }

    // clear previous draw
    pub fn clear<'b>(&self, display: &'b mut Display) {
        if let Some(text_bounds) = self.text_bounds {
            const PAD: u32 = 10;
            let width = text_bounds.size.width + PAD*2;
            let height = text_bounds.size.height + PAD*2;
            let pos = {
                let pos = text_bounds.top_left;
                Point::new(pos.x - PAD as i32, pos.y - PAD as i32)
            };
            let _ = display.fill_solid(
                &Rectangle::new(pos, Size::new(width, height)),
                BACKGROUND_COLOR,
            );
            info!("[graphics] cleared stopwatch ui");
        }
    }

    pub fn draw<'b>(&mut self, display: &'b mut Display) {
        // only redraw if updated
        if !self.updated {
            return;
        }
        self.updated = false;

        // draw actual stopwatch text
        let character_style = MonoTextStyle::new(&FONT_9X18, Rgb565::WHITE);
        let text_style = TextStyleBuilder::new()
            .alignment(Alignment::Center)
            .line_height(LineHeight::Percent(150))
            .build();
    
        let text = Text::with_text_style(
            self.text.as_str(),
            self.pos,
            character_style,
            text_style
        );

        let style = PrimitiveStyleBuilder::new()
            .stroke_width(5)
            .stroke_color(Rgb565::RED)
            .fill_color(Rgb565::GREEN)
            .build();

        let text_bounds = text.bounding_box();
        self.text_bounds = Some(text_bounds);

        let _ = RoundedRectangle::with_equal_corners(
            Rectangle::new(text_bounds.top_left, text_bounds.size),
            Size::new(10, 10)
        )
        .into_styled(style)
        .draw(display);

        let _ = text.draw(display);
    }
}

