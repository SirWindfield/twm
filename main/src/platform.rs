use twm_core::display::Display;
use winapi::um::winuser::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};

use log::trace;
use twm_core::bbox::BBox;

pub fn discover_main_display() -> Display {
    trace!("discover_main_display");
    unsafe {
        let width = GetSystemMetrics(SM_CXSCREEN);
        let height = GetSystemMetrics(SM_CYSCREEN);
        Display::with(0, BBox::with(0, 0, width, height))
    }
}

pub mod taskbar {
    use std::ffi::CString;
    use winapi::shared::windef::HWND;
    use winapi::um::winuser::{FindWindowA, ShowWindow, SW_HIDE, SW_SHOW};

    use log::trace;

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    struct SafeHandle(HWND);
    unsafe impl Send for SafeHandle {}
    unsafe impl Sync for SafeHandle {}

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct Taskbar {
        handle: SafeHandle,
    }

    impl Taskbar {
        pub fn new() -> Self {
            Self {
                handle: SafeHandle(std::ptr::null_mut()),
            }
        }

        pub fn fetch_taskbar_handle(&mut self) {
            trace!("fetch_taskbar_handle");
            let window_name = CString::new("Shell_TrayWnd").unwrap();
            let handle = unsafe { FindWindowA(window_name.as_ptr(), std::ptr::null()) };
            // if handle == NULL {
            //     let error = unsafe { GetLastError() };
            //     warn!("Failed to retrieve taskbar handle, error code {}", error);
            // }
            self.handle = SafeHandle(handle);
        }

        pub fn show(&self) {
            unsafe {
                ShowWindow(self.handle.0, SW_SHOW);
            }
        }

        pub fn hide(&self) {
            unsafe {
                ShowWindow(self.handle.0, SW_HIDE);
            }
        }
    }
}
