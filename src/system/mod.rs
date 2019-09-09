#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "macos")]
mod macos;

pub trait SystemManager {
    fn get_current_window_title(&self) -> Option<String>;
    fn get_current_window_class(&self) -> Option<String>;
    fn get_current_window_executable(&self) -> Option<String>;
}

// TODO: change windows and linux implementations to avoid initialize() call and use constructor instead

// LINUX IMPLEMENTATION
#[cfg(target_os = "linux")]
pub fn get_manager() -> impl SystemManager {
    let manager = linux::LinuxSystemManager{};
    manager.initialize();
    manager
}

// WINDOWS IMPLEMENTATION
#[cfg(target_os = "windows")]
pub fn get_manager() -> impl SystemManager {
    let manager = windows::WindowsSystemManager{};
    manager.initialize();
    manager
}

// MAC IMPLEMENTATION
#[cfg(target_os = "macos")]
pub fn get_manager() -> impl SystemManager {
    macos::MacSystemManager::new()
}