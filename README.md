# Secure OS Layer 🚀🔒

Welcome to **Secure OS Layer** – a robust, privacy-centric operating system layer designed to empower developers and protect user data. This project bridges the gap between your OS and AI-powered applications by offering:

- **User Data Protection** 🔐  
  Users control which apps can access their data.
- **Shared Memory** 🧠  
  Centralized data storage for user context and preferences.
- **Curated AI App Store** 📲  
  Discover and install AI apps safely.
- **Developer-Friendly APIs & SDK** 🛠️  
  Simplify integration with clear APIs and a ready-to-use SDK.
- **Sandboxing & Security Enhancements** 🏰  
  Isolate apps using Docker (on Windows) for maximum security.

---

## Table of Contents 📖

- [Secure OS Layer 🚀🔒](#secure-os-layer-)
  - [Table of Contents 📖](#table-of-contents-)
  - [Features 💡](#features-)
  - [Architecture 🔍](#architecture-)
  - [Installation \& Usage ⚙️](#installation--usage-️)
    - [Backend (Rust)](#backend-rust)
    - [Electron UI](#electron-ui)
  - [SDK for Developers](#sdk-for-developers)
  - [Packaging as an EXE/Installer 🖥️📦](#packaging-as-an-exeinstaller-️)
    - [Build the Executable](#build-the-executable)
    - [Create an Installer with Inno Setup](#create-an-installer-with-inno-setup)
  - [Contributing 🤝](#contributing-)
  - [License 📄](#license-)
  - [Contact 📫](#contact-)

---

## Features 💡

- **Data Protection:** Users grant granular access permissions.
- **Shared Memory:** Consolidated storage for user data.
- **App Store:** A secure marketplace for AI apps.
- **Developer SDK:** Easy-to-use API for developers.
- **Sandboxing:** Isolated execution using Docker containers.
- **Security:** Built-in audit logging and encryption options.

---

## Architecture 🔍

**Secure OS Layer** comprises three primary components:

1. **Rust Backend (Actix-Web):**  
   Provides RESTful APIs, manages user data with SQLite, validates app permissions, and handles sandboxing (via Docker).

2. **Electron Front-End:**  
   A modern, dark/light mode UI for browsing and installing AI apps.

3. **Developer SDK (JavaScript):**  
   A lightweight SDK that wraps the REST API, making integration effortless for developers.

---

## Installation & Usage ⚙️

### Backend (Rust)

1. **Clone the Repository:**

   ```bash
   git clone https://github.com/yourusername/secure_os_layer.git
   cd secure_os_layer

2. **Initialize the Database:**

   Ensure your init.sql (located in the repository root) contains:

   ```sql
   CREATE TABLE IF NOT EXISTS user_data (
    id TEXT PRIMARY KEY,
    key TEXT NOT NULL,
    value TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS registered_apps (
    app_id TEXT PRIMARY KEY,
    app_name TEXT NOT NULL,
    allowed_permissions TEXT
    );

    CREATE TABLE IF NOT EXISTS installed_apps (
    install_id INTEGER PRIMARY KEY AUTOINCREMENT,
    app_id TEXT NOT NULL,
    install_date TEXT DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(app_id) REFERENCES registered_apps(app_id)
    );

Run the script:

   ```bash
   sqlite3 secure_os_layer.db < init.sql
```

3.**Build and Run:**

```bash
cargo build --release
cargo run
```

Your backend should be running at `http://127.0.0.1:8080`

### Electron UI

1.**Navigate to the UI Directory:**

```bash
cd ui-electron
npm install
npm start
```

The Electron app will launch, letting you browse and install AI apps.

## SDK for Developers

A JavaScript SDK is provided in the `sdk/` folder. To use it:

1.**Install via npm (after publishing) or locally:**

```bash
npm install secure-os-layer-sdk

```

2.**Example Usage:**

```js
import { SecureOSLayerSDK } from "secure-os-layer-sdk";

const sdk = new SecureOSLayerSDK("http://127.0.0.1:8080", "my-app-id");

sdk.getStatus()
  .then(status => console.log("Status:", status))
  .catch(err => console.error("Error:", err));
```

For more details, see the `SDK Documentation.`

## Packaging as an EXE/Installer 🖥️📦

### Build the Executable

1.**Run:**

```bash
cargo build --release
```

2.**Find the binary at:**

```bash
target\release\secure_os_layer.exe
```

### Create an Installer with Inno Setup

1.**Install Inno Setup from jrsoftware.org.**
2.**Create an Inno Setup Script `(SecureOSLayerInstaller.iss)`:**

```ini
[Setup]
AppName=Secure OS Layer
AppVersion=1.0.0
DefaultDirName={pf}\Secure OS Layer
DefaultGroupName=Secure OS Layer
OutputBaseFilename=SecureOSLayerInstaller
Compression=lzma
SolidCompression=yes

[Files]
Source: "target\release\secure_os_layer.exe"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\Secure OS Layer"; Filename: "{app}\secure_os_layer.exe"
```

3.**Compile the Script using Inno Setup to produce the installer EXE.**

## Contributing 🤝

We welcome contributions! 🚀 Follow these steps to contribute:

1.**Fork the repository.**

2.**Create a new branch: `git checkout -b my-feature-branch.`**

3.**Make your changes and commit them: `git commit -m "Add new feature".`**

4.**Push to the branch: `git push origin my-feature-branch.`**

5.**Open a pull request.**

Contributions are welcome! Please see `CONTRIBUTING.md` for our guidelines on how to contribute to the project.

## License 📄

This project is licensed under the MIT License. See the LICENSE file for details.

## Contact 📫

Email: <support@cybernix.in>

GitHub: @yashasrnair

Website: <https://www.cybernix.in>
