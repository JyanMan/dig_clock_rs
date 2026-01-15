// GRAPHICS
use embedded_graphics:: pixelcolor::Rgb565;

pub const LCD_WIDTH: u32 = 320;
pub const LCD_HEIGHT: u32 = 240;
pub const BACKGROUND_COLOR: Rgb565 = Rgb565::new(190, 205, 220);

pub const DRAW_BUF_WIDTH: usize = 5;
pub const DRAW_BUF_HEIGHT: usize = 320;
pub const MAX_DRAW_BUF: usize = DRAW_BUF_WIDTH * DRAW_BUF_HEIGHT;
