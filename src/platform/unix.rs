#[cfg(unix)]
use std::error::Error;
use x11::xlib;
use std::ptr;

pub struct UnixMouseController {
    display: *mut xlib::Display,
    root: xlib::Window,
}

impl UnixMouseController {
    pub fn new() -> Self {
        unsafe {
            let display = xlib::XOpenDisplay(ptr::null());
            let root = xlib::XDefaultRootWindow(display);
            Self { display, root }
        }
    }
}

impl super::MouseController for UnixMouseController {
    fn get_position(&self) -> (i32, i32) {
        unsafe {
            let mut root_return: xlib::Window = 0;
            let mut child_return: xlib::Window = 0;
            let mut root_x: i32 = 0;
            let mut root_y: i32 = 0;
            let mut win_x: i32 = 0;
            let mut win_y: i32 = 0;
            let mut mask_return: u32 = 0;

            xlib::XQueryPointer(
                self.display,
                self.root,
                &mut root_return,
                &mut child_return,
                &mut root_x,
                &mut root_y,
                &mut win_x,
                &mut win_y,
                &mut mask_return,
            );

            (root_x, root_y)
        }
    }

    fn set_position(&mut self, x: i32, y: i32) -> Result<(), Box<dyn Error>> {
        unsafe {
            xlib::XWarpPointer(self.display, 0, self.root, 0, 0, 0, 0, x, y);
            xlib::XFlush(self.display);
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

impl Drop for UnixMouseController {
    fn drop(&mut self) {
        unsafe {
            xlib::XCloseDisplay(self.display);
        }
    }
}
