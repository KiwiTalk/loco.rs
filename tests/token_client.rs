use loco::internal::{agent::Os, DeviceRegisterData, LoginData, TokenClient};

pub fn get_device_register_data() -> DeviceRegisterData {
    DeviceRegisterData::new(
        LoginData::new(
            "test@gmail.com".to_string(),
            "password".to_string(),
            "2bf46274-780c-4af1-9583-c5e1d7e866b7",
            "loco.rs".to_string(),
            "10.0".to_string(),
            false,
            false,
        ),
        "0000".to_string(),
    )
}

#[tokio::test]
async fn login() {
    let result = TokenClient::new(Os::Win32).request_login(&get_device_register_data());
    let data = result.unwrap();
    println!("{}", data.status);
}

#[tokio::test]
async fn register_device() {
    let result = TokenClient::new(Os::Win32).register_device(&get_device_register_data());
    let text = result.unwrap();
    println!("{}", text.text().unwrap());
}

#[tokio::test]
async fn request_passcode() {
    let result = TokenClient::new(Os::Win32).request_passcode(&get_device_register_data());
    let text = result.unwrap();
    println!("{}", text.text().unwrap());
}
