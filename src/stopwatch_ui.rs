use embassy_time::{Duration, Timer, Ticker};
use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::Rgb565,
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

pub fn draw_text_with_backg<Display, F>(
    display: &mut Display,
    _font: F,
    draw_pos: Point,
    text: &String,
    text_color: Rgb565,
    back_g_color: Rgb565,
    padding: u32,
    round_corner: Size,
) -> Rectangle
where
    Display: DrawTarget<Color=Rgb565>,
    F: Font
{
    let font_render = FontRenderer::new::<F>();

    // fetch the bounds for the background
    let text_bounds = {
        if let Ok(text_bounds) = font_render.get_rendered_dimensions_aligned(
            text.as_str(),
            draw_pos,
            VerticalPosition::Baseline,
            HorizontalAlignment::Center,
        ) {
            text_bounds.unwrap()
        }
        else {
            return Rectangle::default()
        }
    };


    // create background
    let style = PrimitiveStyleBuilder::new()
        // .stroke_width(2)
        // .stroke_color(Rgb565::new(20, 20, 30))
        .fill_color(back_g_color)
        .build();
     
    let back_g_pos = {
        let pos = text_bounds.top_left;
        let pad = padding as i32;
        Point::new(pos.x - pad, pos.y - pad)
    };
    let back_g_size = {
        let size = text_bounds.size;
        let pad = padding as u32 * 2;
        Size::new(size.width + pad, size.height + pad)
    };


    let occ_bounds = Rectangle::new(back_g_pos, back_g_size);
    
    let back_g  = RoundedRectangle::with_equal_corners(
        occ_bounds,
        round_corner
    )
    .into_styled(style);

    // render background
    let _ = back_g.draw(display);
    // render text
    let _ = font_render.render_aligned(
        text.as_str(),
        draw_pos,
        VerticalPosition::Baseline,
        HorizontalAlignment::Center,
        FontColor::Transparent(text_color),
        display
    );

    occ_bounds
}


#[derive(Debug)]
pub struct ClockStopwatchUi {
    text_hour_min: String,
    text_sec: String,
    text_color: Rgb565,
    back_g_color: Rgb565,
    round_corner: Size,
    pos: Point,
    padding: u32,
}

impl ClockStopwatchUi {

    pub fn new(pos: Point, padding: u32) -> Self {
        Self {
            text_hour_min: String::from("00:00"),
            text_sec: String::from("00"),
            text_color: Rgb565::WHITE,
            back_g_color: Rgb565::new(14, 27, 15),
            round_corner: Size::new(10, 10),
            pos,
            padding
        }
    }

    pub fn pos(&self) -> Point {
        self.pos
    }

    pub fn set_text<'b> (&mut self, text_hour_min: String, text_sec: String) {
        self.text_hour_min = text_hour_min;
        self.text_sec = text_sec;

    }

    pub fn set_pos<'b>(&mut self, new_pos: Point) {
        self.pos = new_pos;
    }

    pub fn draw<'b, Display>(&mut self, display: &'b mut Display, offset: Point)
    where
        Display: DrawTarget<Color=Rgb565> 
    {
        let main_time_pos = Point::new(
            self.pos.x + offset.x,
            self.pos.y + offset.y
        );

        let occ_bounds = draw_text_with_backg(
            display,
            fonts::u8g2_font_logisoso50_tf,
            main_time_pos,
            &self.text_hour_min,
            self.text_color,
            self.back_g_color,
            self.padding,
            self.round_corner
        );

        let sec_pos = Point::new(
            main_time_pos.x + occ_bounds.size.width as i32 - (self.padding as i32)*2,
            main_time_pos.y + (occ_bounds.size.height / 2) as i32 
        );
        draw_text_with_backg(
            display,
            fonts::u8g2_font_logisoso30_tf,
            sec_pos,
            &self.text_sec,
            self.text_color,
            self.back_g_color,
            self.padding,
            self.round_corner
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
        let hour_min_str = format!("{:02}:{:02}", hour, min);
        let sec_str = format!("{:02}", sec);
        
        let res = stopwatch_ui_send.try_send(
            crate::lcd_graphics::ClockUiUpdate::Stopwatch{
                hour_min: hour_min_str,
                sec: sec_str
            });

        info!("[stopwatch_ui] send new counter {}, send data: {:?}", counter, res);
    }
}

