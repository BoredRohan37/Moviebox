#![cfg_attr(windows, windows_subsystem = "windows")]

#[cfg(windows)]
mod service;
#[cfg(windows)]
mod win32;

#[cfg(windows)]
fn main() {
    if let Err(error) = win32::run() {
        eprintln!("Application startup failed: {error}");
    }
}

#[cfg(not(windows))]
fn main() {
    eprintln!("youtube-desktop is a Windows 8.1 application.");
}
