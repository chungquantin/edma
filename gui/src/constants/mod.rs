mod focus;

pub use focus::*;
use tui::{style::Color, widgets::BorderType};

pub const BORDER_TYPE: BorderType = BorderType::Rounded;
pub const PRIMARY_COLOR: Color = Color::DarkGray;
pub const DEFAULT_STATUS_TEXT: &str = "No status displayed...";
