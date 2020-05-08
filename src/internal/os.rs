pub enum Os {
    Win32,
    Mac,
    UWP,
}

impl Os {
    pub fn from_str(str: &str) -> Option<Os> {
        match str {
            "win32" => Some(Os::Win32),
            "mac" => Some(Os::Mac),
            "uwp" => Some(Os::UWP),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Os::Win32 => "win32",
            Os::Mac => "mac",
            Os::UWP => "uwp",
        }
    }
}