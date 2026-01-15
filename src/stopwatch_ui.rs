use embedded_graphics::{
    mono_font::{ascii::FONT_8X13, MonoTextStyle},
    pixelcolor::{Rgb565, Gray8},
    prelude::*,
    text::{Alignment, LineHeight, Text, TextStyleBuilder},
    primitives::{Rectangle, RoundedRectangle, PrimitiveStyleBuilder},
};
use alloc::string::String;
use log::{warn, info, error};


extern crate alloc;

pub struct ClockStopwatchUi {
    text: String,
    pos: Point,
    text_bounds: Option<Rectangle>,
    padding: u32,
}

impl ClockStopwatchUi {

    pub fn new(text: &str, pos: Point, padding: u32) -> Self {
        Self {
            text: String::from(text),
            pos,
            text_bounds: None,
            padding
        }
    }

    pub fn pos(&self) -> Point {
        self.pos
    }

    pub fn set_text<'b> (&mut self, new_text: String) {
        self.text = new_text;

    }

    pub fn set_pos<'b>(&mut self, new_pos: Point) {
        self.pos = new_pos;
    }

    pub fn draw<'b>(
        &mut self,
        display: &'b mut impl DrawTarget<Color=Rgb565>,
        offset: Point
    ) {

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

        let occ_bounds = Rectangle::new(back_g_pos, back_g_size);
        
        let _ = RoundedRectangle::with_equal_corners(
            occ_bounds,
            Size::new(10, 10)
        )
        .into_styled(style)
        .draw(display);

        let _ = text.draw(display);
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


        let occ_bounds = Rectangle::new(back_g_pos, back_g_size);
        
        let back_g  = RoundedRectangle::with_equal_corners(
            occ_bounds,
            Size::new(10, 10)
        )
        .into_styled(style);

        let _ = back_g.draw(display);
        let _ = text.draw(display);
    }
}

#[embassy_executor::task]
pub async fn update_stopwatch() {
    
}

