use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};

use crate::theme::{interpolate_rgb, Theme};

/// Supported ASCII art styles for the header banner
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AsciiArtStyle {
    Slant,
    Cyber,
    Standard,
    Mini,
    Off,
}

impl std::str::FromStr for AsciiArtStyle {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "slant" => Self::Slant,
            "cyber" | "block" | "neon" => Self::Cyber,
            "standard" | "simple" | "retro" => Self::Standard,
            "mini" | "compact" | "small" => Self::Mini,
            "off" | "none" | "false" | "hide" => Self::Off,
            _ => Self::Slant,
        })
    }
}

impl AsciiArtStyle {
    /// Returns height required in terminal rows
    #[must_use]
    pub fn required_height(self) -> u16 {
        match self {
            Self::Slant | Self::Standard => 5,
            Self::Cyber => 6,
            Self::Mini => 1,
            Self::Off => 0,
        }
    }
}

// Braille Spinner Frames for smooth animations
const SPINNER_FRAMES: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

/// Returns the current spinner character frame
#[must_use]
pub fn get_spinner_frame(tick: u64) -> &'static str {
    let idx = (tick as usize) % SPINNER_FRAMES.len();
    SPINNER_FRAMES[idx]
}

// ASCII Banner Art Collections

const SLANT_BANNER: &[&str] = &[
    r"   ____ ___ / /_  _____",
    r"  / __// __// __ \/ ___/",
    r" _\ \ _\ \ / / / (__  ) ",
    r"/___//___//_/ /_/____/  ",
];

const CYBER_BANNER: &[&str] = &[
    r"  ██████╗ ██████╗ ██╗  ██╗███████╗",
    r" ██╔════╝██╔════╝ ██║  ██║██╔════╝",
    r" ╚█████╗ ╚█████╗  ███████║███████╗",
    r"  ╚═══██╗ ╚═══██╗ ██╔══██║╚════██║",
    r" ██████╔╝██████╔╝ ██║  ██║███████║",
];

const STANDARD_BANNER: &[&str] = &[
    r"  ___ ___| |_  ___ ",
    r" (_-</ _ \ ' \(_-< ",
    r" /__/\___/_||_/__/ ",
];

/// Generates styled lines with a horizontal RGB gradient and optional wave animation across the banner.
#[must_use]
pub fn render_banner_lines(style: AsciiArtStyle, theme: &Theme, phase_offset: f32) -> Vec<Line<'static>> {
    let lines = match style {
        AsciiArtStyle::Slant => SLANT_BANNER,
        AsciiArtStyle::Cyber => CYBER_BANNER,
        AsciiArtStyle::Standard => STANDARD_BANNER,
        AsciiArtStyle::Mini | AsciiArtStyle::Off => &[],
    };

    if lines.is_empty() {
        return Vec::new();
    }

    let max_len = lines.iter().map(|l| l.chars().count()).max().unwrap_or(1);

    lines
        .iter()
        .map(|line_str| {
            let mut spans = Vec::new();
            let char_count = line_str.chars().count();

            for (idx, ch) in line_str.chars().enumerate() {
                #[allow(clippy::cast_precision_loss)]
                let raw_factor = if max_len > 1 {
                    idx as f32 / max_len as f32
                } else {
                    0.0
                };

                // Continuous sinusoidal shimmer wave
                let factor = ((raw_factor + phase_offset) * 2.0 * std::f32::consts::PI).sin() * 0.5 + 0.5;
                let color = interpolate_rgb(theme.banner_start, theme.banner_end, factor);
                spans.push(Span::styled(
                    ch.to_string(),
                    Style::default().fg(color).add_modifier(Modifier::BOLD),
                ));
            }

            // Fill trailing padding if any
            if char_count < max_len {
                spans.push(Span::raw(" ".repeat(max_len - char_count)));
            }

            Line::from(spans)
        })
        .collect()
}

/// Generates compact single-line badge banner
#[must_use]
pub fn render_mini_banner(theme: &Theme, total_hosts: usize, filtered_hosts: usize, tick: u64, animate: bool) -> Line<'static> {
    let spinner = if animate {
        get_spinner_frame(tick)
    } else {
        "⚡"
    };

    Line::from(vec![
        Span::styled(
            format!(" {spinner} SSHS "),
            Style::default()
                .fg(theme.selected_fg)
                .bg(theme.primary)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" "),
        Span::styled(
            "SSH Client Terminal UI",
            Style::default().fg(theme.muted).add_modifier(Modifier::ITALIC),
        ),
        Span::raw(" ─── "),
        Span::styled(
            format!("[ {filtered_hosts}/{total_hosts} Hosts ]"),
            theme.badge_style(),
        ),
        Span::raw(" ─── "),
        Span::styled(
            format!("[ Theme: {} ]", theme.display_name),
            Style::default().fg(theme.accent).add_modifier(Modifier::BOLD),
        ),
    ])
}

/// ASCII Art for Empty Search State
#[must_use]
pub fn render_empty_state_lines(theme: &Theme, query: &str, tick: u64) -> Vec<Line<'static>> {
    let is_blinking = (tick / 20).is_multiple_of(6) && (tick % 20) < 3;
    let eyes_line = if is_blinking {
        r"     (-.-)"
    } else {
        r"     (o.o)"
    };

    let ghost = [
        r"      .-.",
        eyes_line,
        r"      |=|",
        r"     __|__",
        r"   //.=|=.\\",
    ];

    let mut result = Vec::new();
    result.push(Line::raw(""));

    for line in &ghost {
        result.push(
            Line::from(vec![
                Span::styled(
                    *line,
                    Style::default().fg(theme.accent).add_modifier(Modifier::BOLD),
                ),
            ])
            .centered(),
        );
    }

    result.push(Line::raw(""));
    result.push(
        Line::from(vec![
            Span::styled(
                "No SSH hosts found",
                Style::default().fg(theme.primary).add_modifier(Modifier::BOLD),
            ),
            if query.trim().is_empty() {
                Span::styled(" in your SSH configuration files.", Style::default().fg(theme.header_fg))
            } else {
                Span::styled(
                    format!(" matching \"{query}\""),
                    Style::default().fg(theme.secondary).add_modifier(Modifier::BOLD),
                )
            },
        ])
        .centered(),
    );

    result.push(
        Line::from(vec![
            Span::styled(
                "Tip: Press ",
                Style::default().fg(theme.muted),
            ),
            Span::styled(
                "Backspace",
                theme.key_badge_style(),
            ),
            Span::styled(
                " to clear search or check your ",
                Style::default().fg(theme.muted),
            ),
            Span::styled(
                "~/.ssh/config",
                Style::default().fg(theme.accent).add_modifier(Modifier::UNDERLINED),
            ),
        ])
        .centered(),
    );

    result
}

/// ASCII Art Header for Help Modal
#[must_use]
pub fn render_help_header(theme: &Theme) -> Vec<Line<'static>> {
    vec![
        Line::from(vec![
            Span::styled("   ____ ____  _   _ ____    _   _ _____ _     ____  ", Style::default().fg(theme.banner_start).add_modifier(Modifier::BOLD)),
        ]).centered(),
        Line::from(vec![
            Span::styled("  / ___/ ___|| | | / ___|  | | | | ____| |   |  _ \\ ", Style::default().fg(interpolate_rgb(theme.banner_start, theme.banner_end, 0.33)).add_modifier(Modifier::BOLD)),
        ]).centered(),
        Line::from(vec![
            Span::styled("  \\___ \\___ \\| |_| \\___ \\  | |_| |  _| | |   | |_) |", Style::default().fg(interpolate_rgb(theme.banner_start, theme.banner_end, 0.66)).add_modifier(Modifier::BOLD)),
        ]).centered(),
        Line::from(vec![
            Span::styled("   ___) |__) |  _  |___) | |  _  | |___| |___|  __/ ", Style::default().fg(theme.banner_end).add_modifier(Modifier::BOLD)),
        ]).centered(),
        Line::from(vec![
            Span::styled("  |____/____/|_| |_|____/  |_| |_|_____|_____|_|    ", Style::default().fg(theme.banner_end).add_modifier(Modifier::BOLD)),
        ]).centered(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::THEME_CATPPUCCIN;

    #[test]
    fn test_ascii_art_style_from_str() {
        use std::str::FromStr;
        assert_eq!(AsciiArtStyle::from_str("slant").unwrap(), AsciiArtStyle::Slant);
        assert_eq!(AsciiArtStyle::from_str("cyber").unwrap(), AsciiArtStyle::Cyber);
        assert_eq!(AsciiArtStyle::from_str("standard").unwrap(), AsciiArtStyle::Standard);
        assert_eq!(AsciiArtStyle::from_str("mini").unwrap(), AsciiArtStyle::Mini);
        assert_eq!(AsciiArtStyle::from_str("off").unwrap(), AsciiArtStyle::Off);
        assert_eq!(AsciiArtStyle::from_str("none").unwrap(), AsciiArtStyle::Off);
    }

    #[test]
    fn test_render_banner_lines() {
        let lines = render_banner_lines(AsciiArtStyle::Slant, &THEME_CATPPUCCIN, 0.0);
        assert_eq!(lines.len(), 4);

        let lines_cyber = render_banner_lines(AsciiArtStyle::Cyber, &THEME_CATPPUCCIN, 0.5);
        assert_eq!(lines_cyber.len(), 5);

        let lines_off = render_banner_lines(AsciiArtStyle::Off, &THEME_CATPPUCCIN, 0.0);
        assert!(lines_off.is_empty());
    }

    #[test]
    fn test_render_empty_state() {
        let lines = render_empty_state_lines(&THEME_CATPPUCCIN, "myserver", 0);
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_spinner_frames() {
        assert_eq!(get_spinner_frame(0), "⠋");
        assert_eq!(get_spinner_frame(1), "⠙");
    }
}
