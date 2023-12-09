use super::SystemCommands;
use nix::sys::reboot::{reboot, RebootMode};
use std::convert::Infallible;
use nix::errno::Errno;

pub struct LinuxCommands;

impl SystemCommands for LinuxCommands {
    /// Locks the current user session.
    /// This operation is not supported on Linux via this method.
    fn lock() -> Result<(), String> {
        Err("Operation not supported on Linux".into())
    }

    /// Logs off the current user session.
    /// This operation is not supported on Linux via this method.
    fn logoff() -> Result<(), String> {
        Err("Operation not supported on Linux".into())
    }

    /// Puts the system into hibernation.
    /// Halts the system without powering off. The system state is not preserved.
    fn hibernate() -> Result<(), String> {
        translate_nix_result(reboot(RebootMode::RB_HALT_SYSTEM))
    }

    /// Suspends the system.
    /// Enters software suspend mode where the system state is preserved.
    fn suspend() -> Result<(), String> {
        translate_nix_result(reboot(RebootMode::RB_SW_SUSPEND))
    }

    /// Restarts the system.
    fn restart() -> Result<(), String> {
        translate_nix_result(reboot(RebootMode::RB_AUTOBOOT))
    }

    /// Shuts down the system.
    fn shutdown() -> Result<(), String> {
        translate_nix_result(reboot(RebootMode::RB_POWER_OFF))
    }
}

/// Translates a Nix result to a standard Rust `Result`.
/// In case of an error, it returns a `String` with the error description.
fn translate_nix_result(result: Result<Infallible, Errno>) -> Result<(), String> {
    match result {
        Ok(_) => unreachable!(), // Infallible indicates this branch can never occur
        Err(err) => Err(format!("System command failed: {}", err)),
    }
}
