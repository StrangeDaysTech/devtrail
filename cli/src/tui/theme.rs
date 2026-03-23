use ratatui::style::Color;
use ratatui::widgets::BorderType;

// Background: dark blue-gray (Catppuccin Mocha inspired)
pub const BG: Color = Color::Rgb(30, 30, 46);
// Surface: slightly lighter for panels
pub const SURFACE: Color = Color::Rgb(36, 36, 54);
// Text: primary readable text
pub const TEXT: Color = Color::Rgb(164, 171, 199);
// Dimmed text: secondary info (counts, dates, badges, hints)
pub const TEXT_DIM: Color = Color::Rgb(148, 155, 180);
// Subtle: borders, separators when inactive
pub const SUBTLE: Color = Color::Rgb(88, 91, 112);
// Subgroup labels (yellow family, accessible on dark bg)
pub const SUBGROUP: Color = Color::Rgb(249, 226, 145);
// User dir labels (magenta/lavender family)
pub const USER_DIR: Color = Color::Rgb(203, 166, 247);

// Accent: panel titles, expand/collapse icons, shortcut keys
pub const ACCENT: Color = Color::Rgb(116, 199, 236);
// Active panel border
pub const BORDER_ACTIVE: Color = Color::Rgb(179, 190, 254);

// Semantic colors (muted variants)
pub const GREEN: Color = Color::Rgb(166, 218, 149);
pub const YELLOW: Color = Color::Rgb(229, 200, 144);
pub const RED: Color = Color::Rgb(237, 135, 150);

// Border type: rounded corners
pub const BORDER_TYPE: BorderType = BorderType::Rounded;
