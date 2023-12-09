use super::SystemCommands;
use winapi::um::winuser::{ExitWindowsEx, LockWorkStation};
use winapi::um::powrprof::SetSuspendState;
use winapi::um::winnt::{EWX_LOGOFF, EWX_REBOOT, EWX_SHUTDOWN, EWX_FORCE};

pub struct WindowsCommands;

impl WindowsCommands {
    pub fn lock() -> Result<(), String> {
        unsafe {
            if LockWorkStation() == 0 {
                return Err("Failed to lock workstation".into());
            }
        }
        Ok(())
    }

    pub fn logoff() -> Result<(), String> {
        unsafe {
            if ExitWindowsEx(EWX_LOGOFF, 0) == 0 {
                return Err("Failed to log off".into());
            }
        }
        Ok(())
    }

    pub fn hibernate() -> Result<(), String> {
        unsafe {
            if SetSuspendState(1, 0, 1) == 0 {
                return Err("Failed to hibernate".into());
            }
        }
        Ok(())
    }

    pub fn suspend() -> Result<(), String> {
        unsafe {
            if SetSuspendState(0, 0, 1) == 0 {
                return Err("Failed to suspend".into());
            }
        }
        Ok(())
    }

    pub fn restart() -> Result<(), String> {
        unsafe {
            if ExitWindowsEx(EWX_REBOOT | EWX_FORCE, 0) == 0 {
                return Err("Failed to restart".into());
            }
        }
        Ok(())
    }

    pub fn shutdown() -> Result<(), String> {
        unsafe {
            if ExitWindowsEx(EWX_SHUTDOWN | EWX_FORCE, 0) == 0 {
                return Err("Failed to shut down".into());
            }
        }
        Ok(())
    }
}
