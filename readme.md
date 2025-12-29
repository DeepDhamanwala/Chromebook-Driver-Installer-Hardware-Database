# One-Click Chromebook Driver Installer

A Rust-based automated driver installation system designed for Chromebooks running Linux. This tool simplifies the post-installation process by automatically detecting hardware and fetching the correct drivers.

## 🚀 Features

* **Automated Hardware Detection:** Identifies device model and chipset (e.g., Kaby Lake, Skylake) via a local JSON database.
* **Structured Database:** Maps over 200+ Chromebook models to their specific driver requirements.
* **Dynamic Updates:** Capable of fetching the latest driver metadata at runtime without rebuilding the application.
* **High Performance:** Built with Rust for memory safety and speed.

## 🛠️ Usage

### Prerequisites
* Rust (Cargo) installed on the host machine.
