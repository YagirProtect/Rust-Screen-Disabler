# Rust Screen Disabler (Global Hotkeys)

A tiny cross-platform Rust app that turns your display **off/on** via global hotkeys.

This project is a practical playground where I learned how to:
- listen to **global hotkeys** across OSes
- call native OS APIs / system tools (WinAPI, `xset`, `pmset`)
- set up **GitHub Actions** workflows for Windows / Linux / macOS builds + artifacts

Created without vibecoding. Only hard teaching exipience and test on many platforms and crys


---

## Screenshots

<img src="/Screenshot.png" width="49%" />


* * *

## What it does

The app runs in the background (console app) and reacts to two hotkeys:

- **OFF hotkey** → tries to turn the display off / sleep the display
- **ON hotkey** → tries to wake the display

I intentionally use **two hotkeys** instead of “toggle”, because the real display state can desync after mouse/keyboard wake.

* * *

## Hotkeys

### Windows
- **OFF:** `Ctrl + Alt + F11`
- **ON:**  `Ctrl + Alt + F12`

### Linux (X11)
- **OFF:** `Ctrl + Alt + K`
- **ON:**  `Ctrl + Alt + L`

> Notes:
> - This implementation uses `xset dpms force off/on` (X11 DPMS).
> - On **Wayland** global hotkeys and DPMS behavior are compositor-specific → this app currently targets **X11**.
> - In **VirtualBox/VMs**, DPMS “power off” may be ignored (normal for VMs).

### macOS
- **OFF:** `Control + Option + F11`
- **ON:**  `Control + Option + F12`

* * *

## How it works (per OS)

### Windows
Uses WinAPI `SendMessageW(..., WM_SYSCOMMAND, SC_MONITORPOWER, state)` to request monitor power state.

### Linux (X11)
Calls:
- `xset dpms force off`
- `xset dpms force on`

### macOS
Calls:
- `pmset displaysleepnow` (sleep display)
- `caffeinate -u -t 1` (simulate user activity to wake display)

* * *

## Quick start

### Build
```bash
cargo build --release
