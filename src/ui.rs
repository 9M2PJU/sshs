use anyhow::Result;
use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
#[allow(clippy::wildcard_imports)]
use ratatui::{prelude::*, widgets::*};
use std::{
    cell::RefCell,
    cmp::{max, min},
    io,
    rc::Rc,
    time::{Duration, Instant},
};
use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;
use unicode_width::UnicodeWidthStr;

use crate::{
    ascii_art::{self, AsciiArtStyle},
    searchable::Searchable,
    ssh,
    theme::{self, Theme},
};

#[derive(Clone, Debug)]
#[allow(clippy::struct_excessive_bools)]
pub struct AppConfig {
    pub config_paths: Vec<String>,

    pub search_filter: Option<String>,
    pub color: String,
    pub ascii_art: String,
    pub no_ascii_art: bool,
    pub sort_by_name: bool,
    pub sort_by_score: bool,
    pub show_proxy_command: bool,
    pub animate: bool,

    pub command_template: String,
    pub command_template_on_session_start: Option<String>,
    pub command_template_on_session_end: Option<String>,
    pub exit_after_ssh_session_ends: bool,
}

pub struct App {
    config: AppConfig,

    search: Input,

    table_state: TableState,
    hosts: Searchable<ssh::Host>,
    table_columns_constraints: Vec<Constraint>,
    page_step: usize,

    theme: Theme,
    ascii_art_style: AsciiArtStyle,
    show_details_modal: bool,
    show_help_modal: bool,
    status_message: Option<(String, Instant)>,
    anim_tick: u64,
    animate: bool,
}

#[derive(PartialEq)]
enum AppKeyAction {
    Ok,
    Stop,
    Continue,
}

impl App {
    /// # Errors
    ///
    /// Will return `Err` if the SSH configuration file cannot be parsed.
    pub fn new(config: &AppConfig) -> Result<App> {
        let hosts = load_hosts(config)?;
        let search_input = config.search_filter.clone().unwrap_or_default();
        let current_theme = theme::theme_by_name(&config.color)?;
        let ascii_style = if config.no_ascii_art {
            AsciiArtStyle::Off
        } else {
            config.ascii_art.parse().unwrap_or(AsciiArtStyle::Slant)
        };

        let mut app = App {
            config: config.clone(),

            search: search_input.clone().into(),

            table_state: TableState::default().with_selected(0),
            table_columns_constraints: Vec::new(),
            page_step: 21,

            theme: current_theme,
            ascii_art_style: ascii_style,
            show_details_modal: false,
            show_help_modal: false,
            status_message: None,
            anim_tick: 0,
            animate: config.animate,

            hosts: Searchable::new(config.sort_by_score, hosts, &search_input),
        };
        app.calculate_table_columns_constraints();

        Ok(app)
    }

    /// # Errors
    ///
    /// Will return `Err` if the terminal cannot be configured.
    pub fn start(&mut self) -> Result<()> {
        let stdout = io::stdout().lock();
        let backend = CrosstermBackend::new(stdout);
        let terminal = Rc::new(RefCell::new(Terminal::new(backend)?));

        setup_terminal(&terminal)?;

        // create app and run it
        let res = self.run(&terminal);

        restore_terminal(&terminal)?;

        if let Err(err) = res {
            println!("{err:?}");
        }

        Ok(())
    }

    fn run<B>(&mut self, terminal: &Rc<RefCell<Terminal<B>>>) -> Result<()>
    where
        B: Backend + std::io::Write,
        <B as Backend>::Error: Send + Sync + 'static,
    {
        let tick_rate = Duration::from_millis(50);

        loop {
            terminal.borrow_mut().draw(|f| ui(f, self))?;

            if crossterm::event::poll(tick_rate)? {
                let ev = event::read()?;

                if let Event::Key(key) = ev {
                    if key.kind == KeyEventKind::Press {
                        let action = self.on_key_press(terminal, key)?;
                        match action {
                            AppKeyAction::Ok => continue,
                            AppKeyAction::Stop => break,
                            AppKeyAction::Continue => {}
                        }
                    }

                    if !self.show_help_modal && !self.show_details_modal {
                        self.handle_search_event(&ev);
                    }
                }
            }

            if self.animate {
                self.anim_tick = self.anim_tick.wrapping_add(1);
            }
        }

        Ok(())
    }

    fn on_key_press<B>(
        &mut self,
        terminal: &Rc<RefCell<Terminal<B>>>,
        key: KeyEvent,
    ) -> Result<AppKeyAction>
    where
        B: Backend + std::io::Write,
        <B as Backend>::Error: Send + Sync + 'static,
    {
        #[allow(clippy::enum_glob_use)]
        use KeyCode::*;

        let is_ctrl_pressed = key.modifiers.contains(KeyModifiers::CONTROL);

        if is_ctrl_pressed {
            let action = self.on_key_press_ctrl(key);
            if action != AppKeyAction::Continue {
                return Ok(action);
            }
        }

        // Handle active modals first
        if self.show_help_modal {
            match key.code {
                Esc | Char('?' | 'q') | Enter => {
                    self.show_help_modal = false;
                    return Ok(AppKeyAction::Ok);
                }
                _ => return Ok(AppKeyAction::Ok),
            }
        }

        if self.show_details_modal {
            match key.code {
                Esc | Tab | Char('q') => {
                    self.show_details_modal = false;
                    return Ok(AppKeyAction::Ok);
                }
                Down => {
                    self.next();
                    return Ok(AppKeyAction::Ok);
                }
                Up => {
                    self.previous();
                    return Ok(AppKeyAction::Ok);
                }
                PageDown => {
                    let i = self.table_state.selected().unwrap_or(0);
                    let target = min(
                        i.saturating_add(self.page_step),
                        self.hosts.len().saturating_sub(1),
                    );
                    self.table_state.select(Some(target));
                    return Ok(AppKeyAction::Ok);
                }
                PageUp => {
                    let i = self.table_state.selected().unwrap_or(0);
                    let target = max(i.saturating_sub(self.page_step), 0);
                    self.table_state.select(Some(target));
                    return Ok(AppKeyAction::Ok);
                }
                Enter => {
                    self.show_details_modal = false;
                    return self.connect_to_selected_host(terminal);
                }
                _ => return Ok(AppKeyAction::Ok),
            }
        }

        match key.code {
            Esc => {
                if !self.search.value().is_empty() {
                    self.search = Input::default();
                    self.hosts.search("");
                    self.table_state.select(Some(0));
                    return Ok(AppKeyAction::Ok);
                }
                return Ok(AppKeyAction::Stop);
            }
            Tab => {
                if !self.hosts.is_empty() {
                    self.show_details_modal = true;
                }
                return Ok(AppKeyAction::Ok);
            }
            F(1) => {
                self.show_help_modal = true;
                return Ok(AppKeyAction::Ok);
            }
            F(2) => {
                self.cycle_theme();
                return Ok(AppKeyAction::Ok);
            }
            Down => self.next(),
            Up => self.previous(),
            Home => self.table_state.select(Some(0)),
            End => self
                .table_state
                .select(Some(self.hosts.len().saturating_sub(1))),
            PageDown => {
                let i = self.table_state.selected().unwrap_or(0);
                let target = min(
                    i.saturating_add(self.page_step),
                    self.hosts.len().saturating_sub(1),
                );

                self.table_state.select(Some(target));
            }
            PageUp => {
                let i = self.table_state.selected().unwrap_or(0);
                let target = max(i.saturating_sub(self.page_step), 0);

                self.table_state.select(Some(target));
            }
            Enter => {
                return self.connect_to_selected_host(terminal);
            }
            _ => return Ok(AppKeyAction::Continue),
        }

        Ok(AppKeyAction::Ok)
    }

    fn connect_to_selected_host<B>(
        &mut self,
        terminal: &Rc<RefCell<Terminal<B>>>,
    ) -> Result<AppKeyAction>
    where
        B: Backend + std::io::Write,
        <B as Backend>::Error: Send + Sync + 'static,
    {
        let selected = self.table_state.selected().unwrap_or(0);
        if selected >= self.hosts.len() {
            return Ok(AppKeyAction::Ok);
        }

        let host: &ssh::Host = &self.hosts[selected];

        restore_terminal(terminal).expect("Failed to restore terminal");

        if let Some(template) = &self.config.command_template_on_session_start {
            host.spawn_command_template(template)?;
        }

        host.spawn_command_template(&self.config.command_template)?;

        if let Some(template) = &self.config.command_template_on_session_end {
            host.spawn_command_template(template)?;
        }

        setup_terminal(terminal).expect("Failed to setup terminal");

        if self.config.exit_after_ssh_session_ends {
            return Ok(AppKeyAction::Stop);
        }

        Ok(AppKeyAction::Ok)
    }

    fn on_key_press_ctrl(&mut self, key: KeyEvent) -> AppKeyAction {
        #[allow(clippy::enum_glob_use)]
        use KeyCode::*;

        match key.code {
            Char('c') => AppKeyAction::Stop,
            Char('t') => {
                self.cycle_theme();
                AppKeyAction::Ok
            }
            Char('s') => {
                if let Err(e) = crate::config::save_user_theme(self.theme.name) {
                    self.set_status_message(format!("Error saving theme: {e}"));
                } else {
                    self.set_status_message(format!("Saved '{}' as default in ~/.config/sshs/config.toml!", self.theme.display_name));
                }
                AppKeyAction::Ok
            }
            Char('h') => {
                self.show_help_modal = !self.show_help_modal;
                AppKeyAction::Ok
            }
            Char('u') => {
                self.search = Input::default();
                self.hosts.search("");
                self.table_state.select(Some(0));
                AppKeyAction::Ok
            }
            Char('j' | 'n') => {
                self.next();
                AppKeyAction::Ok
            }
            Char('k' | 'p') => {
                self.previous();
                AppKeyAction::Ok
            }
            Char('r') => {
                self.reload_hosts();
                self.set_status_message("SSH config reloaded".to_string());
                AppKeyAction::Ok
            }
            Char('a') => {
                self.animate = !self.animate;
                let state = if self.animate { "enabled" } else { "disabled" };
                let _ = crate::config::save_user_animate(self.animate);
                self.set_status_message(format!("Animations {state}"));
                AppKeyAction::Ok
            }
            _ => AppKeyAction::Continue,
        }
    }

    fn cycle_theme(&mut self) {
        let next = theme::next_theme(self.theme.name);
        self.set_status_message(format!("Theme: {} (Ctrl+S to save default)", next.display_name));
        self.theme = next;
    }

    fn set_status_message(&mut self, msg: String) {
        self.status_message = Some((msg, Instant::now()));
    }

    /// Updates the search input from a terminal event, re-filters the host
    /// list, and keeps the table selection valid.
    fn handle_search_event(&mut self, ev: &Event) {
        let search_value_before = self.search.value().to_string();
        self.search.handle_event(ev);

        if self.search.value() == search_value_before {
            let selected = self.table_state.selected().unwrap_or(0);
            if selected >= self.hosts.len() {
                self.table_state.select(Some(match self.hosts.len() {
                    0 => 0,
                    _ => self.hosts.len() - 1,
                }));
            }
        } else {
            self.hosts.search(self.search.value());
            self.table_state.select(Some(0));
        }
    }

    /// Re-reads the configuration files. Keeps the current list when a file
    /// no longer parses.
    fn reload_hosts(&mut self) {
        if let Ok(hosts) = load_hosts(&self.config) {
            self.hosts = Searchable::new(self.config.sort_by_score, hosts, self.search.value());
            self.table_state.select(Some(0));
            self.calculate_table_columns_constraints();
        }
    }

    fn next(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if self.hosts.is_empty() || i >= self.hosts.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if self.hosts.is_empty() {
                    0
                } else if i == 0 {
                    self.hosts.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    fn calculate_table_columns_constraints(&mut self) {
        let mut lengths = Vec::new();

        let name_len = self
            .hosts
            .non_filtered_iter()
            .map(|d| d.name.as_str())
            .map(UnicodeWidthStr::width)
            .max()
            .unwrap_or(0);
        // +2 for the "● " bullet glyph prefix
        lengths.push(name_len + 2);

        let aliases_len = self
            .hosts
            .non_filtered_iter()
            .map(|d| d.aliases.as_str())
            .map(UnicodeWidthStr::width)
            .max()
            .unwrap_or(0);
        lengths.push(aliases_len);

        let user_len = self
            .hosts
            .non_filtered_iter()
            .map(|d| match &d.user {
                Some(user) => user.as_str(),
                None => "",
            })
            .map(UnicodeWidthStr::width)
            .max()
            .unwrap_or(0);
        lengths.push(user_len);

        let destination_len = self
            .hosts
            .non_filtered_iter()
            .map(|d| d.destination.as_str())
            .map(UnicodeWidthStr::width)
            .max()
            .unwrap_or(0);
        lengths.push(destination_len);

        let port_len = self
            .hosts
            .non_filtered_iter()
            .map(|d| match &d.port {
                Some(port) => port.as_str(),
                None => "",
            })
            .map(UnicodeWidthStr::width)
            .max()
            .unwrap_or(0);
        lengths.push(port_len);

        if self.config.show_proxy_command {
            let proxy_len = self
                .hosts
                .non_filtered_iter()
                .map(|d| match &d.proxy_command {
                    Some(proxy) => proxy.as_str(),
                    None => "",
                })
                .map(UnicodeWidthStr::width)
                .max()
                .unwrap_or(0);
            lengths.push(proxy_len);
        }

        let mut new_constraints = vec![
            // +1 for padding
            Constraint::Length(u16::try_from(lengths[0]).unwrap_or_default() + 1),
        ];
        new_constraints.extend(
            lengths
                .iter()
                .skip(1)
                .map(|len| Constraint::Min(u16::try_from(*len).unwrap_or_default() + 1)),
        );

        self.table_columns_constraints = new_constraints;
    }
}

fn load_hosts(config: &AppConfig) -> Result<Vec<ssh::Host>> {
    let mut hosts = Vec::new();

    for path in &config.config_paths {
        let parsed_hosts = match ssh::parse_config(path) {
            Ok(hosts) => hosts,
            Err(err) => {
                if let ssh::ParseConfigError::Io(io_err) = &err {
                    if io_err.kind() == std::io::ErrorKind::NotFound {
                        if path == "/etc/ssh/ssh_config" {
                            // Ignore missing system-wide SSH configuration file
                            continue;
                        }

                        anyhow::bail!(
                            "SSH configuration file not found: {path}\nCreate it, or pass a different path with -c/--config."
                        );
                    }
                }

                anyhow::bail!("Failed to parse SSH configuration file: {err:?}");
            }
        };

        hosts.extend(parsed_hosts);
    }

    if config.sort_by_name {
        hosts.sort_by_key(|host| host.name.to_lowercase());
    }

    Ok(hosts)
}

fn setup_terminal<B>(terminal: &Rc<RefCell<Terminal<B>>>) -> Result<()>
where
    B: Backend + std::io::Write,
    <B as Backend>::Error: Send + Sync + 'static,
{
    let mut terminal = terminal.borrow_mut();

    // setup terminal
    enable_raw_mode()?;
    execute!(terminal.backend_mut(), Hide, EnterAlternateScreen)?;

    Ok(())
}

fn restore_terminal<B>(terminal: &Rc<RefCell<Terminal<B>>>) -> Result<()>
where
    B: Backend + std::io::Write,
    <B as Backend>::Error: Send + Sync + 'static,
{
    let mut terminal = terminal.borrow_mut();
    terminal.clear()?;

    // restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), Show, LeaveAlternateScreen)?;

    Ok(())
}

fn ui(f: &mut Frame, app: &mut App) {
    let total_height = f.area().height;
    let total_width = f.area().width;

    let show_full_banner = !app.config.no_ascii_art
        && app.ascii_art_style != AsciiArtStyle::Off
        && app.ascii_art_style != AsciiArtStyle::Mini
        && total_height >= 22
        && total_width >= 60;

    let show_mini_banner = !show_full_banner
        && !app.config.no_ascii_art
        && app.ascii_art_style != AsciiArtStyle::Off
        && total_height >= 14;

    let banner_height = if show_full_banner {
        app.ascii_art_style.required_height()
    } else if show_mini_banner {
        1
    } else {
        0
    };

    let constraints = if banner_height > 0 {
        vec![
            Constraint::Length(banner_height),
            Constraint::Length(3),
            Constraint::Min(4),
            Constraint::Length(3),
        ]
    } else {
        vec![
            Constraint::Length(3),
            Constraint::Min(4),
            Constraint::Length(3),
        ]
    };

    let rects = Layout::vertical(constraints).split(f.area());

    let (banner_rect, search_rect, table_rect, footer_rect) = if banner_height > 0 {
        (Some(rects[0]), rects[1], rects[2], rects[3])
    } else {
        (None, rects[0], rects[1], rects[2])
    };

    if let Some(b_rect) = banner_rect {
        if show_full_banner {
            render_full_banner(f, app, b_rect);
        } else if show_mini_banner {
            let mini = ascii_art::render_mini_banner(
                &app.theme,
                app.hosts.non_filtered_iter().count(),
                app.hosts.len(),
                app.anim_tick,
                app.animate,
            );
            f.render_widget(Paragraph::new(mini).centered(), b_rect);
        }
    }

    render_searchbar(f, app, search_rect);
    render_table(f, app, table_rect);
    render_footer(f, app, footer_rect);

    // Set cursor in search bar
    if !app.show_help_modal && !app.show_details_modal {
        let mut cursor_position = search_rect.as_position();
        cursor_position.x += u16::try_from(app.search.cursor()).unwrap_or_default() + 5;
        cursor_position.y += 1;
        f.set_cursor_position(cursor_position);
    }

    // Render modals if active
    if app.show_details_modal {
        render_details_modal(f, app);
    } else if app.show_help_modal {
        render_help_modal(f, app);
    }
}

fn render_full_banner(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::horizontal([
        Constraint::Min(32),
        Constraint::Length(38),
    ])
    .split(area);

    // Render ASCII banner with smooth animated gradient
    #[allow(clippy::cast_precision_loss)]
    let phase = if app.animate {
        (app.anim_tick as f32) * 0.02
    } else {
        0.0
    };
    let banner_lines = ascii_art::render_banner_lines(app.ascii_art_style, &app.theme, phase);
    let banner_widget = Paragraph::new(banner_lines);
    f.render_widget(banner_widget, chunks[0]);

    // Render Info Card on the right
    let total_hosts = app.hosts.non_filtered_iter().count();
    let filtered_hosts = app.hosts.len();
    let config_path_display = app
        .config
        .config_paths
        .last()
        .map(|s| {
            if s.len() > 24 {
                format!("...{}", &s[s.len() - 21..])
            } else {
                s.clone()
            }
        })
        .unwrap_or_else(|| "~/.ssh/config".to_string());

    let info_lines = vec![
        Line::from(vec![
            Span::styled("⚡ SSHS ", Style::default().fg(app.theme.primary).add_modifier(Modifier::BOLD)),
            Span::styled(format!("v{}", env!("CARGO_PKG_VERSION")), Style::default().fg(app.theme.muted)),
            Span::raw("  "),
            Span::styled(
                format!("[ {filtered_hosts}/{total_hosts} Hosts ]"),
                app.theme.badge_style(),
            ),
        ]),
        Line::from(vec![
            Span::styled("🎨 Theme: ", Style::default().fg(app.theme.muted)),
            Span::styled(app.theme.display_name, Style::default().fg(app.theme.secondary).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("📁 File:  ", Style::default().fg(app.theme.muted)),
            Span::styled(config_path_display, Style::default().fg(app.theme.accent)),
        ]),
    ];

    let info_widget = Paragraph::new(info_lines).alignment(Alignment::Right);
    f.render_widget(info_widget, chunks[1]);
}

fn render_searchbar(f: &mut Frame, app: &mut App, area: Rect) {
    let search_val = app.search.value();
    let is_empty = search_val.is_empty();

    let total = app.hosts.non_filtered_iter().count();
    let count = app.hosts.len();

    let badge_text = format!(" [ {count}/{total} ] ");

    let search_line = if is_empty {
        let icon = if app.animate {
            ascii_art::get_spinner_frame(app.anim_tick)
        } else {
            "⚡"
        };
        Line::from(vec![
            Span::styled(format!(" {icon} "), Style::default().fg(app.theme.search_icon_fg).add_modifier(Modifier::BOLD)),
            Span::styled(
                "Type to search hosts (e.g. prod, 192.168, user@)...",
                Style::default().fg(app.theme.muted).add_modifier(Modifier::ITALIC),
            ),
        ])
    } else {
        Line::from(vec![
            Span::styled(" 🔍 ", Style::default().fg(app.theme.search_icon_fg).add_modifier(Modifier::BOLD)),
            Span::styled(
                search_val,
                Style::default().fg(app.theme.search_text_fg).add_modifier(Modifier::BOLD),
            ),
        ])
    };

    let searchbar = Paragraph::new(search_line).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(app.theme.active_border_style())
            .border_type(BorderType::Rounded)
            .title(Line::from(Span::styled(" Search ", Style::default().fg(app.theme.primary).add_modifier(Modifier::BOLD))))
            .title(Line::from(Span::styled(badge_text, app.theme.badge_style())).alignment(Alignment::Right)),
    );
    f.render_widget(searchbar, area);
}

fn render_table(f: &mut Frame, app: &mut App, area: Rect) {
    // The visible row count: the area minus the two border lines and the header.
    app.page_step = max(usize::from(area.height.saturating_sub(3)), 1);

    if app.hosts.is_empty() {
        let empty_lines = ascii_art::render_empty_state_lines(&app.theme, app.search.value(), app.anim_tick);
        let empty_widget = Paragraph::new(empty_lines).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(app.theme.border_style())
                .border_type(BorderType::Rounded)
                .title(Line::from(Span::styled(" Hosts ", Style::default().fg(app.theme.primary).add_modifier(Modifier::BOLD)))),
        );
        f.render_widget(empty_widget, area);
        return;
    }

    let header_style = app.theme.table_header_style();
    let selected_style = app.theme.selected_row_style();

    let mut header_names = vec!["Host", "Aliases", "User", "Destination", "Port"];
    if app.config.show_proxy_command {
        header_names.push("Proxy");
    }

    let header = header_names
        .iter()
        .copied()
        .map(Cell::from)
        .collect::<Row>()
        .style(header_style)
        .height(1);

    let rows = app.hosts.iter().map(|host| {
        let name_cell = Cell::from(Text::from(Line::from(vec![
            Span::styled("● ", Style::default().fg(app.theme.primary)),
            Span::styled(host.name.clone(), Style::default().fg(app.theme.host_name).add_modifier(Modifier::BOLD)),
        ])));

        let aliases_cell = Cell::from(Text::from(Span::styled(
            host.aliases.clone(),
            Style::default().fg(app.theme.aliases),
        )));

        let user_cell = Cell::from(Text::from(Span::styled(
            host.user.clone().unwrap_or_default(),
            Style::default().fg(app.theme.user),
        )));

        let dest_cell = Cell::from(Text::from(Span::styled(
            host.destination.clone(),
            Style::default().fg(app.theme.destination),
        )));

        let port_cell = Cell::from(Text::from(Span::styled(
            host.port.clone().unwrap_or_default(),
            Style::default().fg(app.theme.port),
        )));

        let mut cells = vec![name_cell, aliases_cell, user_cell, dest_cell, port_cell];

        if app.config.show_proxy_command {
            cells.push(Cell::from(Text::from(Span::styled(
                host.proxy_command.clone().unwrap_or_default(),
                Style::default().fg(app.theme.proxy),
            ))));
        }

        Row::new(cells)
    });

    let cursor_bar = "▌ ";
    let t = Table::new(rows, app.table_columns_constraints.clone())
        .header(header)
        .row_highlight_style(selected_style)
        .highlight_symbol(Text::from(vec![
            "".into(),
            cursor_bar.into(),
            cursor_bar.into(),
            "".into(),
        ]))
        .highlight_spacing(HighlightSpacing::Always)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(app.theme.border_style())
                .border_type(BorderType::Rounded)
                .title(Line::from(Span::styled(" Hosts ", Style::default().fg(app.theme.primary).add_modifier(Modifier::BOLD)))),
        );

    f.render_stateful_widget(t, area, &mut app.table_state);
}

fn render_footer(f: &mut Frame, app: &mut App, area: Rect) {
    // Check if temporary status message is active (< 3 seconds)
    let is_recent_status = if let Some((msg, timestamp)) = &app.status_message {
        if timestamp.elapsed() < Duration::from_secs(3) {
            Some(msg.clone())
        } else {
            None
        }
    } else {
        None
    };

    let footer_line = if let Some(msg) = is_recent_status {
        Line::from(vec![
            Span::styled(" ℹ ", Style::default().fg(app.theme.secondary).add_modifier(Modifier::BOLD)),
            Span::styled(msg, Style::default().fg(app.theme.header_fg).add_modifier(Modifier::BOLD)),
        ])
    } else {
        Line::from(vec![
            Span::styled(" Enter ", app.theme.key_badge_style()),
            Span::styled(" Connect  ", app.theme.key_desc_style()),
            Span::styled(" Tab ", app.theme.key_badge_style()),
            Span::styled(" Details  ", app.theme.key_desc_style()),
            Span::styled(" ^T ", app.theme.key_badge_style()),
            Span::styled(" Theme  ", app.theme.key_desc_style()),
            Span::styled(" ^R ", app.theme.key_badge_style()),
            Span::styled(" Reload  ", app.theme.key_desc_style()),
            Span::styled(" ? ", app.theme.key_badge_style()),
            Span::styled(" Help  ", app.theme.key_desc_style()),
            Span::styled(" Esc ", app.theme.key_badge_style()),
            Span::styled(" Quit ", app.theme.key_desc_style()),
        ])
    };

    let info_footer = Paragraph::new(footer_line).centered().block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(app.theme.border_style())
            .border_type(BorderType::Rounded),
    );
    f.render_widget(info_footer, area);
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

fn render_details_modal(f: &mut Frame, app: &App) {
    let area = centered_rect(70, 75, f.area());
    f.render_widget(Clear, area);

    let selected = app.table_state.selected().unwrap_or(0);
    if selected >= app.hosts.len() {
        return;
    }

    let host = &app.hosts[selected];
    let rendered_cmd = host
        .render_command_template(&app.config.command_template)
        .unwrap_or_else(|_| format!("ssh \"{}\"", host.name));

    let mut lines = vec![
        Line::raw(""),
        Line::from(vec![
            Span::styled("  Host Name:        ", Style::default().fg(app.theme.muted)),
            Span::styled(&host.name, Style::default().fg(app.theme.primary).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("  Aliases:          ", Style::default().fg(app.theme.muted)),
            Span::styled(if host.aliases.is_empty() { "(none)" } else { &host.aliases }, Style::default().fg(app.theme.aliases)),
        ]),
        Line::from(vec![
            Span::styled("  Destination:      ", Style::default().fg(app.theme.muted)),
            Span::styled(&host.destination, Style::default().fg(app.theme.destination).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("  User:             ", Style::default().fg(app.theme.muted)),
            Span::styled(host.user.as_deref().unwrap_or("(current user)"), Style::default().fg(app.theme.user)),
        ]),
        Line::from(vec![
            Span::styled("  Port:             ", Style::default().fg(app.theme.muted)),
            Span::styled(host.port.as_deref().unwrap_or("22 (default)"), Style::default().fg(app.theme.port)),
        ]),
    ];

    if let Some(identity) = &host.identity_file {
        lines.push(Line::from(vec![
            Span::styled("  Identity File:    ", Style::default().fg(app.theme.muted)),
            Span::styled(identity, Style::default().fg(app.theme.accent)),
        ]));
    }

    if let Some(proxy_jump) = &host.proxy_jump {
        lines.push(Line::from(vec![
            Span::styled("  Proxy Jump:       ", Style::default().fg(app.theme.muted)),
            Span::styled(proxy_jump, Style::default().fg(app.theme.proxy)),
        ]));
    }

    if let Some(proxy_cmd) = &host.proxy_command {
        lines.push(Line::from(vec![
            Span::styled("  Proxy Command:    ", Style::default().fg(app.theme.muted)),
            Span::styled(proxy_cmd, Style::default().fg(app.theme.proxy)),
        ]));
    }

    lines.push(Line::raw(""));
    lines.push(Line::from(vec![
        Span::styled("  Command To Execute:", Style::default().fg(app.theme.secondary).add_modifier(Modifier::BOLD)),
    ]));
    lines.push(Line::from(vec![
        Span::styled(format!("    ❯ {rendered_cmd}"), Style::default().fg(app.theme.header_fg).add_modifier(Modifier::BOLD)),
    ]));
    lines.push(Line::raw(""));
    lines.push(
        Line::from(vec![
            Span::styled(" [ Enter ] ", app.theme.key_badge_style()),
            Span::styled(" Connect   ", app.theme.key_desc_style()),
            Span::styled(" [ ↑/↓ ] ", app.theme.key_badge_style()),
            Span::styled(" Next/Prev Host   ", app.theme.key_desc_style()),
            Span::styled(" [ Esc / Tab ] ", app.theme.key_badge_style()),
            Span::styled(" Close Inspector", app.theme.key_desc_style()),
        ])
        .centered(),
    );

    let modal_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(app.theme.active_border_style())
        .title(Line::from(Span::styled(
            format!(" 🖥 Host Details: {} ", host.name),
            Style::default().fg(app.theme.primary).add_modifier(Modifier::BOLD),
        )));

    let widget = Paragraph::new(lines).block(modal_block);
    f.render_widget(widget, area);
}

fn render_help_modal(f: &mut Frame, app: &App) {
    let area = centered_rect(72, 80, f.area());
    f.render_widget(Clear, area);

    let mut lines = ascii_art::render_help_header(&app.theme);
    lines.push(Line::raw(""));

    let shortcuts: &[(&str, &[(&str, &str)])] = &[
        ("Navigation", &[
            ("↑ / ↓, k / j", "Move selection up / down"),
            ("PageUp / PageDn", "Scroll one page up / down"),
            ("Home / End", "Jump to top / bottom of list"),
        ]),
        ("Actions", &[
            ("Enter", "Connect to selected SSH host"),
            ("Tab", "Open Host Inspector & Details modal"),
            ("Type text", "Fuzzy search and filter hosts"),
            ("Esc", "Clear search query or quit"),
        ]),
        ("Customization & Tools", &[
            ("Ctrl+T / F2", "Cycle visual theme dynamically"),
            ("Ctrl+S", "Save active theme to ~/.config/sshs/config.toml"),
            ("Ctrl+A", "Toggle visual animations on / off"),
            ("Ctrl+R", "Reload SSH config files from disk"),
            ("Ctrl+U", "Clear the entire search query"),
            ("? / F1 / Ctrl+H", "Toggle this help dialog"),
        ]),
    ];

    for (section, items) in shortcuts {
        lines.push(Line::from(vec![
            Span::styled(format!("  ── {section} ──"), Style::default().fg(app.theme.secondary).add_modifier(Modifier::BOLD)),
        ]));

        for (key, desc) in *items {
            lines.push(Line::from(vec![
                Span::styled(format!("    {key:<18}"), app.theme.key_badge_style()),
                Span::styled(format!(" {desc}"), Style::default().fg(app.theme.header_fg)),
            ]));
        }
        lines.push(Line::raw(""));
    }

    lines.push(
        Line::from(vec![
            Span::styled(" [ Esc / ? ] ", app.theme.key_badge_style()),
            Span::styled(" Close Help ", app.theme.key_desc_style()),
        ])
        .centered(),
    );

    let modal_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(app.theme.active_border_style())
        .title(Line::from(Span::styled(
            " ❓ Keyboard Shortcuts & Help ",
            Style::default().fg(app.theme.primary).add_modifier(Modifier::BOLD),
        )));

    let widget = Paragraph::new(lines).block(modal_block);
    f.render_widget(widget, area);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> AppConfig {
        AppConfig {
            config_paths: vec![crate::test_support::testdata("search_selection.conf")
                .to_string_lossy()
                .into_owned()],
            search_filter: None,
            color: "blue".to_string(),
            ascii_art: "slant".to_string(),
            no_ascii_art: false,
            sort_by_name: false,
            sort_by_score: false,
            show_proxy_command: false,
            animate: false,
            command_template: r#"ssh "{{{name}}}""#.to_string(),
            command_template_on_session_start: None,
            command_template_on_session_end: None,
            exit_after_ssh_session_ends: false,
        }
    }

    fn type_char(app: &mut App, c: char) {
        let ev = Event::Key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
        app.handle_search_event(&ev);
    }

    #[test]
    fn test_reload_hosts_picks_up_file_changes() {
        let path = std::env::temp_dir().join("sshs_test_reload.conf");
        std::fs::write(&path, "Host first\n  Hostname first.example.com\n").unwrap();

        let mut config = test_config();
        config.config_paths = vec![path.to_string_lossy().into_owned()];

        let mut app = App::new(&config).unwrap();
        assert_eq!(app.hosts.len(), 1);

        std::fs::write(
            &path,
            "Host first\n  Hostname first.example.com\nHost second\n  Hostname second.example.com\n",
        )
        .unwrap();
        app.reload_hosts();
        std::fs::remove_file(&path).ok();

        assert_eq!(app.hosts.len(), 2);
        assert_eq!(app.table_state.selected(), Some(0));
    }

    #[test]
    fn test_table_columns_constraints_are_computed() {
        let app = App::new(&test_config()).unwrap();

        assert_eq!(app.table_columns_constraints.len(), 5);
        assert!(matches!(
            app.table_columns_constraints[0],
            Constraint::Length(len) if len > 1
        ));
    }

    /// Regression test for <https://github.com/quantumsheep/sshs/issues/120>:
    /// a missing SSH config file used to surface a raw `Io(Os { .. })` debug
    /// error; it should now explain what's missing and how to fix it.
    #[test]
    fn test_missing_config_file_gives_actionable_error() {
        let mut config = test_config();
        config.config_paths = vec!["/nonexistent/path/to/config".to_string()];

        let message = match App::new(&config) {
            Ok(_) => panic!("expected App::new to fail for a missing config file"),
            Err(err) => err.to_string(),
        };

        assert!(
            message.contains("/nonexistent/path/to/config"),
            "error should mention the missing path: {message}"
        );
        assert!(
            !message.contains("Os {"),
            "error should not leak a raw Debug-formatted io::Error: {message}"
        );
    }

    /// Regression test for <https://github.com/quantumsheep/sshs/issues/154>:
    /// typing a search query used to keep the previously selected row index,
    /// which could point at an unrelated host once the list was refiltered.
    #[test]
    fn test_search_resets_selection_to_top_match() {
        let config = test_config();
        let mut app = App::new(&config).unwrap();

        // Sanity check: all 4 hosts are listed in file order before searching.
        assert_eq!(app.hosts.len(), 4);

        // Select the 2nd row ("other"), which won't match the search below.
        app.next();
        assert_eq!(app.table_state.selected(), Some(1));

        for c in "match".chars() {
            type_char(&mut app, c);
        }

        assert_eq!(app.hosts.len(), 3);
        assert_eq!(app.hosts.iter().next().unwrap().name, "match1");

        // The stale index (1) would previously stay selected, highlighting
        // "match2" instead of resetting to the top match "match1".
        assert_eq!(app.table_state.selected(), Some(0));
    }

    #[test]
    fn test_theme_cycling() {
        let config = test_config();
        let mut app = App::new(&config).unwrap();
        let initial_theme = app.theme.name;
        app.cycle_theme();
        assert_ne!(app.theme.name, initial_theme);
    }
}
