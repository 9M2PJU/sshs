use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Represents configuration options stored in `~/.config/sshs/config.toml`.
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct UserConfigFile {
    /// Default theme (e.g. "tokyonight", "dracula", "catppuccin", "cyberpunk", etc.)
    pub theme: Option<String>,

    /// Default ASCII art banner style ("slant", "cyber", "standard", "mini", "off")
    pub ascii_art: Option<String>,

    /// Sort hosts by hostname
    pub sort: Option<bool>,

    /// Sort search results by fuzzy match score
    pub sort_fancy: Option<bool>,

    /// Shows ProxyCommand column
    pub show_proxy_command: Option<bool>,

    /// Custom SSH configuration file paths
    pub config: Option<Vec<String>>,
}

/// Returns the path to the SSHS configuration file (`$XDG_CONFIG_HOME/sshs/config.toml` or `~/.config/sshs/config.toml`).
#[must_use]
pub fn get_user_config_path() -> PathBuf {
    if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
        if !xdg.trim().is_empty() {
            return PathBuf::from(xdg).join("sshs").join("config.toml");
        }
    }

    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".config").join("sshs").join("config.toml")
}

/// Loads the user configuration file if it exists, or returns default.
#[must_use]
pub fn load_user_config() -> UserConfigFile {
    let path = get_user_config_path();
    if !path.exists() {
        return UserConfigFile::default();
    }

    match fs::read_to_string(&path) {
        Ok(content) => match toml::from_str(&content) {
            Ok(cfg) => cfg,
            Err(err) => {
                eprintln!("Warning: Failed to parse {}: {}", path.display(), err);
                UserConfigFile::default()
            }
        },
        Err(_) => UserConfigFile::default(),
    }
}

/// Saves or updates the default theme in the configuration file.
///
/// # Errors
/// Returns an error if directory creation or file writing fails.
pub fn save_user_theme(theme_name: &str) -> anyhow::Result<()> {
    let path = get_user_config_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut config = load_user_config();
    config.theme = Some(theme_name.to_string());

    let toml_str = toml::to_string_pretty(&config)?;
    fs::write(&path, toml_str)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_user_config_toml() {
        let sample = r#"
theme = "dracula"
ascii_art = "cyber"
sort = true
"#;
        let cfg: UserConfigFile = toml::from_str(sample).unwrap();
        assert_eq!(cfg.theme.as_deref(), Some("dracula"));
        assert_eq!(cfg.ascii_art.as_deref(), Some("cyber"));
        assert_eq!(cfg.sort, Some(true));
    }
}
