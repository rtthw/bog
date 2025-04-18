//! Bog Environment



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
