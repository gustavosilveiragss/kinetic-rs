#[cfg(windows)]
use std::error::Error;
use windows_sys::Win32::Foundation::POINT;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    GetCursorPos, SetCursorPos,
};

pub struct WindowsMouseController {
    // Windows doesn't need persistent state like X11 does
}

impl WindowsMouseController {
    pub fn new() -> Self {
        Self {}
    }
}

impl super::MouseController for WindowsMouseController {
    fn get_position(&self) -> (i32, i32) {
        unsafe {
            let mut point = POINT { x: 0, y: 0 };
            GetCursorPos(&mut point);
            (point.x, point.y)
        }
    }

    fn set_position(&mut self, x: i32, y: i32) -> Result<(), Box<dyn Error>> {
        unsafe {
            if SetCursorPos(x, y) == 0 {
                return Err("Failed to set cursor position".into());
            }
        }
        Ok(())
    }

    fn hook_mouse(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn unhook_mouse(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}