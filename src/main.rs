pub mod ascii_art;
pub mod config;
pub mod searchable;
pub mod sftp_explorer;
pub mod ssh;
pub mod ssh_config;
#[cfg(test)]
mod test_support;
pub mod theme;
pub mod ui;

use anyhow::Result;
use clap::Parser;
use ui::{App, AppConfig};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[allow(clippy::struct_excessive_bools)]
struct Args {
    /// Path to the SSH configuration file
    #[arg(
        short,
        long,
        num_args = 1..,
        default_values_t = [
            "/etc/ssh/ssh_config".to_string(),
            "~/.ssh/config".to_string(),
        ],
    )]
    config: Vec<String>,

    /// Shows `ProxyCommand`
    #[arg(long, default_value_t = false)]
    show_proxy_command: bool,

    /// Host search filter
    #[arg(short, long)]
    search: Option<String>,

    /// Theme or color palette of the interface (e.g. catppuccin, dracula, tokyonight, nord, gruvbox, cyberpunk, synthwave, matrix, sunset, monokai, or any Tailwind color like blue, green, rose)
    #[arg(long, visible_alias = "theme", env = "SSHS_THEME")]
    color: Option<String>,

    /// ASCII art banner style (slant, cyber, standard, mini, off)
    #[arg(long, visible_alias = "banner", env = "SSHS_ASCII_ART")]
    ascii_art: Option<String>,

    /// Disable ASCII art banner and use compact mode
    #[arg(long, default_value_t = false)]
    no_ascii_art: bool,

    /// Disable visual animations (gradient wave, spinners)
    #[arg(long, default_value_t = false, env = "SSHS_NO_ANIMATE")]
    no_animate: bool,

    /// Sort hosts by hostname (default)
    #[arg(long, default_value_t = true)]
    sort: bool,

    /// Do not sort hosts, keep the configuration file order
    #[arg(long, default_value_t = false)]
    no_sort: bool,

    /// Sort search results by fuzzy match score, best match first
    #[arg(long, default_value_t = false)]
    sort_fancy: bool,

    /// Handlebars template of the command to execute
    #[arg(short, long, default_value = "ssh \"{{{name}}}\"")]
    template: String,

    /// Handlebars template of the command to execute when an SSH session starts
    #[arg(long, value_name = "TEMPLATE")]
    on_session_start_template: Option<String>,

    /// Handlebars template of the command to execute when an SSH session ends
    #[arg(long, value_name = "TEMPLATE")]
    on_session_end_template: Option<String>,

    /// Exit after ending the SSH session
    #[arg(short, long, default_value_t = false)]
    exit: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let user_config = config::load_user_config();

    let color = args
        .color
        .or(user_config.theme)
        .unwrap_or_else(|| "catppuccin".to_string());

    let ascii_art = args
        .ascii_art
        .or(user_config.ascii_art)
        .unwrap_or_else(|| "slant".to_string());

    let show_proxy_command = args.show_proxy_command || user_config.show_proxy_command.unwrap_or(false);
    let sort_by_name = (args.sort && !args.no_sort) || user_config.sort.unwrap_or(false);
    let sort_by_score = args.sort_fancy || user_config.sort_fancy.unwrap_or(false);
    let animate = !args.no_animate && user_config.animate.unwrap_or(true);

    let config_paths = if args.config == ["/etc/ssh/ssh_config", "~/.ssh/config"] {
        user_config.config.unwrap_or(args.config)
    } else {
        args.config
    };

    let mut app = App::new(&AppConfig {
        config_paths,
        search_filter: args.search,
        color,
        ascii_art,
        no_ascii_art: args.no_ascii_art,
        sort_by_name,
        sort_by_score,
        show_proxy_command,
        animate,
        command_template: args.template,
        command_template_on_session_start: args.on_session_start_template,
        command_template_on_session_end: args.on_session_end_template,
        exit_after_ssh_session_ends: args.exit,
    })?;
    app.start()?;

    Ok(())
}
