use anyhow::Result;
use ratatui::style::{Color, Modifier, Style};
use ratatui::style::palette::tailwind;

/// Represents the visual theme and color scheme for the SSHS interface.
#[derive(Clone, Debug)]
pub struct Theme {
    pub name: &'static str,
    pub display_name: &'static str,

    // Core Theme Colors
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub muted: Color,

    // Borders
    pub border: Color,
    pub border_active: Color,

    // Header & ASCII Banner
    pub banner_start: Color,
    pub banner_end: Color,
    pub header_fg: Color,

    // Table Colors
    pub table_header: Color,
    pub host_name: Color,
    pub aliases: Color,
    pub user: Color,
    pub destination: Color,
    pub port: Color,
    pub proxy: Color,

    // Selection & Highlighting
    pub selected_fg: Color,
    pub selected_bg: Color,
    pub cursor_symbol_fg: Color,

    // Badges & Pills
    pub badge_bg: Color,
    pub badge_fg: Color,

    // Search bar
    pub search_icon_fg: Color,
    pub search_border: Color,
    pub search_text_fg: Color,

    // Footer
    pub footer_key_bg: Color,
    pub footer_key_fg: Color,
    pub footer_desc_fg: Color,
}

impl Theme {
    /// Style for table header
    #[must_use]
    pub fn table_header_style(&self) -> Style {
        Style::default()
            .fg(self.table_header)
            .add_modifier(Modifier::BOLD)
    }

    /// Style for selected row in table
    #[must_use]
    pub fn selected_row_style(&self) -> Style {
        Style::default()
            .fg(self.selected_fg)
            .bg(self.selected_bg)
            .add_modifier(Modifier::BOLD)
    }

    /// Style for normal borders
    #[must_use]
    pub fn border_style(&self) -> Style {
        Style::default().fg(self.border)
    }

    /// Style for focused/active borders
    #[must_use]
    pub fn active_border_style(&self) -> Style {
        Style::default().fg(self.border_active)
    }

    /// Style for keybinding badge pill in footer
    #[must_use]
    pub fn key_badge_style(&self) -> Style {
        Style::default()
            .fg(self.footer_key_fg)
            .bg(self.footer_key_bg)
            .add_modifier(Modifier::BOLD)
    }

    /// Style for keybinding description text in footer
    #[must_use]
    pub fn key_desc_style(&self) -> Style {
        Style::default().fg(self.footer_desc_fg)
    }

    /// Style for status badges (e.g. count, active config)
    #[must_use]
    pub fn badge_style(&self) -> Style {
        Style::default()
            .fg(self.badge_fg)
            .bg(self.badge_bg)
            .add_modifier(Modifier::BOLD)
    }
}

/// Helper to interpolate between two RGB colors.
#[must_use]
pub fn interpolate_rgb(c1: Color, c2: Color, factor: f32) -> Color {
    let factor = factor.clamp(0.0, 1.0);
    let (r1, g1, b1) = color_to_rgb(c1);
    let (r2, g2, b2) = color_to_rgb(c2);

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let r = (f32::from(r1) + (f32::from(r2) - f32::from(r1)) * factor).round() as u8;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let g = (f32::from(g1) + (f32::from(g2) - f32::from(g1)) * factor).round() as u8;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let b = (f32::from(b1) + (f32::from(b2) - f32::from(b1)) * factor).round() as u8;

    Color::Rgb(r, g, b)
}

fn color_to_rgb(color: Color) -> (u8, u8, u8) {
    match color {
        Color::Rgb(r, g, b) => (r, g, b),
        Color::Black => (0, 0, 0),
        Color::Red => (205, 0, 0),
        Color::Green => (0, 205, 0),
        Color::Yellow => (205, 205, 0),
        Color::Blue => (0, 0, 238),
        Color::Magenta => (205, 0, 205),
        Color::Cyan => (0, 205, 205),
        Color::Gray => (229, 229, 229),
        Color::DarkGray => (127, 127, 127),
        Color::LightRed => (255, 0, 0),
        Color::LightGreen => (0, 255, 0),
        Color::LightYellow => (255, 255, 0),
        Color::LightBlue => (92, 92, 255),
        Color::LightMagenta => (255, 0, 255),
        Color::LightCyan => (0, 255, 255),
        Color::White => (255, 255, 255),
        _ => (128, 128, 128),
    }
}

// -------------------------------------------------------------
// Curated Modern Themes
// -------------------------------------------------------------

pub const THEME_CATPPUCCIN: Theme = Theme {
    name: "catppuccin",
    display_name: "Catppuccin Mocha",
    primary: Color::Rgb(203, 166, 247),       // Mauve
    secondary: Color::Rgb(137, 180, 250),     // Sapphire / Blue
    accent: Color::Rgb(148, 226, 213),        // Teal
    muted: Color::Rgb(108, 112, 134),         // Overlay0
    border: Color::Rgb(137, 180, 250),        // Sapphire
    border_active: Color::Rgb(203, 166, 247), // Mauve
    banner_start: Color::Rgb(203, 166, 247),  // Mauve
    banner_end: Color::Rgb(148, 226, 213),    // Teal
    header_fg: Color::Rgb(205, 214, 244),     // Text
    table_header: Color::Rgb(148, 226, 213),  // Teal
    host_name: Color::Rgb(203, 166, 247),     // Mauve (Bold)
    aliases: Color::Rgb(180, 190, 254),       // Lavender
    user: Color::Rgb(166, 227, 161),          // Green
    destination: Color::Rgb(137, 220, 235),   // Sky
    port: Color::Rgb(250, 179, 135),          // Peach
    proxy: Color::Rgb(245, 194, 231),         // Pink
    selected_fg: Color::Rgb(17, 17, 27),      // Crust
    selected_bg: Color::Rgb(203, 166, 247),   // Mauve
    cursor_symbol_fg: Color::Rgb(249, 226, 175), // Yellow
    badge_bg: Color::Rgb(49, 50, 68),         // Surface0
    badge_fg: Color::Rgb(203, 166, 247),      // Mauve
    search_icon_fg: Color::Rgb(249, 226, 175), // Yellow
    search_border: Color::Rgb(137, 180, 250), // Sapphire
    search_text_fg: Color::Rgb(205, 214, 244),
    footer_key_bg: Color::Rgb(49, 50, 68),
    footer_key_fg: Color::Rgb(203, 166, 247),
    footer_desc_fg: Color::Rgb(186, 194, 222),
};

pub const THEME_CATPPUCCIN_LATTE: Theme = Theme {
    name: "catppuccin-latte",
    display_name: "Catppuccin Latte",
    primary: Color::Rgb(136, 57, 239),        // Mauve
    secondary: Color::Rgb(30, 102, 245),      // Blue
    accent: Color::Rgb(23, 146, 153),         // Teal
    muted: Color::Rgb(156, 160, 176),         // Overlay0
    border: Color::Rgb(30, 102, 245),
    border_active: Color::Rgb(136, 57, 239),
    banner_start: Color::Rgb(136, 57, 239),
    banner_end: Color::Rgb(23, 146, 153),
    header_fg: Color::Rgb(76, 79, 105),       // Text
    table_header: Color::Rgb(23, 146, 153),
    host_name: Color::Rgb(136, 57, 239),
    aliases: Color::Rgb(114, 135, 253),       // Lavender
    user: Color::Rgb(64, 160, 43),            // Green
    destination: Color::Rgb(4, 165, 229),     // Sky
    port: Color::Rgb(254, 100, 11),           // Peach
    proxy: Color::Rgb(234, 118, 203),         // Pink
    selected_fg: Color::Rgb(239, 241, 245),   // Base
    selected_bg: Color::Rgb(136, 57, 239),
    cursor_symbol_fg: Color::Rgb(223, 142, 29),
    badge_bg: Color::Rgb(204, 208, 218),
    badge_fg: Color::Rgb(136, 57, 239),
    search_icon_fg: Color::Rgb(223, 142, 29),
    search_border: Color::Rgb(30, 102, 245),
    search_text_fg: Color::Rgb(76, 79, 105),
    footer_key_bg: Color::Rgb(204, 208, 218),
    footer_key_fg: Color::Rgb(136, 57, 239),
    footer_desc_fg: Color::Rgb(76, 79, 105),
};

pub const THEME_DRACULA: Theme = Theme {
    name: "dracula",
    display_name: "Dracula",
    primary: Color::Rgb(189, 147, 249),       // Purple
    secondary: Color::Rgb(255, 121, 198),     // Pink
    accent: Color::Rgb(139, 233, 253),        // Cyan
    muted: Color::Rgb(98, 114, 164),          // Comment
    border: Color::Rgb(189, 147, 249),        // Purple
    border_active: Color::Rgb(255, 121, 198), // Pink
    banner_start: Color::Rgb(189, 147, 249),  // Purple
    banner_end: Color::Rgb(255, 121, 198),    // Pink
    header_fg: Color::Rgb(248, 248, 242),     // Foreground
    table_header: Color::Rgb(139, 233, 253),  // Cyan
    host_name: Color::Rgb(189, 147, 249),     // Purple
    aliases: Color::Rgb(255, 184, 108),       // Orange
    user: Color::Rgb(80, 250, 123),           // Green
    destination: Color::Rgb(139, 233, 253),   // Cyan
    port: Color::Rgb(241, 250, 140),          // Yellow
    proxy: Color::Rgb(255, 121, 198),         // Pink
    selected_fg: Color::Rgb(40, 42, 54),      // Background
    selected_bg: Color::Rgb(189, 147, 249),   // Purple
    cursor_symbol_fg: Color::Rgb(241, 250, 140), // Yellow
    badge_bg: Color::Rgb(68, 71, 90),         // Current Line
    badge_fg: Color::Rgb(139, 233, 253),      // Cyan
    search_icon_fg: Color::Rgb(255, 121, 198),
    search_border: Color::Rgb(189, 147, 249),
    search_text_fg: Color::Rgb(248, 248, 242),
    footer_key_bg: Color::Rgb(68, 71, 90),
    footer_key_fg: Color::Rgb(80, 250, 123),
    footer_desc_fg: Color::Rgb(248, 248, 242),
};

pub const THEME_TOKYONIGHT: Theme = Theme {
    name: "tokyonight",
    display_name: "Tokyo Night",
    primary: Color::Rgb(122, 162, 247),       // Blue
    secondary: Color::Rgb(187, 154, 247),     // Magenta
    accent: Color::Rgb(125, 207, 255),        // Cyan
    muted: Color::Rgb(86, 95, 137),           // Comment
    border: Color::Rgb(122, 162, 247),        // Blue
    border_active: Color::Rgb(187, 154, 247), // Magenta
    banner_start: Color::Rgb(122, 162, 247),
    banner_end: Color::Rgb(187, 154, 247),
    header_fg: Color::Rgb(192, 202, 245),
    table_header: Color::Rgb(125, 207, 255),
    host_name: Color::Rgb(122, 162, 247),
    aliases: Color::Rgb(187, 154, 247),
    user: Color::Rgb(158, 206, 106),          // Green
    destination: Color::Rgb(125, 207, 255),
    port: Color::Rgb(224, 175, 104),          // Orange
    proxy: Color::Rgb(247, 118, 142),         // Red / Pink
    selected_fg: Color::Rgb(26, 27, 38),
    selected_bg: Color::Rgb(122, 162, 247),
    cursor_symbol_fg: Color::Rgb(224, 175, 104),
    badge_bg: Color::Rgb(41, 46, 66),
    badge_fg: Color::Rgb(125, 207, 255),
    search_icon_fg: Color::Rgb(224, 175, 104),
    search_border: Color::Rgb(122, 162, 247),
    search_text_fg: Color::Rgb(192, 202, 245),
    footer_key_bg: Color::Rgb(41, 46, 66),
    footer_key_fg: Color::Rgb(125, 207, 255),
    footer_desc_fg: Color::Rgb(192, 202, 245),
};

pub const THEME_NORD: Theme = Theme {
    name: "nord",
    display_name: "Nord",
    primary: Color::Rgb(136, 192, 208),       // Frost Cyan (nord8)
    secondary: Color::Rgb(129, 161, 193),     // Frost Blue (nord9)
    accent: Color::Rgb(163, 190, 140),        // Aurora Green (nord14)
    muted: Color::Rgb(94, 129, 172),          // nord10
    border: Color::Rgb(129, 161, 193),
    border_active: Color::Rgb(136, 192, 208),
    banner_start: Color::Rgb(136, 192, 208),
    banner_end: Color::Rgb(129, 161, 193),
    header_fg: Color::Rgb(236, 239, 244),
    table_header: Color::Rgb(143, 188, 187),  // Frost Teal (nord7)
    host_name: Color::Rgb(136, 192, 208),
    aliases: Color::Rgb(129, 161, 193),
    user: Color::Rgb(163, 190, 140),          // Green (nord14)
    destination: Color::Rgb(143, 188, 187),
    port: Color::Rgb(235, 203, 139),          // Yellow (nord13)
    proxy: Color::Rgb(180, 142, 173),         // Purple (nord15)
    selected_fg: Color::Rgb(46, 52, 64),
    selected_bg: Color::Rgb(136, 192, 208),
    cursor_symbol_fg: Color::Rgb(235, 203, 139),
    badge_bg: Color::Rgb(59, 66, 82),
    badge_fg: Color::Rgb(136, 192, 208),
    search_icon_fg: Color::Rgb(235, 203, 139),
    search_border: Color::Rgb(129, 161, 193),
    search_text_fg: Color::Rgb(236, 239, 244),
    footer_key_bg: Color::Rgb(59, 66, 82),
    footer_key_fg: Color::Rgb(136, 192, 208),
    footer_desc_fg: Color::Rgb(236, 239, 244),
};

pub const THEME_GRUVBOX: Theme = Theme {
    name: "gruvbox",
    display_name: "Gruvbox Dark",
    primary: Color::Rgb(254, 128, 25),        // Orange
    secondary: Color::Rgb(250, 189, 47),      // Yellow
    accent: Color::Rgb(142, 192, 124),        // Aqua
    muted: Color::Rgb(146, 131, 116),         // Gray
    border: Color::Rgb(254, 128, 25),
    border_active: Color::Rgb(250, 189, 47),
    banner_start: Color::Rgb(254, 128, 25),
    banner_end: Color::Rgb(250, 189, 47),
    header_fg: Color::Rgb(235, 219, 178),
    table_header: Color::Rgb(250, 189, 47),
    host_name: Color::Rgb(254, 128, 25),
    aliases: Color::Rgb(211, 134, 155),       // Purple
    user: Color::Rgb(184, 187, 38),           // Green
    destination: Color::Rgb(142, 192, 124),   // Aqua
    port: Color::Rgb(250, 189, 47),           // Yellow
    proxy: Color::Rgb(204, 36, 29),           // Red
    selected_fg: Color::Rgb(40, 40, 40),
    selected_bg: Color::Rgb(254, 128, 25),
    cursor_symbol_fg: Color::Rgb(250, 189, 47),
    badge_bg: Color::Rgb(60, 56, 54),
    badge_fg: Color::Rgb(250, 189, 47),
    search_icon_fg: Color::Rgb(250, 189, 47),
    search_border: Color::Rgb(254, 128, 25),
    search_text_fg: Color::Rgb(235, 219, 178),
    footer_key_bg: Color::Rgb(60, 56, 54),
    footer_key_fg: Color::Rgb(254, 128, 25),
    footer_desc_fg: Color::Rgb(235, 219, 178),
};

pub const THEME_ROSE_PINE: Theme = Theme {
    name: "rose-pine",
    display_name: "Rosé Pine",
    primary: Color::Rgb(235, 188, 186),       // Rose
    secondary: Color::Rgb(196, 167, 231),     // Iris
    accent: Color::Rgb(156, 207, 216),        // Foam
    muted: Color::Rgb(110, 106, 134),         // Muted
    border: Color::Rgb(196, 167, 231),
    border_active: Color::Rgb(235, 188, 186),
    banner_start: Color::Rgb(235, 188, 186),
    banner_end: Color::Rgb(156, 207, 216),
    header_fg: Color::Rgb(224, 222, 244),     // Text
    table_header: Color::Rgb(156, 207, 216),
    host_name: Color::Rgb(235, 188, 186),
    aliases: Color::Rgb(196, 167, 231),
    user: Color::Rgb(49, 116, 143),           // Pine
    destination: Color::Rgb(156, 207, 216),
    port: Color::Rgb(246, 193, 119),          // Gold
    proxy: Color::Rgb(235, 111, 146),         // Love
    selected_fg: Color::Rgb(25, 23, 36),      // Base
    selected_bg: Color::Rgb(235, 188, 186),
    cursor_symbol_fg: Color::Rgb(246, 193, 119),
    badge_bg: Color::Rgb(38, 35, 58),         // Surface
    badge_fg: Color::Rgb(235, 188, 186),
    search_icon_fg: Color::Rgb(246, 193, 119),
    search_border: Color::Rgb(196, 167, 231),
    search_text_fg: Color::Rgb(224, 222, 244),
    footer_key_bg: Color::Rgb(38, 35, 58),
    footer_key_fg: Color::Rgb(156, 207, 216),
    footer_desc_fg: Color::Rgb(224, 222, 244),
};

pub const THEME_ONEDARK: Theme = Theme {
    name: "onedark",
    display_name: "One Dark",
    primary: Color::Rgb(97, 175, 239),        // Blue
    secondary: Color::Rgb(198, 120, 221),     // Purple
    accent: Color::Rgb(86, 182, 194),         // Cyan
    muted: Color::Rgb(92, 99, 112),           // Comment
    border: Color::Rgb(97, 175, 239),
    border_active: Color::Rgb(198, 120, 221),
    banner_start: Color::Rgb(97, 175, 239),
    banner_end: Color::Rgb(86, 182, 194),
    header_fg: Color::Rgb(171, 178, 191),
    table_header: Color::Rgb(86, 182, 194),
    host_name: Color::Rgb(97, 175, 239),
    aliases: Color::Rgb(198, 120, 221),
    user: Color::Rgb(152, 195, 121),          // Green
    destination: Color::Rgb(86, 182, 194),
    port: Color::Rgb(229, 192, 123),          // Yellow
    proxy: Color::Rgb(224, 108, 117),         // Red
    selected_fg: Color::Rgb(40, 44, 52),
    selected_bg: Color::Rgb(97, 175, 239),
    cursor_symbol_fg: Color::Rgb(229, 192, 123),
    badge_bg: Color::Rgb(49, 54, 63),
    badge_fg: Color::Rgb(97, 175, 239),
    search_icon_fg: Color::Rgb(229, 192, 123),
    search_border: Color::Rgb(97, 175, 239),
    search_text_fg: Color::Rgb(171, 178, 191),
    footer_key_bg: Color::Rgb(49, 54, 63),
    footer_key_fg: Color::Rgb(152, 195, 121),
    footer_desc_fg: Color::Rgb(171, 178, 191),
};

pub const THEME_KANAGAWA: Theme = Theme {
    name: "kanagawa",
    display_name: "Kanagawa",
    primary: Color::Rgb(126, 156, 216),       // Wave Blue
    secondary: Color::Rgb(149, 127, 184),     // Oni Violet
    accent: Color::Rgb(122, 168, 159),        // Crystal Blue
    muted: Color::Rgb(114, 113, 105),         // Fuji Gray
    border: Color::Rgb(126, 156, 216),
    border_active: Color::Rgb(149, 127, 184),
    banner_start: Color::Rgb(126, 156, 216),
    banner_end: Color::Rgb(122, 168, 159),
    header_fg: Color::Rgb(220, 215, 186),     // Fuji White
    table_header: Color::Rgb(122, 168, 159),
    host_name: Color::Rgb(126, 156, 216),
    aliases: Color::Rgb(149, 127, 184),
    user: Color::Rgb(152, 187, 108),          // Spring Green
    destination: Color::Rgb(122, 168, 159),
    port: Color::Rgb(230, 195, 132),          // Carp Yellow
    proxy: Color::Rgb(255, 160, 102),         // Surimi Orange
    selected_fg: Color::Rgb(31, 31, 40),      // Sumi Ink
    selected_bg: Color::Rgb(126, 156, 216),
    cursor_symbol_fg: Color::Rgb(230, 195, 132),
    badge_bg: Color::Rgb(42, 42, 55),
    badge_fg: Color::Rgb(126, 156, 216),
    search_icon_fg: Color::Rgb(230, 195, 132),
    search_border: Color::Rgb(126, 156, 216),
    search_text_fg: Color::Rgb(220, 215, 186),
    footer_key_bg: Color::Rgb(42, 42, 55),
    footer_key_fg: Color::Rgb(152, 187, 108),
    footer_desc_fg: Color::Rgb(220, 215, 186),
};

pub const THEME_EVERFOREST: Theme = Theme {
    name: "everforest",
    display_name: "Everforest",
    primary: Color::Rgb(167, 192, 128),       // Green
    secondary: Color::Rgb(131, 192, 146),     // Aqua
    accent: Color::Rgb(127, 187, 179),        // Blue
    muted: Color::Rgb(133, 146, 137),
    border: Color::Rgb(167, 192, 128),
    border_active: Color::Rgb(131, 192, 146),
    banner_start: Color::Rgb(167, 192, 128),
    banner_end: Color::Rgb(127, 187, 179),
    header_fg: Color::Rgb(211, 198, 170),
    table_header: Color::Rgb(131, 192, 146),
    host_name: Color::Rgb(167, 192, 128),
    aliases: Color::Rgb(214, 153, 182),       // Purple
    user: Color::Rgb(131, 192, 146),
    destination: Color::Rgb(127, 187, 179),
    port: Color::Rgb(219, 188, 127),          // Yellow
    proxy: Color::Rgb(230, 152, 117),         // Orange
    selected_fg: Color::Rgb(45, 53, 59),
    selected_bg: Color::Rgb(167, 192, 128),
    cursor_symbol_fg: Color::Rgb(219, 188, 127),
    badge_bg: Color::Rgb(61, 70, 75),
    badge_fg: Color::Rgb(167, 192, 128),
    search_icon_fg: Color::Rgb(219, 188, 127),
    search_border: Color::Rgb(167, 192, 128),
    search_text_fg: Color::Rgb(211, 198, 170),
    footer_key_bg: Color::Rgb(61, 70, 75),
    footer_key_fg: Color::Rgb(167, 192, 128),
    footer_desc_fg: Color::Rgb(211, 198, 170),
};

pub const THEME_SOLARIZED_DARK: Theme = Theme {
    name: "solarized-dark",
    display_name: "Solarized Dark",
    primary: Color::Rgb(38, 139, 210),        // Blue
    secondary: Color::Rgb(42, 161, 152),      // Cyan
    accent: Color::Rgb(133, 153, 0),          // Green
    muted: Color::Rgb(101, 123, 131),         // Base00
    border: Color::Rgb(38, 139, 210),
    border_active: Color::Rgb(42, 161, 152),
    banner_start: Color::Rgb(38, 139, 210),
    banner_end: Color::Rgb(42, 161, 152),
    header_fg: Color::Rgb(147, 161, 161),
    table_header: Color::Rgb(42, 161, 152),
    host_name: Color::Rgb(38, 139, 210),
    aliases: Color::Rgb(108, 113, 196),       // Violet
    user: Color::Rgb(133, 153, 0),            // Green
    destination: Color::Rgb(42, 161, 152),
    port: Color::Rgb(181, 137, 0),            // Yellow
    proxy: Color::Rgb(203, 75, 22),           // Orange
    selected_fg: Color::Rgb(0, 43, 54),       // Base03
    selected_bg: Color::Rgb(38, 139, 210),
    cursor_symbol_fg: Color::Rgb(181, 137, 0),
    badge_bg: Color::Rgb(7, 54, 66),          // Base02
    badge_fg: Color::Rgb(38, 139, 210),
    search_icon_fg: Color::Rgb(181, 137, 0),
    search_border: Color::Rgb(38, 139, 210),
    search_text_fg: Color::Rgb(147, 161, 161),
    footer_key_bg: Color::Rgb(7, 54, 66),
    footer_key_fg: Color::Rgb(42, 161, 152),
    footer_desc_fg: Color::Rgb(147, 161, 161),
};

pub const THEME_AYU_DARK: Theme = Theme {
    name: "ayu-dark",
    display_name: "Ayu Dark",
    primary: Color::Rgb(255, 180, 84),        // Orange
    secondary: Color::Rgb(230, 180, 80),      // Yellow
    accent: Color::Rgb(57, 186, 230),         // Blue
    muted: Color::Rgb(115, 118, 126),
    border: Color::Rgb(255, 180, 84),
    border_active: Color::Rgb(57, 186, 230),
    banner_start: Color::Rgb(255, 180, 84),
    banner_end: Color::Rgb(57, 186, 230),
    header_fg: Color::Rgb(203, 204, 198),
    table_header: Color::Rgb(57, 186, 230),
    host_name: Color::Rgb(255, 180, 84),
    aliases: Color::Rgb(210, 166, 255),       // Purple
    user: Color::Rgb(170, 217, 76),           // Green
    destination: Color::Rgb(57, 186, 230),
    port: Color::Rgb(230, 180, 80),
    proxy: Color::Rgb(240, 113, 120),         // Red
    selected_fg: Color::Rgb(15, 20, 25),
    selected_bg: Color::Rgb(255, 180, 84),
    cursor_symbol_fg: Color::Rgb(230, 180, 80),
    badge_bg: Color::Rgb(25, 33, 44),
    badge_fg: Color::Rgb(255, 180, 84),
    search_icon_fg: Color::Rgb(230, 180, 80),
    search_border: Color::Rgb(255, 180, 84),
    search_text_fg: Color::Rgb(203, 204, 198),
    footer_key_bg: Color::Rgb(25, 33, 44),
    footer_key_fg: Color::Rgb(170, 217, 76),
    footer_desc_fg: Color::Rgb(203, 204, 198),
};

pub const THEME_CYBERPUNK: Theme = Theme {
    name: "cyberpunk",
    display_name: "Cyberpunk Neon",
    primary: Color::Rgb(254, 231, 21),        // Neon Yellow
    secondary: Color::Rgb(255, 0, 127),       // Neon Pink
    accent: Color::Rgb(0, 240, 255),          // Neon Cyan
    muted: Color::Rgb(113, 113, 122),
    border: Color::Rgb(0, 240, 255),          // Neon Cyan
    border_active: Color::Rgb(254, 231, 21),  // Neon Yellow
    banner_start: Color::Rgb(255, 0, 127),    // Neon Pink
    banner_end: Color::Rgb(0, 240, 255),      // Neon Cyan
    header_fg: Color::Rgb(254, 231, 21),
    table_header: Color::Rgb(0, 240, 255),
    host_name: Color::Rgb(254, 231, 21),      // Bright Yellow
    aliases: Color::Rgb(255, 0, 127),         // Pink
    user: Color::Rgb(57, 255, 20),            // Matrix Green
    destination: Color::Rgb(0, 240, 255),     // Cyan
    port: Color::Rgb(255, 149, 0),            // Orange
    proxy: Color::Rgb(190, 0, 254),           // Purple
    selected_fg: Color::Rgb(10, 10, 15),
    selected_bg: Color::Rgb(0, 240, 255),
    cursor_symbol_fg: Color::Rgb(254, 231, 21),
    badge_bg: Color::Rgb(30, 27, 46),
    badge_fg: Color::Rgb(254, 231, 21),
    search_icon_fg: Color::Rgb(254, 231, 21),
    search_border: Color::Rgb(0, 240, 255),
    search_text_fg: Color::Rgb(255, 255, 255),
    footer_key_bg: Color::Rgb(30, 27, 46),
    footer_key_fg: Color::Rgb(0, 240, 255),
    footer_desc_fg: Color::Rgb(255, 255, 255),
};

pub const THEME_SYNTHWAVE: Theme = Theme {
    name: "synthwave",
    display_name: "Synthwave '84",
    primary: Color::Rgb(255, 126, 219),       // Neon Magenta
    secondary: Color::Rgb(54, 243, 209),      // Neon Cyan
    accent: Color::Rgb(254, 222, 93),         // Neon Yellow
    muted: Color::Rgb(105, 90, 140),
    border: Color::Rgb(255, 126, 219),
    border_active: Color::Rgb(54, 243, 209),
    banner_start: Color::Rgb(255, 126, 219),
    banner_end: Color::Rgb(54, 243, 209),
    header_fg: Color::Rgb(254, 222, 93),
    table_header: Color::Rgb(54, 243, 209),
    host_name: Color::Rgb(255, 126, 219),
    aliases: Color::Rgb(184, 142, 252),
    user: Color::Rgb(54, 243, 209),
    destination: Color::Rgb(114, 241, 254),
    port: Color::Rgb(254, 222, 93),
    proxy: Color::Rgb(254, 68, 80),
    selected_fg: Color::Rgb(36, 27, 47),
    selected_bg: Color::Rgb(255, 126, 219),
    cursor_symbol_fg: Color::Rgb(254, 222, 93),
    badge_bg: Color::Rgb(52, 41, 68),
    badge_fg: Color::Rgb(54, 243, 209),
    search_icon_fg: Color::Rgb(254, 222, 93),
    search_border: Color::Rgb(255, 126, 219),
    search_text_fg: Color::Rgb(255, 255, 255),
    footer_key_bg: Color::Rgb(52, 41, 68),
    footer_key_fg: Color::Rgb(255, 126, 219),
    footer_desc_fg: Color::Rgb(255, 255, 255),
};

pub const THEME_MATRIX: Theme = Theme {
    name: "matrix",
    display_name: "Matrix",
    primary: Color::Rgb(0, 255, 65),          // Bright Matrix Green
    secondary: Color::Rgb(0, 143, 17),        // Dark Matrix Green
    accent: Color::Rgb(57, 255, 20),          // Lime
    muted: Color::Rgb(0, 59, 0),
    border: Color::Rgb(0, 143, 17),
    border_active: Color::Rgb(0, 255, 65),
    banner_start: Color::Rgb(0, 255, 65),
    banner_end: Color::Rgb(0, 143, 17),
    header_fg: Color::Rgb(0, 255, 65),
    table_header: Color::Rgb(57, 255, 20),
    host_name: Color::Rgb(0, 255, 65),
    aliases: Color::Rgb(140, 230, 140),
    user: Color::Rgb(57, 255, 20),
    destination: Color::Rgb(180, 255, 180),
    port: Color::Rgb(0, 200, 80),
    proxy: Color::Rgb(0, 255, 170),
    selected_fg: Color::Rgb(10, 15, 10),
    selected_bg: Color::Rgb(0, 255, 65),
    cursor_symbol_fg: Color::Rgb(255, 255, 255),
    badge_bg: Color::Rgb(15, 35, 15),
    badge_fg: Color::Rgb(0, 255, 65),
    search_icon_fg: Color::Rgb(0, 255, 65),
    search_border: Color::Rgb(0, 143, 17),
    search_text_fg: Color::Rgb(0, 255, 65),
    footer_key_bg: Color::Rgb(15, 35, 15),
    footer_key_fg: Color::Rgb(0, 255, 65),
    footer_desc_fg: Color::Rgb(180, 255, 180),
};

pub const THEME_HACKER_AMBER: Theme = Theme {
    name: "hacker-amber",
    display_name: "Amber CRT",
    primary: Color::Rgb(255, 176, 0),         // Amber
    secondary: Color::Rgb(179, 122, 0),
    accent: Color::Rgb(255, 200, 50),
    muted: Color::Rgb(90, 60, 0),
    border: Color::Rgb(179, 122, 0),
    border_active: Color::Rgb(255, 176, 0),
    banner_start: Color::Rgb(255, 176, 0),
    banner_end: Color::Rgb(179, 122, 0),
    header_fg: Color::Rgb(255, 200, 50),
    table_header: Color::Rgb(255, 176, 0),
    host_name: Color::Rgb(255, 176, 0),
    aliases: Color::Rgb(255, 220, 120),
    user: Color::Rgb(255, 200, 50),
    destination: Color::Rgb(255, 230, 160),
    port: Color::Rgb(255, 150, 0),
    proxy: Color::Rgb(255, 100, 0),
    selected_fg: Color::Rgb(20, 15, 0),
    selected_bg: Color::Rgb(255, 176, 0),
    cursor_symbol_fg: Color::Rgb(255, 255, 255),
    badge_bg: Color::Rgb(40, 25, 0),
    badge_fg: Color::Rgb(255, 176, 0),
    search_icon_fg: Color::Rgb(255, 176, 0),
    search_border: Color::Rgb(179, 122, 0),
    search_text_fg: Color::Rgb(255, 200, 50),
    footer_key_bg: Color::Rgb(40, 25, 0),
    footer_key_fg: Color::Rgb(255, 176, 0),
    footer_desc_fg: Color::Rgb(255, 200, 50),
};

pub const THEME_SUNSET: Theme = Theme {
    name: "sunset",
    display_name: "Sunset",
    primary: Color::Rgb(255, 107, 107),       // Coral Pink
    secondary: Color::Rgb(254, 202, 87),      // Warm Gold
    accent: Color::Rgb(255, 159, 243),        // Lavender Pink
    muted: Color::Rgb(130, 90, 120),
    border: Color::Rgb(255, 107, 107),
    border_active: Color::Rgb(254, 202, 87),
    banner_start: Color::Rgb(255, 107, 107),
    banner_end: Color::Rgb(254, 202, 87),
    header_fg: Color::Rgb(255, 240, 245),
    table_header: Color::Rgb(254, 202, 87),
    host_name: Color::Rgb(255, 107, 107),
    aliases: Color::Rgb(255, 159, 243),
    user: Color::Rgb(72, 219, 251),
    destination: Color::Rgb(254, 202, 87),
    port: Color::Rgb(255, 159, 67),
    proxy: Color::Rgb(238, 82, 83),
    selected_fg: Color::Rgb(30, 20, 30),
    selected_bg: Color::Rgb(255, 107, 107),
    cursor_symbol_fg: Color::Rgb(254, 202, 87),
    badge_bg: Color::Rgb(50, 30, 45),
    badge_fg: Color::Rgb(254, 202, 87),
    search_icon_fg: Color::Rgb(254, 202, 87),
    search_border: Color::Rgb(255, 107, 107),
    search_text_fg: Color::Rgb(255, 255, 255),
    footer_key_bg: Color::Rgb(50, 30, 45),
    footer_key_fg: Color::Rgb(255, 107, 107),
    footer_desc_fg: Color::Rgb(255, 240, 245),
};

pub const THEME_MONOKAI: Theme = Theme {
    name: "monokai",
    display_name: "Monokai Pro",
    primary: Color::Rgb(255, 216, 102),       // Yellow
    secondary: Color::Rgb(255, 97, 136),      // Red / Pink
    accent: Color::Rgb(169, 220, 107),        // Green
    muted: Color::Rgb(114, 112, 114),
    border: Color::Rgb(255, 216, 102),
    border_active: Color::Rgb(255, 97, 136),
    banner_start: Color::Rgb(255, 97, 136),
    banner_end: Color::Rgb(255, 216, 102),
    header_fg: Color::Rgb(252, 252, 250),
    table_header: Color::Rgb(120, 220, 232),  // Cyan
    host_name: Color::Rgb(255, 216, 102),
    aliases: Color::Rgb(171, 157, 242),       // Purple
    user: Color::Rgb(169, 220, 107),          // Green
    destination: Color::Rgb(120, 220, 232),   // Cyan
    port: Color::Rgb(255, 97, 136),           // Red
    proxy: Color::Rgb(171, 157, 242),         // Purple
    selected_fg: Color::Rgb(45, 42, 46),
    selected_bg: Color::Rgb(255, 216, 102),
    cursor_symbol_fg: Color::Rgb(255, 97, 136),
    badge_bg: Color::Rgb(64, 62, 65),
    badge_fg: Color::Rgb(255, 216, 102),
    search_icon_fg: Color::Rgb(255, 97, 136),
    search_border: Color::Rgb(255, 216, 102),
    search_text_fg: Color::Rgb(252, 252, 250),
    footer_key_bg: Color::Rgb(64, 62, 65),
    footer_key_fg: Color::Rgb(169, 220, 107),
    footer_desc_fg: Color::Rgb(252, 252, 250),
};

pub const THEME_OCEAN: Theme = Theme {
    name: "ocean",
    display_name: "Oceanic",
    primary: Color::Rgb(72, 219, 251),        // Bright Sky
    secondary: Color::Rgb(0, 210, 211),       // Aqua
    accent: Color::Rgb(29, 209, 161),        // Mint
    muted: Color::Rgb(46, 75, 110),
    border: Color::Rgb(72, 219, 251),
    border_active: Color::Rgb(0, 210, 211),
    banner_start: Color::Rgb(72, 219, 251),
    banner_end: Color::Rgb(29, 209, 161),
    header_fg: Color::Rgb(220, 245, 255),
    table_header: Color::Rgb(0, 210, 211),
    host_name: Color::Rgb(72, 219, 251),
    aliases: Color::Rgb(154, 236, 219),
    user: Color::Rgb(29, 209, 161),
    destination: Color::Rgb(129, 236, 236),
    port: Color::Rgb(254, 202, 87),
    proxy: Color::Rgb(108, 92, 231),
    selected_fg: Color::Rgb(10, 25, 45),
    selected_bg: Color::Rgb(72, 219, 251),
    cursor_symbol_fg: Color::Rgb(254, 202, 87),
    badge_bg: Color::Rgb(15, 40, 70),
    badge_fg: Color::Rgb(72, 219, 251),
    search_icon_fg: Color::Rgb(29, 209, 161),
    search_border: Color::Rgb(72, 219, 251),
    search_text_fg: Color::Rgb(220, 245, 255),
    footer_key_bg: Color::Rgb(15, 40, 70),
    footer_key_fg: Color::Rgb(29, 209, 161),
    footer_desc_fg: Color::Rgb(220, 245, 255),
};

pub const THEME_CRIMSON: Theme = Theme {
    name: "crimson",
    display_name: "Crimson",
    primary: Color::Rgb(255, 75, 75),         // Crimson
    secondary: Color::Rgb(255, 110, 60),      // Blood Orange
    accent: Color::Rgb(255, 195, 18),         // Gold
    muted: Color::Rgb(120, 50, 50),
    border: Color::Rgb(255, 75, 75),
    border_active: Color::Rgb(255, 110, 60),
    banner_start: Color::Rgb(255, 75, 75),
    banner_end: Color::Rgb(255, 195, 18),
    header_fg: Color::Rgb(255, 230, 230),
    table_header: Color::Rgb(255, 195, 18),
    host_name: Color::Rgb(255, 75, 75),
    aliases: Color::Rgb(255, 150, 150),
    user: Color::Rgb(255, 195, 18),
    destination: Color::Rgb(255, 160, 120),
    port: Color::Rgb(255, 110, 60),
    proxy: Color::Rgb(237, 76, 103),
    selected_fg: Color::Rgb(30, 10, 10),
    selected_bg: Color::Rgb(255, 75, 75),
    cursor_symbol_fg: Color::Rgb(255, 195, 18),
    badge_bg: Color::Rgb(50, 20, 20),
    badge_fg: Color::Rgb(255, 75, 75),
    search_icon_fg: Color::Rgb(255, 195, 18),
    search_border: Color::Rgb(255, 75, 75),
    search_text_fg: Color::Rgb(255, 230, 230),
    footer_key_bg: Color::Rgb(50, 20, 20),
    footer_key_fg: Color::Rgb(255, 75, 75),
    footer_desc_fg: Color::Rgb(255, 230, 230),
};

pub const THEME_LAVENDER: Theme = Theme {
    name: "lavender",
    display_name: "Lavender Mist",
    primary: Color::Rgb(186, 147, 255),       // Soft Lavender
    secondary: Color::Rgb(255, 166, 201),     // Pastel Pink
    accent: Color::Rgb(166, 227, 255),        // Soft Sky
    muted: Color::Rgb(100, 85, 130),
    border: Color::Rgb(186, 147, 255),
    border_active: Color::Rgb(255, 166, 201),
    banner_start: Color::Rgb(186, 147, 255),
    banner_end: Color::Rgb(166, 227, 255),
    header_fg: Color::Rgb(240, 235, 255),
    table_header: Color::Rgb(166, 227, 255),
    host_name: Color::Rgb(186, 147, 255),
    aliases: Color::Rgb(255, 166, 201),
    user: Color::Rgb(168, 230, 207),          // Mint
    destination: Color::Rgb(166, 227, 255),
    port: Color::Rgb(255, 211, 181),
    proxy: Color::Rgb(255, 139, 148),
    selected_fg: Color::Rgb(25, 20, 35),
    selected_bg: Color::Rgb(186, 147, 255),
    cursor_symbol_fg: Color::Rgb(255, 166, 201),
    badge_bg: Color::Rgb(45, 35, 60),
    badge_fg: Color::Rgb(186, 147, 255),
    search_icon_fg: Color::Rgb(255, 166, 201),
    search_border: Color::Rgb(186, 147, 255),
    search_text_fg: Color::Rgb(240, 235, 255),
    footer_key_bg: Color::Rgb(45, 35, 60),
    footer_key_fg: Color::Rgb(186, 147, 255),
    footer_desc_fg: Color::Rgb(240, 235, 255),
};

/// Generates a full `Theme` from any Tailwind palette name.
#[must_use]
pub fn theme_from_tailwind(name: &'static str, p: tailwind::Palette) -> Theme {
    Theme {
        name,
        display_name: name,
        primary: p.c400,
        secondary: p.c500,
        accent: tailwind::CYAN.c400,
        muted: p.c700,
        border: p.c400,
        border_active: p.c300,
        banner_start: p.c300,
        banner_end: p.c600,
        header_fg: p.c100,
        table_header: tailwind::CYAN.c400,
        host_name: p.c400,
        aliases: p.c300,
        user: tailwind::EMERALD.c400,
        destination: tailwind::CYAN.c400,
        port: tailwind::AMBER.c400,
        proxy: tailwind::PURPLE.c400,
        selected_fg: Color::Black,
        selected_bg: p.c400,
        cursor_symbol_fg: tailwind::YELLOW.c400,
        badge_bg: p.c900,
        badge_fg: p.c300,
        search_icon_fg: tailwind::YELLOW.c400,
        search_border: p.c400,
        search_text_fg: p.c100,
        footer_key_bg: p.c900,
        footer_key_fg: p.c300,
        footer_desc_fg: p.c100,
    }
}

/// All available themes in order for cycling.
pub const ALL_THEMES: &[&str] = &[
    "catppuccin",
    "catppuccin-latte",
    "dracula",
    "tokyonight",
    "nord",
    "gruvbox",
    "rose-pine",
    "onedark",
    "kanagawa",
    "everforest",
    "solarized-dark",
    "ayu-dark",
    "cyberpunk",
    "synthwave",
    "matrix",
    "hacker-amber",
    "sunset",
    "monokai",
    "ocean",
    "crimson",
    "lavender",
    "emerald",
    "blue",
    "violet",
    "amber",
    "rose",
    "cyan",
    "sky",
    "indigo",
    "purple",
    "fuchsia",
    "pink",
    "slate",
    "gray",
    "zinc",
    "neutral",
    "stone",
    "red",
    "orange",
    "yellow",
    "lime",
    "green",
    "teal",
];

/// Finds a theme by name (supports both modern themes and Tailwind colors).
///
/// # Errors
/// Returns an error with valid theme names if `name` is unknown.
pub fn theme_by_name(name: &str) -> Result<Theme> {
    match name.to_lowercase().as_str() {
        "catppuccin" | "catppuccin-mocha" | "mocha" => Ok(THEME_CATPPUCCIN),
        "catppuccin-latte" | "latte" => Ok(THEME_CATPPUCCIN_LATTE),
        "dracula" => Ok(THEME_DRACULA),
        "tokyonight" | "tokyo-night" | "tokyo" => Ok(THEME_TOKYONIGHT),
        "nord" => Ok(THEME_NORD),
        "gruvbox" | "gruvbox-dark" => Ok(THEME_GRUVBOX),
        "rose-pine" | "rosepine" => Ok(THEME_ROSE_PINE),
        "onedark" | "one-dark" => Ok(THEME_ONEDARK),
        "kanagawa" => Ok(THEME_KANAGAWA),
        "everforest" => Ok(THEME_EVERFOREST),
        "solarized" | "solarized-dark" => Ok(THEME_SOLARIZED_DARK),
        "ayu" | "ayu-dark" => Ok(THEME_AYU_DARK),
        "cyberpunk" | "neon" => Ok(THEME_CYBERPUNK),
        "synthwave" | "synthwave84" => Ok(THEME_SYNTHWAVE),
        "matrix" => Ok(THEME_MATRIX),
        "hacker-amber" | "crt" => Ok(THEME_HACKER_AMBER),
        "sunset" => Ok(THEME_SUNSET),
        "monokai" | "monokai-pro" => Ok(THEME_MONOKAI),
        "ocean" | "oceanic" => Ok(THEME_OCEAN),
        "crimson" | "blood" => Ok(THEME_CRIMSON),
        "lavender" => Ok(THEME_LAVENDER),

        // Tailwind palettes
        "slate" => Ok(theme_from_tailwind("slate", tailwind::SLATE)),
        "gray" => Ok(theme_from_tailwind("gray", tailwind::GRAY)),
        "zinc" => Ok(theme_from_tailwind("zinc", tailwind::ZINC)),
        "neutral" => Ok(theme_from_tailwind("neutral", tailwind::NEUTRAL)),
        "stone" => Ok(theme_from_tailwind("stone", tailwind::STONE)),
        "red" => Ok(theme_from_tailwind("red", tailwind::RED)),
        "orange" => Ok(theme_from_tailwind("orange", tailwind::ORANGE)),
        "amber" => Ok(theme_from_tailwind("amber", tailwind::AMBER)),
        "yellow" => Ok(theme_from_tailwind("yellow", tailwind::YELLOW)),
        "lime" => Ok(theme_from_tailwind("lime", tailwind::LIME)),
        "green" => Ok(theme_from_tailwind("green", tailwind::GREEN)),
        "emerald" => Ok(theme_from_tailwind("emerald", tailwind::EMERALD)),
        "teal" => Ok(theme_from_tailwind("teal", tailwind::TEAL)),
        "cyan" => Ok(theme_from_tailwind("cyan", tailwind::CYAN)),
        "sky" => Ok(theme_from_tailwind("sky", tailwind::SKY)),
        "blue" => Ok(theme_from_tailwind("blue", tailwind::BLUE)),
        "indigo" => Ok(theme_from_tailwind("indigo", tailwind::INDIGO)),
        "violet" => Ok(theme_from_tailwind("violet", tailwind::VIOLET)),
        "purple" => Ok(theme_from_tailwind("purple", tailwind::PURPLE)),
        "fuchsia" => Ok(theme_from_tailwind("fuchsia", tailwind::FUCHSIA)),
        "pink" => Ok(theme_from_tailwind("pink", tailwind::PINK)),
        "rose" => Ok(theme_from_tailwind("rose", tailwind::ROSE)),

        _ => anyhow::bail!(
            "Unknown theme: {name}\nValid themes: catppuccin, catppuccin-latte, dracula, tokyonight, nord, gruvbox, rose-pine, onedark, kanagawa, everforest, solarized-dark, ayu-dark, cyberpunk, synthwave, matrix, hacker-amber, sunset, monokai, ocean, crimson, lavender, blue, emerald, violet, rose, cyan, sky, indigo, purple, fuchsia, pink, slate, gray, zinc, neutral, stone, red, orange, yellow, lime, green, teal"
        ),
    }
}

/// Cycle to next theme
#[must_use]
pub fn next_theme(current_name: &str) -> Theme {
    let current_lower = current_name.to_lowercase();
    let idx = ALL_THEMES
        .iter()
        .position(|&t| t == current_lower)
        .unwrap_or(0);
    let next_idx = (idx + 1) % ALL_THEMES.len();
    theme_by_name(ALL_THEMES[next_idx]).unwrap_or(THEME_CATPPUCCIN)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_themes_can_be_loaded() {
        for name in ALL_THEMES {
            let theme = theme_by_name(name);
            assert!(theme.is_ok(), "Theme '{name}' failed to load: {:?}", theme.err());
        }
    }

    #[test]
    fn test_unknown_theme_returns_helpful_error() {
        let err = theme_by_name("nonexistent_theme").unwrap_err().to_string();
        assert!(err.contains("Unknown theme: nonexistent_theme"));
        assert!(err.contains("catppuccin"));
        assert!(err.contains("dracula"));
    }

    #[test]
    fn test_next_theme_cycles_through_all_themes() {
        let first = ALL_THEMES[0];
        let mut current = first.to_string();
        for &expected_next in ALL_THEMES.iter().skip(1) {
            let next = next_theme(&current);
            assert_eq!(next.name, expected_next);
            current = next.name.to_string();
        }
        // Should wrap around to first
        let wrap = next_theme(&current);
        assert_eq!(wrap.name, first);
    }

    #[test]
    fn test_interpolate_rgb() {
        let c1 = Color::Rgb(0, 0, 0);
        let c2 = Color::Rgb(100, 200, 50);
        let mid = interpolate_rgb(c1, c2, 0.5);
        assert_eq!(mid, Color::Rgb(50, 100, 25));

        let start = interpolate_rgb(c1, c2, 0.0);
        assert_eq!(start, c1);

        let end = interpolate_rgb(c1, c2, 1.0);
        assert_eq!(end, c2);
    }
}
