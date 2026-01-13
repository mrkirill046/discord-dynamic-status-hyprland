# `discord-dynamic-status-hyprland` 

![Rust](https://img.shields.io/badge/Rust-1.70+-orange?style=for-the-badge\&logo=rust)
![License MIT](https://img.shields.io/badge/license-MIT-green?style=for-the-badge)
![Platform: Linux](https://img.shields.io/badge/Platform-Linux-blue?style=for-the-badge) 
![WM](https://img.shields.io/badge/WM-Hyprland-purple?style=for-the-badge)

**Dynamic Discord Rich Presence for Hyprland (Wayland).**
Automatically updates your Discord status based on the active window.

---

## ✨ Features

* Shows the current active window in Discord
* Supports any application (settings in `config.sjon`)
* Lightweight and fast (Rust + Hyprland events)
* Works on Hyprland (Wayland)

---

## 🏗 Installation and Run

### 1. Clone repository
> Or use `yay -S dynamic-drpc-hyprland-bin` or `yay -S dynamic-drpc-hyprland-git`
```bash
git clone https://github.com/mrkirill046/discord-dynamic-status-hyprland.git
cd discord-dynamic-status-hyprland
cargo run --release
```

### 2. Configure `config.json` (in the `~/.local/share/dynamic-drpc-hyprland`)

```json
{
  "app_id": "1460605258072985705 (Done by me)",
  "default": {
    "state": "Chilling",
    "details": "At the workspace",
    "large_text": "Arch Linux-zen x86_64",
    "large_image": "arch",
    "small_text": "Hyprland (Wayland)",
    "small_image": "hyprland"
  },
  "classes": {
    "kitty": {
      "state": "At kitty",
      "details": "Writing command lines",
      "large_text": "Arch Linux-zen x86_64",
      "large_image": "arch",
      "small_text": "Kitty with ZSH",
      "small_image": "kitty"
    }
  }
} // etc
```

> **Important:** use your **Application ID** from Discord Developer Portal, **not a bot token**.

* I have also already added all images in the current default config using my App ID

---

## 🛠 Troubleshooting

* Discord must be **online** and **not in Invisible** mode
* All assets in `config.json` must exist in Discord Developer Portal → Art Assets
* Of course, you should have an internet connection :)

---

## 📝 License

MIT License — see [LICENSE](LICENSE)
