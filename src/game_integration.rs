use std::ptr;
use winapi::um::winuser::{FindWindowA, GetWindowThreadProcessId};

pub struct GameIntegration {
    hwnd: *mut std::ffi::c_void,
}

impl GameIntegration {
    pub fn new() -> Self {
        let hwnd = unsafe { FindWindowA(ptr::null_mut(), "Rainbow Six Siege") };
        GameIntegration { hwnd }
    }

    pub fn get_process_id(&self) -> Option<u32> {
        if self.hwnd.is_null() {
            return None;
        }
        let mut process_id: u32 = 0;
        unsafe {
            GetWindowThreadProcessId(self.hwnd as _, &mut process_id);
        }
        Some(process_id)
    }
}