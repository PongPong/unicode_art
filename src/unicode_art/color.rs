use image::Rgba;

/// ANSI background colour escapes.
pub const ANSI_BG_COLOUR_ESCAPES: [&str; 8] = [
    "\x1B[40m", "\x1B[41m", "\x1B[42m", "\x1B[43m", "\x1B[44m", "\x1B[45m", "\x1B[46m", "\x1B[47m",
];
/// Reset ANSI attributes
pub const ANSI_RESET_ATTRIBUTES: &'static str = "\x1B[0m";

pub trait AnsiColor {
    fn foreground(&self) -> String;
    fn background(&self) -> String;
}

impl AnsiColor for Rgba<u8> {
    #[inline]
    fn foreground(&self) -> String {
        format!("\x1B[38;2;{};{};{}m", self[0], self[1], self[2])
    }

    #[inline]
    fn background(&self) -> String {
        format!("\x1B[48;2;{};{};{}m", self[0], self[1], self[2])
    }
}
