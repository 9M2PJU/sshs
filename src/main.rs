pub mod searchable;
pub mod ssh;
pub mod ssh_config;
#[cfg(test)]
mod test_support;
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

    /// Color of the interface (a Tailwind palette name, for example blue, green, rose)
    #[arg(long, default_value = "blue")]
    color: String,

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

    let mut app = App::new(&AppConfig {
        config_paths: args.config,
        search_filter: args.search,
        color: args.color,
        sort_by_name: args.sort && !args.no_sort,
        sort_by_score: args.sort_fancy,
        show_proxy_command: args.show_proxy_command,
        command_template: args.template,
        command_template_on_session_start: args.on_session_start_template,
        command_template_on_session_end: args.on_session_end_template,
        exit_after_ssh_session_ends: args.exit,
    })?;
    app.start()?;

    Ok(())
}
