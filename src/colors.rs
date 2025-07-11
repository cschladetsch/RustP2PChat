//! Terminal color support for the P2P chat application.
//!
//! This module provides ANSI color codes for enhanced terminal output,
//! making the chat interface more user-friendly and visually appealing.
//! Colors are used to distinguish between different types of messages
//! and system notifications.
//!
//! # Features
//!
//! - ANSI color code constants for cross-platform terminal support
//! - Predefined color scheme for consistent UI appearance
//! - Easy-to-use color constants for different message types
//! - Automatic color reset functionality
//!
//! # Usage
//!
//! ```rust
//! use rust_p2p_chat::colors::Colors;
//!
//! // Print colored text
//! println!("{}Success message{}", Colors::GREEN, Colors::RESET);
//! println!("{}Error message{}", Colors::RED, Colors::RESET);
//! println!("{}Info message{}", Colors::CYAN, Colors::RESET);
//! ```
//!
//! # Color Scheme
//!
//! The application uses a consistent color scheme:
//! - **Green**: User messages, success notifications
//! - **Cyan**: Peer messages, information
//! - **Yellow**: System messages, warnings
//! - **Red**: Error messages, failures
//! - **Bright variants**: Important highlights

/// ANSI color code provider for terminal output.
///
/// `Colors` provides a collection of ANSI escape codes for coloring
/// terminal output. These constants can be used to make the chat
/// interface more visually appealing and easier to read.
///
/// # Cross-Platform Support
///
/// ANSI color codes are supported on:
/// - All Unix-like systems (Linux, macOS, BSD)
/// - Windows 10+ with modern terminal applications
/// - Windows Terminal, PowerShell Core, WSL
///
/// # Examples
///
/// ```rust
/// use rust_p2p_chat::colors::Colors;
///
/// // Basic colored output
/// println!("{}This is green text{}", Colors::GREEN, Colors::RESET);
///
/// // Multiple colors in one line
/// println!("{}User:{} {}Hello world!{}",
///          Colors::BOLD, Colors::BRIGHT_GREEN,
///          Colors::RESET, Colors::RESET);
///
/// // Error message styling
/// eprintln!("{}Error:{} {}Connection failed{}",
///           Colors::BOLD, Colors::RED,
///           Colors::RESET, Colors::RESET);
/// ```
pub struct Colors;

impl Colors {
    /// Reset all text formatting to default.
    pub const RESET: &'static str = "\x1b[0m";

    /// Standard red color - used for errors and failures.
    pub const RED: &'static str = "\x1b[31m";

    /// Standard green color - used for success messages and user text.
    pub const GREEN: &'static str = "\x1b[32m";

    /// Standard yellow color - used for warnings and system messages.
    pub const YELLOW: &'static str = "\x1b[33m";

    /// Standard blue color - used for informational messages.
    pub const BLUE: &'static str = "\x1b[34m";

    /// Standard magenta color - used for special highlights.
    pub const MAGENTA: &'static str = "\x1b[35m";

    /// Standard cyan color - used for peer messages and information.
    pub const CYAN: &'static str = "\x1b[36m";

    /// Standard white color - used for neutral text.
    pub const WHITE: &'static str = "\x1b[37m";

    /// Gray color - used for dimmed or secondary text.
    pub const GRAY: &'static str = "\x1b[90m";

    /// Bright red color - used for critical errors.
    pub const BRIGHT_RED: &'static str = "\x1b[91m";

    /// Bright green color - used for important success messages.
    pub const BRIGHT_GREEN: &'static str = "\x1b[92m";

    /// Bright yellow color - used for important warnings.
    pub const BRIGHT_YELLOW: &'static str = "\x1b[93m";

    /// Bright blue color - used for highlighted information.
    pub const BRIGHT_BLUE: &'static str = "\x1b[94m";

    /// Bright magenta color - used for special emphasis.
    pub const BRIGHT_MAGENTA: &'static str = "\x1b[95m";

    /// Bright cyan color - used for highlighted peer information.
    pub const BRIGHT_CYAN: &'static str = "\x1b[96m";

    /// Bold text formatting - used for emphasis and headers.
    pub const BOLD: &'static str = "\x1b[1m";

    /// Dim text formatting - used for secondary information.
    pub const DIM: &'static str = "\x1b[2m";

    /// Italic text formatting - used for special notes.
    pub const ITALIC: &'static str = "\x1b[3m";

    /// Underlined text formatting - used for important highlights.
    pub const UNDERLINE: &'static str = "\x1b[4m";
}
