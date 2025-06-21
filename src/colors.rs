//! ANSI color codes and terminal styling for the P2P chat application.
//!
//! This module provides constants for terminal color codes and text styling
//! to enhance the user interface of the chat application.

/// ANSI color codes and text styling constants for terminal output.
///
/// This struct provides a centralized location for all terminal styling
/// constants used throughout the application to ensure consistent formatting.
///
/// # Examples
///
/// ```rust
/// use rust_p2p_chat::colors::Colors;
///
/// println!("{}Error: Connection failed{}", Colors::RED, Colors::RESET);
/// println!("{}✓ Success{}", Colors::GREEN, Colors::RESET);
/// ```
pub struct Colors;

impl Colors {
    // Text colors
    pub const RESET: &'static str = "\x1b[0m";
    pub const RED: &'static str = "\x1b[31m";
    pub const GREEN: &'static str = "\x1b[32m";
    pub const YELLOW: &'static str = "\x1b[33m";
    pub const BLUE: &'static str = "\x1b[34m";
    pub const MAGENTA: &'static str = "\x1b[35m";
    pub const CYAN: &'static str = "\x1b[36m";
    pub const WHITE: &'static str = "\x1b[37m";
    pub const GRAY: &'static str = "\x1b[90m";

    // Bright colors
    pub const BRIGHT_RED: &'static str = "\x1b[91m";
    pub const BRIGHT_GREEN: &'static str = "\x1b[92m";
    pub const BRIGHT_YELLOW: &'static str = "\x1b[93m";
    pub const BRIGHT_BLUE: &'static str = "\x1b[94m";
    pub const BRIGHT_MAGENTA: &'static str = "\x1b[95m";
    pub const BRIGHT_CYAN: &'static str = "\x1b[96m";

    // Text styles
    pub const BOLD: &'static str = "\x1b[1m";
    pub const DIM: &'static str = "\x1b[2m";
    pub const ITALIC: &'static str = "\x1b[3m";
    pub const UNDERLINE: &'static str = "\x1b[4m";
}
