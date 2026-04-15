#[cfg(windows)]
mod platform {
    use windows::Win32::UI::WindowsAndMessaging::{SetWindowPos, HWND_TOPMOST, SWP_SHOWWINDOW, SWP_NOMOVE, SWP_NOSIZE, FindWindowA};
    
    pub fn set_always_on_top() {
        unsafe {
            // Windows 下使用 FindWindow 查找窗口
            let hwnd = FindWindowA(
                None,
                windows::core::PCSTR(b"Color Clicker\0".as_ptr()),
            );
            
            if !hwnd.is_invalid() {
                // 设置窗口置顶
                let _ = SetWindowPos(
                    hwnd,
                    HWND_TOPMOST,
                    0, 0, 0, 0,
                    SWP_SHOWWINDOW | SWP_NOMOVE | SWP_NOSIZE,
                );
            }
        }
    }
}

#[cfg(not(windows))]
mod platform {
    pub fn set_always_on_top() {}
}

pub fn enforce_always_on_top() {
    platform::set_always_on_top();
}
