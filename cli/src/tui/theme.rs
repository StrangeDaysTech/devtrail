use ratatui::style::Color;
use ratatui::widgets::BorderType;

// Background: dark blue-gray (Catppuccin Mocha inspired)
pub const BG: Color = Color::Rgb(30, 30, 46);
// Surface: slightly lighter for panels
pub const SURFACE: Color = Color::Rgb(36, 36, 54);
// Text
#[allow(dead_code)]
pub const TEXT: Color = Color::Rgb(205, 214, 244);
// Dimmed text
pub const TEXT_DIM: Color = Color::Rgb(108, 112, 134);

// Border type: rounded corners
pub const BORDER_TYPE: BorderType = BorderType::Rounded;
