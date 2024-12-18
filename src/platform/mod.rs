pub trait MouseController {
    type Error;

    fn new() -> Self;

    fn get_position(&self) -> (i32, i32);
    fn set_position(&mut self, x: i32, y: i32) -> Result<(), Self::Error>;
    fn hook_mouse(&mut self) -> Result<(), Self::Error>;
    fn unhook_mouse(&mut self) -> Result<(), Self::Error>;
}

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
pub type PlatformController = linux::LinuxMouseController;
#[cfg(target_os = "macos")]
pub type PlatformController = macos::MacOSMouseController;
#[cfg(target_os = "windows")]
pub type PlatformController = windows::WindowsMouseController;

