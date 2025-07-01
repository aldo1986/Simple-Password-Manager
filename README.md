# 🔐 Password Keeper

**Password Keeper** is a minimal, secure password manager built with **Rust** and a native desktop GUI using `eframe`/`egui`.


---

## 🚀 Features

- 🔐 Master password protection with `Argon2`
- 🔒 AES-GCM encryption for all entries
- 🧠 Memory-zeroing with `zeroize` to avoid data leaks
- 📋 Clipboard support for quick copy
- ➕ Add / 🗑️ Delete password entries
- 💾 Persistent encrypted storage (`serde` + `base64`)

---

## 🧪 Tech Stack

- **Rust** (safe systems programming)
- `aes-gcm`, `argon2`, `rand`, `zeroize` for crypto
- `eframe` / `egui` for the GUI
- `serde`, `serde_json` for serialization
- `arboard` for clipboard access

---
---
📦 Distribution
This application is cross-platform but currently tested on:

✅ Windows 10/11

🧪 Linux (X11/Wayland)

🚧 macOS (build support via GitHub Actions)

🔐 Security Notes
Your data is encrypted locally.

Encryption is done using AES-256-GCM.

The key is derived using Argon2 from your master password.

All sensitive fields are zeroized from memory when possible.

📄 License
MIT License. Open for contributions or suggestions!

🙋‍♂️ About the Author
Built as a personal project to explore secure storage and GUI development in Rust.
Feel free to connect on LinkedIn or open issues for ideas or bugs.
## ⚙️ Build & Run

```bash
# Clone and build
git clone https://github.com/your-username/password-keeper.git
cd password-keeper
cargo build --release


