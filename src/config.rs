use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

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

    /// Enable visual animations (banner gradient waves, glowing cursors)
    pub animate: Option<bool>,
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

/// Saves or updates the animation preference in the configuration file.
///
/// # Errors
/// Returns an error if directory creation or file writing fails.
pub fn save_user_animate(animate: bool) -> anyhow::Result<()> {
    let path = get_user_config_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut config = load_user_config();
    config.animate = Some(animate);

    let toml_str = toml::to_string_pretty(&config)?;
    fs::write(&path, toml_str)?;

    Ok(())
}

/// Appends a new SSH host entry to a specific file.
///
/// # Errors
/// Returns an error if file creation or writing fails.
pub fn append_host_to_file(
    path: &Path,
    name: &str,
    hostname: &str,
    user: Option<&str>,
    port: Option<&str>,
    identity_file: Option<&str>,
    proxy_jump: Option<&str>,
) -> anyhow::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let file_content = if path.exists() {
        fs::read_to_string(path)?
    } else {
        String::new()
    };

    let mut stanza = String::new();
    if !file_content.is_empty() {
        if !file_content.ends_with('\n') {
            stanza.push('\n');
        }
        if !file_content.ends_with("\n\n") {
            stanza.push('\n');
        }
    }

    stanza.push_str(&format!("Host {name}\n"));
    stanza.push_str(&format!("  HostName {hostname}\n"));

    if let Some(u) = user {
        let trimmed = u.trim();
        if !trimmed.is_empty() {
            stanza.push_str(&format!("  User {trimmed}\n"));
        }
    }

    if let Some(p) = port {
        let trimmed = p.trim();
        if !trimmed.is_empty() && trimmed != "22" {
            stanza.push_str(&format!("  Port {trimmed}\n"));
        }
    }

    if let Some(id) = identity_file {
        let trimmed = id.trim();
        if !trimmed.is_empty() {
            stanza.push_str(&format!("  IdentityFile {trimmed}\n"));
        }
    }

    if let Some(pj) = proxy_jump {
        let trimmed = pj.trim();
        if !trimmed.is_empty() {
            stanza.push_str(&format!("  ProxyJump {trimmed}\n"));
        }
    }

    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;
    file.write_all(stanza.as_bytes())?;

    Ok(())
}

/// Appends a new SSH host entry to `~/.ssh/config`.
///
/// # Errors
/// Returns an error if file creation or writing fails.
pub fn append_host_to_ssh_config(
    name: &str,
    hostname: &str,
    user: Option<&str>,
    port: Option<&str>,
    identity_file: Option<&str>,
    proxy_jump: Option<&str>,
) -> anyhow::Result<PathBuf> {
    let ssh_dir = shellexpand::tilde("~/.ssh").into_owned();
    let config_path = PathBuf::from(&ssh_dir).join("config");
    append_host_to_file(
        &config_path,
        name,
        hostname,
        user,
        port,
        identity_file,
        proxy_jump,
    )?;
    Ok(config_path)
}

/// Removes an SSH host stanza from a specific file.
///
/// # Errors
/// Returns an error if reading or writing to the file fails.
pub fn remove_host_from_file(path: &Path, name: &str) -> anyhow::Result<bool> {
    if !path.exists() {
        return Ok(false);
    }

    let content = fs::read_to_string(path)?;
    let mut new_lines = Vec::new();
    let mut in_target_host = false;
    let mut removed = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("Host ") || trimmed.starts_with("host ") {
            let host_names: Vec<&str> = trimmed[5..].split_whitespace().collect();
            if host_names.iter().any(|&h| h.trim_matches('"') == name) {
                in_target_host = true;
                removed = true;
                continue;
            }
            in_target_host = false;
        } else if trimmed.starts_with("Match ") || trimmed.starts_with("match ") {
            in_target_host = false;
        }

        if !in_target_host {
            new_lines.push(line);
        }
    }

    if removed {
        let mut final_content = new_lines.join("\n");
        if !final_content.ends_with('\n') && !final_content.is_empty() {
            final_content.push('\n');
        }
        fs::write(path, final_content)?;
    }

    Ok(removed)
}

/// Removes an SSH host stanza from `~/.ssh/config`.
///
/// # Errors
/// Returns an error if reading or writing to the file fails.
pub fn remove_host_from_ssh_config(name: &str) -> anyhow::Result<bool> {
    let ssh_dir = shellexpand::tilde("~/.ssh").into_owned();
    let config_path = PathBuf::from(&ssh_dir).join("config");
    remove_host_from_file(&config_path, name)
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

    #[test]
    fn test_append_host_to_file() {
        let temp_dir = std::env::temp_dir().join("sshs_test_append");
        let config_file = temp_dir.join("config");
        let _ = fs::remove_file(&config_file);

        append_host_to_file(
            &config_file,
            "myserver",
            "192.168.1.100",
            Some("admin"),
            Some("2222"),
            Some("~/.ssh/id_ed25519"),
            None,
        )
        .unwrap();

        let content = fs::read_to_string(&config_file).unwrap();
        assert!(content.contains("Host myserver"));
        assert!(content.contains("HostName 192.168.1.100"));
        assert!(content.contains("User admin"));
        assert!(content.contains("Port 2222"));
        assert!(content.contains("IdentityFile ~/.ssh/id_ed25519"));

        // Clean up
        let _ = fs::remove_file(&config_file);
    }

    #[test]
    fn test_remove_host_from_file() {
        let temp_dir = std::env::temp_dir().join("sshs_test_remove");
        let config_file = temp_dir.join("config");
        let sample = "Host host1\n  HostName 1.1.1.1\n\nHost host2\n  HostName 2.2.2.2\n  User root\n\nHost host3\n  HostName 3.3.3.3\n";
        fs::create_dir_all(&temp_dir).unwrap();
        fs::write(&config_file, sample).unwrap();

        let removed = remove_host_from_file(&config_file, "host2").unwrap();
        assert!(removed);

        let content = fs::read_to_string(&config_file).unwrap();
        assert!(content.contains("Host host1"));
        assert!(!content.contains("Host host2"));
        assert!(!content.contains("HostName 2.2.2.2"));
        assert!(content.contains("Host host3"));

        let _ = fs::remove_file(&config_file);
    }
}
