//! Bog Environment



pub struct Platform {
    pub cpu_arch: &'static str,
    pub os_name: &'static str,
    pub os_family: &'static str,
}

pub const fn current_platform() -> Platform {
    Platform {
        cpu_arch: std::env::consts::ARCH,
        os_name: std::env::consts::OS,
        os_family: std::env::consts::FAMILY,
    }
}
