use base64::Engine;
use base64::engine;
use serde_urlencoded::to_string;
use std::borrow::Cow;

pub fn get_credential(client_id: String, secret: String) -> String {
    let url_encoded = to_string(&[("secret", Cow::Borrowed(&secret))]).unwrap();
    let (_, encoded_secret) = url_encoded
        .split_once("=")
        .unwrap_or(("", &url_encoded[..]));
    let credential = format!(
        "Basic {}",
        engine::general_purpose::STANDARD
            .encode(format!("{}:{}", client_id, encoded_secret).as_bytes())
    );
    credential
}
