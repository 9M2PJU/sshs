use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Cell, Clear, Paragraph, Row, Table, TableState},
    Frame,
};
use std::{
    cmp::min,
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{Instant, UNIX_EPOCH},
};
use tui_input::Input;

use crate::{ssh::Host, theme::Theme};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileEntry {
    pub name: String,
    pub is_dir: bool,
    pub size: u64,
    pub size_display: String,
    pub modified: String,
    pub permissions: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActivePane {
    Local,
    Remote,
}

#[derive(Clone, Debug)]
pub struct SftpExplorer {
    pub host: Host,
    pub active_pane: ActivePane,

    // Local pane
    pub local_path: PathBuf,
    pub local_entries: Vec<FileEntry>,
    pub local_selected: usize,
    pub local_table_state: TableState,

    // Remote pane
    pub remote_path: String,
    pub remote_entries: Vec<FileEntry>,
    pub remote_selected: usize,
    pub remote_table_state: TableState,

    // Modals and popups
    pub status_message: Option<(String, Instant)>,
    pub viewer_content: Option<(String, String)>, // (file_title, text_content)
    pub mkdir_input: Option<Input>,
    pub confirm_delete: Option<String>,
    pub is_loading: bool,
}

impl SftpExplorer {
    #[must_use]
    pub fn new(host: Host) -> Self {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let local_path = PathBuf::from(home);

        let mut explorer = Self {
            host,
            active_pane: ActivePane::Local,
            local_path,
            local_entries: Vec::new(),
            local_selected: 0,
            local_table_state: TableState::default().with_selected(0),
            remote_path: ".".to_string(),
            remote_entries: Vec::new(),
            remote_selected: 0,
            remote_table_state: TableState::default().with_selected(0),
            status_message: None,
            viewer_content: None,
            mkdir_input: None,
            confirm_delete: None,
            is_loading: false,
        };

        explorer.refresh_local();
        explorer.refresh_remote();
        explorer
    }

    pub fn set_status(&mut self, msg: String) {
        self.status_message = Some((msg, Instant::now()));
    }

    pub fn refresh_local(&mut self) {
        let mut entries = Vec::new();

        // Add parent directory entry if not at root
        if self.local_path.parent().is_some() {
            entries.push(FileEntry {
                name: "..".to_string(),
                is_dir: true,
                size: 0,
                size_display: "<UP--DIR>".to_string(),
                modified: String::new(),
                permissions: "drwxr-xr-x".to_string(),
            });
        }

        if let Ok(read_dir) = fs::read_dir(&self.local_path) {
            let mut dirs = Vec::new();
            let mut files = Vec::new();

            for entry in read_dir.flatten() {
                let name = entry.file_name().to_string_lossy().into_owned();
                let metadata = entry.metadata().ok();
                let is_dir = metadata.as_ref().is_some_and(fs::Metadata::is_dir);
                let size = metadata.as_ref().map_or(0, fs::Metadata::len);
                let size_display = if is_dir {
                    "<DIR>".to_string()
                } else {
                    format_bytes(size)
                };

                let modified = metadata
                    .and_then(|m| m.modified().ok())
                    .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
                    .map_or_else(String::new, |dur| format_timestamp(dur.as_secs()));

                let permissions = if is_dir {
                    "drwxr-xr-x".to_string()
                } else {
                    "-rw-r--r--".to_string()
                };

                let item = FileEntry {
                    name,
                    is_dir,
                    size,
                    size_display,
                    modified,
                    permissions,
                };

                if is_dir {
                    dirs.push(item);
                } else {
                    files.push(item);
                }
            }

            dirs.sort_by_key(|a| a.name.to_lowercase());
            files.sort_by_key(|a| a.name.to_lowercase());

            entries.extend(dirs);
            entries.extend(files);
        }

        self.local_entries = entries;
        if self.local_selected >= self.local_entries.len() {
            self.local_selected = self.local_entries.len().saturating_sub(1);
        }
        self.local_table_state.select(Some(self.local_selected));
    }

    pub fn refresh_remote(&mut self) {
        let host_name = &self.host.name;
        let path = &self.remote_path;

        let port_args = if let Some(p) = &self.host.port {
            if !p.is_empty() && p != "22" {
                vec!["-p".to_string(), p.clone()]
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };

        let mut cmd = Command::new("ssh");
        cmd.arg("-o").arg("BatchMode=yes");
        cmd.arg("-o").arg("ConnectTimeout=5");
        for arg in port_args {
            cmd.arg(arg);
        }
        cmd.arg(host_name);
        cmd.arg(format!("ls -la --time-style=long-iso \"{path}\" 2>/dev/null || ls -la \"{path}\""));

        if let Ok(output) = cmd.output() {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                self.remote_entries = Self::parse_remote_ls(&stdout, path != "/" && path != ".");
                if self.remote_selected >= self.remote_entries.len() {
                    self.remote_selected = self.remote_entries.len().saturating_sub(1);
                }
                self.remote_table_state.select(Some(self.remote_selected));
            } else {
                self.set_status(format!("Remote ls failed on '{host_name}'"));
            }
        }
    }

    #[must_use]
    pub fn parse_remote_ls(output: &str, show_parent: bool) -> Vec<FileEntry> {
        let mut dirs = Vec::new();
        let mut files = Vec::new();

        if show_parent {
            dirs.push(FileEntry {
                name: "..".to_string(),
                is_dir: true,
                size: 0,
                size_display: "<UP--DIR>".to_string(),
                modified: String::new(),
                permissions: "drwxr-xr-x".to_string(),
            });
        }

        for line in output.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("total") {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 8 {
                continue;
            }

            let perms = parts[0];
            let is_dir = perms.starts_with('d') || perms.starts_with('l');
            let size = parts[4].parse::<u64>().unwrap_or(0);

            // Reconstruct name and date
            // Format 1 (long-iso): perms links user group size YYYY-MM-DD HH:MM name... (>= 8 parts)
            // Format 2 (standard): perms links user group size Month Day HH:MM name... (>= 9 parts)
            let (modified, name) = if parts.len() >= 8 && parts[5].contains('-') {
                let date_str = format!("{} {}", parts[5], parts[6]);
                let file_name = parts[7..].join(" ");
                (date_str, file_name)
            } else if parts.len() >= 9 {
                let date_str = format!("{} {} {}", parts[5], parts[6], parts[7]);
                let file_name = parts[8..].join(" ");
                (date_str, file_name)
            } else {
                let file_name = parts[parts.len() - 1].to_string();
                (String::new(), file_name)
            };

            if name == "." || name == ".." {
                continue;
            }

            let size_display = if is_dir {
                "<DIR>".to_string()
            } else {
                format_bytes(size)
            };

            let item = FileEntry {
                name,
                is_dir,
                size,
                size_display,
                modified,
                permissions: perms.to_string(),
            };

            if is_dir {
                dirs.push(item);
            } else {
                files.push(item);
            }
        }

        dirs.sort_by_key(|a| a.name.to_lowercase());
        files.sort_by_key(|a| a.name.to_lowercase());

        let mut result = dirs;
        result.extend(files);
        result
    }

    pub fn select_next(&mut self) {
        match self.active_pane {
            ActivePane::Local => {
                if !self.local_entries.is_empty() {
                    self.local_selected = min(self.local_selected + 1, self.local_entries.len() - 1);
                    self.local_table_state.select(Some(self.local_selected));
                }
            }
            ActivePane::Remote => {
                if !self.remote_entries.is_empty() {
                    self.remote_selected = min(self.remote_selected + 1, self.remote_entries.len() - 1);
                    self.remote_table_state.select(Some(self.remote_selected));
                }
            }
        }
    }

    pub fn select_previous(&mut self) {
        match self.active_pane {
            ActivePane::Local => {
                self.local_selected = self.local_selected.saturating_sub(1);
                self.local_table_state.select(Some(self.local_selected));
            }
            ActivePane::Remote => {
                self.remote_selected = self.remote_selected.saturating_sub(1);
                self.remote_table_state.select(Some(self.remote_selected));
            }
        }
    }

    pub fn switch_pane(&mut self) {
        self.active_pane = match self.active_pane {
            ActivePane::Local => ActivePane::Remote,
            ActivePane::Remote => ActivePane::Local,
        };
    }

    pub fn enter_directory(&mut self) {
        match self.active_pane {
            ActivePane::Local => {
                if let Some(entry) = self.local_entries.get(self.local_selected) {
                    if entry.name == ".." {
                        if let Some(parent) = self.local_path.parent() {
                            self.local_path = parent.to_path_buf();
                            self.local_selected = 0;
                            self.refresh_local();
                        }
                    } else if entry.is_dir {
                        self.local_path = self.local_path.join(&entry.name);
                        self.local_selected = 0;
                        self.refresh_local();
                    } else {
                        self.set_status(format!("Selected local file: {}", entry.name));
                    }
                }
            }
            ActivePane::Remote => {
                if let Some(entry) = self.remote_entries.get(self.remote_selected) {
                    if entry.name == ".." {
                        let path = Path::new(&self.remote_path);
                        let parent = path.parent().unwrap_or_else(|| Path::new("."));
                        self.remote_path = parent.to_string_lossy().into_owned();
                        if self.remote_path.is_empty() {
                            self.remote_path = ".".to_string();
                        }
                        self.remote_selected = 0;
                        self.refresh_remote();
                    } else if entry.is_dir {
                        if self.remote_path == "." || self.remote_path.is_empty() {
                            self.remote_path = entry.name.clone();
                        } else {
                            self.remote_path = format!("{}/{}", self.remote_path, entry.name);
                        }
                        self.remote_selected = 0;
                        self.refresh_remote();
                    } else {
                        self.set_status(format!("Selected remote file: {}", entry.name));
                    }
                }
            }
        }
    }

    pub fn copy_or_transfer(&mut self) {
        match self.active_pane {
            ActivePane::Local => {
                // Upload local to remote
                let Some(entry) = self.local_entries.get(self.local_selected).cloned() else {
                    return;
                };
                if entry.name == ".." {
                    return;
                }
                let local_file = self.local_path.join(&entry.name);
                let remote_target = format!("{}:{}", self.host.name, self.remote_path);
                let local_str = local_file.to_string_lossy().into_owned();

                let mut cmd = Command::new("scp");
                cmd.arg("-r");
                if let Some(p) = &self.host.port {
                    if !p.is_empty() && p != "22" {
                        cmd.arg("-P").arg(p);
                    }
                }
                cmd.arg(&local_str);
                cmd.arg(&remote_target);

                self.set_status(format!("Uploading '{}' to remote...", entry.name));
                match cmd.status() {
                    Ok(s) if s.success() => {
                        self.set_status(format!("Uploaded '{}' successfully!", entry.name));
                        self.refresh_remote();
                    }
                    Ok(_) => self.set_status(format!("Upload of '{}' failed", entry.name)),
                    Err(e) => self.set_status(format!("Upload error: {e}")),
                }
            }
            ActivePane::Remote => {
                // Download remote to local
                let Some(entry) = self.remote_entries.get(self.remote_selected).cloned() else {
                    return;
                };
                if entry.name == ".." {
                    return;
                }
                let remote_src = if self.remote_path == "." || self.remote_path.is_empty() {
                    format!("{}:{}", self.host.name, entry.name)
                } else {
                    format!("{}:{}/{}", self.host.name, self.remote_path, entry.name)
                };
                let local_target = self.local_path.to_string_lossy().into_owned();

                let mut cmd = Command::new("scp");
                cmd.arg("-r");
                if let Some(p) = &self.host.port {
                    if !p.is_empty() && p != "22" {
                        cmd.arg("-P").arg(p);
                    }
                }
                cmd.arg(&remote_src);
                cmd.arg(&local_target);

                self.set_status(format!("Downloading '{}' from remote...", entry.name));
                match cmd.status() {
                    Ok(s) if s.success() => {
                        self.set_status(format!("Downloaded '{}' successfully!", entry.name));
                        self.refresh_local();
                    }
                    Ok(_) => self.set_status(format!("Download of '{}' failed", entry.name)),
                    Err(e) => self.set_status(format!("Download error: {e}")),
                }
            }
        }
    }

    pub fn delete_selected(&mut self) {
        match self.active_pane {
            ActivePane::Local => {
                if let Some(entry) = self.local_entries.get(self.local_selected) {
                    if entry.name == ".." {
                        return;
                    }
                    let target = self.local_path.join(&entry.name);
                    let res = if entry.is_dir {
                        fs::remove_dir_all(target)
                    } else {
                        fs::remove_file(target)
                    };
                    match res {
                        Ok(()) => {
                            self.set_status(format!("Deleted local '{}'", entry.name));
                            self.refresh_local();
                        }
                        Err(e) => self.set_status(format!("Delete error: {e}")),
                    }
                }
            }
            ActivePane::Remote => {
                if let Some(entry) = self.remote_entries.get(self.remote_selected) {
                    if entry.name == ".." {
                        return;
                    }
                    let full_remote = if self.remote_path == "." || self.remote_path.is_empty() {
                        entry.name.clone()
                    } else {
                        format!("{}/{}", self.remote_path, entry.name)
                    };

                    let mut cmd = Command::new("ssh");
                    if let Some(p) = &self.host.port {
                        if !p.is_empty() && p != "22" {
                            cmd.arg("-p").arg(p);
                        }
                    }
                    cmd.arg(&self.host.name);
                    cmd.arg(format!("rm -rf \"{full_remote}\""));

                    match cmd.status() {
                        Ok(s) if s.success() => {
                            self.set_status(format!("Deleted remote '{}'", entry.name));
                            self.refresh_remote();
                        }
                        Ok(_) => self.set_status(format!("Delete of remote '{}' failed", entry.name)),
                        Err(e) => self.set_status(format!("Delete error: {e}")),
                    }
                }
            }
        }
    }

    pub fn view_selected(&mut self) {
        match self.active_pane {
            ActivePane::Local => {
                if let Some(entry) = self.local_entries.get(self.local_selected) {
                    if entry.is_dir {
                        self.enter_directory();
                        return;
                    }
                    let target = self.local_path.join(&entry.name);
                    if let Ok(content) = fs::read_to_string(&target) {
                        let preview = content.lines().take(100).collect::<Vec<_>>().join("\n");
                        self.viewer_content = Some((format!("Local: {}", entry.name), preview));
                    } else {
                        self.set_status("Binary or unreadable file".to_string());
                    }
                }
            }
            ActivePane::Remote => {
                if let Some(entry) = self.remote_entries.get(self.remote_selected) {
                    if entry.is_dir {
                        self.enter_directory();
                        return;
                    }
                    let full_remote = if self.remote_path == "." || self.remote_path.is_empty() {
                        entry.name.clone()
                    } else {
                        format!("{}/{}", self.remote_path, entry.name)
                    };

                    let mut cmd = Command::new("ssh");
                    if let Some(p) = &self.host.port {
                        if !p.is_empty() && p != "22" {
                            cmd.arg("-p").arg(p);
                        }
                    }
                    cmd.arg(&self.host.name);
                    cmd.arg(format!("head -n 100 \"{full_remote}\""));

                    if let Ok(output) = cmd.output() {
                        let text = String::from_utf8_lossy(&output.stdout).into_owned();
                        self.viewer_content = Some((format!("Remote: {}", entry.name), text));
                    } else {
                        self.set_status("Failed to read remote file".to_string());
                    }
                }
            }
        }
    }

    pub fn make_directory(&mut self, name: &str) {
        let name = name.trim();
        if name.is_empty() {
            return;
        }

        match self.active_pane {
            ActivePane::Local => {
                let target = self.local_path.join(name);
                match fs::create_dir_all(target) {
                    Ok(()) => {
                        self.set_status(format!("Created directory '{name}'"));
                        self.refresh_local();
                    }
                    Err(e) => self.set_status(format!("Mkdir error: {e}")),
                }
            }
            ActivePane::Remote => {
                let target = if self.remote_path == "." || self.remote_path.is_empty() {
                    name.to_string()
                } else {
                    format!("{}/{name}", self.remote_path)
                };

                let mut cmd = Command::new("ssh");
                if let Some(p) = &self.host.port {
                    if !p.is_empty() && p != "22" {
                        cmd.arg("-p").arg(p);
                    }
                }
                cmd.arg(&self.host.name);
                cmd.arg(format!("mkdir -p \"{target}\""));

                match cmd.status() {
                    Ok(s) if s.success() => {
                        self.set_status(format!("Created remote directory '{name}'"));
                        self.refresh_remote();
                    }
                    Ok(_) => self.set_status(format!("Remote mkdir '{name}' failed")),
                    Err(e) => self.set_status(format!("Mkdir error: {e}")),
                }
            }
        }
    }
}

#[must_use]
pub fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = 1024 * KB;
    const GB: u64 = 1024 * MB;

    #[allow(clippy::cast_precision_loss)]
    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{bytes} B")
    }
}

#[must_use]
pub fn format_timestamp(epoch_secs: u64) -> String {
    let days = epoch_secs / 86400;
    let rem_secs = epoch_secs % 86400;
    let hours = rem_secs / 3600;
    let mins = (rem_secs % 3600) / 60;

    let mut year = 1970;
    let mut day_count = days;

    loop {
        let leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
        let days_in_year = if leap { 366 } else { 365 };
        if day_count < days_in_year {
            break;
        }
        day_count -= days_in_year;
        year += 1;
    }

    let leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
    let days_in_months = [
        31,
        if leap { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];

    let mut month = 1;
    for &m_days in &days_in_months {
        if day_count < m_days {
            break;
        }
        day_count -= m_days;
        month += 1;
    }
    let day = day_count + 1;

    format!("{year:04}-{month:02}-{day:02} {hours:02}:{mins:02}")
}

pub fn render_sftp_explorer(
    f: &mut Frame,
    theme: &Theme,
    explorer: &mut SftpExplorer,
) {
    let area = f.area();
    f.render_widget(Clear, area);

    // Layout: Header (3), Main dual panes (Min 10), Status & Footer (3)
    let chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(10),
        Constraint::Length(3),
    ])
    .split(area);

    // 1. Top Header
    let local_path_str = explorer.local_path.to_string_lossy();
    let remote_path_str = &explorer.remote_path;
    let header_line = Line::from(vec![
        Span::styled(" 📂 Dual-Pane SFTP File Commander ", Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)),
        Span::styled(format!(" [ Host: {} ] ", explorer.host.name), theme.badge_style()),
        Span::raw("  "),
        Span::styled("Local: ", Style::default().fg(theme.muted)),
        Span::styled(local_path_str.as_ref(), Style::default().fg(theme.header_fg).add_modifier(Modifier::BOLD)),
        Span::raw(" │ "),
        Span::styled("Remote: ", Style::default().fg(theme.muted)),
        Span::styled(remote_path_str.as_str(), Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
    ]);

    let header_widget = Paragraph::new(header_line).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(theme.border_style())
            .border_type(BorderType::Rounded)
            .title(Line::from(Span::styled(" Midnight SFTP Explorer ", Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)))),
    );
    f.render_widget(header_widget, chunks[0]);

    // 2. Dual Panes (Left: Local, Right: Remote)
    let pane_chunks = Layout::horizontal([
        Constraint::Percentage(50),
        Constraint::Percentage(50),
    ])
    .split(chunks[1]);

    // Render Left (Local) Pane
    render_pane(
        f,
        theme,
        " 💻 Local Filesystem ",
        &explorer.local_path.to_string_lossy(),
        &explorer.local_entries,
        &mut explorer.local_table_state,
        explorer.active_pane == ActivePane::Local,
        pane_chunks[0],
    );

    // Render Right (Remote) Pane
    render_pane(
        f,
        theme,
        &format!(" 🌐 Remote SFTP [{}] ", explorer.host.name),
        &explorer.remote_path,
        &explorer.remote_entries,
        &mut explorer.remote_table_state,
        explorer.active_pane == ActivePane::Remote,
        pane_chunks[1],
    );

    // 3. Footer with Midnight Commander style Function Bar
    let is_recent_status = if let Some((msg, timestamp)) = &explorer.status_message {
        if timestamp.elapsed().as_secs() < 3 {
            Some(msg.clone())
        } else {
            None
        }
    } else {
        None
    };

    let footer_line = if let Some(msg) = is_recent_status {
        Line::from(vec![
            Span::styled(" ℹ ", Style::default().fg(theme.secondary).add_modifier(Modifier::BOLD)),
            Span::styled(msg, Style::default().fg(theme.header_fg).add_modifier(Modifier::BOLD)),
        ])
    } else {
        Line::from(vec![
            Span::styled(" Tab ", theme.key_badge_style()),
            Span::styled(" Switch Pane ", theme.key_desc_style()),
            Span::styled(" Enter ", theme.key_badge_style()),
            Span::styled(" Open/Chdir ", theme.key_desc_style()),
            Span::styled(" F3/v ", theme.key_badge_style()),
            Span::styled(" View ", theme.key_desc_style()),
            Span::styled(" F5/c ", theme.key_badge_style()),
            Span::styled(" Copy/Xfer ", theme.key_desc_style()),
            Span::styled(" F7/m ", theme.key_badge_style()),
            Span::styled(" Mkdir ", theme.key_desc_style()),
            Span::styled(" F8/d ", theme.key_badge_style()),
            Span::styled(" Delete ", theme.key_desc_style()),
            Span::styled(" Esc/q ", theme.key_badge_style()),
            Span::styled(" Close ", theme.key_desc_style()),
        ])
    };

    let footer_widget = Paragraph::new(footer_line).centered().block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(theme.border_style())
            .border_type(BorderType::Rounded),
    );
    f.render_widget(footer_widget, chunks[2]);

    // Render Popups if active (Viewer, Mkdir, Delete confirmation)
    if let Some((title, text)) = &explorer.viewer_content {
        render_viewer_popup(f, theme, title, text);
    } else if let Some(input) = &explorer.mkdir_input {
        render_mkdir_popup(f, theme, input);
    }
}

#[allow(clippy::too_many_arguments)]
fn render_pane(
    f: &mut Frame,
    theme: &Theme,
    title: &str,
    path_str: &str,
    entries: &[FileEntry],
    table_state: &mut TableState,
    is_active: bool,
    area: Rect,
) {
    let border_style = if is_active {
        theme.active_border_style()
    } else {
        theme.border_style()
    };

    let header_names = ["Name", "Size", "Permissions", "Modified"];
    let header = Row::new(header_names.iter().map(|&h| Cell::from(Span::styled(h, theme.table_header_style()))))
        .height(1);

    let rows = entries.iter().map(|e| {
        let (icon, name_color) = if e.name == ".." {
            ("📁 ", theme.secondary)
        } else if e.is_dir {
            ("📁 ", theme.primary)
        } else {
            ("📄 ", theme.header_fg)
        };

        let name_cell = Cell::from(Line::from(vec![
            Span::styled(icon, Style::default().fg(name_color)),
            Span::styled(&e.name, Style::default().fg(name_color).add_modifier(if e.is_dir { Modifier::BOLD } else { Modifier::empty() })),
        ]));

        let size_cell = Cell::from(Span::styled(
            &e.size_display,
            Style::default().fg(theme.port),
        ));

        let perm_cell = Cell::from(Span::styled(
            &e.permissions,
            Style::default().fg(theme.muted),
        ));

        let mod_cell = Cell::from(Span::styled(
            &e.modified,
            Style::default().fg(theme.aliases),
        ));

        Row::new(vec![name_cell, size_cell, perm_cell, mod_cell])
    });

    let constraints = [
        Constraint::Percentage(45),
        Constraint::Length(12),
        Constraint::Length(12),
        Constraint::Min(16),
    ];

    let count_text = format!(" [ {} items ] ", entries.len());
    let title_line = Line::from(vec![
        Span::styled(title, if is_active { Style::default().fg(theme.primary).add_modifier(Modifier::BOLD) } else { Style::default().fg(theme.muted) }),
        Span::styled(format!(" ( {path_str} ) "), Style::default().fg(theme.aliases)),
    ]);

    let table = Table::new(rows, constraints)
        .header(header)
        .row_highlight_style(theme.selected_row_style())
        .highlight_symbol("▌ ")
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(border_style)
                .border_type(if is_active { BorderType::Double } else { BorderType::Rounded })
                .title(title_line)
                .title(Line::from(Span::styled(count_text, theme.badge_style())).alignment(Alignment::Right)),
        );

    f.render_stateful_widget(table, area, table_state);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(r);

    Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(popup_layout[1])[1]
}

fn render_viewer_popup(f: &mut Frame, theme: &Theme, title: &str, content: &str) {
    let area = centered_rect(80, 80, f.area());
    f.render_widget(Clear, area);

    let lines = content
        .lines()
        .map(|l| Line::from(Span::styled(l, Style::default().fg(theme.header_fg))))
        .collect::<Vec<_>>();

    let widget = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(theme.active_border_style())
            .title(Line::from(Span::styled(
                format!(" 👁 File Viewer: {title} "),
                Style::default().fg(theme.primary).add_modifier(Modifier::BOLD),
            )))
            .title(Line::from(Span::styled(" [ Esc / q to close ] ", theme.badge_style())).alignment(Alignment::Right)),
    );

    f.render_widget(widget, area);
}

fn render_mkdir_popup(f: &mut Frame, theme: &Theme, input: &Input) {
    let area = centered_rect(50, 25, f.area());
    f.render_widget(Clear, area);

    let val = input.value();
    let lines = vec![
        Line::raw(""),
        Line::from(vec![
            Span::styled("  Enter new directory name:", Style::default().fg(theme.header_fg).add_modifier(Modifier::BOLD)),
        ]),
        Line::raw(""),
        Line::from(vec![
            Span::styled("   [ ", Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)),
            Span::styled(if val.is_empty() { "new_folder" } else { val }, if val.is_empty() { Style::default().fg(theme.muted) } else { Style::default().fg(theme.header_fg).add_modifier(Modifier::BOLD) }),
            Span::styled(" ]", Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)),
        ]),
        Line::raw(""),
        Line::from(vec![
            Span::styled(" [ Enter ] ", theme.key_badge_style()),
            Span::styled(" Create    ", theme.key_desc_style()),
            Span::styled(" [ Esc ] ", theme.key_badge_style()),
            Span::styled(" Cancel", theme.key_desc_style()),
        ]).centered(),
    ];

    let widget = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(theme.active_border_style())
            .title(Line::from(Span::styled(
                " 📁 Create Directory ",
                Style::default().fg(theme.primary).add_modifier(Modifier::BOLD),
            ))),
    );

    f.render_widget(widget, area);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(500), "500 B");
        assert_eq!(format_bytes(1024), "1.0 KB");
        assert_eq!(format_bytes(1024 * 1024), "1.0 MB");
        assert_eq!(format_bytes(1024 * 1024 * 1024 * 2), "2.0 GB");
    }

    #[test]
    fn test_format_timestamp() {
        // 2026-08-30
        let s = format_timestamp(1788091200);
        assert!(s.contains("2026-08-30"));
    }

    #[test]
    fn test_parse_remote_ls_output() {
        let sample_output = r#"
total 32
drwxr-xr-x 4 root root 4096 2026-08-30 11:30 .
drwxr-xr-x 8 root root 4096 2026-08-29 10:00 ..
drwxr-xr-x 2 user user 4096 2026-08-30 11:45 my_folder
-rw-r--r-- 1 user user 1234 2026-08-30 12:00 config.json
-rwxr-xr-x 1 user user 9876 2026-08-30 12:05 script.sh
"#;

        let entries = SftpExplorer::parse_remote_ls(sample_output, true);
        assert_eq!(entries.len(), 4); // .. + my_folder + config.json + script.sh

        assert_eq!(entries[0].name, "..");
        assert!(entries[0].is_dir);

        assert_eq!(entries[1].name, "my_folder");
        assert!(entries[1].is_dir);

        assert_eq!(entries[2].name, "config.json");
        assert!(!entries[2].is_dir);
        assert_eq!(entries[2].size, 1234);

        assert_eq!(entries[3].name, "script.sh");
        assert!(!entries[3].is_dir);
        assert_eq!(entries[3].size, 9876);
    }
}
