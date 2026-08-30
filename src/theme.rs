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

pub const THEME_CATPPUCCIN_MACCHIATO: Theme = Theme {
    name: "catppuccin-macchiato",
    display_name: "Catppuccin Macchiato",
    primary: Color::Rgb(198, 160, 246),       // Mauve
    secondary: Color::Rgb(183, 189, 248),     // Lavender
    accent: Color::Rgb(139, 213, 202),        // Teal
    muted: Color::Rgb(110, 115, 141),
    border: Color::Rgb(198, 160, 246),
    border_active: Color::Rgb(138, 173, 244),
    banner_start: Color::Rgb(198, 160, 246),
    banner_end: Color::Rgb(138, 173, 244),
    header_fg: Color::Rgb(202, 211, 245),
    table_header: Color::Rgb(138, 173, 244),
    host_name: Color::Rgb(198, 160, 246),
    aliases: Color::Rgb(183, 189, 248),
    user: Color::Rgb(166, 218, 149),          // Green
    destination: Color::Rgb(145, 215, 227),   // Sapphire
    port: Color::Rgb(245, 169, 127),          // Peach
    proxy: Color::Rgb(245, 194, 231),         // Pink
    selected_fg: Color::Rgb(24, 25, 38),
    selected_bg: Color::Rgb(198, 160, 246),
    cursor_symbol_fg: Color::Rgb(238, 212, 159),
    badge_bg: Color::Rgb(36, 39, 58),
    badge_fg: Color::Rgb(198, 160, 246),
    search_icon_fg: Color::Rgb(238, 212, 159),
    search_border: Color::Rgb(198, 160, 246),
    search_text_fg: Color::Rgb(202, 211, 245),
    footer_key_bg: Color::Rgb(36, 39, 58),
    footer_key_fg: Color::Rgb(198, 160, 246),
    footer_desc_fg: Color::Rgb(202, 211, 245),
};

pub const THEME_CATPPUCCIN_FRAPPE: Theme = Theme {
    name: "catppuccin-frappe",
    display_name: "Catppuccin Frappé",
    primary: Color::Rgb(202, 158, 230),       // Mauve
    secondary: Color::Rgb(186, 187, 241),     // Lavender
    accent: Color::Rgb(129, 200, 190),        // Teal
    muted: Color::Rgb(115, 121, 148),
    border: Color::Rgb(202, 158, 230),
    border_active: Color::Rgb(140, 170, 238),
    banner_start: Color::Rgb(202, 158, 230),
    banner_end: Color::Rgb(140, 170, 238),
    header_fg: Color::Rgb(198, 208, 245),
    table_header: Color::Rgb(140, 170, 238),
    host_name: Color::Rgb(202, 158, 230),
    aliases: Color::Rgb(186, 187, 241),
    user: Color::Rgb(166, 209, 137),          // Green
    destination: Color::Rgb(133, 193, 220),   // Sapphire
    port: Color::Rgb(239, 159, 118),          // Peach
    proxy: Color::Rgb(244, 184, 228),         // Pink
    selected_fg: Color::Rgb(35, 38, 52),
    selected_bg: Color::Rgb(202, 158, 230),
    cursor_symbol_fg: Color::Rgb(229, 200, 144),
    badge_bg: Color::Rgb(48, 52, 70),
    badge_fg: Color::Rgb(202, 158, 230),
    search_icon_fg: Color::Rgb(229, 200, 144),
    search_border: Color::Rgb(202, 158, 230),
    search_text_fg: Color::Rgb(198, 208, 245),
    footer_key_bg: Color::Rgb(48, 52, 70),
    footer_key_fg: Color::Rgb(202, 158, 230),
    footer_desc_fg: Color::Rgb(198, 208, 245),
};

pub const THEME_TOKYONIGHT_STORM: Theme = Theme {
    name: "tokyonight-storm",
    display_name: "Tokyo Night Storm",
    primary: Color::Rgb(122, 162, 247),       // Blue
    secondary: Color::Rgb(187, 154, 247),     // Magenta
    accent: Color::Rgb(125, 207, 255),        // Cyan
    muted: Color::Rgb(86, 95, 137),
    border: Color::Rgb(122, 162, 247),
    border_active: Color::Rgb(187, 154, 247),
    banner_start: Color::Rgb(122, 162, 247),
    banner_end: Color::Rgb(187, 154, 247),
    header_fg: Color::Rgb(192, 202, 245),
    table_header: Color::Rgb(125, 207, 255),
    host_name: Color::Rgb(122, 162, 247),
    aliases: Color::Rgb(187, 154, 247),
    user: Color::Rgb(158, 206, 106),          // Green
    destination: Color::Rgb(125, 207, 255),
    port: Color::Rgb(255, 158, 100),          // Orange
    proxy: Color::Rgb(247, 118, 142),         // Red
    selected_fg: Color::Rgb(29, 32, 48),
    selected_bg: Color::Rgb(122, 162, 247),
    cursor_symbol_fg: Color::Rgb(224, 175, 104),
    badge_bg: Color::Rgb(36, 40, 59),
    badge_fg: Color::Rgb(122, 162, 247),
    search_icon_fg: Color::Rgb(224, 175, 104),
    search_border: Color::Rgb(122, 162, 247),
    search_text_fg: Color::Rgb(192, 202, 245),
    footer_key_bg: Color::Rgb(36, 40, 59),
    footer_key_fg: Color::Rgb(122, 162, 247),
    footer_desc_fg: Color::Rgb(192, 202, 245),
};

pub const THEME_TOKYONIGHT_LIGHT: Theme = Theme {
    name: "tokyonight-light",
    display_name: "Tokyo Night Light",
    primary: Color::Rgb(52, 84, 139),         // Deep Blue
    secondary: Color::Rgb(143, 86, 177),      // Purple
    accent: Color::Rgb(15, 75, 110),          // Cyan
    muted: Color::Rgb(142, 146, 166),
    border: Color::Rgb(52, 84, 139),
    border_active: Color::Rgb(143, 86, 177),
    banner_start: Color::Rgb(52, 84, 139),
    banner_end: Color::Rgb(143, 86, 177),
    header_fg: Color::Rgb(52, 59, 88),
    table_header: Color::Rgb(15, 75, 110),
    host_name: Color::Rgb(52, 84, 139),
    aliases: Color::Rgb(143, 86, 177),
    user: Color::Rgb(72, 94, 28),             // Dark Green
    destination: Color::Rgb(15, 75, 110),
    port: Color::Rgb(150, 80, 20),            // Brown/Orange
    proxy: Color::Rgb(140, 67, 86),           // Crimson
    selected_fg: Color::Rgb(240, 242, 248),
    selected_bg: Color::Rgb(52, 84, 139),
    cursor_symbol_fg: Color::Rgb(150, 80, 20),
    badge_bg: Color::Rgb(215, 218, 228),
    badge_fg: Color::Rgb(52, 84, 139),
    search_icon_fg: Color::Rgb(150, 80, 20),
    search_border: Color::Rgb(52, 84, 139),
    search_text_fg: Color::Rgb(52, 59, 88),
    footer_key_bg: Color::Rgb(215, 218, 228),
    footer_key_fg: Color::Rgb(52, 84, 139),
    footer_desc_fg: Color::Rgb(52, 59, 88),
};

pub const THEME_ROSE_PINE_MOON: Theme = Theme {
    name: "rose-pine-moon",
    display_name: "Rosé Pine Moon",
    primary: Color::Rgb(235, 188, 186),       // Rose
    secondary: Color::Rgb(196, 167, 231),     // Iris
    accent: Color::Rgb(156, 207, 216),        // Foam
    muted: Color::Rgb(110, 106, 134),
    border: Color::Rgb(235, 188, 186),
    border_active: Color::Rgb(196, 167, 231),
    banner_start: Color::Rgb(235, 188, 186),
    banner_end: Color::Rgb(196, 167, 231),
    header_fg: Color::Rgb(224, 222, 244),
    table_header: Color::Rgb(156, 207, 216),
    host_name: Color::Rgb(235, 188, 186),
    aliases: Color::Rgb(196, 167, 231),
    user: Color::Rgb(62, 143, 176),           // Pine
    destination: Color::Rgb(156, 207, 216),
    port: Color::Rgb(246, 193, 119),          // Gold
    proxy: Color::Rgb(235, 111, 146),         // Love
    selected_fg: Color::Rgb(35, 33, 54),
    selected_bg: Color::Rgb(235, 188, 186),
    cursor_symbol_fg: Color::Rgb(246, 193, 119),
    badge_bg: Color::Rgb(42, 39, 63),
    badge_fg: Color::Rgb(235, 188, 186),
    search_icon_fg: Color::Rgb(246, 193, 119),
    search_border: Color::Rgb(235, 188, 186),
    search_text_fg: Color::Rgb(224, 222, 244),
    footer_key_bg: Color::Rgb(42, 39, 63),
    footer_key_fg: Color::Rgb(235, 188, 186),
    footer_desc_fg: Color::Rgb(224, 222, 244),
};

pub const THEME_ROSE_PINE_DAWN: Theme = Theme {
    name: "rose-pine-dawn",
    display_name: "Rosé Pine Dawn",
    primary: Color::Rgb(215, 130, 126),       // Rose
    secondary: Color::Rgb(144, 122, 169),     // Iris
    accent: Color::Rgb(86, 148, 159),         // Foam
    muted: Color::Rgb(152, 147, 165),
    border: Color::Rgb(215, 130, 126),
    border_active: Color::Rgb(144, 122, 169),
    banner_start: Color::Rgb(215, 130, 126),
    banner_end: Color::Rgb(144, 122, 169),
    header_fg: Color::Rgb(87, 82, 121),
    table_header: Color::Rgb(86, 148, 159),
    host_name: Color::Rgb(215, 130, 126),
    aliases: Color::Rgb(144, 122, 169),
    user: Color::Rgb(40, 105, 131),           // Pine
    destination: Color::Rgb(86, 148, 159),
    port: Color::Rgb(234, 157, 52),           // Gold
    proxy: Color::Rgb(180, 99, 122),          // Love
    selected_fg: Color::Rgb(250, 244, 237),
    selected_bg: Color::Rgb(215, 130, 126),
    cursor_symbol_fg: Color::Rgb(234, 157, 52),
    badge_bg: Color::Rgb(242, 233, 222),
    badge_fg: Color::Rgb(215, 130, 126),
    search_icon_fg: Color::Rgb(234, 157, 52),
    search_border: Color::Rgb(215, 130, 126),
    search_text_fg: Color::Rgb(87, 82, 121),
    footer_key_bg: Color::Rgb(242, 233, 222),
    footer_key_fg: Color::Rgb(215, 130, 126),
    footer_desc_fg: Color::Rgb(87, 82, 121),
};

pub const THEME_GITHUB_DARK: Theme = Theme {
    name: "github-dark",
    display_name: "GitHub Dark",
    primary: Color::Rgb(88, 166, 255),        // Blue
    secondary: Color::Rgb(210, 168, 255),     // Purple
    accent: Color::Rgb(56, 139, 253),         // Accent Blue
    muted: Color::Rgb(110, 118, 129),
    border: Color::Rgb(88, 166, 255),
    border_active: Color::Rgb(210, 168, 255),
    banner_start: Color::Rgb(88, 166, 255),
    banner_end: Color::Rgb(210, 168, 255),
    header_fg: Color::Rgb(240, 246, 252),
    table_header: Color::Rgb(88, 166, 255),
    host_name: Color::Rgb(88, 166, 255),
    aliases: Color::Rgb(210, 168, 255),
    user: Color::Rgb(63, 185, 80),            // Green
    destination: Color::Rgb(121, 192, 255),
    port: Color::Rgb(240, 136, 62),           // Orange
    proxy: Color::Rgb(248, 81, 73),           // Red
    selected_fg: Color::Rgb(13, 17, 23),
    selected_bg: Color::Rgb(88, 166, 255),
    cursor_symbol_fg: Color::Rgb(210, 153, 34),
    badge_bg: Color::Rgb(22, 27, 34),
    badge_fg: Color::Rgb(88, 166, 255),
    search_icon_fg: Color::Rgb(210, 153, 34),
    search_border: Color::Rgb(88, 166, 255),
    search_text_fg: Color::Rgb(240, 246, 252),
    footer_key_bg: Color::Rgb(22, 27, 34),
    footer_key_fg: Color::Rgb(88, 166, 255),
    footer_desc_fg: Color::Rgb(240, 246, 252),
};

pub const THEME_GITHUB_LIGHT: Theme = Theme {
    name: "github-light",
    display_name: "GitHub Light",
    primary: Color::Rgb(9, 105, 218),         // Blue
    secondary: Color::Rgb(130, 80, 223),      // Purple
    accent: Color::Rgb(5, 80, 174),
    muted: Color::Rgb(101, 109, 118),
    border: Color::Rgb(9, 105, 218),
    border_active: Color::Rgb(130, 80, 223),
    banner_start: Color::Rgb(9, 105, 218),
    banner_end: Color::Rgb(130, 80, 223),
    header_fg: Color::Rgb(31, 35, 40),
    table_header: Color::Rgb(9, 105, 218),
    host_name: Color::Rgb(9, 105, 218),
    aliases: Color::Rgb(130, 80, 223),
    user: Color::Rgb(26, 127, 55),            // Green
    destination: Color::Rgb(9, 105, 218),
    port: Color::Rgb(188, 76, 0),             // Orange
    proxy: Color::Rgb(207, 34, 46),           // Red
    selected_fg: Color::Rgb(255, 255, 255),
    selected_bg: Color::Rgb(9, 105, 218),
    cursor_symbol_fg: Color::Rgb(154, 103, 0),
    badge_bg: Color::Rgb(235, 238, 242),
    badge_fg: Color::Rgb(9, 105, 218),
    search_icon_fg: Color::Rgb(154, 103, 0),
    search_border: Color::Rgb(9, 105, 218),
    search_text_fg: Color::Rgb(31, 35, 40),
    footer_key_bg: Color::Rgb(235, 238, 242),
    footer_key_fg: Color::Rgb(9, 105, 218),
    footer_desc_fg: Color::Rgb(31, 35, 40),
};

pub const THEME_HORIZON: Theme = Theme {
    name: "horizon",
    display_name: "Horizon",
    primary: Color::Rgb(233, 92, 108),        // Accent Coral
    secondary: Color::Rgb(176, 134, 247),     // Lavender
    accent: Color::Rgb(38, 209, 207),         // Cyan
    muted: Color::Rgb(108, 109, 120),
    border: Color::Rgb(233, 92, 108),
    border_active: Color::Rgb(250, 180, 90),
    banner_start: Color::Rgb(233, 92, 108),
    banner_end: Color::Rgb(250, 180, 90),
    header_fg: Color::Rgb(249, 249, 247),
    table_header: Color::Rgb(38, 209, 207),
    host_name: Color::Rgb(233, 92, 108),
    aliases: Color::Rgb(176, 134, 247),
    user: Color::Rgb(41, 211, 152),           // Neon Green
    destination: Color::Rgb(38, 209, 207),
    port: Color::Rgb(250, 180, 90),           // Sun Gold
    proxy: Color::Rgb(240, 117, 130),
    selected_fg: Color::Rgb(28, 30, 38),
    selected_bg: Color::Rgb(233, 92, 108),
    cursor_symbol_fg: Color::Rgb(250, 180, 90),
    badge_bg: Color::Rgb(35, 38, 48),
    badge_fg: Color::Rgb(233, 92, 108),
    search_icon_fg: Color::Rgb(250, 180, 90),
    search_border: Color::Rgb(233, 92, 108),
    search_text_fg: Color::Rgb(249, 249, 247),
    footer_key_bg: Color::Rgb(35, 38, 48),
    footer_key_fg: Color::Rgb(233, 92, 108),
    footer_desc_fg: Color::Rgb(249, 249, 247),
};

pub const THEME_POIMANDRES: Theme = Theme {
    name: "poimandres",
    display_name: "Poimandres",
    primary: Color::Rgb(93, 228, 199),        // Mint
    secondary: Color::Rgb(186, 174, 242),     // Soft Violet
    accent: Color::Rgb(137, 221, 255),        // Pale Blue
    muted: Color::Rgb(115, 125, 140),
    border: Color::Rgb(93, 228, 199),
    border_active: Color::Rgb(137, 221, 255),
    banner_start: Color::Rgb(93, 228, 199),
    banner_end: Color::Rgb(137, 221, 255),
    header_fg: Color::Rgb(228, 240, 251),
    table_header: Color::Rgb(137, 221, 255),
    host_name: Color::Rgb(93, 228, 199),
    aliases: Color::Rgb(186, 174, 242),
    user: Color::Rgb(93, 228, 199),
    destination: Color::Rgb(137, 221, 255),
    port: Color::Rgb(255, 202, 123),          // Amber
    proxy: Color::Rgb(208, 103, 157),
    selected_fg: Color::Rgb(27, 29, 35),
    selected_bg: Color::Rgb(93, 228, 199),
    cursor_symbol_fg: Color::Rgb(255, 202, 123),
    badge_bg: Color::Rgb(37, 40, 48),
    badge_fg: Color::Rgb(93, 228, 199),
    search_icon_fg: Color::Rgb(255, 202, 123),
    search_border: Color::Rgb(93, 228, 199),
    search_text_fg: Color::Rgb(228, 240, 251),
    footer_key_bg: Color::Rgb(37, 40, 48),
    footer_key_fg: Color::Rgb(93, 228, 199),
    footer_desc_fg: Color::Rgb(228, 240, 251),
};

pub const THEME_VESPER: Theme = Theme {
    name: "vesper",
    display_name: "Vesper",
    primary: Color::Rgb(255, 199, 153),       // Peach Orange
    secondary: Color::Rgb(255, 199, 119),     // Amber
    accent: Color::Rgb(153, 187, 255),        // Sky
    muted: Color::Rgb(90, 90, 90),
    border: Color::Rgb(255, 199, 153),
    border_active: Color::Rgb(255, 199, 119),
    banner_start: Color::Rgb(255, 199, 153),
    banner_end: Color::Rgb(255, 199, 119),
    header_fg: Color::Rgb(255, 255, 255),
    table_header: Color::Rgb(255, 199, 119),
    host_name: Color::Rgb(255, 199, 153),
    aliases: Color::Rgb(153, 187, 255),
    user: Color::Rgb(153, 238, 187),          // Light Green
    destination: Color::Rgb(153, 187, 255),
    port: Color::Rgb(255, 199, 119),
    proxy: Color::Rgb(255, 153, 153),
    selected_fg: Color::Rgb(16, 16, 16),
    selected_bg: Color::Rgb(255, 199, 153),
    cursor_symbol_fg: Color::Rgb(255, 199, 119),
    badge_bg: Color::Rgb(35, 35, 35),
    badge_fg: Color::Rgb(255, 199, 153),
    search_icon_fg: Color::Rgb(255, 199, 119),
    search_border: Color::Rgb(255, 199, 153),
    search_text_fg: Color::Rgb(255, 255, 255),
    footer_key_bg: Color::Rgb(35, 35, 35),
    footer_key_fg: Color::Rgb(255, 199, 153),
    footer_desc_fg: Color::Rgb(255, 255, 255),
};

pub const THEME_NIGHT_OWL: Theme = Theme {
    name: "night-owl",
    display_name: "Night Owl",
    primary: Color::Rgb(127, 219, 202),       // Cyan
    secondary: Color::Rgb(199, 146, 234),     // Purple
    accent: Color::Rgb(236, 196, 141),        // Gold
    muted: Color::Rgb(95, 126, 155),
    border: Color::Rgb(127, 219, 202),
    border_active: Color::Rgb(199, 146, 234),
    banner_start: Color::Rgb(127, 219, 202),
    banner_end: Color::Rgb(199, 146, 234),
    header_fg: Color::Rgb(214, 222, 235),
    table_header: Color::Rgb(127, 219, 202),
    host_name: Color::Rgb(127, 219, 202),
    aliases: Color::Rgb(199, 146, 234),
    user: Color::Rgb(173, 219, 103),          // Olive Green
    destination: Color::Rgb(130, 170, 255),   // Blue
    port: Color::Rgb(247, 140, 108),          // Orange
    proxy: Color::Rgb(239, 83, 80),           // Red
    selected_fg: Color::Rgb(1, 22, 39),
    selected_bg: Color::Rgb(127, 219, 202),
    cursor_symbol_fg: Color::Rgb(236, 196, 141),
    badge_bg: Color::Rgb(10, 40, 65),
    badge_fg: Color::Rgb(127, 219, 202),
    search_icon_fg: Color::Rgb(236, 196, 141),
    search_border: Color::Rgb(127, 219, 202),
    search_text_fg: Color::Rgb(214, 222, 235),
    footer_key_bg: Color::Rgb(10, 40, 65),
    footer_key_fg: Color::Rgb(127, 219, 202),
    footer_desc_fg: Color::Rgb(214, 222, 235),
};

pub const THEME_COBALT2: Theme = Theme {
    name: "cobalt2",
    display_name: "Cobalt2",
    primary: Color::Rgb(255, 196, 0),         // Cobalt Yellow
    secondary: Color::Rgb(0, 136, 255),       // Bright Blue
    accent: Color::Rgb(0, 225, 255),          // Cyan
    muted: Color::Rgb(80, 110, 140),
    border: Color::Rgb(255, 196, 0),
    border_active: Color::Rgb(0, 225, 255),
    banner_start: Color::Rgb(255, 196, 0),
    banner_end: Color::Rgb(0, 136, 255),
    header_fg: Color::Rgb(255, 255, 255),
    table_header: Color::Rgb(0, 225, 255),
    host_name: Color::Rgb(255, 196, 0),
    aliases: Color::Rgb(0, 225, 255),
    user: Color::Rgb(58, 217, 0),             // Electric Green
    destination: Color::Rgb(102, 178, 255),
    port: Color::Rgb(255, 157, 0),            // Orange
    proxy: Color::Rgb(255, 0, 90),            // Magenta
    selected_fg: Color::Rgb(20, 40, 60),
    selected_bg: Color::Rgb(255, 196, 0),
    cursor_symbol_fg: Color::Rgb(0, 225, 255),
    badge_bg: Color::Rgb(25, 53, 78),
    badge_fg: Color::Rgb(255, 196, 0),
    search_icon_fg: Color::Rgb(255, 196, 0),
    search_border: Color::Rgb(255, 196, 0),
    search_text_fg: Color::Rgb(255, 255, 255),
    footer_key_bg: Color::Rgb(25, 53, 78),
    footer_key_fg: Color::Rgb(255, 196, 0),
    footer_desc_fg: Color::Rgb(255, 255, 255),
};

pub const THEME_PALENIGHT: Theme = Theme {
    name: "palenight",
    display_name: "Palenight",
    primary: Color::Rgb(199, 146, 234),       // Lavender
    secondary: Color::Rgb(137, 221, 255),     // Sky Cyan
    accent: Color::Rgb(255, 203, 107),        // Amber
    muted: Color::Rgb(103, 110, 149),
    border: Color::Rgb(199, 146, 234),
    border_active: Color::Rgb(137, 221, 255),
    banner_start: Color::Rgb(199, 146, 234),
    banner_end: Color::Rgb(137, 221, 255),
    header_fg: Color::Rgb(191, 199, 213),
    table_header: Color::Rgb(137, 221, 255),
    host_name: Color::Rgb(199, 146, 234),
    aliases: Color::Rgb(137, 221, 255),
    user: Color::Rgb(195, 232, 141),          // Mint
    destination: Color::Rgb(130, 170, 255),
    port: Color::Rgb(247, 140, 108),          // Peach
    proxy: Color::Rgb(255, 83, 112),          // Coral
    selected_fg: Color::Rgb(41, 45, 62),
    selected_bg: Color::Rgb(199, 146, 234),
    cursor_symbol_fg: Color::Rgb(255, 203, 107),
    badge_bg: Color::Rgb(51, 55, 78),
    badge_fg: Color::Rgb(199, 146, 234),
    search_icon_fg: Color::Rgb(255, 203, 107),
    search_border: Color::Rgb(199, 146, 234),
    search_text_fg: Color::Rgb(191, 199, 213),
    footer_key_bg: Color::Rgb(51, 55, 78),
    footer_key_fg: Color::Rgb(199, 146, 234),
    footer_desc_fg: Color::Rgb(191, 199, 213),
};

pub const THEME_LASERWAVE: Theme = Theme {
    name: "laserwave",
    display_name: "LaserWave",
    primary: Color::Rgb(235, 100, 160),       // Hot Pink
    secondary: Color::Rgb(116, 238, 238),     // Neon Cyan
    accent: Color::Rgb(180, 140, 255),        // Violet
    muted: Color::Rgb(110, 95, 140),
    border: Color::Rgb(235, 100, 160),
    border_active: Color::Rgb(116, 238, 238),
    banner_start: Color::Rgb(235, 100, 160),
    banner_end: Color::Rgb(116, 238, 238),
    header_fg: Color::Rgb(255, 255, 255),
    table_header: Color::Rgb(116, 238, 238),
    host_name: Color::Rgb(235, 100, 160),
    aliases: Color::Rgb(180, 140, 255),
    user: Color::Rgb(180, 255, 150),          // Neon Lime
    destination: Color::Rgb(116, 238, 238),
    port: Color::Rgb(255, 230, 109),          // Yellow
    proxy: Color::Rgb(255, 120, 120),
    selected_fg: Color::Rgb(27, 24, 41),
    selected_bg: Color::Rgb(235, 100, 160),
    cursor_symbol_fg: Color::Rgb(255, 230, 109),
    badge_bg: Color::Rgb(45, 35, 65),
    badge_fg: Color::Rgb(235, 100, 160),
    search_icon_fg: Color::Rgb(116, 238, 238),
    search_border: Color::Rgb(235, 100, 160),
    search_text_fg: Color::Rgb(255, 255, 255),
    footer_key_bg: Color::Rgb(45, 35, 65),
    footer_key_fg: Color::Rgb(235, 100, 160),
    footer_desc_fg: Color::Rgb(255, 255, 255),
};

pub const THEME_SHADES_OF_PURPLE: Theme = Theme {
    name: "shades-of-purple",
    display_name: "Shades of Purple",
    primary: Color::Rgb(250, 208, 0),         // Super Yellow
    secondary: Color::Rgb(179, 98, 255),      // Purple
    accent: Color::Rgb(0, 241, 255),          // Cyan
    muted: Color::Rgb(130, 120, 180),
    border: Color::Rgb(250, 208, 0),
    border_active: Color::Rgb(0, 241, 255),
    banner_start: Color::Rgb(250, 208, 0),
    banner_end: Color::Rgb(179, 98, 255),
    header_fg: Color::Rgb(255, 255, 255),
    table_header: Color::Rgb(0, 241, 255),
    host_name: Color::Rgb(250, 208, 0),
    aliases: Color::Rgb(179, 98, 255),
    user: Color::Rgb(165, 255, 18),           // Neon Green
    destination: Color::Rgb(0, 241, 255),
    port: Color::Rgb(255, 157, 0),            // Orange
    proxy: Color::Rgb(255, 98, 142),          // Pink
    selected_fg: Color::Rgb(45, 43, 85),
    selected_bg: Color::Rgb(250, 208, 0),
    cursor_symbol_fg: Color::Rgb(0, 241, 255),
    badge_bg: Color::Rgb(55, 50, 105),
    badge_fg: Color::Rgb(250, 208, 0),
    search_icon_fg: Color::Rgb(250, 208, 0),
    search_border: Color::Rgb(250, 208, 0),
    search_text_fg: Color::Rgb(255, 255, 255),
    footer_key_bg: Color::Rgb(55, 50, 105),
    footer_key_fg: Color::Rgb(250, 208, 0),
    footer_desc_fg: Color::Rgb(255, 255, 255),
};

pub const THEME_AURA: Theme = Theme {
    name: "aura",
    display_name: "Aura",
    primary: Color::Rgb(162, 119, 255),       // Purple
    secondary: Color::Rgb(97, 255, 202),      // Green
    accent: Color::Rgb(246, 152, 237),        // Pink
    muted: Color::Rgb(110, 100, 130),
    border: Color::Rgb(162, 119, 255),
    border_active: Color::Rgb(97, 255, 202),
    banner_start: Color::Rgb(162, 119, 255),
    banner_end: Color::Rgb(97, 255, 202),
    header_fg: Color::Rgb(237, 236, 238),
    table_header: Color::Rgb(97, 255, 202),
    host_name: Color::Rgb(162, 119, 255),
    aliases: Color::Rgb(246, 152, 237),
    user: Color::Rgb(97, 255, 202),
    destination: Color::Rgb(108, 224, 255),   // Cyan
    port: Color::Rgb(255, 202, 133),          // Orange
    proxy: Color::Rgb(255, 103, 103),         // Red
    selected_fg: Color::Rgb(21, 20, 27),
    selected_bg: Color::Rgb(162, 119, 255),
    cursor_symbol_fg: Color::Rgb(97, 255, 202),
    badge_bg: Color::Rgb(35, 30, 48),
    badge_fg: Color::Rgb(162, 119, 255),
    search_icon_fg: Color::Rgb(97, 255, 202),
    search_border: Color::Rgb(162, 119, 255),
    search_text_fg: Color::Rgb(237, 236, 238),
    footer_key_bg: Color::Rgb(35, 30, 48),
    footer_key_fg: Color::Rgb(162, 119, 255),
    footer_desc_fg: Color::Rgb(237, 236, 238),
};

pub const THEME_MOONLIGHT: Theme = Theme {
    name: "moonlight",
    display_name: "Moonlight",
    primary: Color::Rgb(68, 241, 233),        // Turquoise
    secondary: Color::Rgb(192, 153, 255),     // Violet
    accent: Color::Rgb(134, 225, 254),        // Sky
    muted: Color::Rgb(90, 100, 140),
    border: Color::Rgb(68, 241, 233),
    border_active: Color::Rgb(192, 153, 255),
    banner_start: Color::Rgb(68, 241, 233),
    banner_end: Color::Rgb(192, 153, 255),
    header_fg: Color::Rgb(192, 202, 245),
    table_header: Color::Rgb(134, 225, 254),
    host_name: Color::Rgb(68, 241, 233),
    aliases: Color::Rgb(192, 153, 255),
    user: Color::Rgb(139, 233, 253),
    destination: Color::Rgb(134, 225, 254),
    port: Color::Rgb(255, 201, 107),          // Gold
    proxy: Color::Rgb(255, 117, 127),         // Coral
    selected_fg: Color::Rgb(33, 35, 54),
    selected_bg: Color::Rgb(68, 241, 233),
    cursor_symbol_fg: Color::Rgb(255, 201, 107),
    badge_bg: Color::Rgb(40, 45, 70),
    badge_fg: Color::Rgb(68, 241, 233),
    search_icon_fg: Color::Rgb(68, 241, 233),
    search_border: Color::Rgb(68, 241, 233),
    search_text_fg: Color::Rgb(192, 202, 245),
    footer_key_bg: Color::Rgb(40, 45, 70),
    footer_key_fg: Color::Rgb(68, 241, 233),
    footer_desc_fg: Color::Rgb(192, 202, 245),
};

pub const THEME_OXOCARBON: Theme = Theme {
    name: "oxocarbon",
    display_name: "Oxocarbon",
    primary: Color::Rgb(255, 126, 182),       // Pink
    secondary: Color::Rgb(61, 219, 217),      // Cyan
    accent: Color::Rgb(190, 149, 255),        // Purple
    muted: Color::Rgb(82, 82, 82),
    border: Color::Rgb(255, 126, 182),
    border_active: Color::Rgb(61, 219, 217),
    banner_start: Color::Rgb(255, 126, 182),
    banner_end: Color::Rgb(61, 219, 217),
    header_fg: Color::Rgb(244, 244, 244),
    table_header: Color::Rgb(61, 219, 217),
    host_name: Color::Rgb(255, 126, 182),
    aliases: Color::Rgb(190, 149, 255),
    user: Color::Rgb(66, 190, 101),           // Turquoise Green
    destination: Color::Rgb(61, 219, 217),
    port: Color::Rgb(255, 180, 100),
    proxy: Color::Rgb(238, 83, 150),
    selected_fg: Color::Rgb(22, 22, 22),
    selected_bg: Color::Rgb(255, 126, 182),
    cursor_symbol_fg: Color::Rgb(61, 219, 217),
    badge_bg: Color::Rgb(38, 38, 38),
    badge_fg: Color::Rgb(255, 126, 182),
    search_icon_fg: Color::Rgb(61, 219, 217),
    search_border: Color::Rgb(255, 126, 182),
    search_text_fg: Color::Rgb(244, 244, 244),
    footer_key_bg: Color::Rgb(38, 38, 38),
    footer_key_fg: Color::Rgb(255, 126, 182),
    footer_desc_fg: Color::Rgb(244, 244, 244),
};

pub const THEME_FLEXOKI_DARK: Theme = Theme {
    name: "flexoki-dark",
    display_name: "Flexoki Dark",
    primary: Color::Rgb(218, 112, 44),        // Orange
    secondary: Color::Rgb(135, 154, 57),      // Green
    accent: Color::Rgb(36, 131, 123),         // Cyan
    muted: Color::Rgb(102, 100, 94),
    border: Color::Rgb(218, 112, 44),
    border_active: Color::Rgb(135, 154, 57),
    banner_start: Color::Rgb(218, 112, 44),
    banner_end: Color::Rgb(135, 154, 57),
    header_fg: Color::Rgb(206, 205, 195),
    table_header: Color::Rgb(36, 131, 123),
    host_name: Color::Rgb(218, 112, 44),
    aliases: Color::Rgb(139, 125, 178),       // Purple
    user: Color::Rgb(135, 154, 57),
    destination: Color::Rgb(67, 133, 190),    // Blue
    port: Color::Rgb(208, 162, 21),           // Yellow
    proxy: Color::Rgb(209, 77, 65),           // Red
    selected_fg: Color::Rgb(16, 15, 15),
    selected_bg: Color::Rgb(218, 112, 44),
    cursor_symbol_fg: Color::Rgb(208, 162, 21),
    badge_bg: Color::Rgb(40, 39, 38),
    badge_fg: Color::Rgb(218, 112, 44),
    search_icon_fg: Color::Rgb(208, 162, 21),
    search_border: Color::Rgb(218, 112, 44),
    search_text_fg: Color::Rgb(206, 205, 195),
    footer_key_bg: Color::Rgb(40, 39, 38),
    footer_key_fg: Color::Rgb(218, 112, 44),
    footer_desc_fg: Color::Rgb(206, 205, 195),
};

pub const THEME_FLEXOKI_LIGHT: Theme = Theme {
    name: "flexoki-light",
    display_name: "Flexoki Light",
    primary: Color::Rgb(188, 82, 21),         // Orange
    secondary: Color::Rgb(102, 128, 11),      // Green
    accent: Color::Rgb(36, 131, 123),         // Cyan
    muted: Color::Rgb(135, 133, 128),
    border: Color::Rgb(188, 82, 21),
    border_active: Color::Rgb(102, 128, 11),
    banner_start: Color::Rgb(188, 82, 21),
    banner_end: Color::Rgb(102, 128, 11),
    header_fg: Color::Rgb(16, 15, 15),
    table_header: Color::Rgb(36, 131, 123),
    host_name: Color::Rgb(188, 82, 21),
    aliases: Color::Rgb(94, 64, 157),         // Purple
    user: Color::Rgb(102, 128, 11),
    destination: Color::Rgb(32, 94, 166),     // Blue
    port: Color::Rgb(173, 131, 1),            // Yellow
    proxy: Color::Rgb(175, 48, 41),           // Red
    selected_fg: Color::Rgb(255, 252, 240),
    selected_bg: Color::Rgb(188, 82, 21),
    cursor_symbol_fg: Color::Rgb(173, 131, 1),
    badge_bg: Color::Rgb(242, 238, 223),
    badge_fg: Color::Rgb(188, 82, 21),
    search_icon_fg: Color::Rgb(173, 131, 1),
    search_border: Color::Rgb(188, 82, 21),
    search_text_fg: Color::Rgb(16, 15, 15),
    footer_key_bg: Color::Rgb(242, 238, 223),
    footer_key_fg: Color::Rgb(188, 82, 21),
    footer_desc_fg: Color::Rgb(16, 15, 15),
};

pub const THEME_ZENBURN: Theme = Theme {
    name: "zenburn",
    display_name: "Zenburn",
    primary: Color::Rgb(240, 223, 175),       // Khaki Yellow
    secondary: Color::Rgb(140, 208, 211),     // Sky
    accent: Color::Rgb(127, 159, 127),        // Sage
    muted: Color::Rgb(127, 127, 127),
    border: Color::Rgb(240, 223, 175),
    border_active: Color::Rgb(140, 208, 211),
    banner_start: Color::Rgb(240, 223, 175),
    banner_end: Color::Rgb(140, 208, 211),
    header_fg: Color::Rgb(220, 220, 204),
    table_header: Color::Rgb(140, 208, 211),
    host_name: Color::Rgb(240, 223, 175),
    aliases: Color::Rgb(220, 163, 163),       // Rose
    user: Color::Rgb(127, 159, 127),
    destination: Color::Rgb(140, 208, 211),
    port: Color::Rgb(223, 175, 143),          // Orange
    proxy: Color::Rgb(204, 147, 147),
    selected_fg: Color::Rgb(47, 47, 47),
    selected_bg: Color::Rgb(240, 223, 175),
    cursor_symbol_fg: Color::Rgb(223, 175, 143),
    badge_bg: Color::Rgb(79, 79, 79),
    badge_fg: Color::Rgb(240, 223, 175),
    search_icon_fg: Color::Rgb(240, 223, 175),
    search_border: Color::Rgb(240, 223, 175),
    search_text_fg: Color::Rgb(220, 220, 204),
    footer_key_bg: Color::Rgb(79, 79, 79),
    footer_key_fg: Color::Rgb(240, 223, 175),
    footer_desc_fg: Color::Rgb(220, 220, 204),
};

pub const THEME_DOOM_ONE: Theme = Theme {
    name: "doom-one",
    display_name: "Doom One",
    primary: Color::Rgb(81, 175, 239),        // Blue
    secondary: Color::Rgb(198, 120, 221),     // Magenta
    accent: Color::Rgb(70, 209, 217),         // Cyan
    muted: Color::Rgb(91, 98, 114),
    border: Color::Rgb(81, 175, 239),
    border_active: Color::Rgb(198, 120, 221),
    banner_start: Color::Rgb(81, 175, 239),
    banner_end: Color::Rgb(198, 120, 221),
    header_fg: Color::Rgb(187, 194, 207),
    table_header: Color::Rgb(70, 209, 217),
    host_name: Color::Rgb(81, 175, 239),
    aliases: Color::Rgb(198, 120, 221),
    user: Color::Rgb(152, 190, 101),          // Green
    destination: Color::Rgb(70, 209, 217),
    port: Color::Rgb(218, 133, 74),           // Orange
    proxy: Color::Rgb(255, 108, 136),         // Red
    selected_fg: Color::Rgb(40, 44, 52),
    selected_bg: Color::Rgb(81, 175, 239),
    cursor_symbol_fg: Color::Rgb(236, 190, 123),
    badge_bg: Color::Rgb(45, 52, 64),
    badge_fg: Color::Rgb(81, 175, 239),
    search_icon_fg: Color::Rgb(236, 190, 123),
    search_border: Color::Rgb(81, 175, 239),
    search_text_fg: Color::Rgb(187, 194, 207),
    footer_key_bg: Color::Rgb(45, 52, 64),
    footer_key_fg: Color::Rgb(81, 175, 239),
    footer_desc_fg: Color::Rgb(187, 194, 207),
};

pub const THEME_AQUARIUM: Theme = Theme {
    name: "aquarium",
    display_name: "Aquarium",
    primary: Color::Rgb(99, 181, 234),        // Ocean Blue
    secondary: Color::Rgb(184, 156, 219),     // Orchid
    accent: Color::Rgb(164, 214, 180),        // Seafoam
    muted: Color::Rgb(99, 110, 135),
    border: Color::Rgb(99, 181, 234),
    border_active: Color::Rgb(184, 156, 219),
    banner_start: Color::Rgb(99, 181, 234),
    banner_end: Color::Rgb(184, 156, 219),
    header_fg: Color::Rgb(235, 240, 245),
    table_header: Color::Rgb(164, 214, 180),
    host_name: Color::Rgb(99, 181, 234),
    aliases: Color::Rgb(184, 156, 219),
    user: Color::Rgb(164, 214, 180),
    destination: Color::Rgb(99, 181, 234),
    port: Color::Rgb(230, 197, 140),          // Sunlight
    proxy: Color::Rgb(235, 142, 154),         // Coral
    selected_fg: Color::Rgb(32, 37, 51),
    selected_bg: Color::Rgb(99, 181, 234),
    cursor_symbol_fg: Color::Rgb(230, 197, 140),
    badge_bg: Color::Rgb(45, 52, 72),
    badge_fg: Color::Rgb(99, 181, 234),
    search_icon_fg: Color::Rgb(230, 197, 140),
    search_border: Color::Rgb(99, 181, 234),
    search_text_fg: Color::Rgb(235, 240, 245),
    footer_key_bg: Color::Rgb(45, 52, 72),
    footer_key_fg: Color::Rgb(99, 181, 234),
    footer_desc_fg: Color::Rgb(235, 240, 245),
};

pub const THEME_GRUVBOX_LIGHT: Theme = Theme {
    name: "gruvbox-light",
    display_name: "Gruvbox Light",
    primary: Color::Rgb(175, 58, 3),          // Rust Orange
    secondary: Color::Rgb(143, 63, 113),      // Purple
    accent: Color::Rgb(69, 133, 136),         // Blue
    muted: Color::Rgb(146, 131, 116),
    border: Color::Rgb(175, 58, 3),
    border_active: Color::Rgb(181, 118, 20),
    banner_start: Color::Rgb(175, 58, 3),
    banner_end: Color::Rgb(181, 118, 20),
    header_fg: Color::Rgb(60, 56, 54),
    table_header: Color::Rgb(69, 133, 136),
    host_name: Color::Rgb(175, 58, 3),
    aliases: Color::Rgb(143, 63, 113),
    user: Color::Rgb(121, 116, 14),           // Green
    destination: Color::Rgb(69, 133, 136),
    port: Color::Rgb(181, 118, 20),           // Yellow
    proxy: Color::Rgb(157, 0, 6),             // Red
    selected_fg: Color::Rgb(251, 241, 199),
    selected_bg: Color::Rgb(175, 58, 3),
    cursor_symbol_fg: Color::Rgb(181, 118, 20),
    badge_bg: Color::Rgb(235, 219, 178),
    badge_fg: Color::Rgb(175, 58, 3),
    search_icon_fg: Color::Rgb(181, 118, 20),
    search_border: Color::Rgb(175, 58, 3),
    search_text_fg: Color::Rgb(60, 56, 54),
    footer_key_bg: Color::Rgb(235, 219, 178),
    footer_key_fg: Color::Rgb(175, 58, 3),
    footer_desc_fg: Color::Rgb(60, 56, 54),
};

pub const THEME_SOLARIZED_LIGHT: Theme = Theme {
    name: "solarized-light",
    display_name: "Solarized Light",
    primary: Color::Rgb(38, 139, 210),        // Blue
    secondary: Color::Rgb(211, 54, 130),      // Magenta
    accent: Color::Rgb(42, 161, 152),         // Cyan
    muted: Color::Rgb(147, 161, 161),
    border: Color::Rgb(38, 139, 210),
    border_active: Color::Rgb(211, 54, 130),
    banner_start: Color::Rgb(38, 139, 210),
    banner_end: Color::Rgb(42, 161, 152),
    header_fg: Color::Rgb(101, 123, 131),
    table_header: Color::Rgb(42, 161, 152),
    host_name: Color::Rgb(38, 139, 210),
    aliases: Color::Rgb(211, 54, 130),
    user: Color::Rgb(133, 153, 0),            // Green
    destination: Color::Rgb(42, 161, 152),
    port: Color::Rgb(181, 137, 0),            // Yellow
    proxy: Color::Rgb(203, 75, 22),           // Orange
    selected_fg: Color::Rgb(253, 246, 227),
    selected_bg: Color::Rgb(38, 139, 210),
    cursor_symbol_fg: Color::Rgb(181, 137, 0),
    badge_bg: Color::Rgb(238, 232, 213),
    badge_fg: Color::Rgb(38, 139, 210),
    search_icon_fg: Color::Rgb(181, 137, 0),
    search_border: Color::Rgb(38, 139, 210),
    search_text_fg: Color::Rgb(101, 123, 131),
    footer_key_bg: Color::Rgb(238, 232, 213),
    footer_key_fg: Color::Rgb(38, 139, 210),
    footer_desc_fg: Color::Rgb(101, 123, 131),
};

pub const THEME_MATERIAL_OCEAN: Theme = Theme {
    name: "material-ocean",
    display_name: "Material Ocean",
    primary: Color::Rgb(130, 170, 255),       // Blue
    secondary: Color::Rgb(199, 146, 234),     // Purple
    accent: Color::Rgb(142, 255, 235),        // Neon Cyan
    muted: Color::Rgb(70, 75, 95),
    border: Color::Rgb(130, 170, 255),
    border_active: Color::Rgb(142, 255, 235),
    banner_start: Color::Rgb(130, 170, 255),
    banner_end: Color::Rgb(142, 255, 235),
    header_fg: Color::Rgb(238, 255, 255),
    table_header: Color::Rgb(142, 255, 235),
    host_name: Color::Rgb(130, 170, 255),
    aliases: Color::Rgb(199, 146, 234),
    user: Color::Rgb(195, 232, 141),          // Green
    destination: Color::Rgb(142, 255, 235),
    port: Color::Rgb(255, 203, 107),          // Yellow
    proxy: Color::Rgb(255, 83, 112),          // Coral
    selected_fg: Color::Rgb(15, 17, 26),
    selected_bg: Color::Rgb(130, 170, 255),
    cursor_symbol_fg: Color::Rgb(255, 203, 107),
    badge_bg: Color::Rgb(28, 32, 48),
    badge_fg: Color::Rgb(130, 170, 255),
    search_icon_fg: Color::Rgb(142, 255, 235),
    search_border: Color::Rgb(130, 170, 255),
    search_text_fg: Color::Rgb(238, 255, 255),
    footer_key_bg: Color::Rgb(28, 32, 48),
    footer_key_fg: Color::Rgb(130, 170, 255),
    footer_desc_fg: Color::Rgb(238, 255, 255),
};

pub const THEME_MATERIAL_DARKER: Theme = Theme {
    name: "material-darker",
    display_name: "Material Darker",
    primary: Color::Rgb(247, 140, 108),       // Orange
    secondary: Color::Rgb(137, 221, 255),     // Cyan
    accent: Color::Rgb(199, 146, 234),        // Purple
    muted: Color::Rgb(97, 97, 97),
    border: Color::Rgb(247, 140, 108),
    border_active: Color::Rgb(137, 221, 255),
    banner_start: Color::Rgb(247, 140, 108),
    banner_end: Color::Rgb(137, 221, 255),
    header_fg: Color::Rgb(238, 255, 255),
    table_header: Color::Rgb(137, 221, 255),
    host_name: Color::Rgb(247, 140, 108),
    aliases: Color::Rgb(199, 146, 234),
    user: Color::Rgb(195, 232, 141),          // Green
    destination: Color::Rgb(137, 221, 255),
    port: Color::Rgb(255, 203, 107),          // Yellow
    proxy: Color::Rgb(255, 83, 112),          // Red
    selected_fg: Color::Rgb(33, 33, 33),
    selected_bg: Color::Rgb(247, 140, 108),
    cursor_symbol_fg: Color::Rgb(255, 203, 107),
    badge_bg: Color::Rgb(48, 48, 48),
    badge_fg: Color::Rgb(247, 140, 108),
    search_icon_fg: Color::Rgb(255, 203, 107),
    search_border: Color::Rgb(247, 140, 108),
    search_text_fg: Color::Rgb(238, 255, 255),
    footer_key_bg: Color::Rgb(48, 48, 48),
    footer_key_fg: Color::Rgb(247, 140, 108),
    footer_desc_fg: Color::Rgb(238, 255, 255),
};

pub const THEME_BASE16_DARK: Theme = Theme {
    name: "base16",
    display_name: "Base16 Dark",
    primary: Color::Rgb(124, 175, 194),       // Blue
    secondary: Color::Rgb(161, 110, 167),     // Magenta
    accent: Color::Rgb(134, 193, 185),        // Cyan
    muted: Color::Rgb(88, 88, 88),
    border: Color::Rgb(124, 175, 194),
    border_active: Color::Rgb(161, 110, 167),
    banner_start: Color::Rgb(124, 175, 194),
    banner_end: Color::Rgb(161, 110, 167),
    header_fg: Color::Rgb(216, 216, 216),
    table_header: Color::Rgb(134, 193, 185),
    host_name: Color::Rgb(124, 175, 194),
    aliases: Color::Rgb(161, 110, 167),
    user: Color::Rgb(144, 169, 89),           // Green
    destination: Color::Rgb(134, 193, 185),
    port: Color::Rgb(244, 207, 142),          // Yellow
    proxy: Color::Rgb(171, 70, 66),           // Red
    selected_fg: Color::Rgb(24, 24, 24),
    selected_bg: Color::Rgb(124, 175, 194),
    cursor_symbol_fg: Color::Rgb(244, 207, 142),
    badge_bg: Color::Rgb(40, 40, 40),
    badge_fg: Color::Rgb(124, 175, 194),
    search_icon_fg: Color::Rgb(244, 207, 142),
    search_border: Color::Rgb(124, 175, 194),
    search_text_fg: Color::Rgb(216, 216, 216),
    footer_key_bg: Color::Rgb(40, 40, 40),
    footer_key_fg: Color::Rgb(124, 175, 194),
    footer_desc_fg: Color::Rgb(216, 216, 216),
};

pub const THEME_DRACULA_SOFT: Theme = Theme {
    name: "dracula-soft",
    display_name: "Dracula Soft",
    primary: Color::Rgb(189, 147, 249),       // Purple
    secondary: Color::Rgb(255, 121, 198),     // Pink
    accent: Color::Rgb(139, 233, 253),        // Cyan
    muted: Color::Rgb(98, 114, 164),
    border: Color::Rgb(189, 147, 249),
    border_active: Color::Rgb(255, 121, 198),
    banner_start: Color::Rgb(189, 147, 249),
    banner_end: Color::Rgb(255, 121, 198),
    header_fg: Color::Rgb(248, 248, 242),
    table_header: Color::Rgb(139, 233, 253),
    host_name: Color::Rgb(189, 147, 249),
    aliases: Color::Rgb(255, 121, 198),
    user: Color::Rgb(80, 250, 123),           // Green
    destination: Color::Rgb(139, 233, 253),
    port: Color::Rgb(241, 250, 140),          // Yellow
    proxy: Color::Rgb(255, 184, 108),         // Orange
    selected_fg: Color::Rgb(40, 42, 54),
    selected_bg: Color::Rgb(189, 147, 249),
    cursor_symbol_fg: Color::Rgb(241, 250, 140),
    badge_bg: Color::Rgb(55, 58, 75),
    badge_fg: Color::Rgb(189, 147, 249),
    search_icon_fg: Color::Rgb(139, 233, 253),
    search_border: Color::Rgb(189, 147, 249),
    search_text_fg: Color::Rgb(248, 248, 242),
    footer_key_bg: Color::Rgb(55, 58, 75),
    footer_key_fg: Color::Rgb(189, 147, 249),
    footer_desc_fg: Color::Rgb(248, 248, 242),
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
    "catppuccin-macchiato",
    "catppuccin-frappe",
    "catppuccin-latte",
    "dracula",
    "dracula-soft",
    "tokyonight",
    "tokyonight-storm",
    "tokyonight-light",
    "nord",
    "gruvbox",
    "gruvbox-light",
    "rose-pine",
    "rose-pine-moon",
    "rose-pine-dawn",
    "onedark",
    "kanagawa",
    "everforest",
    "solarized-dark",
    "solarized-light",
    "ayu-dark",
    "github-dark",
    "github-light",
    "horizon",
    "poimandres",
    "vesper",
    "night-owl",
    "cobalt2",
    "palenight",
    "laserwave",
    "shades-of-purple",
    "aura",
    "moonlight",
    "oxocarbon",
    "flexoki-dark",
    "flexoki-light",
    "zenburn",
    "doom-one",
    "aquarium",
    "material-ocean",
    "material-darker",
    "base16",
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
        "catppuccin-macchiato" | "macchiato" => Ok(THEME_CATPPUCCIN_MACCHIATO),
        "catppuccin-frappe" | "frappe" => Ok(THEME_CATPPUCCIN_FRAPPE),
        "catppuccin-latte" | "latte" => Ok(THEME_CATPPUCCIN_LATTE),
        "dracula" => Ok(THEME_DRACULA),
        "dracula-soft" => Ok(THEME_DRACULA_SOFT),
        "tokyonight" | "tokyo-night" | "tokyo" => Ok(THEME_TOKYONIGHT),
        "tokyonight-storm" | "tokyo-storm" => Ok(THEME_TOKYONIGHT_STORM),
        "tokyonight-light" | "tokyo-light" => Ok(THEME_TOKYONIGHT_LIGHT),
        "nord" => Ok(THEME_NORD),
        "gruvbox" | "gruvbox-dark" => Ok(THEME_GRUVBOX),
        "gruvbox-light" => Ok(THEME_GRUVBOX_LIGHT),
        "rose-pine" | "rosepine" => Ok(THEME_ROSE_PINE),
        "rose-pine-moon" | "rosepine-moon" => Ok(THEME_ROSE_PINE_MOON),
        "rose-pine-dawn" | "rosepine-dawn" => Ok(THEME_ROSE_PINE_DAWN),
        "onedark" | "one-dark" => Ok(THEME_ONEDARK),
        "kanagawa" => Ok(THEME_KANAGAWA),
        "everforest" => Ok(THEME_EVERFOREST),
        "solarized" | "solarized-dark" => Ok(THEME_SOLARIZED_DARK),
        "solarized-light" => Ok(THEME_SOLARIZED_LIGHT),
        "ayu" | "ayu-dark" => Ok(THEME_AYU_DARK),
        "github-dark" | "github" => Ok(THEME_GITHUB_DARK),
        "github-light" => Ok(THEME_GITHUB_LIGHT),
        "horizon" => Ok(THEME_HORIZON),
        "poimandres" => Ok(THEME_POIMANDRES),
        "vesper" => Ok(THEME_VESPER),
        "night-owl" | "nightowl" => Ok(THEME_NIGHT_OWL),
        "cobalt2" | "cobalt" => Ok(THEME_COBALT2),
        "palenight" => Ok(THEME_PALENIGHT),
        "laserwave" => Ok(THEME_LASERWAVE),
        "shades-of-purple" | "purple-shades" => Ok(THEME_SHADES_OF_PURPLE),
        "aura" => Ok(THEME_AURA),
        "moonlight" => Ok(THEME_MOONLIGHT),
        "oxocarbon" => Ok(THEME_OXOCARBON),
        "flexoki-dark" | "flexoki" => Ok(THEME_FLEXOKI_DARK),
        "flexoki-light" => Ok(THEME_FLEXOKI_LIGHT),
        "zenburn" => Ok(THEME_ZENBURN),
        "doom-one" | "doom" => Ok(THEME_DOOM_ONE),
        "aquarium" => Ok(THEME_AQUARIUM),
        "material-ocean" => Ok(THEME_MATERIAL_OCEAN),
        "material-darker" => Ok(THEME_MATERIAL_DARKER),
        "base16" | "base16-dark" => Ok(THEME_BASE16_DARK),
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
