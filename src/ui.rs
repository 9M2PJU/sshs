use anyhow::Result;
use crossterm::{
    cursor::{Hide, Show},
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind,
        KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
#[allow(clippy::wildcard_imports)]
use ratatui::{prelude::*, widgets::*};
use std::{
    cell::RefCell,
    cmp::{max, min},
    io,
    process::Command,
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

#[derive(Clone, Debug, Default)]
pub struct AddHostForm {
    pub name: Input,
    pub hostname: Input,
    pub user: Input,
    pub port: Input,
    pub identity_file: Input,
    pub proxy_jump: Input,
    pub active_field: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TunnelMode {
    Local,     // -L local:remote
    Remote,    // -R remote:local
    Dynamic,   // -D local_socks
    VpnTun,    // -w any:any TUN/TAP VPN
    Tor,       // Tor SOCKS proxy routing
    PortKnock, // Port knock sequence before connect
    X11,       // X11 GUI Forwarding (-X / -Y)
    Pqc,       // Post-Quantum Cryptography (ML-KEM / sntrup761)
}

impl TunnelMode {
    #[must_use]
    pub fn next(self) -> Self {
        match self {
            Self::Local => Self::Remote,
            Self::Remote => Self::Dynamic,
            Self::Dynamic => Self::VpnTun,
            Self::VpnTun => Self::Tor,
            Self::Tor => Self::PortKnock,
            Self::PortKnock => Self::X11,
            Self::X11 => Self::Pqc,
            Self::Pqc => Self::Local,
        }
    }

    #[must_use]
    pub fn prev(self) -> Self {
        match self {
            Self::Local => Self::Pqc,
            Self::Remote => Self::Local,
            Self::Dynamic => Self::Remote,
            Self::VpnTun => Self::Dynamic,
            Self::Tor => Self::VpnTun,
            Self::PortKnock => Self::Tor,
            Self::X11 => Self::PortKnock,
            Self::Pqc => Self::X11,
        }
    }

    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Self::Local => "Local (-L)",
            Self::Remote => "Remote (-R)",
            Self::Dynamic => "SOCKS5 (-D)",
            Self::VpnTun => "TUN/TAP (-w)",
            Self::Tor => "Tor Onion",
            Self::PortKnock => "Port Knock",
            Self::X11 => "X11 GUI (-X/-Y)",
            Self::Pqc => "Post-Quantum (PQC)",
        }
    }
}

#[derive(Clone, Debug)]
pub struct TunnelForm {
    pub host: ssh::Host,
    pub mode: TunnelMode,
    pub local_port: Input,
    pub remote_target: Input,
    pub background: bool,
    pub active_field: usize,
}

#[derive(Clone, Debug)]
pub struct PostConnectPrompt {
    pub host_name: String,
    pub destination: String,
    pub user: Option<String>,
    pub port: Option<String>,
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
    show_add_host_modal: bool,
    show_tunnel_modal: bool,
    show_delete_modal: Option<String>,
    add_host_form: AddHostForm,
    tunnel_form: Option<TunnelForm>,
    pub sftp_explorer: Option<crate::sftp_explorer::SftpExplorer>,
    post_connect_prompt: Option<PostConnectPrompt>,
    status_message: Option<(String, Instant)>,
    anim_tick: u64,
    animate: bool,
}

#[derive(PartialEq, Eq, Debug)]
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
            show_add_host_modal: false,
            show_tunnel_modal: false,
            show_delete_modal: None,
            add_host_form: AddHostForm::default(),
            tunnel_form: None,
            sftp_explorer: None,
            post_connect_prompt: None,
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

                match ev {
                    Event::Key(key) => {
                        if key.kind == KeyEventKind::Press {
                            let action = self.on_key_press(terminal, key, &ev)?;
                            match action {
                                AppKeyAction::Ok => continue,
                                AppKeyAction::Stop => break,
                                AppKeyAction::Continue => {}
                            }
                        }

                        if !self.show_help_modal
                            && !self.show_details_modal
                            && !self.show_add_host_modal
                            && !self.show_tunnel_modal
                            && self.show_delete_modal.is_none()
                            && self.post_connect_prompt.is_none()
                            && self.sftp_explorer.is_none()
                        {
                            self.handle_search_event(&ev);
                        }
                    }
                    Event::Mouse(mouse) => {
                        let action = self.on_mouse_event(terminal, mouse)?;
                        match action {
                            AppKeyAction::Ok => continue,
                            AppKeyAction::Stop => break,
                            AppKeyAction::Continue => {}
                        }
                    }
                    Event::Resize(_w, _h) => {
                        let _ = terminal.borrow_mut().clear();
                        terminal.borrow_mut().draw(|f| ui(f, self))?;
                    }
                    Event::FocusGained => {
                        let _ = terminal.borrow_mut().clear();
                    }
                    _ => {}
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
        ev: &Event,
    ) -> Result<AppKeyAction>
    where
        B: Backend + std::io::Write,
        <B as Backend>::Error: Send + Sync + 'static,
    {
        #[allow(clippy::enum_glob_use)]
        use KeyCode::*;

        let is_ctrl_pressed = key.modifiers.contains(KeyModifiers::CONTROL);

        // 0. SSHS Explorer View (Dual-Pane TUI)
        if let Some(explorer) = &mut self.sftp_explorer {
            if explorer.active_transfer.is_some() {
                if key.code == Esc || key.code == Char('q') {
                    explorer.active_transfer = None;
                    explorer.set_status("Transfer cancelled by user".to_string());
                }
                return Ok(AppKeyAction::Ok);
            }

            if explorer.viewer_content.is_some() {
                match key.code {
                    Esc | Char('q') | Enter => {
                        explorer.viewer_content = None;
                        return Ok(AppKeyAction::Ok);
                    }
                    _ => return Ok(AppKeyAction::Ok),
                }
            }

            if let Some(input) = &mut explorer.mkdir_input {
                match key.code {
                    Esc => {
                        explorer.mkdir_input = None;
                        return Ok(AppKeyAction::Ok);
                    }
                    Enter => {
                        let name = input.value().to_string();
                        explorer.mkdir_input = None;
                        explorer.make_directory(&name);
                        return Ok(AppKeyAction::Ok);
                    }
                    _ => {
                        input.handle_event(ev);
                        return Ok(AppKeyAction::Ok);
                    }
                }
            }

            if let Some(menu) = &mut explorer.context_menu {
                match key.code {
                    Esc | Char('q') => {
                        explorer.context_menu = None;
                        return Ok(AppKeyAction::Ok);
                    }
                    Down | Char('j') => {
                        menu.selected_index = (menu.selected_index + 1) % 7;
                        return Ok(AppKeyAction::Ok);
                    }
                    Up | Char('k') => {
                        menu.selected_index = (menu.selected_index + 6) % 7;
                        return Ok(AppKeyAction::Ok);
                    }
                    Enter => {
                        let opt = menu.selected_index;
                        explorer.execute_context_menu_option(opt);
                        return Ok(AppKeyAction::Ok);
                    }
                    Char('1') => {
                        explorer.execute_context_menu_option(0);
                        return Ok(AppKeyAction::Ok);
                    }
                    Char('2') => {
                        explorer.execute_context_menu_option(1);
                        return Ok(AppKeyAction::Ok);
                    }
                    Char('3') => {
                        explorer.execute_context_menu_option(2);
                        return Ok(AppKeyAction::Ok);
                    }
                    Char('4') => {
                        explorer.execute_context_menu_option(3);
                        return Ok(AppKeyAction::Ok);
                    }
                    Char('5') => {
                        explorer.execute_context_menu_option(4);
                        return Ok(AppKeyAction::Ok);
                    }
                    Char('6') => {
                        explorer.execute_context_menu_option(5);
                        return Ok(AppKeyAction::Ok);
                    }
                    Char('7') => {
                        explorer.context_menu = None;
                        return Ok(AppKeyAction::Ok);
                    }
                    _ => return Ok(AppKeyAction::Ok),
                }
            }

            match key.code {
                Esc | Char('q') => {
                    self.sftp_explorer = None;
                    return Ok(AppKeyAction::Ok);
                }
                Tab => {
                    explorer.switch_pane();
                    return Ok(AppKeyAction::Ok);
                }
                Down | Char('j') => {
                    explorer.select_next();
                    return Ok(AppKeyAction::Ok);
                }
                Up | Char('k') => {
                    explorer.select_previous();
                    return Ok(AppKeyAction::Ok);
                }
                Enter => {
                    explorer.enter_directory();
                    return Ok(AppKeyAction::Ok);
                }
                F(3) | Char('v') => {
                    explorer.view_selected();
                    return Ok(AppKeyAction::Ok);
                }
                F(5) | Char('c') => {
                    explorer.copy_or_transfer();
                    return Ok(AppKeyAction::Ok);
                }
                F(7) => {
                    explorer.mkdir_input = Some("new_folder".into());
                    return Ok(AppKeyAction::Ok);
                }
                F(8) | Char('d') | Delete => {
                    explorer.delete_selected();
                    return Ok(AppKeyAction::Ok);
                }
                Char('m') => {
                    explorer.open_context_menu_at(20, 8);
                    return Ok(AppKeyAction::Ok);
                }
                Char('r') => {
                    explorer.refresh_local();
                    explorer.refresh_remote();
                    explorer.set_status("Refreshed directory listings".to_string());
                    return Ok(AppKeyAction::Ok);
                }
                F(9) | Char('o') => {
                    explorer.open_external();
                    return Ok(AppKeyAction::Ok);
                }
                _ => return Ok(AppKeyAction::Ok),
            }
        }

        // 1. Delete Confirmation Modal
        if let Some(host_name) = self.show_delete_modal.clone() {
            match key.code {
                Char('y' | 'Y') | Enter => {
                    self.show_delete_modal = None;
                    self.show_details_modal = false;
                    match crate::config::remove_host_from_ssh_config(&host_name) {
                        Ok(true) => {
                            self.reload_hosts();
                            self.set_status_message(format!("Removed host '{host_name}' from ~/.ssh/config!"));
                        }
                        Ok(false) => {
                            self.set_status_message(format!("Host '{host_name}' not found in ~/.ssh/config"));
                        }
                        Err(e) => {
                            self.set_status_message(format!("Error deleting host: {e}"));
                        }
                    }
                    return Ok(AppKeyAction::Ok);
                }
                Char('n' | 'N') | Esc => {
                    self.show_delete_modal = None;
                    return Ok(AppKeyAction::Ok);
                }
                _ => return Ok(AppKeyAction::Ok),
            }
        }

        // 2. Advanced SSH Networking / Tunnel / TOR / Port Knocking Modal
        if self.show_tunnel_modal {
            if let Some(form) = &mut self.tunnel_form {
                match key.code {
                    Esc => {
                        self.show_tunnel_modal = false;
                        return Ok(AppKeyAction::Ok);
                    }
                    Char('1') => {
                        form.mode = TunnelMode::Local;
                        form.local_port = "8080".into();
                        form.remote_target = "localhost:80".into();
                        return Ok(AppKeyAction::Ok);
                    }
                    Char('2') => {
                        form.mode = TunnelMode::Remote;
                        form.local_port = "9000".into();
                        form.remote_target = "localhost:3000".into();
                        return Ok(AppKeyAction::Ok);
                    }
                    Char('3') => {
                        form.mode = TunnelMode::Dynamic;
                        form.local_port = "1080".into();
                        return Ok(AppKeyAction::Ok);
                    }
                    Char('4') => {
                        form.mode = TunnelMode::VpnTun;
                        form.remote_target = "any:any".into();
                        return Ok(AppKeyAction::Ok);
                    }
                    Char('5') => {
                        form.mode = TunnelMode::Tor;
                        form.local_port = "127.0.0.1:9050".into();
                        return Ok(AppKeyAction::Ok);
                    }
                    Char('6') => {
                        form.mode = TunnelMode::PortKnock;
                        form.local_port = "7000, 8000, 9000".into();
                        return Ok(AppKeyAction::Ok);
                    }
                    Char('7') => {
                        form.mode = TunnelMode::X11;
                        form.local_port = "trusted".into();
                        form.remote_target = "compressed".into();
                        return Ok(AppKeyAction::Ok);
                    }
                    Char('8') => {
                        form.mode = TunnelMode::Pqc;
                        form.local_port = "mlkem768x25519-sha512@openssh.com,sntrup761x25519-sha512@openssh.com".into();
                        return Ok(AppKeyAction::Ok);
                    }
                    Tab | Down => {
                        form.active_field = (form.active_field + 1) % 4;
                        return Ok(AppKeyAction::Ok);
                    }
                    BackTab | Up => {
                        form.active_field = (form.active_field + 3) % 4;
                        return Ok(AppKeyAction::Ok);
                    }
                    Char(' ') | Left | Right => {
                        if form.active_field == 0 {
                            if key.code == Left {
                                form.mode = form.mode.prev();
                            } else {
                                form.mode = form.mode.next();
                            }
                            return Ok(AppKeyAction::Ok);
                        } else if form.active_field == 3 {
                            form.background = !form.background;
                            return Ok(AppKeyAction::Ok);
                        }
                    }
                    Enter => {
                        let host = form.host.clone();
                        let mode = form.mode;
                        let bg = form.background;
                        let l_val = form.local_port.value().trim().to_string();
                        let r_val = form.remote_target.value().trim().to_string();
                        self.show_tunnel_modal = false;

                        match mode {
                            TunnelMode::Local => {
                                if l_val.is_empty() || r_val.is_empty() {
                                    self.set_status_message("Local port and remote target are required!".to_string());
                                    return Ok(AppKeyAction::Ok);
                                }
                                let spec = format!("{l_val}:{r_val}");
                                restore_terminal(terminal).expect("Failed to restore terminal");
                                println!("\n🚇 Spawning Local Tunnel: ssh -L {spec} {}\n", host.name);
                                let res = host.spawn_tunnel("-L", &spec, bg);
                                setup_terminal(terminal).expect("Failed to setup terminal");
                                match res {
                                    Ok(_) if bg => self.set_status_message(format!("Local tunnel active: -L {spec} via '{}' (Background)", host.name)),
                                    Ok(_) => self.set_status_message(format!("Local tunnel closed: -L {spec}")),
                                    Err(e) => self.set_status_message(format!("Tunnel error: {e}")),
                                }
                            }
                            TunnelMode::Remote => {
                                if l_val.is_empty() || r_val.is_empty() {
                                    self.set_status_message("Remote port and local target are required!".to_string());
                                    return Ok(AppKeyAction::Ok);
                                }
                                let spec = format!("{l_val}:{r_val}");
                                restore_terminal(terminal).expect("Failed to restore terminal");
                                println!("\n🚇 Spawning Remote Tunnel: ssh -R {spec} {}\n", host.name);
                                let res = host.spawn_tunnel("-R", &spec, bg);
                                setup_terminal(terminal).expect("Failed to setup terminal");
                                match res {
                                    Ok(_) if bg => self.set_status_message(format!("Remote tunnel active: -R {spec} via '{}' (Background)", host.name)),
                                    Ok(_) => self.set_status_message(format!("Remote tunnel closed: -R {spec}")),
                                    Err(e) => self.set_status_message(format!("Tunnel error: {e}")),
                                }
                            }
                            TunnelMode::Dynamic => {
                                if l_val.is_empty() {
                                    self.set_status_message("SOCKS proxy port is required!".to_string());
                                    return Ok(AppKeyAction::Ok);
                                }
                                restore_terminal(terminal).expect("Failed to restore terminal");
                                println!("\n🛡️ Spawning Dynamic SOCKS5 Proxy: ssh -D {l_val} {}\n", host.name);
                                let res = host.spawn_tunnel("-D", &l_val, bg);
                                setup_terminal(terminal).expect("Failed to setup terminal");
                                match res {
                                    Ok(_) if bg => self.set_status_message(format!("SOCKS5 proxy active on port {l_val} via '{}' (Background)", host.name)),
                                    Ok(_) => self.set_status_message(format!("SOCKS5 proxy on port {l_val} closed")),
                                    Err(e) => self.set_status_message(format!("SOCKS5 error: {e}")),
                                }
                            }
                            TunnelMode::VpnTun => {
                                let dev = if r_val.is_empty() { "any:any" } else { &r_val };
                                restore_terminal(terminal).expect("Failed to restore terminal");
                                println!("\n🌐 Spawning VPN TUN/TAP Tunnel: ssh -w {dev} {}\n", host.name);
                                let res = host.spawn_vpn_tun(dev, bg);
                                setup_terminal(terminal).expect("Failed to setup terminal");
                                match res {
                                    Ok(_) if bg => self.set_status_message(format!("VPN TUN/TAP tunnel active (-w {dev}) via '{}' (Background)", host.name)),
                                    Ok(_) => self.set_status_message("VPN tunnel session closed".to_string()),
                                    Err(e) => self.set_status_message(format!("VPN error: {e}")),
                                }
                            }
                            TunnelMode::Tor => {
                                let proxy = if l_val.is_empty() { "127.0.0.1:9050" } else { &l_val };
                                restore_terminal(terminal).expect("Failed to restore terminal");
                                println!("\n🧅 Connecting to '{}' over Tor (SOCKS5 {})...\n", host.name, proxy);
                                let res = host.spawn_over_tor(proxy);
                                setup_terminal(terminal).expect("Failed to setup terminal");
                                match res {
                                    Ok(_) => self.set_status_message(format!("Tor SSH session with '{}' closed", host.name)),
                                    Err(e) => self.set_status_message(format!("Tor connection error: {e}")),
                                }
                            }
                            TunnelMode::PortKnock => {
                                let ports: Vec<u16> = l_val
                                    .split([',', ' ', ';'])
                                    .filter_map(|s| s.trim().parse::<u16>().ok())
                                    .collect();
                                if ports.is_empty() {
                                    self.set_status_message("At least one port number is required for knocking!".to_string());
                                    return Ok(AppKeyAction::Ok);
                                }
                                restore_terminal(terminal).expect("Failed to restore terminal");
                                println!("\n🚪 Sending port knock sequence {:?} to '{}'...\n", ports, host.name);
                                let _ = host.perform_port_knock(&ports, 100);
                                println!("Connecting via SSH...\n");
                                let res = host.spawn_command_template(&self.config.command_template);
                                setup_terminal(terminal).expect("Failed to setup terminal");
                                match res {
                                    Ok(_) => self.set_status_message(format!("Port knocked & session with '{}' closed", host.name)),
                                    Err(e) => self.set_status_message(format!("SSH connection error: {e}")),
                                }
                            }
                            TunnelMode::X11 => {
                                let is_untrusted = l_val.eq_ignore_ascii_case("untrusted") || l_val.eq_ignore_ascii_case("standard");
                                let is_trusted = !is_untrusted;
                                let is_compress = !r_val.eq_ignore_ascii_case("no") && !r_val.eq_ignore_ascii_case("false") && !r_val.eq_ignore_ascii_case("none");
                                let flag = if is_trusted { "-Y" } else { "-X" };
                                let c_flag = if is_compress { " -C" } else { "" };
                                restore_terminal(terminal).expect("Failed to restore terminal");
                                println!("\n🖥️ Launching X11 GUI Forwarding Session: ssh {flag}{c_flag} {}\n", host.name);
                                let res = host.spawn_x11(is_trusted, is_compress);
                                setup_terminal(terminal).expect("Failed to setup terminal");
                                match res {
                                    Ok(_) => self.set_status_message(format!("X11 session ({flag}) with '{}' closed", host.name)),
                                    Err(e) => self.set_status_message(format!("X11 connection error: {e}")),
                                }
                            }
                            TunnelMode::Pqc => {
                                let kex = if l_val.is_empty() {
                                    "mlkem768x25519-sha512@openssh.com,sntrup761x25519-sha512@openssh.com".to_string()
                                } else {
                                    l_val.clone()
                                };
                                restore_terminal(terminal).expect("Failed to restore terminal");
                                println!("\n⚛️ Launching Post-Quantum (PQC) Session to '{}'...\n", host.name);
                                println!("KEX: {kex}\n");
                                let mut cmd = Command::new("ssh");
                                cmd.arg("-o").arg(format!("KexAlgorithms={kex}"));
                                if let Some(p) = &host.port {
                                    if !p.is_empty() && p != "22" {
                                        cmd.arg("-p").arg(p);
                                    }
                                }
                                cmd.arg(&host.name);
                                let res = cmd.status();
                                setup_terminal(terminal).expect("Failed to setup terminal");
                                match res {
                                    Ok(_) => self.set_status_message(format!("Post-Quantum session with '{}' closed", host.name)),
                                    Err(e) => self.set_status_message(format!("PQC connection error: {e}")),
                                }
                            }
                        }
                        return Ok(AppKeyAction::Ok);
                    }
                    _ => {
                        if form.active_field == 1 {
                            form.local_port.handle_event(ev);
                        } else if form.active_field == 2 {
                            form.remote_target.handle_event(ev);
                        }
                        return Ok(AppKeyAction::Ok);
                    }
                }
            }
        }

        // 3. Post-Connect Passwordless Setup Modal
        if let Some(prompt) = self.post_connect_prompt.clone() {
            match key.code {
                Char('y' | 'Y') => {
                    self.post_connect_prompt = None;
                    self.run_ssh_copy_id(terminal, &prompt)?;
                    return Ok(AppKeyAction::Ok);
                }
                Char('n' | 'N') | Esc | Enter => {
                    self.post_connect_prompt = None;
                    return Ok(AppKeyAction::Ok);
                }
                _ => return Ok(AppKeyAction::Ok),
            }
        }

        // 4. Add New Host Modal
        if self.show_add_host_modal {
            match key.code {
                Esc => {
                    self.show_add_host_modal = false;
                    return Ok(AppKeyAction::Ok);
                }
                Tab | Down => {
                    self.add_host_form.active_field = (self.add_host_form.active_field + 1) % 6;
                    return Ok(AppKeyAction::Ok);
                }
                BackTab | Up => {
                    self.add_host_form.active_field = (self.add_host_form.active_field + 5) % 6;
                    return Ok(AppKeyAction::Ok);
                }
                Enter => {
                    let name = self.add_host_form.name.value().trim().to_string();
                    let hostname = self.add_host_form.hostname.value().trim().to_string();
                    if name.is_empty() || hostname.is_empty() {
                        self.set_status_message("Host Alias and HostName (IP) are required!".to_string());
                        return Ok(AppKeyAction::Ok);
                    }

                    let user_val = self.add_host_form.user.value().trim();
                    let port_val = self.add_host_form.port.value().trim();
                    let id_val = self.add_host_form.identity_file.value().trim();
                    let pj_val = self.add_host_form.proxy_jump.value().trim();

                    match crate::config::append_host_to_ssh_config(
                        &name,
                        &hostname,
                        if user_val.is_empty() { None } else { Some(user_val) },
                        if port_val.is_empty() { None } else { Some(port_val) },
                        if id_val.is_empty() { None } else { Some(id_val) },
                        if pj_val.is_empty() { None } else { Some(pj_val) },
                    ) {
                        Ok(_) => {
                            self.reload_hosts();
                            self.show_add_host_modal = false;
                            self.set_status_message(format!("Added host '{name}' to ~/.ssh/config!"));
                        }
                        Err(e) => {
                            self.set_status_message(format!("Error saving host: {e}"));
                        }
                    }
                    return Ok(AppKeyAction::Ok);
                }
                _ => {
                    let active_input = match self.add_host_form.active_field {
                        0 => &mut self.add_host_form.name,
                        1 => &mut self.add_host_form.hostname,
                        2 => &mut self.add_host_form.user,
                        3 => &mut self.add_host_form.port,
                        4 => &mut self.add_host_form.identity_file,
                        _ => &mut self.add_host_form.proxy_jump,
                    };
                    active_input.handle_event(ev);
                    return Ok(AppKeyAction::Ok);
                }
            }
        }

        // 5. Ctrl Key Shortcuts
        if is_ctrl_pressed {
            let action = self.on_key_press_ctrl(terminal, key);
            if action != AppKeyAction::Continue {
                return Ok(action);
            }
        }

        // 6. Help Modal
        if self.show_help_modal {
            match key.code {
                Esc | Char('q') | Enter => {
                    self.show_help_modal = false;
                    return Ok(AppKeyAction::Ok);
                }
                _ => return Ok(AppKeyAction::Ok),
            }
        }

        // 7. Details Inspector Modal
        if self.show_details_modal {
            match key.code {
                Esc | Tab | Char('q') => {
                    self.show_details_modal = false;
                    return Ok(AppKeyAction::Ok);
                }
                Char('s' | 'f') => {
                    self.show_details_modal = false;
                    return self.launch_sftp_for_selected_host(terminal);
                }
                Char('x') => {
                    self.show_details_modal = false;
                    return self.launch_x11_for_selected_host(terminal);
                }
                Char('t' | 'p') => {
                    self.show_details_modal = false;
                    self.open_tunnel_modal_for_selected_host();
                    return Ok(AppKeyAction::Ok);
                }
                Char('d') | Delete => {
                    let selected = self.table_state.selected().unwrap_or(0);
                    if selected < self.hosts.len() {
                        self.show_delete_modal = Some(self.hosts[selected].name.clone());
                    }
                    return Ok(AppKeyAction::Ok);
                }
                Char('k') => {
                    self.show_details_modal = false;
                    let selected = self.table_state.selected().unwrap_or(0);
                    if selected < self.hosts.len() {
                        let host = &self.hosts[selected];
                        let prompt = PostConnectPrompt {
                            host_name: host.name.clone(),
                            destination: host.destination.clone(),
                            user: host.user.clone(),
                            port: host.port.clone(),
                        };
                        self.run_ssh_copy_id(terminal, &prompt)?;
                    }
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

        // 8. Normal Table View Controls
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
            Delete => {
                let selected = self.table_state.selected().unwrap_or(0);
                if selected < self.hosts.len() {
                    self.show_delete_modal = Some(self.hosts[selected].name.clone());
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

        let host: ssh::Host = self.hosts[selected].clone();

        restore_terminal(terminal).expect("Failed to restore terminal");

        if let Some(template) = &self.config.command_template_on_session_start {
            host.spawn_command_template(template)?;
        }

        let _cmd_status = host.spawn_command_template(&self.config.command_template);

        if let Some(template) = &self.config.command_template_on_session_end {
            host.spawn_command_template(template)?;
        }

        setup_terminal(terminal).expect("Failed to setup terminal");

        if self.config.exit_after_ssh_session_ends {
            return Ok(AppKeyAction::Stop);
        }

        Ok(AppKeyAction::Ok)
    }

    fn launch_sftp_for_selected_host<B>(
        &mut self,
        _terminal: &Rc<RefCell<Terminal<B>>>,
    ) -> Result<AppKeyAction>
    where
        B: Backend + std::io::Write,
        <B as Backend>::Error: Send + Sync + 'static,
    {
        let selected = self.table_state.selected().unwrap_or(0);
        if selected >= self.hosts.len() {
            return Ok(AppKeyAction::Ok);
        }

        let host: ssh::Host = self.hosts[selected].clone();
        self.sftp_explorer = Some(crate::sftp_explorer::SftpExplorer::new(host));
        Ok(AppKeyAction::Ok)
    }

    fn launch_x11_for_selected_host<B>(
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

        let host: ssh::Host = self.hosts[selected].clone();

        restore_terminal(terminal).expect("Failed to restore terminal");
        println!("\n🖥️ Starting X11 GUI Forwarding Session to '{}' (ssh -Y -C)...\n", host.name);

        let res = host.spawn_x11(true, true);

        setup_terminal(terminal).expect("Failed to setup terminal");

        match res {
            Ok(_) => {
                self.set_status_message(format!("X11 session with '{}' closed", host.name));
            }
            Err(e) => {
                self.set_status_message(format!("X11 error: {e}"));
            }
        }

        Ok(AppKeyAction::Ok)
    }

    fn open_tunnel_modal_for_selected_host(&mut self) {
        let selected = self.table_state.selected().unwrap_or(0);
        if selected >= self.hosts.len() {
            return;
        }

        let host = self.hosts[selected].clone();
        self.tunnel_form = Some(TunnelForm {
            host,
            mode: TunnelMode::Local,
            local_port: "8080".into(),
            remote_target: "localhost:80".into(),
            background: true,
            active_field: 0,
        });
        self.show_tunnel_modal = true;
    }

    fn run_ssh_copy_id<B>(
        &mut self,
        terminal: &Rc<RefCell<Terminal<B>>>,
        prompt: &PostConnectPrompt,
    ) -> Result<()>
    where
        B: Backend + std::io::Write,
        <B as Backend>::Error: Send + Sync + 'static,
    {
        let target = if let Some(u) = &prompt.user {
            if !u.is_empty() {
                format!("{u}@{}", prompt.destination)
            } else {
                prompt.destination.clone()
            }
        } else {
            prompt.destination.clone()
        };

        restore_terminal(terminal).expect("Failed to restore terminal");

        println!("\n🔑 Setting up passwordless login with ssh-copy-id to '{target}'...\n");

        let mut cmd = Command::new("ssh-copy-id");
        if let Some(port) = &prompt.port {
            if !port.is_empty() && port != "22" {
                cmd.arg("-p").arg(port);
            }
        }
        cmd.arg(&target);

        let status = cmd.status();

        setup_terminal(terminal).expect("Failed to setup terminal");

        match status {
            Ok(s) if s.success() => {
                self.set_status_message("SSH public key successfully installed!".to_string());
            }
            Ok(_) => {
                self.set_status_message("ssh-copy-id exited with non-zero status".to_string());
            }
            Err(e) => {
                self.set_status_message(format!("Failed to run ssh-copy-id: {e}"));
            }
        }

        Ok(())
    }

    fn on_key_press_ctrl<B>(
        &mut self,
        terminal: &Rc<RefCell<Terminal<B>>>,
        key: KeyEvent,
    ) -> AppKeyAction
    where
        B: Backend + std::io::Write,
        <B as Backend>::Error: Send + Sync + 'static,
    {
        #[allow(clippy::enum_glob_use)]
        use KeyCode::*;

        match key.code {
            Char('c') => AppKeyAction::Stop,
            Char('n') => {
                self.show_add_host_modal = true;
                self.add_host_form = AddHostForm {
                    port: "22".into(),
                    ..Default::default()
                };
                AppKeyAction::Ok
            }
            Char('f') => {
                let _ = self.launch_sftp_for_selected_host(terminal);
                AppKeyAction::Ok
            }
            Char('x') => {
                let _ = self.launch_x11_for_selected_host(terminal);
                AppKeyAction::Ok
            }
            Char('l' | 'p') => {
                self.open_tunnel_modal_for_selected_host();
                AppKeyAction::Ok
            }
            Char('d') => {
                let selected = self.table_state.selected().unwrap_or(0);
                if selected < self.hosts.len() {
                    self.show_delete_modal = Some(self.hosts[selected].name.clone());
                }
                AppKeyAction::Ok
            }
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
            Char('?' | '/' | 'h') => {
                self.show_help_modal = !self.show_help_modal;
                AppKeyAction::Ok
            }
            Char('u') => {
                self.search = Input::default();
                self.hosts.search("");
                self.table_state.select(Some(0));
                AppKeyAction::Ok
            }
            Char('j') => {
                self.next();
                AppKeyAction::Ok
            }
            Char('k') => {
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

        let search_value = self.search.value();
        if search_value == search_value_before {
            return;
        }

        self.hosts.search(search_value);
        self.table_state.select(Some(0));
    }

    fn next(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= self.hosts.len().saturating_sub(1) {
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
                if i == 0 {
                    self.hosts.len().saturating_sub(1)
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    fn get_layout_offsets(&self, height: u16) -> (u16, u16) {
        let (show_full, show_mini, banner_height) = if height >= 24 {
            let req_h = self.ascii_art_style.required_height();
            if req_h > 0 { (true, false, req_h) } else { (false, false, 0) }
        } else if height >= 14 && self.ascii_art_style != AsciiArtStyle::Off {
            (false, true, 1)
        } else {
            (false, false, 0)
        };
        let _ = (show_full, show_mini);
        (banner_height, 3)
    }

    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
    fn on_mouse_event<B>(
        &mut self,
        terminal: &Rc<RefCell<Terminal<B>>>,
        mouse: MouseEvent,
    ) -> Result<AppKeyAction>
    where
        B: Backend + std::io::Write,
        <B as Backend>::Error: Send + Sync + 'static,
    {
        let term_size = terminal.borrow().size()?;
        let width = term_size.width;
        let height = term_size.height;

        // 1. If SSHS Explorer is open:
        if let Some(explorer) = &mut self.sftp_explorer {
            match mouse.kind {
                MouseEventKind::ScrollDown => {
                    explorer.select_next();
                    return Ok(AppKeyAction::Ok);
                }
                MouseEventKind::ScrollUp => {
                    explorer.select_previous();
                    return Ok(AppKeyAction::Ok);
                }
                MouseEventKind::Moved | MouseEventKind::Drag(MouseButton::Left) => {
                    if let Some(menu) = &mut explorer.context_menu {
                        let menu_w = 44u16;
                        #[allow(clippy::cast_possible_truncation)]
                        let menu_h = (crate::sftp_explorer::CONTEXT_MENU_OPTIONS.len() as u16) + 2;
                        let x = if menu.x + menu_w > width { width.saturating_sub(menu_w + 1) } else { menu.x };
                        let y = if menu.y + menu_h > height { height.saturating_sub(menu_h + 1) } else { menu.y };

                        if mouse.column >= x && mouse.column < x + menu_w && mouse.row > y && mouse.row <= y + (crate::sftp_explorer::CONTEXT_MENU_OPTIONS.len() as u16) {
                            let opt = (mouse.row - (y + 1)) as usize;
                            if opt < crate::sftp_explorer::CONTEXT_MENU_OPTIONS.len() {
                                menu.selected_index = opt;
                            }
                        }
                    }
                    return Ok(AppKeyAction::Ok);
                }
                MouseEventKind::Down(MouseButton::Right) => {
                    if explorer.viewer_content.is_some() || explorer.mkdir_input.is_some() {
                        explorer.viewer_content = None;
                        explorer.mkdir_input = None;
                        explorer.context_menu = None;
                        return Ok(AppKeyAction::Ok);
                    }

                    let is_left_pane = mouse.column < width / 2;
                    explorer.active_pane = if is_left_pane { crate::sftp_explorer::ActivePane::Local } else { crate::sftp_explorer::ActivePane::Remote };

                    if mouse.row >= 5 && mouse.row < height.saturating_sub(3) {
                        let rel_row = (mouse.row - 5) as usize;
                        if is_left_pane {
                            let clicked_idx = explorer.local_table_state.offset() + rel_row;
                            if clicked_idx < explorer.local_entries.len() {
                                explorer.local_selected = clicked_idx;
                                explorer.local_table_state.select(Some(clicked_idx));
                            }
                        } else {
                            let clicked_idx = explorer.remote_table_state.offset() + rel_row;
                            if clicked_idx < explorer.remote_entries.len() {
                                explorer.remote_selected = clicked_idx;
                                explorer.remote_table_state.select(Some(clicked_idx));
                            }
                        }
                    }
                    explorer.open_context_menu_at(mouse.column, mouse.row);
                    return Ok(AppKeyAction::Ok);
                }
                MouseEventKind::Down(MouseButton::Left) => {
                    if explorer.viewer_content.is_some() {
                        explorer.viewer_content = None;
                        return Ok(AppKeyAction::Ok);
                    }

                    // If Context menu is open, handle click on context menu
                    if let Some(menu) = explorer.context_menu.clone() {
                        let menu_w = 44u16;
                        #[allow(clippy::cast_possible_truncation)]
                        let menu_h = (crate::sftp_explorer::CONTEXT_MENU_OPTIONS.len() as u16) + 2;
                        let x = if menu.x + menu_w > width { width.saturating_sub(menu_w + 1) } else { menu.x };
                        let y = if menu.y + menu_h > height { height.saturating_sub(menu_h + 1) } else { menu.y };

                        if mouse.column >= x && mouse.column < x + menu_w && mouse.row > y && mouse.row <= y + (crate::sftp_explorer::CONTEXT_MENU_OPTIONS.len() as u16) {
                            let clicked_opt = (mouse.row - (y + 1)) as usize;
                            explorer.execute_context_menu_option(clicked_opt);
                        } else {
                            explorer.context_menu = None;
                        }
                        return Ok(AppKeyAction::Ok);
                    }

                    // Check footer click
                    if mouse.row >= height.saturating_sub(3) {
                        let col_pct = (mouse.column as f32) / (width as f32);
                        if col_pct < 0.12 {
                            explorer.switch_pane();
                        } else if col_pct < 0.24 {
                            explorer.enter_directory();
                        } else if col_pct < 0.35 {
                            explorer.view_selected();
                        } else if col_pct < 0.48 {
                            explorer.copy_or_transfer();
                        } else if col_pct < 0.60 {
                            explorer.mkdir_input = Some("new_folder".into());
                        } else if col_pct < 0.72 {
                            explorer.delete_selected();
                        } else if col_pct < 0.85 {
                            explorer.open_context_menu_at(mouse.column, mouse.row.saturating_sub(10));
                        } else {
                            self.sftp_explorer = None;
                        }
                        return Ok(AppKeyAction::Ok);
                    }

                    // Dual panes click
                    let is_left_pane = mouse.column < width / 2;
                    if is_left_pane {
                        explorer.active_pane = crate::sftp_explorer::ActivePane::Local;
                        if mouse.row >= 5 && mouse.row < height.saturating_sub(3) {
                            let rel_row = (mouse.row - 5) as usize;
                            let clicked_idx = explorer.local_table_state.offset() + rel_row;
                            if clicked_idx < explorer.local_entries.len() {
                                if explorer.local_selected == clicked_idx {
                                    explorer.enter_directory();
                                } else {
                                    explorer.local_selected = clicked_idx;
                                    explorer.local_table_state.select(Some(clicked_idx));
                                }
                            }
                        }
                    } else {
                        explorer.active_pane = crate::sftp_explorer::ActivePane::Remote;
                        if mouse.row >= 5 && mouse.row < height.saturating_sub(3) {
                            let rel_row = (mouse.row - 5) as usize;
                            let clicked_idx = explorer.remote_table_state.offset() + rel_row;
                            if clicked_idx < explorer.remote_entries.len() {
                                if explorer.remote_selected == clicked_idx {
                                    explorer.enter_directory();
                                } else {
                                    explorer.remote_selected = clicked_idx;
                                    explorer.remote_table_state.select(Some(clicked_idx));
                                }
                            }
                        }
                    }
                    return Ok(AppKeyAction::Ok);
                }
                _ => return Ok(AppKeyAction::Ok),
            }
        }

        // 2. Modals (Delete, Tunnel, Add, Help, Details)
        if self.show_delete_modal.is_some() || self.show_tunnel_modal || self.show_add_host_modal || self.show_help_modal || self.show_details_modal || self.post_connect_prompt.is_some() {
            match mouse.kind {
                MouseEventKind::Down(MouseButton::Right) => {
                    self.show_delete_modal = None;
                    self.show_tunnel_modal = false;
                    self.show_add_host_modal = false;
                    self.show_help_modal = false;
                    self.show_details_modal = false;
                    self.post_connect_prompt = None;
                    return Ok(AppKeyAction::Ok);
                }
                MouseEventKind::ScrollDown => {
                    self.next();
                    return Ok(AppKeyAction::Ok);
                }
                MouseEventKind::ScrollUp => {
                    self.previous();
                    return Ok(AppKeyAction::Ok);
                }
                MouseEventKind::Down(MouseButton::Left)
                    if self.show_details_modal && mouse.row >= height.saturating_sub(6) =>
                {
                    let col_pct = (mouse.column as f32) / (width as f32);
                    if col_pct < 0.35 {
                        self.show_details_modal = false;
                        return self.connect_to_selected_host(terminal);
                    } else if col_pct < 0.50 {
                        self.show_details_modal = false;
                        return self.launch_sftp_for_selected_host(terminal);
                    } else if col_pct < 0.65 {
                        self.show_details_modal = false;
                        return self.launch_x11_for_selected_host(terminal);
                    } else if col_pct < 0.80 {
                        self.show_details_modal = false;
                        self.open_tunnel_modal_for_selected_host();
                    } else {
                        self.show_details_modal = false;
                    }
                    return Ok(AppKeyAction::Ok);
                }
                _ => {}
            }
            return Ok(AppKeyAction::Ok);
        }

        // 3. Main Host Table view
        match mouse.kind {
            MouseEventKind::ScrollDown => {
                self.next();
                Ok(AppKeyAction::Ok)
            }
            MouseEventKind::ScrollUp => {
                self.previous();
                Ok(AppKeyAction::Ok)
            }
            MouseEventKind::Down(MouseButton::Right) => {
                // Right click on table row selects and opens Details modal
                let (banner_h, search_h) = self.get_layout_offsets(height);
                let table_top = banner_h + search_h;
                let table_rows_start = table_top + 2; // header + border
                let footer_start = height.saturating_sub(3);
                if mouse.row >= table_rows_start && mouse.row < footer_start {
                    let rel_row = (mouse.row - table_rows_start) as usize;
                    let clicked_row = self.table_state.offset() + rel_row;
                    if clicked_row < self.hosts.len() {
                        self.table_state.select(Some(clicked_row));
                        self.show_details_modal = true;
                    }
                }
                Ok(AppKeyAction::Ok)
            }
            MouseEventKind::Down(MouseButton::Left) => {
                let (banner_h, search_h) = self.get_layout_offsets(height);
                let search_start = banner_h;
                let table_top = banner_h + search_h;
                let table_rows_start = table_top + 2;
                let footer_start = height.saturating_sub(3);

                // Click on Search bar
                if mouse.row >= search_start && mouse.row < search_start + search_h {
                    return Ok(AppKeyAction::Ok);
                }

                // Click on Footer button bar
                if mouse.row >= footer_start {
                    let col_pct = (mouse.column as f32) / (width as f32);
                    if col_pct < 0.10 {
                        // Connect
                        return self.connect_to_selected_host(terminal);
                    } else if col_pct < 0.20 {
                        // Details
                        self.show_details_modal = true;
                    } else if col_pct < 0.30 {
                        // SFTP
                        return self.launch_sftp_for_selected_host(terminal);
                    } else if col_pct < 0.40 {
                        // X11
                        return self.launch_x11_for_selected_host(terminal);
                    } else if col_pct < 0.52 {
                        // Tunnel / TOR
                        self.open_tunnel_modal_for_selected_host();
                    } else if col_pct < 0.62 {
                        // Add
                        self.show_add_host_modal = true;
                        self.add_host_form = AddHostForm { port: "22".into(), ..Default::default() };
                    } else if col_pct < 0.72 {
                        // Del
                        let selected = self.table_state.selected().unwrap_or(0);
                        if selected < self.hosts.len() {
                            self.show_delete_modal = Some(self.hosts[selected].name.clone());
                        }
                    } else if col_pct < 0.82 {
                        // Theme
                        self.cycle_theme();
                    } else if col_pct < 0.92 {
                        // Help
                        self.show_help_modal = true;
                    } else {
                        // Quit
                        return Ok(AppKeyAction::Stop);
                    }
                    return Ok(AppKeyAction::Ok);
                }

                // Click on Host table row
                if mouse.row >= table_rows_start && mouse.row < footer_start {
                    let rel_row = (mouse.row - table_rows_start) as usize;
                    let clicked_row = self.table_state.offset() + rel_row;
                    if clicked_row < self.hosts.len() {
                        let prev_selected = self.table_state.selected();
                        if prev_selected == Some(clicked_row) {
                            // Double clicking or clicking already selected row connects
                            return self.connect_to_selected_host(terminal);
                        }
                        self.table_state.select(Some(clicked_row));
                    }
                }
                Ok(AppKeyAction::Ok)
            }
            _ => Ok(AppKeyAction::Ok),
        }
    }

    fn reload_hosts(&mut self) {
        let selected = self.table_state.selected().unwrap_or(0);
        let selected_name = if selected < self.hosts.len() {
            Some(self.hosts[selected].name.clone())
        } else {
            None
        };

        let Ok(hosts) = load_hosts(&self.config) else {
            return;
        };

        self.hosts = Searchable::new(self.config.sort_by_score, hosts, self.search.value());

        let next_selected = selected_name
            .and_then(|name| self.hosts.iter().position(|h| h.name == name))
            .unwrap_or(0);
        self.table_state.select(Some(next_selected));

        self.calculate_table_columns_constraints();
    }

    fn calculate_table_columns_constraints(&mut self) {
        let mut column_widths = [
            "Host".width(),
            "Aliases".width(),
            "User".width(),
            "Destination".width(),
            "Port".width(),
            "Proxy".width(),
        ];

        for host in self.hosts.non_filtered_iter() {
            column_widths[0] = max(column_widths[0], host.name.width());
            column_widths[1] = max(column_widths[1], host.aliases.width());
            column_widths[2] = max(
                column_widths[2],
                host.user.as_deref().map_or(0, UnicodeWidthStr::width),
            );
            column_widths[3] = max(column_widths[3], host.destination.width());
            column_widths[4] = max(
                column_widths[4],
                host.port.as_deref().map_or(0, UnicodeWidthStr::width),
            );
            column_widths[5] = max(
                column_widths[5],
                host.proxy_command
                    .as_deref()
                    .map_or(0, UnicodeWidthStr::width),
            );
        }

        let mut constraints = vec![
            Constraint::Length(u16::try_from(column_widths[0]).unwrap_or_default() + 2), // +2 for bullet point
            Constraint::Length(u16::try_from(column_widths[1]).unwrap_or_default()),
            Constraint::Length(u16::try_from(column_widths[2]).unwrap_or_default()),
            Constraint::Length(u16::try_from(column_widths[3]).unwrap_or_default()),
            Constraint::Length(u16::try_from(column_widths[4]).unwrap_or_default()),
        ];

        if self.config.show_proxy_command {
            constraints.push(Constraint::Length(
                u16::try_from(column_widths[5]).unwrap_or_default(),
            ));
        }

        self.table_columns_constraints = constraints;
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
        hosts.sort_by(|a, b| a.name.cmp(&b.name));
    }

    Ok(hosts)
}

fn setup_terminal<B>(terminal: &Rc<RefCell<Terminal<B>>>) -> Result<()>
where
    B: Backend + std::io::Write,
    <B as Backend>::Error: Send + Sync + 'static,
{
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen, Hide, EnableMouseCapture)?;
    terminal.borrow_mut().clear()?;
    terminal.borrow_mut().hide_cursor()?;

    Ok(())
}

fn restore_terminal<B>(terminal: &Rc<RefCell<Terminal<B>>>) -> Result<()>
where
    B: Backend + std::io::Write,
    <B as Backend>::Error: Send + Sync + 'static,
{
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen, Show, DisableMouseCapture)?;
    terminal.borrow_mut().show_cursor()?;

    Ok(())
}

fn ui(f: &mut Frame, app: &mut App) {
    if let Some(explorer) = &mut app.sftp_explorer {
        explorer.check_active_transfer();
        crate::sftp_explorer::render_sftp_explorer(f, &app.theme, explorer, app.anim_tick);
        return;
    }

    let size = f.area();

    // Determine layout based on terminal height
    let (show_full_banner, show_mini_banner, banner_height) = if size.height >= 24 {
        let req_h = app.ascii_art_style.required_height();
        if req_h > 0 {
            (true, false, req_h)
        } else {
            (false, false, 0)
        }
    } else if size.height >= 14 && app.ascii_art_style != AsciiArtStyle::Off {
        (false, true, 1)
    } else {
        (false, false, 0)
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

    // Render modals if active
    if let Some(del_name) = &app.show_delete_modal.clone() {
        render_delete_modal(f, app, del_name);
    } else if app.show_tunnel_modal {
        if let Some(form) = &app.tunnel_form {
            render_tunnel_modal(f, app, form);
        }
    } else if app.show_add_host_modal {
        render_add_host_modal(f, app);
    } else if let Some(prompt) = &app.post_connect_prompt {
        render_post_connect_modal(f, app, prompt);
    } else if app.show_details_modal {
        render_details_modal(f, app);
    } else if app.show_help_modal {
        render_help_modal(f, app);
    } else {
        // Set cursor in search bar
        let mut cursor_position = search_rect.as_position();
        cursor_position.x += u16::try_from(app.search.cursor()).unwrap_or_default() + 5;
        cursor_position.y += 1;
        f.set_cursor_position(cursor_position);
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
    } else if area.width < 100 {
        Line::from(vec![
            Span::styled(" Enter ", app.theme.key_badge_style()),
            Span::styled(" SSH ", app.theme.key_desc_style()),
            Span::styled(" Tab ", app.theme.key_badge_style()),
            Span::styled(" Info ", app.theme.key_desc_style()),
            Span::styled(" ^F ", app.theme.key_badge_style()),
            Span::styled(" SFTP ", app.theme.key_desc_style()),
            Span::styled(" ^X ", app.theme.key_badge_style()),
            Span::styled(" X11 ", app.theme.key_desc_style()),
            Span::styled(" ^L ", app.theme.key_badge_style()),
            Span::styled(" Tunnel ", app.theme.key_desc_style()),
            Span::styled(" ^N ", app.theme.key_badge_style()),
            Span::styled(" Add ", app.theme.key_desc_style()),
            Span::styled(" ^T ", app.theme.key_badge_style()),
            Span::styled(" Theme ", app.theme.key_desc_style()),
            Span::styled(" ^? ", app.theme.key_badge_style()),
            Span::styled(" Help ", app.theme.key_desc_style()),
            Span::styled(" Esc ", app.theme.key_badge_style()),
            Span::styled(" Quit ", app.theme.key_desc_style()),
        ])
    } else {
        Line::from(vec![
            Span::styled(" Enter ", app.theme.key_badge_style()),
            Span::styled(" Connect  ", app.theme.key_desc_style()),
            Span::styled(" Tab ", app.theme.key_badge_style()),
            Span::styled(" Details  ", app.theme.key_desc_style()),
            Span::styled(" ^F ", app.theme.key_badge_style()),
            Span::styled(" SFTP  ", app.theme.key_desc_style()),
            Span::styled(" ^X ", app.theme.key_badge_style()),
            Span::styled(" X11  ", app.theme.key_desc_style()),
            Span::styled(" ^L ", app.theme.key_badge_style()),
            Span::styled(" Tunnel/TOR  ", app.theme.key_desc_style()),
            Span::styled(" ^N ", app.theme.key_badge_style()),
            Span::styled(" Add  ", app.theme.key_desc_style()),
            Span::styled(" ^D ", app.theme.key_badge_style()),
            Span::styled(" Del  ", app.theme.key_desc_style()),
            Span::styled(" ^T ", app.theme.key_badge_style()),
            Span::styled(" Theme  ", app.theme.key_desc_style()),
            Span::styled(" ^? ", app.theme.key_badge_style()),
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

fn render_delete_modal(f: &mut Frame, app: &App, host_name: &str) {
    let area = centered_rect(62, 30, f.area());
    f.render_widget(Clear, area);

    let lines = vec![
        Line::raw(""),
        Line::from(vec![
            Span::styled("  Are you sure you want to delete SSH host ", Style::default().fg(app.theme.header_fg)),
            Span::styled(format!("'{host_name}'"), Style::default().fg(app.theme.primary).add_modifier(Modifier::BOLD)),
            Span::styled("?", Style::default().fg(app.theme.header_fg)),
        ]).centered(),
        Line::raw(""),
        Line::from(vec![
            Span::styled("  This will remove its configuration stanza from ~/.ssh/config.", Style::default().fg(app.theme.muted)),
        ]).centered(),
        Line::raw(""),
        Line::from(vec![
            Span::styled(" [ y / Enter ] ", Style::default().fg(app.theme.selected_fg).bg(app.theme.primary).add_modifier(Modifier::BOLD)),
            Span::styled(" Yes, Delete Host    ", app.theme.key_desc_style()),
            Span::styled(" [ n / Esc ] ", app.theme.key_badge_style()),
            Span::styled(" Cancel", app.theme.key_desc_style()),
        ]).centered(),
    ];

    let modal_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(app.theme.active_border_style())
        .title(Line::from(Span::styled(
            " 🗑 Delete SSH Host Profile ",
            Style::default().fg(app.theme.primary).add_modifier(Modifier::BOLD),
        )));

    let widget = Paragraph::new(lines).block(modal_block);
    f.render_widget(widget, area);
}

fn render_tunnel_modal(f: &mut Frame, app: &App, form: &TunnelForm) {
    let area = centered_rect(84, 76, f.area());
    f.render_widget(Clear, area);

    let mut lines = Vec::new();
    lines.push(Line::raw(""));

    // Target Info
    lines.push(Line::from(vec![
        Span::styled("  Target Host: ", Style::default().fg(app.theme.muted)),
        Span::styled(&form.host.name, Style::default().fg(app.theme.primary).add_modifier(Modifier::BOLD)),
        Span::styled(format!(" ({})", form.host.destination), Style::default().fg(app.theme.destination)),
    ]));
    lines.push(Line::raw(""));

    // Field 0: Tunnel Mode
    let mode_active = form.active_field == 0;
    let mode_prefix = if mode_active { " ❯ " } else { "   " };
    let mode_style = if mode_active {
        Style::default().fg(app.theme.primary).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(app.theme.muted)
    };

    lines.push(Line::from(vec![
        Span::styled(mode_prefix, mode_style),
        Span::styled("Networking Mode (Keys 1-8 or Space / ← / → to change):", mode_style),
    ]));

    let modes = [
        (TunnelMode::Local, "1. Local (-L)"),
        (TunnelMode::Remote, "2. Remote (-R)"),
        (TunnelMode::Dynamic, "3. SOCKS5 (-D)"),
        (TunnelMode::VpnTun, "4. TUN/TAP (-w)"),
        (TunnelMode::Tor, "5. Tor Onion"),
        (TunnelMode::PortKnock, "6. Port Knock"),
        (TunnelMode::X11, "7. X11 GUI (-X/-Y)"),
        (TunnelMode::Pqc, "8. Post-Quantum (PQC)"),
    ];

    let row1 = modes[0..4].iter().map(|&(m, label)| {
        if form.mode == m {
            Span::styled(format!(" [◉ {label}] "), Style::default().fg(app.theme.selected_fg).bg(app.theme.primary).add_modifier(Modifier::BOLD))
        } else {
            Span::styled(format!(" [○ {label}] "), Style::default().fg(app.theme.muted))
        }
    }).collect::<Vec<_>>();

    let row2 = modes[4..8].iter().map(|&(m, label)| {
        if form.mode == m {
            Span::styled(format!(" [◉ {label}] "), Style::default().fg(app.theme.selected_fg).bg(app.theme.primary).add_modifier(Modifier::BOLD))
        } else {
            Span::styled(format!(" [○ {label}] "), Style::default().fg(app.theme.muted))
        }
    }).collect::<Vec<_>>();

    lines.push(Line::from(row1));
    lines.push(Line::from(row2));
    lines.push(Line::raw(""));

    // Field 1: Dynamic Parameter 1
    let lp_active = form.active_field == 1;
    let lp_prefix = if lp_active { " ❯ " } else { "   " };
    let lp_label = match form.mode {
        TunnelMode::Local => "Local Listening Port (e.g. 8080):",
        TunnelMode::Remote => "Remote Listening Port (e.g. 9000):",
        TunnelMode::Dynamic => "Local SOCKS5 Proxy Port (e.g. 1080):",
        TunnelMode::VpnTun => "VPN Gateway Parameter (Optional):",
        TunnelMode::Tor => "Tor SOCKS5 Proxy Address (e.g. 127.0.0.1:9050):",
        TunnelMode::PortKnock => "Port Knocking Sequence (e.g. 7000, 8000, 9000):",
        TunnelMode::X11 => "X11 Security Mode ('trusted' [-Y] or 'standard' [-X]):",
        TunnelMode::Pqc => "Enforced Post-Quantum Algorithms (ML-KEM / sntrup761):",
    };
    let lp_style = if lp_active {
        Style::default().fg(app.theme.primary).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(app.theme.muted)
    };

    lines.push(Line::from(vec![
        Span::styled(lp_prefix, lp_style),
        Span::styled(lp_label, lp_style),
    ]));

    let lp_val = form.local_port.value();
    let lp_placeholder = match form.mode {
        TunnelMode::Local => "8080",
        TunnelMode::Remote => "9000",
        TunnelMode::Dynamic => "1080",
        TunnelMode::VpnTun => "(default)",
        TunnelMode::Tor => "127.0.0.1:9050",
        TunnelMode::PortKnock => "7000, 8000, 9000",
        TunnelMode::X11 => "trusted",
        TunnelMode::Pqc => "mlkem768x25519-sha512@openssh.com,sntrup761x25519-sha512@openssh.com",
    };

    lines.push(Line::from(vec![
        Span::styled("     [ ", if lp_active { Style::default().fg(app.theme.primary).add_modifier(Modifier::BOLD) } else { Style::default().fg(app.theme.border) }),
        Span::styled(if lp_val.is_empty() { lp_placeholder } else { lp_val }, if lp_val.is_empty() { Style::default().fg(app.theme.muted) } else { Style::default().fg(app.theme.header_fg).add_modifier(Modifier::BOLD) }),
        Span::styled(" ]", if lp_active { Style::default().fg(app.theme.primary).add_modifier(Modifier::BOLD) } else { Style::default().fg(app.theme.border) }),
    ]));
    lines.push(Line::raw(""));

    // Field 2: Target / Interface parameter
    if form.mode == TunnelMode::Local || form.mode == TunnelMode::Remote || form.mode == TunnelMode::VpnTun || form.mode == TunnelMode::X11 {
        let rt_active = form.active_field == 2;
        let rt_prefix = if rt_active { " ❯ " } else { "   " };
        let rt_label = match form.mode {
            TunnelMode::Local => "Remote Destination Endpoint (host:port):",
            TunnelMode::Remote => "Local Target to Expose (host:port):",
            TunnelMode::VpnTun => "Virtual TUN Network Device (local:remote):",
            TunnelMode::X11 => "Compression Mode ('compressed' [-C] or 'none'):",
            _ => "",
        };
        let rt_placeholder = match form.mode {
            TunnelMode::Local => "localhost:80",
            TunnelMode::Remote => "localhost:3000",
            TunnelMode::VpnTun => "any:any",
            TunnelMode::X11 => "compressed",
            _ => "",
        };
        let rt_style = if rt_active {
            Style::default().fg(app.theme.primary).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(app.theme.muted)
        };

        lines.push(Line::from(vec![
            Span::styled(rt_prefix, rt_style),
            Span::styled(rt_label, rt_style),
        ]));

        let rt_val = form.remote_target.value();
        lines.push(Line::from(vec![
            Span::styled("     [ ", if rt_active { Style::default().fg(app.theme.primary).add_modifier(Modifier::BOLD) } else { Style::default().fg(app.theme.border) }),
            Span::styled(if rt_val.is_empty() { rt_placeholder } else { rt_val }, if rt_val.is_empty() { Style::default().fg(app.theme.muted) } else { Style::default().fg(app.theme.header_fg).add_modifier(Modifier::BOLD) }),
            Span::styled(" ]", if rt_active { Style::default().fg(app.theme.primary).add_modifier(Modifier::BOLD) } else { Style::default().fg(app.theme.border) }),
        ]));
        lines.push(Line::raw(""));
    }

    // Field 3: Background execution
    if form.mode != TunnelMode::Tor && form.mode != TunnelMode::PortKnock && form.mode != TunnelMode::X11 && form.mode != TunnelMode::Pqc {
        let bg_active = form.active_field == 3;
        let bg_prefix = if bg_active { " ❯ " } else { "   " };
        let bg_style = if bg_active {
            Style::default().fg(app.theme.primary).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(app.theme.muted)
        };

        lines.push(Line::from(vec![
            Span::styled(bg_prefix, bg_style),
            Span::styled("Execution Mode (Space to toggle):", bg_style),
        ]));

        let (bg_btn, fg_btn) = if form.background {
            (
                Span::styled(" [◉ Run in background daemon (-f -N)] ", Style::default().fg(app.theme.selected_fg).bg(app.theme.primary).add_modifier(Modifier::BOLD)),
                Span::styled(" [○ Attach interactive shell] ", Style::default().fg(app.theme.muted)),
            )
        } else {
            (
                Span::styled(" [○ Run in background daemon (-f -N)] ", Style::default().fg(app.theme.muted)),
                Span::styled(" [◉ Attach interactive shell] ", Style::default().fg(app.theme.selected_fg).bg(app.theme.primary).add_modifier(Modifier::BOLD)),
            )
        };

        lines.push(Line::from(vec![
            Span::raw("     "),
            bg_btn,
            fg_btn,
        ]));
        lines.push(Line::raw(""));
    }

    // Command Preview
    let lp = if form.local_port.value().is_empty() { "8080" } else { form.local_port.value() };
    let bg_flag = if form.background && form.mode != TunnelMode::Tor && form.mode != TunnelMode::PortKnock && form.mode != TunnelMode::X11 && form.mode != TunnelMode::Pqc { "-f -N " } else { "" };
    let cmd_preview = match form.mode {
        TunnelMode::Local => {
            let rt = if form.remote_target.value().is_empty() { "localhost:80" } else { form.remote_target.value() };
            format!("ssh {bg_flag}-L {lp}:{rt} {}", form.host.name)
        }
        TunnelMode::Remote => {
            let rt = if form.remote_target.value().is_empty() { "localhost:3000" } else { form.remote_target.value() };
            format!("ssh {bg_flag}-R {lp}:{rt} {}", form.host.name)
        }
        TunnelMode::Dynamic => {
            format!("ssh {bg_flag}-D {lp} {}", form.host.name)
        }
        TunnelMode::VpnTun => {
            let dev = if form.remote_target.value().is_empty() { "any:any" } else { form.remote_target.value() };
            format!("ssh {bg_flag}-w {dev} -o Tunnel=point-to-point {}", form.host.name)
        }
        TunnelMode::Tor => {
            let prx = if form.local_port.value().is_empty() { "127.0.0.1:9050" } else { form.local_port.value() };
            format!("ssh -o \"ProxyCommand=nc -X 5 -x {prx} %h %p\" {}", form.host.name)
        }
        TunnelMode::PortKnock => {
            let seq = if form.local_port.value().is_empty() { "7000, 8000, 9000" } else { form.local_port.value() };
            format!("knock {} {seq} && ssh {}", form.host.name, form.host.name)
        }
        TunnelMode::X11 => {
            let is_untrusted = form.local_port.value().eq_ignore_ascii_case("untrusted") || form.local_port.value().eq_ignore_ascii_case("standard");
            let flag = if is_untrusted { "-X" } else { "-Y" };
            let is_compress = !form.remote_target.value().eq_ignore_ascii_case("none") && !form.remote_target.value().eq_ignore_ascii_case("no");
            let c_flag = if is_compress { " -C" } else { "" };
            format!("ssh {flag}{c_flag} {}", form.host.name)
        }
        TunnelMode::Pqc => {
            let kex = if form.local_port.value().is_empty() { "mlkem768x25519-sha512@openssh.com,sntrup761x25519-sha512@openssh.com" } else { form.local_port.value() };
            format!("ssh -o \"KexAlgorithms={kex}\" {}", form.host.name)
        }
    };

    lines.push(Line::from(vec![
        Span::styled("  Command Preview: ", Style::default().fg(app.theme.secondary).add_modifier(Modifier::BOLD)),
        Span::styled(cmd_preview, Style::default().fg(app.theme.header_fg).add_modifier(Modifier::BOLD)),
    ]));
    lines.push(Line::raw(""));

    lines.push(
        Line::from(vec![
            Span::styled(" [ Tab/↓ ] ", app.theme.key_badge_style()),
            Span::styled(" Next   ", app.theme.key_desc_style()),
            Span::styled(" [ 1-8 ] ", app.theme.key_badge_style()),
            Span::styled(" Mode   ", app.theme.key_desc_style()),
            Span::styled(" [ Enter ] ", app.theme.key_badge_style()),
            Span::styled(" Launch / Connect   ", app.theme.key_desc_style()),
            Span::styled(" [ Esc ] ", app.theme.key_badge_style()),
            Span::styled(" Cancel", app.theme.key_desc_style()),
        ])
        .centered(),
    );

    let modal_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(app.theme.active_border_style())
        .title(Line::from(Span::styled(
            " 🌐 SSH Networking, Tunneling & Security Hub ",
            Style::default().fg(app.theme.primary).add_modifier(Modifier::BOLD),
        )));

    let widget = Paragraph::new(lines).block(modal_block);
    f.render_widget(widget, area);
}

fn render_add_host_modal(f: &mut Frame, app: &App) {
    let area = centered_rect(65, 70, f.area());
    f.render_widget(Clear, area);

    let fields = [
        ("Host Alias / Name (Required)", &app.add_host_form.name, "e.g. web-prod, vps-us"),
        ("HostName / IP (Required)", &app.add_host_form.hostname, "e.g. 192.168.1.100, server.com"),
        ("User (Optional)", &app.add_host_form.user, "e.g. root, ubuntu, debian"),
        ("Port (Optional)", &app.add_host_form.port, "default 22"),
        ("Identity File (Optional)", &app.add_host_form.identity_file, "e.g. ~/.ssh/id_ed25519"),
        ("Proxy Jump (Optional)", &app.add_host_form.proxy_jump, "e.g. bastion.example.com"),
    ];

    let mut lines = Vec::new();
    lines.push(Line::raw(""));

    for (idx, (label, input, placeholder)) in fields.iter().enumerate() {
        let is_active = idx == app.add_host_form.active_field;
        let prefix = if is_active { " ❯ " } else { "   " };
        let label_style = if is_active {
            Style::default().fg(app.theme.primary).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(app.theme.muted)
        };

        lines.push(Line::from(vec![
            Span::styled(prefix, label_style),
            Span::styled(*label, label_style),
        ]));

        let val = input.value();
        let input_line = if val.is_empty() {
            if is_active {
                Line::from(vec![
                    Span::styled("   [ ", Style::default().fg(app.theme.border_active)),
                    Span::styled(*placeholder, Style::default().fg(app.theme.muted).add_modifier(Modifier::ITALIC)),
                    Span::styled(" ]", Style::default().fg(app.theme.border_active)),
                ])
            } else {
                Line::from(vec![
                    Span::styled("   [ ", Style::default().fg(app.theme.border)),
                    Span::styled(*placeholder, Style::default().fg(app.theme.muted)),
                    Span::styled(" ]", Style::default().fg(app.theme.border)),
                ])
            }
        } else {
            let val_style = if is_active {
                Style::default().fg(app.theme.header_fg).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(app.theme.header_fg)
            };
            let bracket_style = if is_active {
                Style::default().fg(app.theme.primary).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(app.theme.border)
            };

            Line::from(vec![
                Span::styled("   [ ", bracket_style),
                Span::styled(val, val_style),
                Span::styled(" ]", bracket_style),
            ])
        };

        lines.push(input_line);
        lines.push(Line::raw(""));
    }

    lines.push(
        Line::from(vec![
            Span::styled(" [ Tab/↓ ] ", app.theme.key_badge_style()),
            Span::styled(" Next   ", app.theme.key_desc_style()),
            Span::styled(" [ Shift+Tab/↑ ] ", app.theme.key_badge_style()),
            Span::styled(" Prev   ", app.theme.key_desc_style()),
            Span::styled(" [ Enter ] ", app.theme.key_badge_style()),
            Span::styled(" Save Host   ", app.theme.key_desc_style()),
            Span::styled(" [ Esc ] ", app.theme.key_badge_style()),
            Span::styled(" Cancel", app.theme.key_desc_style()),
        ])
        .centered(),
    );

    let modal_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(app.theme.active_border_style())
        .title(Line::from(Span::styled(
            " ➕ Add New SSH Host ",
            Style::default().fg(app.theme.primary).add_modifier(Modifier::BOLD),
        )));

    let widget = Paragraph::new(lines).block(modal_block);
    f.render_widget(widget, area);
}

fn render_post_connect_modal(f: &mut Frame, app: &App, prompt: &PostConnectPrompt) {
    let area = centered_rect(60, 32, f.area());
    f.render_widget(Clear, area);

    let target = if let Some(u) = &prompt.user {
        if !u.is_empty() {
            format!("{u}@{}", prompt.destination)
        } else {
            prompt.destination.clone()
        }
    } else {
        prompt.destination.clone()
    };

    let lines = vec![
        Line::raw(""),
        Line::from(vec![
            Span::styled("  Session with '", Style::default().fg(app.theme.header_fg)),
            Span::styled(&prompt.host_name, Style::default().fg(app.theme.primary).add_modifier(Modifier::BOLD)),
            Span::styled("' ended successfully.", Style::default().fg(app.theme.header_fg)),
        ]).centered(),
        Line::raw(""),
        Line::from(vec![
            Span::styled("  Would you like to install your SSH public key using ", Style::default().fg(app.theme.muted)),
            Span::styled("ssh-copy-id", Style::default().fg(app.theme.accent).add_modifier(Modifier::BOLD)),
        ]).centered(),
        Line::from(vec![
            Span::styled("  to enable seamless passwordless login in the future?", Style::default().fg(app.theme.muted)),
        ]).centered(),
        Line::raw(""),
        Line::from(vec![
            Span::styled(format!("    ❯ ssh-copy-id {target}"), Style::default().fg(app.theme.secondary).add_modifier(Modifier::BOLD)),
        ]).centered(),
        Line::raw(""),
        Line::from(vec![
            Span::styled(" [ y ] ", Style::default().fg(app.theme.selected_fg).bg(app.theme.user).add_modifier(Modifier::BOLD)),
            Span::styled(" Yes, Run ssh-copy-id    ", app.theme.key_desc_style()),
            Span::styled(" [ n / Esc ] ", app.theme.key_badge_style()),
            Span::styled(" Skip", app.theme.key_desc_style()),
        ]).centered(),
    ];

    let modal_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(app.theme.active_border_style())
        .title(Line::from(Span::styled(
            " 🔑 Passwordless Key Setup ",
            Style::default().fg(app.theme.primary).add_modifier(Modifier::BOLD),
        )));

    let widget = Paragraph::new(lines).block(modal_block);
    f.render_widget(widget, area);
}

fn render_details_modal(f: &mut Frame, app: &App) {
    let area = centered_rect(84, 82, f.area());
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
        Span::styled("  SSH Command:", Style::default().fg(app.theme.secondary).add_modifier(Modifier::BOLD)),
    ]));
    lines.push(Line::from(vec![
        Span::styled(format!("    ❯ {rendered_cmd}"), Style::default().fg(app.theme.header_fg).add_modifier(Modifier::BOLD)),
    ]));
    lines.push(Line::raw(""));
    lines.push(
        Line::from(vec![
            Span::styled(" [ Enter ] ", app.theme.key_badge_style()),
            Span::styled(" Connect    ", app.theme.key_desc_style()),
            Span::styled(" [ s / ^F ] ", app.theme.key_badge_style()),
            Span::styled(" SFTP    ", app.theme.key_desc_style()),
            Span::styled(" [ x / ^X ] ", app.theme.key_badge_style()),
            Span::styled(" X11    ", app.theme.key_desc_style()),
            Span::styled(" [ t / ^L ] ", app.theme.key_badge_style()),
            Span::styled(" Tunnel/TOR", app.theme.key_desc_style()),
        ])
        .centered(),
    );
    lines.push(
        Line::from(vec![
            Span::styled(" [ k ] ", app.theme.key_badge_style()),
            Span::styled(" Copy Key (ssh-copy-id)    ", app.theme.key_desc_style()),
            Span::styled(" [ d / ^D ] ", app.theme.key_badge_style()),
            Span::styled(" Delete Host    ", app.theme.key_desc_style()),
            Span::styled(" [ Esc ] ", app.theme.key_badge_style()),
            Span::styled(" Close", app.theme.key_desc_style()),
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
    let area = centered_rect(84, 88, f.area());
    f.render_widget(Clear, area);

    let mut lines = ascii_art::render_help_header(&app.theme);
    lines.push(Line::raw(""));

    let shortcuts: &[(&str, &[(&str, &str)])] = &[
        ("Navigation & Mouse", &[
            ("↑ / ↓, k / j", "Move selection up / down"),
            ("PageUp / PageDn", "Scroll one page up / down"),
            ("Home / End", "Jump to top / bottom of list"),
            ("Left Click", "Select / Double-click to connect"),
            ("Right Click", "Inspect Host / Open Context Menu"),
        ]),
        ("Connections & Networking", &[
            ("Enter", "Connect to selected SSH host"),
            ("Ctrl+F / s", "SSHS Explorer (Dual-Pane SFTP Commander)"),
            ("Ctrl+X / x", "Launch X11 GUI forwarding session (-Y -C)"),
            ("Ctrl+L / Ctrl+P", "Tunneling, TOR SOCKS, Port Knocking & X11 Hub"),
            ("Tab", "Open Host Inspector & Details modal"),
            ("Ctrl+N", "Add a new SSH host to ~/.ssh/config"),
            ("Ctrl+D / Delete", "Delete selected host from ~/.ssh/config"),
            ("Type text", "Fuzzy search and filter hosts"),
            ("Esc", "Clear search query or quit"),
        ]),
        ("Customization & Help", &[
            ("Ctrl+T / F2", "Cycle visual theme dynamically"),
            ("Ctrl+S", "Save active theme to ~/.config/sshs/config.toml"),
            ("Ctrl+A", "Toggle visual animations on / off"),
            ("Ctrl+R", "Reload SSH config files from disk"),
            ("Ctrl+U", "Clear the entire search query"),
            ("Ctrl+? / Ctrl+H / F1", "Toggle this help dialog"),
        ]),
    ];

    for (section, items) in shortcuts {
        lines.push(Line::from(vec![
            Span::styled(format!("  ── {section} ──"), Style::default().fg(app.theme.secondary).add_modifier(Modifier::BOLD)),
        ]));

        for (key, desc) in *items {
            lines.push(Line::from(vec![
                Span::styled(format!("    {key:<22}"), app.theme.key_badge_style()),
                Span::styled(format!(" {desc}"), Style::default().fg(app.theme.header_fg)),
            ]));
        }
        lines.push(Line::raw(""));
    }

    lines.push(
        Line::from(vec![
            Span::styled(" [ Esc / Enter ] ", app.theme.key_badge_style()),
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

    #[test]
    fn test_search_resets_selection_to_top_match() {
        let config = test_config();
        let mut app = App::new(&config).unwrap();

        assert_eq!(app.hosts.len(), 4);

        app.next();
        assert_eq!(app.table_state.selected(), Some(1));

        for c in "match".chars() {
            type_char(&mut app, c);
        }

        assert_eq!(app.hosts.len(), 3);
        assert_eq!(app.hosts.iter().next().unwrap().name, "match1");
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

    #[test]
    fn test_add_host_modal_toggle() {
        let config = test_config();
        let mut app = App::new(&config).unwrap();
        assert!(!app.show_add_host_modal);

        let stdout = io::stdout().lock();
        let backend = CrosstermBackend::new(stdout);
        let terminal = Rc::new(RefCell::new(Terminal::new(backend).unwrap()));

        let key = KeyEvent::new(KeyCode::Char('n'), KeyModifiers::CONTROL);
        let action = app.on_key_press_ctrl(&terminal, key);
        assert_eq!(action, AppKeyAction::Ok);
        assert!(app.show_add_host_modal);
    }

    #[test]
    fn test_delete_host_modal_toggle() {
        let config = test_config();
        let mut app = App::new(&config).unwrap();
        assert!(app.show_delete_modal.is_none());

        let stdout = io::stdout().lock();
        let backend = CrosstermBackend::new(stdout);
        let terminal = Rc::new(RefCell::new(Terminal::new(backend).unwrap()));

        let key = KeyEvent::new(KeyCode::Char('d'), KeyModifiers::CONTROL);
        let action = app.on_key_press_ctrl(&terminal, key);
        assert_eq!(action, AppKeyAction::Ok);
        assert_eq!(app.show_delete_modal.as_deref(), Some("match1"));
    }

    #[test]
    fn test_tunnel_modal_toggle() {
        let config = test_config();
        let mut app = App::new(&config).unwrap();
        assert!(!app.show_tunnel_modal);

        let stdout = io::stdout().lock();
        let backend = CrosstermBackend::new(stdout);
        let terminal = Rc::new(RefCell::new(Terminal::new(backend).unwrap()));

        let key = KeyEvent::new(KeyCode::Char('l'), KeyModifiers::CONTROL);
        let action = app.on_key_press_ctrl(&terminal, key);
        assert_eq!(action, AppKeyAction::Ok);
        assert!(app.show_tunnel_modal);
        assert_eq!(app.tunnel_form.as_ref().unwrap().host.name, "match1");
    }
}
