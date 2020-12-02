use serde_repr::*;
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(i32)]
pub enum StatusCode {
	Success = 0,
	WrongPassword = 12,
	CannotFindId = 30,

	DeviceNotRegistered = -100,

	WrongConfirmCode = -111,
	TooManyConfirmRequest = -112,
}