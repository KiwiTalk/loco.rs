use crate::internal::{Client, LoginData};

#[tokio::test]
async fn login() {
    let client = Client::new();
    let login_data = LoginData::new(
        "djdisodo@gmail.com".to_string(),
        "password".to_string(),
        "YWFhc2FmaGthc2hmamtoZGtmamFz".to_string(),
        "djdisodo".to_string(),
        false,
        "10.0".to_string()
    );
    let result = client.request_login(&login_data).await;
    let text = result.ok().unwrap();
    println!("{}", text.text().await.ok().unwrap());
}