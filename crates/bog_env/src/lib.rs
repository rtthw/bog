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
