// GRAPHICS
use embedded_graphics:: pixelcolor::Rgb565;

pub const LCD_WIDTH: u32 = 320;
pub const LCD_HEIGHT: u32 = 240;
pub const BACKGROUND_COLOR: Rgb565 = Rgb565::new(6, 15, 8);

pub const DRAW_BUF_WIDTH: usize = 5; // thickness of a strip
pub const DRAW_BUF_HEIGHT: usize = LCD_HEIGHT as usize;
pub const MAX_DRAW_BUF: usize = DRAW_BUF_WIDTH * DRAW_BUF_HEIGHT;
