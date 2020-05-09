use loco::internal::{agent::Os, Client, DeviceRegisterData, LoginData};
use data_encoding::BASE64;

pub fn get_device_register_data() -> DeviceRegisterData {
    DeviceRegisterData::new(
        LoginData::new(
            "test@gmail.com".to_string(),
            "password".to_string(),
            BASE64.encode("02B9D784-75FE-47A7-AD47-C18B0AF11A1C".as_bytes()),
            "loco.rs".to_string(),
            "10.0".to_string(),
            false,
            false
        ),
        "0000".to_string()
    )
}

#[tokio::test]
async fn login() {
    let result = Client::new(Os::Win32).request_login(&get_device_register_data()).await;
    let text = result.unwrap();
    println!("{}", text.text().await.unwrap());
}

#[tokio::test]
async fn register_device() {
    let result = Client::new(Os::Win32).register_device(&get_device_register_data()).await;
    let text = result.unwrap();
    println!("{}", text.text().await.unwrap());
}

#[tokio::test]
async fn request_passcode() {
    let result = Client::new(Os::Win32).request_passcode(&get_device_register_data()).await;
    let text = result.unwrap();
    println!("{}", text.text().await.unwrap());
}

