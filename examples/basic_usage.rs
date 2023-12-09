use sysctrlcmd::SystemCommandsImpl;

fn main() {
    // Attempt to lock the system screen
    //SystemCommandsImpl::lock().expect("Failed to lock the system screen");
    
    //SystemCommandsImpl::logoff().expect("Failed to log off");
    //SystemCommandsImpl::hibernate().expect("Failed to hibernate");
    //SystemCommandsImpl::suspend().expect("Failed to suspend");
    //SystemCommandsImpl::restart().expect("Failed to restart");
    SystemCommandsImpl::shutdown().expect("Failed to shut down");
}
