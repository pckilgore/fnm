use crate::system_info::{platform_arch, platform_name};

#[derive(Clone, Debug)]
pub enum Arch {
    X86,
    X64,
    Arm64,
    Armv7l,
    Ppc64le,
    Ppc64,
    S390x,
}

/// Get a sane default architecture for the platform.
pub fn default_str() -> &'static str {
    // TODO: Handle (arch, name, version) when Node v15+ supports darwin-arm64
    match (platform_arch(), platform_name()) {
        ("darwin", "arm64") => "x64",
        (_, arch) => arch,
    }
}

pub fn get_default() -> Arch {
    match from_str(default_str()) {
        Ok(arch) => arch,
        Err(e) => panic!("{}", e.details),
    }
}

fn from_str(s: &str) -> Result<Arch, ArchError> {
    match s {
        "x86" => Ok(Arch::X86),
        "x64" => Ok(Arch::X64),
        "arm64" => Ok(Arch::Arm64),
        "armv7l" => Ok(Arch::Armv7l),
        "ppc64le" => Ok(Arch::Ppc64le),
        "ppc64" => Ok(Arch::Ppc64),
        "s390x" => Ok(Arch::S390x),
        unknown => Err(ArchError::new(&format!("Unknown Arch: {}", unknown))),
    }
}
impl std::str::FromStr for Arch {
    type Err = ArchError;
    fn from_str(s: &str) -> Result<Arch, Self::Err> {
        from_str(s)
    }
}

impl std::fmt::Display for Arch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let arch_str = match self {
            Arch::X86 => String::from("x86"),
            Arch::X64 => String::from("x64"),
            Arch::Arm64 => String::from("arm64"),
            Arch::Armv7l => String::from("armv7l"),
            Arch::Ppc64le => String::from("ppc64le"),
            Arch::Ppc64 => String::from("ppc64"),
            Arch::S390x => String::from("s390x"),
        };

        write!(f, "{}", arch_str)
    }
}

#[derive(Debug)]
pub struct ArchError {
    details: String,
}

impl ArchError {
    fn new(msg: &str) -> ArchError {
        ArchError {
            details: msg.to_string(),
        }
    }
}

impl std::fmt::Display for ArchError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl std::error::Error for ArchError {
    fn description(&self) -> &str {
        &self.details
    }
}
