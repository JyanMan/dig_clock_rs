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
    mono_font::{ascii::FONT_8X13, MonoTextStyle},
    pixelcolor::{Rgb565, Gray8},
    prelude::*,
    text::{Alignment, LineHeight, Text, TextStyleBuilder},
    primitives::{Rectangle, RoundedRectangle, PrimitiveStyle, PrimitiveStyleBuilder, Styled},
};
use embedded_graphics_framebuf::{FrameBuf, backends::FrameBufferBackend};
use log::{warn, info, error};
use display_interface_spi::SPIInterface;
use embedded_hal_bus::spi::{ExclusiveDevice, NoDelay};
use ili9341::{DisplaySize240x320, Ili9341, Orientation};
extern crate alloc;
use alloc::string::String;

use crate::config::{BACKGROUND_COLOR,
    MAX_DRAW_BUF, DRAW_BUF_WIDTH, DRAW_BUF_HEIGHT,
    LCD_WIDTH, LCD_HEIGHT};

pub type Display<'a> 
    = Ili9341<SPIInterface<ExclusiveDevice<
            Spi<'a, Blocking>, Output<'a>, NoDelay
        >, Output<'a>>, Output<'a>>;

pub struct Renderer<'a> {
    display: Display<'a>,
    framebuf: FrameBuf<Rgb565, [Rgb565; MAX_DRAW_BUF]>
}

pub struct ClockStopwatchUi {
    text: String,
    pos: Point,
    text_bounds: Option<Rectangle>,
    pub updated: bool,
    padding: u32,
}

impl ClockStopwatchUi {

    pub fn new(text: &str, pos: Point, padding: u32) -> Self {
        Self {
            text: String::from(text),
            pos,
            text_bounds: None,
            updated: true, // allow redraw on init
            padding
        }
    }

    pub fn pos(&self) -> Point {
        self.pos
    }

    pub fn set_text<'b> (&mut self, new_text: String) {
        self.text = new_text;
        self.updated = true;

    }

    pub fn set_pos<'b>(&mut self, new_pos: Point) {
        self.pos = new_pos;
        self.updated = true;
    }

    // clear previous draw
    pub fn clear<'b>(&self, display: &'b mut impl DrawTarget<Color=Rgb565>, area: Rectangle) {
        // if let Some(text_bounds) = self.text_bounds {
        //     let pad: u32 = 10 + self.padding;
        //     let width = text_bounds.size.width + pad*2;
        //     let height = text_bounds.size.height + pad*2;
        //     let pos = {
        //         let pos = text_bounds.top_left;
        //         Point::new(pos.x - pad as i32, pos.y - pad as i32)
        //     };
        //     let _ = display.fill_solid(
        //         &Rectangle::new(pos, Size::new(width, height)),
        //         BACKGROUND_COLOR,
        //     );
        //     info!("[graphics] cleared stopwatch ui");
        // }
        let pad: u32 = 10 + self.padding;
        let width = area.size.width + pad*2;
        let height = area.size.height + pad*2;
        let pos = {
            let pos = area.top_left;
            Point::new(pos.x - pad as i32, pos.y - pad as i32)
        };
        let _ = display.fill_solid(
            &Rectangle::new(pos, Size::new(width, height)),
            BACKGROUND_COLOR,
        );
        info!("[graphics] cleared stopwatch ui");
    }

    pub fn draw<'b>(
        &mut self,
        // data: &mut [Rgb565; MAX_DRAW_BUF]
        display: &'b mut impl DrawTarget<Color=Rgb565>,
        offset: Point
    ) {

        // clear its previous position
        // self.clear(display);
        self.updated = false;

        let draw_pos = Point::new(
            self.pos.x + offset.x,
            self.pos.y + offset.y
        );


        // draw actual stopwatch text
        let character_style = MonoTextStyle::new(&FONT_8X13, Rgb565::WHITE);
        let text_style = TextStyleBuilder::new()
            .alignment(Alignment::Center)
            .line_height(LineHeight::Percent(150))
            .build();
    
        let text = Text::with_text_style(
            self.text.as_str(),
            draw_pos,
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

        let back_g_pos = {
            let pos = text_bounds.top_left;
            let pad = self.padding as i32;
            Point::new(pos.x - pad, pos.y - pad)
        };
        let back_g_size = {
            let size = text_bounds.size;
            let pad = self.padding as u32 * 2;
            Size::new(size.width + pad, size.height + pad)
        };

        // let draw_dest = Rectangle::new(
        //     draw_pos,
        //     display.size()
        // );

        let occ_bounds = Rectangle::new(back_g_pos, back_g_size);
        
        let _ = RoundedRectangle::with_equal_corners(
            occ_bounds,
            Size::new(10, 10)
        )
        .into_styled(style)
        .draw(display);

        let _ = text.draw(display);


        // draw_dest
    }

    pub fn get_to_draw<'b>(
        &self,
        // data: &mut [Rgb565; MAX_DRAW_BUF]
        // display: &'b mut impl DrawTarget<Color=Rgb565>,
        offset: Point
    ) -> (
        Text<'_, MonoTextStyle<'_, Rgb565>>,
        Styled<RoundedRectangle, PrimitiveStyle<Rgb565>>,
        Rectangle
    ) {

        // clear its previous position
        // self.clear(display);
        // self.updated = false;

        let draw_pos = Point::new(
            self.pos.x + offset.x,
            self.pos.y + offset.y
        );


        // draw actual stopwatch text
        let character_style = MonoTextStyle::new(&FONT_8X13, Rgb565::WHITE);
        let text_style = TextStyleBuilder::new()
            .alignment(Alignment::Center)
            .line_height(LineHeight::Percent(150))
            .build();
    
        let text = Text::with_text_style(
            self.text.as_str(),
            draw_pos,
            character_style,
            text_style
        );

        let style = PrimitiveStyleBuilder::new()
            .stroke_width(5)
            .stroke_color(Rgb565::RED)
            .fill_color(Rgb565::new(40, 50, 70))
            .build();

        let text_bounds = text.bounding_box();
        // self.text_bounds = Some(text_bounds);

        let back_g_pos = {
            let pos = text_bounds.top_left;
            let pad = self.padding as i32;
            Point::new(pos.x - pad, pos.y - pad)
        };
        let back_g_size = {
            let size = text_bounds.size;
            let pad = self.padding as u32 * 2;
            Size::new(size.width + pad, size.height + pad)
        };

        // let draw_dest = Rectangle::new(
        //     draw_pos,
        //     display.size()
        // );

        let occ_bounds = Rectangle::new(back_g_pos, back_g_size);
        
        let back_g  = RoundedRectangle::with_equal_corners(
            occ_bounds,
            Size::new(10, 10)
        )
        .into_styled(style);
        // .draw(display);

        // self.clear(display);
        // let _ = back_g.draw(display);
        // let _ = text.draw(display);

        (text, back_g, occ_bounds)


        // draw_dest
    }
}

