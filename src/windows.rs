use winapi::um::winuser::{ExitWindowsEx, LockWorkStation};
use winapi::um::powrprof::SetSuspendState;
use winapi::um::winuser::{EWX_LOGOFF, EWX_REBOOT, EWX_SHUTDOWN, EWX_FORCE};
use winapi::um::winnt::{TOKEN_ADJUST_PRIVILEGES, TOKEN_QUERY, SE_PRIVILEGE_ENABLED, LUID_AND_ATTRIBUTES, TOKEN_PRIVILEGES};
use winapi::um::securitybaseapi::AdjustTokenPrivileges;
use winapi::um::processthreadsapi::{OpenProcessToken, GetCurrentProcess};
use winapi::um::errhandlingapi::GetLastError;
use std::ptr;
use std::ffi::CString;
use winapi::um::winbase::LookupPrivilegeValueA;

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
        WindowsCommands::enable_shutdown_privilege()?;
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
        WindowsCommands::enable_shutdown_privilege()?;
        unsafe {
            if ExitWindowsEx(EWX_SHUTDOWN | EWX_FORCE, 0) == 0 {
                return Err("Failed to shut down".into());
            }
        }
        Ok(())
    }
    /// Enables the shutdown privilege for the current process.
    fn enable_shutdown_privilege() -> Result<(), String> {
        unsafe {
            let mut h_token = ptr::null_mut();
            let process = GetCurrentProcess();
            if OpenProcessToken(process, TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY, &mut h_token) == 0 {
                return Err(format!("Failed to open process token: {}", GetLastError()));
            }

            let mut luid = winapi::um::winnt::LUID {
                LowPart: 0,
                HighPart: 0,
            };
            let se_shutdown_name = CString::new("SeShutdownPrivilege").expect("CString::new failed");
            if LookupPrivilegeValueA(ptr::null_mut(), se_shutdown_name.as_ptr(), &mut luid) == 0 {
                return Err(format!("Failed to look up privilege value: {}", GetLastError()));
            }

            let mut tkp = TOKEN_PRIVILEGES {
                PrivilegeCount: 1,
                Privileges: [LUID_AND_ATTRIBUTES {
                    Attributes: SE_PRIVILEGE_ENABLED,
                    Luid: luid,
                }],
            };

            if AdjustTokenPrivileges(h_token, 0, &mut tkp, 0, ptr::null_mut(), ptr::null_mut()) == 0 {
                return Err(format!("Failed to adjust token privileges: {}", GetLastError()));
            }
        }
        Ok(())
    }
}
