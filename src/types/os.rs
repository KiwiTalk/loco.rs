use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Os {
    Win32,
    Mac,
    IPad,
    UWP,
}

impl Os {
    pub fn as_str(&self) -> &'static str {
        match self {
            Os::Win32 => "win32",
            Os::Mac => "mac",
            Os::IPad => "ipad",
            Os::UWP => "uwp",
        }
    }
}
