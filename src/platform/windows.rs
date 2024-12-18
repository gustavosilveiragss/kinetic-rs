use windows_sys::Win32::Foundation::POINT;
use windows_sys::Win32::UI::WindowsAndMessaging::{GetCursorPos, SetCursorPos};

pub struct WindowsMouseController;

impl super::MouseController for WindowsMouseController {
    type Error = ();

    fn new() -> Self {
        Self
    }

    fn get_position(&self) -> (i32, i32) {
        unsafe {
            let mut point = POINT { x: 0, y: 0 };
            GetCursorPos(&mut point);
            (point.x, point.y)
        }
    }

    fn set_position(&mut self, x: i32, y: i32) -> Result<(), Self::Error> {
        unsafe {
            if SetCursorPos(x, y) == 0 {
                return Err(());
            }
        }
        Ok(())
    }

    fn hook_mouse(&mut self) -> Result<(), Self::Error> {
        // Not implemented.
        Ok(())
    }

    fn unhook_mouse(&mut self) -> Result<(), Self::Error> {
        // Not implemented.
        Ok(())
    }
}

