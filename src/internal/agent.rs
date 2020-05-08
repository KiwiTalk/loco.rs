use std::str::FromStr;

pub enum Os {
    Win32,
    Mac,
    UWP,
}

impl FromStr for Os {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "win32" => Ok(Os::Win32),
            "mac" => Ok(Os::Mac),
            "uwp" => Ok(Os::UWP),
            _ => Err(()),
        }
    }
}

impl Os {
    pub fn as_str(&self) -> &'static str {
        match self {
            Os::Win32 => "win32",
            Os::Mac => "mac",
            Os::UWP => "uwp",
        }
    }
}
