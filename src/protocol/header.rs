use serde::{Deserialize, Serialize};
use std::string::FromUtf8Error;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct LocoHeader {
	pub id: u32,
	pub status: i16,
	pub name: [u8; 11],
	pub data_type: u8,
	pub data_size: u32
}

impl LocoHeader {

	/// Extract String from name field
	pub fn name(&self) -> Result<String, FromUtf8Error> {
		let size = self.name.iter().position(|&c| c == b'\0').unwrap_or(11);

		String::from_utf8(self.name[..size].into())
	}

	/// set name field from str. Will be sliced to 11 bytes max.
	pub fn set_name(&mut self, name: &str) {
		self.name = [0u8; 11];
		let bytes = name.as_bytes();
		self.name.copy_from_slice(bytes);
	}
}