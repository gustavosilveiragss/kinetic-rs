use std::error::Error;

pub trait MouseController {
    fn get_position(&self) -> (i32, i32);
    fn set_position(&mut self, x: i32, y: i32) -> Result<(), Box<dyn Error>>;
    fn hook_mouse(&mut self) -> Result<(), Box<dyn Error>>;
    fn unhook_mouse(&mut self) -> Result<(), Box<dyn Error>>;
}

#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod windows;

#[cfg(unix)]
pub use unix::UnixMouseController;
#[cfg(windows)]
pub use windows::WindowsMouseController;

#[cfg(unix)]
pub fn create_controller() -> Box<dyn MouseController> {
    Box::new(UnixMouseController::new())
}

#[cfg(windows)]
pub fn create_controller() -> Box<dyn MouseController> {
    Box::new(WindowsMouseController::new())
}