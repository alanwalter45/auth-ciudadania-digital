use base64::Engine;
use base64::engine;
use serde_urlencoded::to_string;
use std::borrow::Cow;
use std::env;

pub fn get_credential() -> String {
    let id_client = env::var("APP_ID_CLIENT").unwrap();
    let secret = env::var("APP_SECRET").unwrap();
    let url_encoded = to_string(&[("secret", Cow::Borrowed(&secret))]).unwrap();
    let (_, encoded_secret) = url_encoded
        .split_once("=")
        .unwrap_or(("", &url_encoded[..]));
    let credential = format!(
        "Basic {}",
        engine::general_purpose::STANDARD
            .encode(format!("{}:{}", id_client, encoded_secret).as_bytes())
    );
    credential
}
