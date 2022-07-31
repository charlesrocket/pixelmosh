use std::env;

/// Logo
pub const BANNER: &str = "┌─────────────────────────────────────┐\n\
                          │ █▀▄ █ ▀▄▀ ██▀ █   █▄ ▄█ ▄▀▄ ▄▀▀ █▄█ │\n\
                          │ █▀  █ █ █ █▄▄ █▄▄ █ ▀ █ ▀▄▀ ▄██ █ █ │\n\
                          └─────────────────────────────────────┘";

/// TTY
pub const SPINNER_1: [&str; 7] = [
    "∙∙∙∙∙",
    "●∙∙∙∙",
    "∙●∙∙∙",
    "∙∙●∙∙",
    "∙∙∙●∙",
    "∙∙∙∙●",
    "∙∙∙∙∙",
];

/// Terminal
pub const SPINNER_2: [&str; 7] = [
    "▱▱▱▱▱",
    "▰▱▱▱▱",
    "▱▰▱▱▱",
    "▱▱▰▱▱",
    "▱▱▱▰▱",
    "▱▱▱▱▰",
    "▰▰▰▰▰",
];

/// Checks for TTY.
///
/// Returns `true` if a terminal emulator is detected.
#[must_use]
pub fn display_var() -> bool {
    // TODO
    // $TERM?
    matches!(env::var("DISPLAY"), Ok(_))
}
