use winapi::um::winuser::{ExitWindowsEx, LockWorkStation};
use winapi::um::powrprof::SetSuspendState;
use winapi::um::winuser::{EWX_LOGOFF, EWX_REBOOT, EWX_SHUTDOWN, EWX_FORCE};

/// Commands for controlling system actions on Windows.
pub struct WindowsCommands;

impl WindowsCommands {
    /// Locks the workstation.
    /// 
    /// # Returns
    /// `Ok(())` on success, `Err(String)` on failure.
    pub fn lock() -> Result<(), String> {
        unsafe {
            if LockWorkStation() == 0 {
                return Err("Failed to lock workstation".into());
            }
        }
        Ok(())
    }

    /// Logs off the current user.
    /// 
    /// # Returns
    /// `Ok(())` on success, `Err(String)` on failure.
    pub fn logoff() -> Result<(), String> {
        unsafe {
            if ExitWindowsEx(EWX_LOGOFF, 0) == 0 {
                return Err("Failed to log off".into());
            }
        }
        Ok(())
    }

    /// Puts the computer into hibernation.
    /// 
    /// # Returns
    /// `Ok(())` on success, `Err(String)` on failure.
    pub fn hibernate() -> Result<(), String> {
        unsafe {
            if SetSuspendState(1, 0, 1) == 0 {
                return Err("Failed to hibernate".into());
            }
        }
        Ok(())
    }

    /// Suspends the system.
    /// 
    /// # Returns
    /// `Ok(())` on success, `Err(String)` on failure.
    pub fn suspend() -> Result<(), String> {
        unsafe {
            if SetSuspendState(0, 0, 1) == 0 {
                return Err("Failed to suspend".into());
            }
        }
        Ok(())
    }

    /// Restarts the system.
    /// 
    /// # Returns
    /// `Ok(())` on success, `Err(String)` on failure.
    pub fn restart() -> Result<(), String> {
        unsafe {
            if ExitWindowsEx(EWX_REBOOT | EWX_FORCE, 0) == 0 {
                return Err("Failed to restart".into());
            }
        }
        Ok(())
    }

    /// Shuts down the system.
    /// 
    /// # Returns
    /// `Ok(())` on success, `Err(String)` on failure.
    pub fn shutdown() -> Result<(), String> {
        unsafe {
            if ExitWindowsEx(EWX_SHUTDOWN | EWX_FORCE, 0) == 0 {
                return Err("Failed to shut down".into());
            }
        }
        Ok(())
    }
}
