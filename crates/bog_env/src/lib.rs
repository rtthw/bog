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



pub struct PlatformFeature(u8);
pub struct FilesystemFeature(u8);
pub struct WindowingFeature(u8);

bitflags::bitflags! {
    impl PlatformFeature: u8 {
        const PROCESS       = 0b_00000001;
        const FILESYSTEM    = 0b_00000010;
        const WINDOW        = 0b_00000100;
        const NETWORK       = 0b_00001000;
        const KEYBOARD      = 0b_00010000;
        const POINTER       = 0b_00100000;
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
        const MULTIPLE          = 0b_00000001;
        const GRAB_FOCUS        = 0b_00000010;
        const REQUEST_ATTENTION = 0b_00000100;
        const GRAB_POINTER      = 0b_00001000;
        const HIDE_POINTER      = 0b_00010000;
        const IGNORE_POINTER    = 0b_00010000;
    }
}
