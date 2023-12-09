# sysctrlcmd
Rust crate offering unified, cross-platform system command functionalities like lock, logoff, hibernate, suspend, restart, and shutdown, simplifying system-level operations in Rust applications.


## Supported Commands

| Command        | Windows | Linux | MacOS |
|----------------|---------|-------|-------|
| Lock           | ✔       | ❌     | ❌     |
| Logoff         | ✔       | ❌     | ❌    |
| Hibernate      | ✔       | ❌     | ❌     |
| Suspend        | ✔       | ❌     | ❌     |
| Restart        | ✔       | ❌     | ❌     |
| Shutdown       | ✔       | ❌     | ❌     |

* ✔ - Supported
* ❌ - Not Supported / Not Applicable
* Note: Some commands may require administrative privileges.
