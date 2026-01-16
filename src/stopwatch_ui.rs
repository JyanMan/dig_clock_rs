use embassy_time::{Duration, Timer, Ticker};
use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::{Rgb565, Gray8},
    prelude::*,
    text::{Alignment, LineHeight, Text, TextStyleBuilder},
    primitives::{Rectangle, RoundedRectangle, PrimitiveStyleBuilder},
};
use alloc::string::String;
use log::{warn, info, error};
use embassy_sync::channel::{Channel, Sender};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use u8g2_fonts::*;
use u8g2_fonts::types::*;



extern crate alloc;
use alloc::format;

#[derive(Debug)]
pub struct ClockStopwatchUi {
    pub text: String,
    pos: Point,
    padding: u32,
}

impl ClockStopwatchUi {

    pub fn new(text: &str, pos: Point, padding: u32) -> Self {
        Self {
            text: String::from(text),
            pos,
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

    pub fn draw<'b, Display>(&mut self, display: &'b mut Display, offset: Point)
    where
        Display: DrawTarget<Color=Rgb565> 
    {

        let draw_pos = Point::new(
            self.pos.x + offset.x,
            self.pos.y + offset.y
        );

        let font = FontRenderer::new::<fonts::u8g2_font_t0_40_tf>();

        // fetch the bounds for the background
        let text_bounds = {
            if let Ok(text_bounds) = font.get_rendered_dimensions_aligned(
                self.text.as_str(),
                draw_pos,
                VerticalPosition::Baseline,
                HorizontalAlignment::Center,
            ) {
                text_bounds.unwrap()
            }
            else {
                return
            }
        };


        let style = PrimitiveStyleBuilder::new()
            .stroke_width(2)
            .stroke_color(Rgb565::new(20, 20, 30))
            .fill_color(Rgb565::new(12, 10, 13))
            .build();
         
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
        let _ = font.render_aligned(
            self.text.as_str(),
            draw_pos,
            VerticalPosition::Baseline,
            HorizontalAlignment::Center,
            FontColor::Transparent(Rgb565::WHITE),
            display
        );
    }
}

#[embassy_executor::task]
pub async fn increment_stopwatch(
    stopwatch_ui_send:
        Sender<'static, CriticalSectionRawMutex, crate::lcd_graphics::ClockUiUpdate, 10>
) {
    let mut counter = 0;

    let mut ticker = Ticker::every(Duration::from_secs(1));

    loop {
        ticker.next().await;
        counter += 1;

        let sec = counter % 60;
        let min = (counter / 60) as i32;
        let hour = (counter / 3600) as i32;
        let time_str = format!("{:02}:{:02}:{:02}", hour, min, sec);
        
        let res = stopwatch_ui_send.try_send(
            crate::lcd_graphics::ClockUiUpdate::Stopwatch(time_str));

        info!("[stopwatch_ui] send new counter {}, send data: {:?}", counter, res);
    }
}

