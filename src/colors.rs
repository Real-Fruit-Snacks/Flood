// Catppuccin Mocha color palette for terminal output.
// Each constant is an RGB tuple used with crossterm's Color::Rgb.

// Base colors
pub const BASE: (u8, u8, u8) = (30, 30, 46); // #1e1e2e
pub const MANTLE: (u8, u8, u8) = (24, 24, 37); // #181825
pub const CRUST: (u8, u8, u8) = (17, 17, 27); // #11111b

// Surface colors
pub const SURFACE0: (u8, u8, u8) = (49, 50, 68); // #313244
pub const SURFACE1: (u8, u8, u8) = (69, 71, 90); // #45475a
pub const SURFACE2: (u8, u8, u8) = (88, 91, 112); // #585b70

// Overlay colors
pub const OVERLAY0: (u8, u8, u8) = (108, 112, 134); // #6c7086
pub const OVERLAY1: (u8, u8, u8) = (127, 132, 156); // #7f849c
pub const OVERLAY2: (u8, u8, u8) = (147, 153, 178); // #939ab7

// Text colors
pub const SUBTEXT0: (u8, u8, u8) = (166, 173, 200); // #a6adc8
pub const SUBTEXT1: (u8, u8, u8) = (186, 194, 222); // #bac2de
pub const TEXT: (u8, u8, u8) = (205, 214, 244); // #cdd6f4

// Accent colors
pub const ROSEWATER: (u8, u8, u8) = (245, 224, 220); // #f5e0dc
pub const FLAMINGO: (u8, u8, u8) = (242, 205, 205); // #f2cdcd
pub const PINK: (u8, u8, u8) = (245, 194, 231); // #f5c2e7
pub const MAUVE: (u8, u8, u8) = (203, 166, 247); // #cba6f7
pub const RED: (u8, u8, u8) = (243, 139, 168); // #f38ba8
pub const MAROON: (u8, u8, u8) = (235, 160, 172); // #eba0ac
pub const PEACH: (u8, u8, u8) = (250, 179, 135); // #fab387
pub const YELLOW: (u8, u8, u8) = (249, 226, 175); // #f9e2af
pub const GREEN: (u8, u8, u8) = (166, 227, 161); // #a6e3a1
pub const TEAL: (u8, u8, u8) = (148, 226, 213); // #94e2d5
pub const SKY: (u8, u8, u8) = (137, 220, 235); // #89dceb
pub const SAPPHIRE: (u8, u8, u8) = (116, 199, 236); // #74c7ec
pub const BLUE: (u8, u8, u8) = (137, 180, 250); // #89b4fa
pub const LAVENDER: (u8, u8, u8) = (180, 190, 254); // #b4befe

/// Convert an RGB tuple to a crossterm Color.
pub fn rgb(color: (u8, u8, u8)) -> crossterm::style::Color {
    crossterm::style::Color::Rgb {
        r: color.0,
        g: color.1,
        b: color.2,
    }
}

/// Return the Catppuccin color for an HTTP status code.
pub fn status_color(code: u16) -> (u8, u8, u8) {
    match code {
        200..=299 => GREEN,
        300..=399 => YELLOW,
        400..=499 => PEACH,
        500..=599 => RED,
        _ => TEXT,
    }
}
