pub trait SystemCommands {
    fn lock() -> Result<(), String>;
    fn logoff() -> Result<(), String>;
    fn hibernate() -> Result<(), String>;
    fn suspend() -> Result<(), String>;
    fn restart() -> Result<(), String>;
    fn shutdown() -> Result<(), String>;
}

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "linux")]
mod linux;
//#[cfg(target_os = "macos")]
//mod mac;

#[cfg(target_os = "windows")]
pub use windows::WindowsCommands as SystemCommandsImpl;
#[cfg(target_os = "linux")]
pub use linux::LinuxCommands as SystemCommandsImpl;
//#[cfg(target_os = "macos")]
//pub use mac::MacCommands as SystemCommandsImpl;
