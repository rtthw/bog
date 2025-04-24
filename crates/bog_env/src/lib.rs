//! # Environment management
//!
//! You can think of your computer's "environment" as a set of components. The core components to
//! the average user's environment are:
//!
//! 1. Process manager.
//! 2. Filesystem.
//! 3. Device drivers.
//! 4. Windowing system.
//! 5. Networking interface.
//!     **Networking support is not yet planned.**
//!
//! ## Supported Environments
//!
//! - Linux
//!     - X11 and Wayland both supported.
//! - Android
//! - Windows
//!     - All versions released within the last 20 years are supported (Vista, 7, 8, 10, 11).
//! - Redox
//! - macOS
//! - iOS
//!
//! ## Features
//!
//! - Processing.
//!     - Spawning child processes.
//!     - Listing other processes.
//!     - Killing other processes.
//! - Windowing.
//!     - Creation.
//!     - Event processing.
//!     - Multiple windows.
//!
//! ## Component Lists
//!
//! ### Filesystems
//!
//! 1. [Unix-like](https://en.wikipedia.org/wiki/File_system#Unix_and_Unix-like_operating_systems),
//!    for Linux, Android, macOS, Redox, and pretty much anything else that isn't Windows.
//!
//! ### Windowing Systems
//!
//! The most common windowing systems accounted for are:
//!
//! 1. [X11](https://en.wikipedia.org/wiki/X_Window_System), for Linux.
//! 2. [Wayland](https://en.wikipedia.org/wiki/Wayland_(protocol)), for Linux.
//! 3. [DWM](https://en.wikipedia.org/wiki/Desktop_Window_Manager), for Windows.
//! 4. [SurfaceFlinger](https://source.android.com/docs/core/graphics/surfaceflinger-windowmanager),
//!    for Android
//! 5. [Quartz](https://en.wikipedia.org/wiki/Quartz_Compositor), for macOS.
//! 6. [Orbital](https://github.com/redox-os/orbital), for Redox.



pub const fn current_platform() -> Platform {
    Platform {
        cpu_arch: std::env::consts::ARCH,
        os_family: std::env::consts::FAMILY,
        os_name: std::env::consts::OS,
    }
}

pub struct Platform {
    pub cpu_arch: &'static str,
    pub os_family: &'static str,
    pub os_name: &'static str,
}

impl Platform {
    pub fn could_be_desktop(&self) -> bool {
        match (self.os_family, self.os_name) {
            (_, "linux") => true,
            (_, "redox") => true,
            (_, "windows") => true,
            (_, "macos") => true,
            (_, "openbsd") => true,
            (_, "freebsd") => true,
            (_, "netbsd") => true,
            (_, "haiku") => true,

            _ => false,
        }
    }

    pub fn could_be_embedded(&self) -> bool {
        match (self.os_family, self.os_name) {
            (_, "linux") => true,
            (_, "redox") => true,
            (_, "openbsd") => true,
            (_, "freebsd") => true,
            (_, "netbsd") => true,
            ("itron", _) => true,

            _ => false,
        }
    }

    pub fn could_be_mobile(&self) -> bool {
        match (self.os_family, self.os_name) {
            (_, "linux") => true,
            (_, "redox") => true,
            (_, "android") => true,
            (_, "ios") => true,

            _ => false,
        }
    }

    pub fn could_be_web(&self) -> bool {
        match (self.os_family, self.os_name) {
            (_, "wasi") => true,

            _ => false,
        }
    }

    pub fn could_support_display(&self) -> bool {
        match (self.os_family, self.os_name) {
            (_, "linux") => true,
            (_, "redox") => true,
            (_, "windows") => true,
            (_, "macos") => true,
            (_, "android") => true,
            (_, "ios") => true,
            (_, "openbsd") => true,
            (_, "freebsd") => true,
            (_, "netbsd") => true,
            (_, "haiku") => true,
            (_, "wasi") => true,

            _ => false,
        }
    }
}



/// Common features a wide number of platforms support.
pub struct PlatformFeature(u8);
/// Common features filesystems support.
pub struct FilesystemFeature(u8);
/// Common features windowing systems support.
pub struct WindowingFeature(u8);

bitflags::bitflags! {
    impl PlatformFeature: u8 {
        /// Platform supports process management (spawning children, killing processes, etc.).
        const PROCESS       = 0b_00000001;
        /// Platform supports a filesystem.
        const FILESYSTEM    = 0b_00000010;
        /// Platform has graphical windowing capabilities.
        const WINDOW        = 0b_00000100;
        /// Platform has networking capabilities.
        const NETWORK       = 0b_00001000;
        /// Platform will respond to "keyed" devices.
        const KEYBOARD      = 0b_00010000;
        /// Platform will respond to pointing devices.
        const POINTER       = 0b_00100000;
        /// Platform supports complex touch gestures.
        const GESTURES      = 0b_01000000;
    }

    impl FilesystemFeature: u8 {
        const FILES         = 0b_00000001;
        const DIRECTORIES   = 0b_00000010;
        const SOFT_LINK     = 0b_00000100;
        const HARD_LINK     = 0b_00001000;
        const SHARED_MEM    = 0b_00010000;
        const HOME_DIR      = 0b_00100000;
    }

    impl WindowingFeature: u8 {
        /// Clients can create multiple windows.
        const MULTIPLE          = 0b_00000001;
        /// Windows can stack on top of one another.
        const STACKING          = 0b_00000010;
        /// Platform supports "focus stealing" (when a client brings a window to the front).
        const GRAB_FOCUS        = 0b_00000100;
        /// Platform supports attention requests (think blinnking taskbar on Windows).
        const REQUEST_ATTENTION = 0b_00001000;
        /// Clients can "grab" the mouse cursor (stop it from leaving windows).
        const GRAB_POINTER      = 0b_00010000;
        /// Clients can hide the default mouse cursor.
        const HIDE_POINTER      = 0b_00100000;
        /// Windows can ignore the mouse cursor's inputs.
        const IGNORE_POINTER    = 0b_01000000;
    }
}
