//! Platform detection utilities
//!
//! Provides cross-platform detection for OS and CPU architecture.

use std::fmt;

/// Operating System type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Os {
    Windows,
    MacOS,
    Linux,
    Unknown,
}

impl fmt::Display for Os {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Os::Windows => write!(f, "Windows"),
            Os::MacOS => write!(f, "macOS"),
            Os::Linux => write!(f, "Linux"),
            Os::Unknown => write!(f, "Unknown"),
        }
    }
}

/// CPU Architecture type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Arch {
    X86_64,
    Aarch64,
    X86,
    Unknown,
}

impl fmt::Display for Arch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Arch::X86_64 => write!(f, "x86_64"),
            Arch::Aarch64 => write!(f, "aarch64 (ARM64)"),
            Arch::X86 => write!(f, "x86"),
            Arch::Unknown => write!(f, "unknown"),
        }
    }
}

/// Platform information (OS + Architecture)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Platform {
    pub os: Os,
    pub arch: Arch,
}

impl Platform {
    /// Detect the current platform at compile time
    pub fn detect() -> Self {
        let os = if cfg!(windows) {
            Os::Windows
        } else if cfg!(target_os = "macos") {
            Os::MacOS
        } else if cfg!(target_os = "linux") {
            Os::Linux
        } else {
            Os::Unknown
        };

        let arch = if cfg!(target_arch = "x86_64") {
            Arch::X86_64
        } else if cfg!(target_arch = "aarch64") {
            Arch::Aarch64
        } else if cfg!(target_arch = "x86") {
            Arch::X86
        } else {
            Arch::Unknown
        };

        Self { os, arch }
    }

    /// Check if running on Windows
    #[allow(dead_code)]
    pub fn is_windows(&self) -> bool {
        self.os == Os::Windows
    }

    /// Check if running on macOS
    #[allow(dead_code)]
    pub fn is_macos(&self) -> bool {
        self.os == Os::MacOS
    }

    /// Check if running on Linux
    #[allow(dead_code)]
    pub fn is_linux(&self) -> bool {
        self.os == Os::Linux
    }

    /// Check if running on ARM64 architecture
    #[allow(dead_code)]
    pub fn is_arm64(&self) -> bool {
        self.arch == Arch::Aarch64
    }
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.os, self.arch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_detect() {
        let platform = Platform::detect();
        // Should not panic and should have valid values
        assert!(!format!("{}", platform).is_empty());
    }

    #[test]
    fn test_os_display() {
        assert_eq!(format!("{}", Os::Windows), "Windows");
        assert_eq!(format!("{}", Os::MacOS), "macOS");
        assert_eq!(format!("{}", Os::Linux), "Linux");
    }

    #[test]
    fn test_arch_display() {
        assert_eq!(format!("{}", Arch::X86_64), "x86_64");
        assert_eq!(format!("{}", Arch::Aarch64), "aarch64 (ARM64)");
    }
}
