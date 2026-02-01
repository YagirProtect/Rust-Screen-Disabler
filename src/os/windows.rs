use std::io;
use windows_sys::Win32::System::Console::GetConsoleWindow;
use windows_sys::Win32::UI::WindowsAndMessaging::{SendMessageW, HWND_BROADCAST, WM_SYSCOMMAND};

///Const from microsoft
///https://learn.microsoft.com/en-us/windows/win32/menurc/wm-syscommand?utm_source=chatgpt.com
/// SC_MAXIMIZE  Maximizes the window.
/// 0xF030

const SC_MONITORPOWER: usize = 0xF170;

pub enum EScreenState{
    PowerOn = -1isize,
    LowPoser = 1isize,
    Disabled = 2isize,
}

pub fn set_monitor_power(on: bool) -> io::Result<()> {
    let state: isize = if on { EScreenState::PowerOn } else { EScreenState::Disabled } as isize;
    let hwnd = unsafe { GetConsoleWindow() };

    let target = if hwnd != std::ptr::null_mut() { hwnd } else { HWND_BROADCAST };
    
    unsafe {
        SendMessageW(target, WM_SYSCOMMAND, SC_MONITORPOWER, state)
    };
    Ok(())
}