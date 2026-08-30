# sshs User Manual

A comprehensive guide to using, configuring, and mastering **sshs** — the modern Terminal User Interface (TUI) for SSH.

---

## Table of Contents

1. [Overview](#1-overview)
2. [Installation & Requirements](#2-installation--requirements)
3. [Quick Start & Basic Navigation](#3-quick-start--basic-navigation)
4. [Search & Filtering](#4-search--filtering)
5. [Themes & Visual Customization](#5-themes--visual-customization)
6. [ASCII Art Banners & Animations](#6-ascii-art-banners--animations)
7. [Host Details Inspector & Key Management](#7-host-details-inspector--key-management)
8. [SSHS Explorer (Dual-Pane File Commander)](#8-sshs-explorer-dual-pane-file-commander)
9. [Networking, Tunneling & Security Hub](#9-networking-tunneling--security-hub)
10. [Mouse & Touchpad Support](#10-mouse--touchpad-support)
11. [Host Management (Add & Delete)](#11-host-management-add--delete)
12. [Configuration Reference](#12-configuration-reference)
13. [Keyboard Shortcuts Matrix](#13-keyboard-shortcuts-matrix)
14. [Troubleshooting & FAQ](#14-troubleshooting--faq)

---

## 1. Overview

`sshs` parses your existing OpenSSH configuration files (`~/.ssh/config`, `/etc/ssh/ssh_config`, and included files) and presents an interactive, search-optimized terminal user interface.

With `sshs`, you can:
* Search, filter, and connect to remote SSH servers in milliseconds.
* Inspect complete connection parameters, JumpHosts, and identity files.
* Browse and transfer files using a built-in dual-pane file commander (SSHS Explorer) with real-time transfer progress gauges.
* Configure port forwarding, SOCKS5 proxies, TOR routing, VPN tunnels, and Post-Quantum cryptographic sessions.
* Customize your terminal experience with 51 curated color themes, 22 Tailwind palettes, and RGB gradient ASCII banners.

---

## 2. Installation & Requirements

### System Requirements
* **OpenSSH client** (`ssh` and `scp`) installed and in your `$PATH`.
* Optional: `rsync` on client/server for high-speed file transfers.
* Linux, macOS, BSD, or Windows (via Windows Terminal / WSL).

### Package Managers

#### Arch Linux / CachyOS
```bash
pacman -S sshs
```

#### macOS (Homebrew)
```bash
brew install sshs
```

#### Debian / Ubuntu
```bash
curl -LO https://github.com/quantumsheep/sshs/releases/latest/download/sshs-linux-amd64.deb
sudo apt install ./sshs-linux-amd64.deb
```

#### Alpine Linux
```bash
apk add sshs
```

#### Cargo (From Source)
```bash
cargo install --git https://github.com/quantumsheep/sshs
```

---

## 3. Quick Start & Basic Navigation

Run `sshs` in your terminal:
```bash
sshs
```

### Basic Controls
* **Move Selection**: Use <kbd>↑</kbd> / <kbd>↓</kbd> or Vim keys <kbd>k</kbd> / <kbd>j</kbd>.
* **Page Scroll**: <kbd>PageUp</kbd> / <kbd>PageDown</kbd> or <kbd>Ctrl+U</kbd> / <kbd>Ctrl+D</kbd>.
* **Jump to Boundaries**: <kbd>Home</kbd> / <kbd>End</kbd> or <kbd>g</kbd> / <kbd>G</kbd>.
* **Connect**: Press <kbd>Enter</kbd> (or left-click) to open an SSH session to the selected host.
* **Exit**: Press <kbd>Esc</kbd> or <kbd>Ctrl+C</kbd>.

---

## 4. Search & Filtering

`sshs` features a responsive multi-token fuzzy search engine.

* Simply start typing anywhere in the main view to search across:
  * Host names and patterns (`Host web-prod-01`)
  * Host aliases and nicknames
  * Actual host destinations / IP addresses (`HostName 192.168.1.50`)
  * Usernames (`User debian`)
* **Multi-Token Matching**: Separate words with spaces (e.g. `prod db 10.0`) to require all terms to match.
* **Clear Search**: Press <kbd>Ctrl+U</kbd> or press <kbd>Esc</kbd>.

---

## 5. Themes & Visual Customization

`sshs` ships with **51 built-in color themes** and supports all 22 Tailwind color palettes.

### Cycling Themes In-App
* Press **<kbd>Ctrl+T</kbd>** or **<kbd>F2</kbd>** to cycle through themes in real-time.
* Press **<kbd>Ctrl+S</kbd>** to immediately save the currently selected theme as your default in `~/.config/sshs/config.toml`.

### Specifying Themes via CLI
```bash
# Curated themes:
sshs --theme catppuccin
sshs --theme tokyonight
sshs --theme dracula
sshs --theme nord
sshs --theme gruvbox
sshs --theme rose-pine
sshs --theme onedark
sshs --theme cyberpunk
sshs --theme synthwave
sshs --theme matrix

# Tailwind palettes (emerald, violet, blue, amber, rose, cyan, etc.):
sshs --color emerald
```

### Supported Theme List
| Category | Themes |
|---|---|
| **Catppuccin** | `catppuccin` (Mocha), `catppuccin-macchiato`, `catppuccin-frappe`, `catppuccin-latte` |
| **Tokyo Night** | `tokyonight`, `tokyonight-storm`, `tokyonight-light` |
| **Rosé Pine** | `rose-pine`, `rose-pine-moon`, `rose-pine-dawn` |
| **Developer Classics** | `dracula`, `dracula-soft`, `nord`, `onedark`, `kanagawa`, `everforest`, `ayu-dark`, `solarized-dark`, `solarized-light`, `gruvbox`, `gruvbox-light` |
| **Vibrant & Retro** | `cyberpunk`, `synthwave`, `matrix`, `hacker-amber`, `sunset`, `ocean`, `crimson`, `lavender`, `aquarium` |
| **Tailwind Palettes** | `slate`, `gray`, `zinc`, `neutral`, `stone`, `red`, `orange`, `amber`, `yellow`, `lime`, `green`, `emerald`, `teal`, `cyan`, `sky`, `blue`, `indigo`, `violet`, `purple`, `fuchsia`, `pink`, `rose` |

---

## 6. ASCII Art Banners & Animations

Add custom RGB gradient ASCII art headers to your terminal interface:

```bash
# Available styles: slant (default), cyber, standard, mini, off
sshs --ascii-art slant
sshs --ascii-art cyber
sshs --ascii-art standard
sshs --ascii-art mini

# Minimalist mode (no ASCII banner)
sshs --no-ascii-art

# Disable animations (or toggle in-app with Ctrl+A)
sshs --no-animate
```

---

## 7. Host Details Inspector & Key Management

Press **<kbd>Tab</kbd>** on any host to open the **Host Details Inspector**:

```text
 ╭── Host Details Inspector: DietPi-Kerkut ────────────────────────────────╮
 │                                                                         │
 │  Host:          DietPi-Kerkut                                           │
 │  HostName:      100.96.93.90                                            │
 │  User:          pju                                                     │
 │  Port:          22                                                      │
 │  IdentityFile:  ~/.ssh/id_ed25519                                       │
 │  ProxyJump:     none                                                    │
 │                                                                         │
 │  Connection Preview:                                                    │
 │  ssh -p 22 -i ~/.ssh/id_ed25519 pju@100.96.93.90                        │
 │                                                                         │
 │  [ Enter: Connect ]  [ s: SFTP ]  [ t: Tunnel ]  [ k: Copy Key ]        │
 ╰─────────────────────────────────────────────────────────────────────────╯
```

### On-Demand SSH Key Installation (`ssh-copy-id`)
* While inside the Inspector modal, press **<kbd>k</kbd>**.
* `sshs` automatically invokes `ssh-copy-id` to copy your public key to the remote host for passwordless authentication.

---

## 8. SSHS Explorer (Dual-Pane File Commander)

Press **<kbd>Ctrl+F</kbd>** (or **<kbd>s</kbd>** in the Inspector) to launch **SSHS Explorer** — a built-in dual-pane file manager for local and remote SFTP/SSH filesystems.

```text
 ╭─ 💻 Local Filesystem (/home/x) ─╮ ╭─ 🌐 Remote SFTP [Home-DietPi] (.) ─╮
 │ ▌ [..]                          │ │   [..]                             │
 │   Documents/                    │ │   .bashrc                          │
 │   Downloads/                    │ │   9M2PJU-Bearing-Finder/           │
 │   backup.tar.gz                 │ │   dietpi.txt                       │
 ╰─────────────────────────────────╯ ╰────────────────────────────────────╯
```

### High-Speed `rsync` & `scp -O` Transfer Engine
* Transfers automatically prioritize streaming `rsync -avz --info=progress2` for high throughput, directory synchronization, and real-time metrics.
* If `rsync` is absent on a minimal server (e.g. DietPi, Raspberry Pi, FreeBSD, OpenWrt, Dropbear), `sshs` automatically falls back to legacy SCP (`scp -r -O`), guaranteeing 100% transfer compatibility.

### Real-Time Transfer Progress Gauge
When uploading or downloading files or directories (<kbd>F5</kbd> / <kbd>c</kbd>), an interactive modal tracks live metrics:

```text
 ╭── 📤 Uploading: project-build ──────────────────────── [Esc to Cancel] ──╮
 │                                                                          │
 │   Source:      /home/x/project-build                                     │
 │   Destination: Home-DietPi:./                                            │
 │                                                                          │
 │   [████████████████████████████░░░░░░░░░░░░░░░░] ⠹ 68% (14.2 MB/20.8 MB) │
 │                                                                          │
 │   ⚡ Speed: 3.45 MB/s        ⏱ Elapsed: 00:04                            │
 ╰──────────────────────────────────────────────────────────────────────────╯
```

### Explorer Shortcuts
| Key | Action |
|---|---|
| <kbd>Tab</kbd> | Switch between Local and Remote panes |
| <kbd>Enter</kbd> | Open selected directory or view file |
| <kbd>F3</kbd> / <kbd>v</kbd> | View file contents in TUI viewer popup |
| <kbd>F5</kbd> / <kbd>c</kbd> | Copy / Transfer file or directory to opposite pane |
| <kbd>F7</kbd> / <kbd>m</kbd> | Create new directory |
| <kbd>F8</kbd> / <kbd>d</kbd> / <kbd>Delete</kbd> | Delete selected file or directory |
| <kbd>F9</kbd> / <kbd>o</kbd> | Open host in external file manager (e.g. Dolphin `sftp://`) |
| <kbd>r</kbd> | Refresh directory listings |
| <kbd>Esc</kbd> / <kbd>q</kbd> | Exit SSHS Explorer |

---

## 9. Networking, Tunneling & Security Hub

Press **<kbd>Ctrl+L</kbd>**, **<kbd>Ctrl+P</kbd>**, or **<kbd>t</kbd>** in the Inspector to open the **Networking & Tunneling Hub**:

```text
 ╭── SSH Tunneling & Networking Hub: Home-DietPi ──────────────────────────╮
 │                                                                         │
 │  Select an advanced connection mode:                                    │
 │                                                                         │
 │  [1] Local Port Forward (-L)      [5] TOR Onion Routing                 │
 │  [2] Remote Port Forward (-R)     [6] Port Knocking Automation          │
 │  [3] Dynamic SOCKS5 Proxy (-D)    [7] X11 GUI Forwarding (-Y)           │
 │  [4] TUN/TAP VPN Tunnel (-w)      [8] Post-Quantum Crypto (PQC)         │
 │                                                                         │
 ╰─────────────────────────────────────────────────────────────────────────╯
```

### Tunneling Modes Explained

1. **Local Port Forwarding (`-L`)**:
   Forward a local port to a remote destination:
   ```text
   8080:localhost:80
   5432:db.internal.net:5432
   ```

2. **Remote Reverse Forwarding (`-R`)**:
   Expose local development ports to the remote server:
   ```text
   9000:localhost:3000
   ```

3. **Dynamic SOCKS5 Proxy (`-D`)**:
   Turn the remote SSH connection into a private encrypted SOCKS5 proxy:
   ```text
   1080
   ```

4. **TUN / TAP VPN Tunneling (`-w`)**:
   Create point-to-point Layer 3 (TUN) or Layer 2 (TAP) virtual network devices (`any:any`).

5. **SSH over TOR**:
   Route SSH connections through the local Tor SOCKS proxy (`127.0.0.1:9050`) or `.onion` hidden services.

6. **Port Knocking Automation**:
   Send a TCP port knock sequence (e.g. `7000, 8000, 9000`) to unlock firewalls prior to establishing the SSH handshake.

7. **X11 GUI Forwarding (`-Y -C`)**:
   Run graphical X11 Linux applications from the remote server on your local screen with compression.

8. **Post-Quantum Cryptography (PQC) Mode**:
   Enforce post-quantum hybrid key exchange algorithms (`mlkem768x25519-sha512@openssh.com`, `sntrup761x25519-sha512@openssh.com`) to safeguard traffic against future quantum decryption.

---

## 10. Mouse & Touchpad Support

`sshs` provides full mouse support out of the box:

* **Left Click**:
  * Click any row in the host table to select it; click again or double-click to connect.
  * Click any bottom toolbar button (`Connect`, `Details`, `SFTP`, `X11`, `Tunnel`, `Add`, `Del`, `Theme`, `Help`, `Quit`).
  * Click between Explorer panes to switch active focus; double-click a folder to enter it.
  * Click any modal button (`[ Connect ]`, `[ Cancel ]`, `[ Delete ]`, `[ Mode 1-8 ]`).
* **Right Click**:
  * Right-click any host in the table to open its **Host Details Inspector**.
  * Right-click any file/folder in SSHS Explorer to open the **Context Menu**.
  * Right-click inside any modal to dismiss/close it.
* **Scroll Wheel**:
  * Scroll up/down to navigate host lists, explorer file tables, and help menus.

---

## 11. Host Management (Add & Delete)

### Adding a Host (<kbd>Ctrl+N</kbd>)
Press **<kbd>Ctrl+N</kbd>** to open the interactive **Add Host** modal:
* Enter Host alias, HostName/IP, User, Port, and IdentityFile.
* `sshs` automatically writes the new block cleanly to `~/.ssh/config`.

### Deleting a Host (<kbd>Ctrl+D</kbd> / <kbd>Delete</kbd>)
Press **<kbd>Ctrl+D</kbd>** or **<kbd>Delete</kbd>** on any host:
* A confirmation prompt will appear before removing the host definition from `~/.ssh/config`.

---

## 12. Configuration Reference

Configuration files are stored at `~/.config/sshs/config.toml` (or `$XDG_CONFIG_HOME/sshs/config.toml`).

### Sample `config.toml`
```toml
# Default theme (e.g. "tokyonight", "catppuccin", "dracula", "nord", "emerald")
theme = "tokyonight"

# Default ASCII art banner style ("slant", "cyber", "standard", "mini", "off")
ascii_art = "slant"

# Enable or disable visual RGB gradient animations
animate = true

# Sort hosts alphabetically
sort = true

# Show proxy command details in table
show_proxy_command = false
```

### Environment Variables
* `SSHS_THEME`: Override the active color theme.
* `SSHS_ASCII_ART`: Override the ASCII art style (`slant`, `cyber`, `standard`, `mini`, `off`).

---

## 13. Keyboard Shortcuts Matrix

| Keybinding | Context | Action |
|---|---|---|
| <kbd>↑</kbd> / <kbd>↓</kbd>, <kbd>k</kbd> / <kbd>j</kbd> | Main View | Navigate up / down |
| <kbd>PageUp</kbd> / <kbd>PageDown</kbd> | Main View | Scroll page up / down |
| <kbd>Home</kbd> / <kbd>End</kbd> | Main View | Jump to top / bottom |
| <kbd>Enter</kbd> | Main View | Connect to selected SSH host |
| <kbd>Tab</kbd> | Main View | Open Host Details Inspector |
| <kbd>k</kbd> | Inspector | Copy SSH public key (`ssh-copy-id`) |
| <kbd>Ctrl+F</kbd> | Main View | Open SSHS Explorer |
| <kbd>Ctrl+X</kbd> | Main View | Connect with X11 GUI Forwarding (`ssh -Y -C`) |
| <kbd>Ctrl+L</kbd> / <kbd>Ctrl+P</kbd> | Main View | Open Networking & Tunneling Hub |
| <kbd>Ctrl+N</kbd> | Main View | Add new SSH host to config |
| <kbd>Ctrl+D</kbd> / <kbd>Delete</kbd> | Main View | Delete selected SSH host |
| <kbd>Ctrl+T</kbd> / <kbd>F2</kbd> | Main View | Cycle through color themes |
| <kbd>Ctrl+S</kbd> | Main View | Save active theme as default in `config.toml` |
| <kbd>Ctrl+A</kbd> | Main View | Toggle animations on / off |
| <kbd>Ctrl+R</kbd> | Main View | Reload SSH config files from disk |
| <kbd>Ctrl+U</kbd> | Main View | Clear active search query |
| <kbd>?</kbd> / <kbd>F1</kbd> / <kbd>Ctrl+H</kbd> | Main View | Open Help shortcut cheat sheet |
| <kbd>Esc</kbd> | Global | Dismiss modal, clear search, or quit |
| <kbd>Tab</kbd> | Explorer | Switch active pane (Local ↔ Remote) |
| <kbd>F3</kbd> / <kbd>v</kbd> | Explorer | Preview selected file |
| <kbd>F5</kbd> / <kbd>c</kbd> | Explorer | Transfer selected file or folder |
| <kbd>F7</kbd> / <kbd>m</kbd> | Explorer | Create new directory |
| <kbd>F8</kbd> / <kbd>d</kbd> | Explorer | Delete file or directory |
| <kbd>F9</kbd> / <kbd>o</kbd> | Explorer | Open host in external file manager (Dolphin) |
| <kbd>r</kbd> | Explorer | Refresh directory listings |

---

## 14. Troubleshooting & FAQ

### 1. `~/.ssh/config: No such file or directory`
Create the file if it does not exist:
```bash
mkdir -p ~/.ssh && touch ~/.ssh/config && chmod 600 ~/.ssh/config
```

### 2. "Permission denied (publickey)" in SSHS Explorer
Ensure your SSH public key is installed on the remote host. Open the Inspector (<kbd>Tab</kbd>) and press **<kbd>k</kbd>** to install your key with `ssh-copy-id`.

### 3. OpenSSH Post-Quantum Algorithm Warning
Modern OpenSSH versions (10.5p1+) display an informational notice when connecting to older SSH servers:
`** WARNING: connection is not using a post-quantum key exchange algorithm.`
`sshs` automatically filters this warning banner from error dialogs so transfers and commands execute seamlessly.

---

*Authored by 9M2PJU for the sshs community.*
