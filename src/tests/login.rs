use crate::internal::{Client, LoginData};
use data_encoding::BASE64;

#[tokio::test]
async fn login() {
    let client = Client::new();
    let login_data = LoginData::new(
        "test@gmail.com".to_string(),
        "password".to_string(),
        BASE64.encode("02B9D784-75FE-47A7-AD47-C18B0AF11A1C".as_ref()),
        "loco.rs".to_string(),
        false,
        "10.0".to_string()
    );
    let result = client.request_login(&login_data).await;
    let text = result.ok().unwrap();
    println!("{}", text.text().await.ok().unwrap());
}