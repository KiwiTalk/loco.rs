use loco::internal::{agent::Os, Client, DeviceRegisterData, LoginData};
use data_encoding::BASE64;

#[tokio::test]
async fn login() {
    let client = Client::new(Os::Win32);
    let device_register_data = DeviceRegisterData::new(
        LoginData::new(
            "test@gmail.com".to_string(),
            "password".to_string(),
            BASE64.encode("02B9D784-75FE-47A7-AD47-C18B0AF11A1C".as_ref()),
            "loco.rs".to_string(),
            "10.0".to_string(),
            false,
            false
        ),
        0000 /* passcode */
    );
    let result = client.register_device(&device_register_data).await;
    let text = result.ok().unwrap();
    println!("{}", text.text().await.ok().unwrap());
}