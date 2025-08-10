use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Architecture {
    Win32X64,
    Win32Arm64,
    DarwinX64,
    DarwinArm64,
    LinuxX64,
    LinuxArm64,
    LinuxArmhf,
    AlpineX64,
    AlpineArm64,
    Universal,
}

impl Architecture {
    #[must_use]
    pub fn detect() -> Self {
        let os = std::env::consts::OS;
        let arch = std::env::consts::ARCH;

        match (os, arch) {
            ("windows", "x86_64") => Architecture::Win32X64,
            ("windows", "aarch64") => Architecture::Win32Arm64,
            ("macos", "x86_64") => Architecture::DarwinX64,
            ("macos", "aarch64") => Architecture::DarwinArm64,
            ("linux", "x86_64") => Architecture::LinuxX64,
            ("linux", "aarch64") => Architecture::LinuxArm64,
            ("linux", "arm") => Architecture::LinuxArmhf,
            _ => Architecture::Universal,
        }
    }

    #[must_use]
    pub fn to_platform_string(&self) -> Option<&str> {
        match self {
            Architecture::Win32X64 => Some("win32-x64"),
            Architecture::Win32Arm64 => Some("win32-arm64"),
            Architecture::DarwinX64 => Some("darwin-x64"),
            Architecture::DarwinArm64 => Some("darwin-arm64"),
            Architecture::LinuxX64 => Some("linux-x64"),
            Architecture::LinuxArm64 => Some("linux-arm64"),
            Architecture::LinuxArmhf => Some("linux-armhf"),
            Architecture::AlpineX64 => Some("alpine-x64"),
            Architecture::AlpineArm64 => Some("alpine-arm64"),
            Architecture::Universal => None,
        }
    }
}

impl fmt::Display for Architecture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_platform_string().unwrap_or("universal"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_architecture_to_platform_string() {
        assert_eq!(
            Architecture::Win32X64.to_platform_string(),
            Some("win32-x64")
        );
        assert_eq!(
            Architecture::DarwinArm64.to_platform_string(),
            Some("darwin-arm64")
        );
        assert_eq!(
            Architecture::LinuxX64.to_platform_string(),
            Some("linux-x64")
        );
        assert_eq!(Architecture::Universal.to_platform_string(), None);
    }

    #[test]
    fn test_architecture_display() {
        assert_eq!(Architecture::Win32X64.to_string(), "win32-x64");
        assert_eq!(Architecture::Universal.to_string(), "universal");
    }
}
