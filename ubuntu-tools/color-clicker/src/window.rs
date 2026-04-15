#[cfg(target_os = "linux")]
mod platform {
    use x11::xlib::*;
    use std::ffi::{CStr, CString};
    use std::ptr;
    
    unsafe fn find_window_by_title(display: *mut Display, root: Window, title: &str) -> Window {
        let mut root_return: Window = 0;
        let mut parent_return: Window = 0;
        let mut children: *mut Window = ptr::null_mut();
        let mut nchildren: u32 = 0;
        
        if XQueryTree(display, root, &mut root_return, &mut parent_return, &mut children, &mut nchildren) != 0 {
            for i in 0..nchildren {
                let child = *children.offset(i as isize);
                
                // 获取窗口标题
                let mut actual_type: Atom = 0;
                let mut actual_format: i32 = 0;
                let mut nitems: u64 = 0;
                let mut bytes_after: u64 = 0;
                let mut prop: *mut u8 = ptr::null_mut();
                
                let net_wm_name = XInternAtom(display, CString::new("_NET_WM_NAME").unwrap().as_ptr(), 0);
                
                if XGetWindowProperty(display, child, net_wm_name, 0, 1024, 0, 0, &mut actual_type, &mut actual_format, &mut nitems, &mut bytes_after, &mut prop) == 0 {
                    if !prop.is_null() {
                        let window_title = CStr::from_ptr(prop as *const i8).to_string_lossy();
                        XFree(prop as *mut _);
                        
                        if window_title.contains(title) {
                            // 递归查找子窗口
                            let sub = find_window_by_title(display, child, title);
                            if sub != 0 {
                                XFree(children as *mut _);
                                return sub;
                            }
                            XFree(children as *mut _);
                            return child;
                        }
                    }
                }
                
                // 递归搜索子窗口
                let sub = find_window_by_title(display, child, title);
                if sub != 0 {
                    XFree(children as *mut _);
                    return sub;
                }
            }
            
            if !children.is_null() {
                XFree(children as *mut _);
            }
        }
        
        0
    }
    
    pub fn set_always_on_top() {
        unsafe {
            let display = XOpenDisplay(ptr::null());
            if display.is_null() {
                return;
            }
            
            let root = XDefaultRootWindow(display);
            
            // 查找标题包含 "Color Clicker" 的窗口
            let window = find_window_by_title(display, root, "Color Clicker");
            
            if window != 0 {
                // 设置置顶
                XRaiseWindow(display, window);
                
                // 设置 _NET_WM_STATE_ABOVE
                let net_wm_state = XInternAtom(display, CString::new("_NET_WM_STATE").unwrap().as_ptr(), 0);
                let net_wm_state_above = XInternAtom(display, CString::new("_NET_WM_STATE_ABOVE").unwrap().as_ptr(), 0);
                
                if net_wm_state != 0 && net_wm_state_above != 0 {
                    XChangeProperty(display, window, net_wm_state, 4, 32, PropModeReplace, &net_wm_state_above as *const _ as *const u8, 1);
                }
                
                XFlush(display);
            }
            
            XCloseDisplay(display);
        }
    }
}

#[cfg(not(target_os = "linux"))]
mod platform {
    pub fn set_always_on_top() {}
}

pub fn enforce_always_on_top() {
    platform::set_always_on_top();
}
