# 🦀 Tauri Simple Login

A minimal Tauri app demonstrating a simple login system with a **Rust backend** and **Vanilla JS frontend**.  
Uses an in-memory user store — built for learning and demo purposes.

![Rust](https://img.shields.io/badge/Rust-CE422B?style=flat&logo=rust&logoColor=white)
![JavaScript](https://img.shields.io/badge/Vanilla_JS-F6C90E?style=flat&logo=javascript&logoColor=black)
![Tauri](https://img.shields.io/badge/Tauri-Cross--Platform-1D9E75?style=flat&logo=tauri&logoColor=white)

---

> ⚠️ **Demo only.** Passwords are stored in plain text. Do not use this in production.

---

## Features

- Simple login form (username & password)
- Backend validation via Tauri commands
- Fully client–server separation using Tauri's `invoke`
- Cross-platform desktop app support

---

## Default Credentials

| Username | Password     |
|----------|--------------|
| `admin`  | `password123` |

---

## Directory Structure

```
tauri-simple-login/
├── src-tauri/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── src/
│   ├── index.html
│   └── index.js
├── package.json
├── tauri.conf.json
└── README.md
```

---

## Installation

**1. Install prerequisites**

Make sure you have [Node.js](https://nodejs.org/) and [Rust](https://www.rust-lang.org/tools/install) (with Cargo) installed.

**2. Clone the repo**

```bash
git clone https://github.com/blaxkmiradev/tauri-simple-login.git
cd tauri-simple-login
```

**3. Install frontend dependencies**

```bash
npm install
```

---

## Usage

### Dev mode

```bash
npm run dev
```

Opens a Tauri window with the login form. Use the default credentials (`admin` / `password123`) to sign in.

### Production build

```bash
npm run build
```

Generates a standalone executable for your platform. Check `src-tauri/target/release` for the output.

---

## How It Works

- The frontend sends login input to the Rust backend using Tauri's `invoke()` API
- The Rust backend validates credentials against an in-memory user store
- A success or error response is returned to the frontend

---

## License

MIT
