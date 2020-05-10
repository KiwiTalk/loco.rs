use loco::internal::{agent::Os, request, Client, DeviceRegisterData, LoginData};

pub fn get_device_register_data() -> DeviceRegisterData {
    DeviceRegisterData::new(
        LoginData::new(
            "test@gmail.com".to_string(),
            "password".to_string(),
            "2bf46274-780c-4af1-9583-c5e1d7e866b7",
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
    let loco_client = Client::new(Os::Win32);
    let result = request::Certification::login(&loco_client, &get_device_register_data()).await;
    let text = result.unwrap();

    println!("{}", text.text().await.unwrap());
}

#[tokio::test]
async fn register_device() {
    let loco_client = Client::new(Os::Win32);
    let result = request::Certification::device(&loco_client, &get_device_register_data()).await;
    let text = result.unwrap();

    println!("{}", text.text().await.unwrap());
}

#[tokio::test]
async fn passcode() {
    let loco_client = Client::new(Os::Win32);
    let result = request::Certification::passcode(&loco_client, &get_device_register_data()).await;
    let text = result.unwrap();

    println!("{}", text.text().await.unwrap());
}

