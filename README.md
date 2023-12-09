# sysctrlcmd
sysctrlcmd is a Rust crate providing a unified and cross-platform interface for executing system-level commands like locking the workstation, logging off the current user, hibernating, suspending, restarting, and shutting down the system. It's designed to simplify the integration of system command functionalities in Rust applications, offering a consistent API across different operating systems.


## Supported Commands
The table below outlines the support for various system commands across Windows, Linux, and MacOS:

| Command        | Windows | Linux | MacOS |
|----------------|---------|-------|-------|
| Lock           | ✔       | ❌     | ❌     |
| Logoff         | ✔       | ❌     | ❌    |
| Hibernate      | ✔       | ✔      | ❌     |
| Suspend        | ✔       | ✔      | ❌     |
| Restart        | ✔       | ✔      | ❌     |
| Shutdown       | ✔       | ✔      | ❌     |

* ✔ - Supported
* ❌ - Not Supported / Not Applicable
* Note: Some commands may require administrative privileges.

## Usage
```rust
use sysctrlcmd::SystemCommandsImpl;

fn main() {
    SystemCommandsImpl::lock().expect("Failed to lock the system screen");
    SystemCommandsImpl::logoff().expect("Failed to log off");
    SystemCommandsImpl::hibernate().expect("Failed to hibernate");
    SystemCommandsImpl::suspend().expect("Failed to suspend");
    SystemCommandsImpl::restart().expect("Failed to restart");
    SystemCommandsImpl::shutdown().expect("Failed to shut down");
}
```

## Installation
Add the following to your `Cargo.toml` file:

```toml
[dependencies]
sysctrlcmd = "*"
```
