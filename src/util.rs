use std::str;

pub fn encryptpass(password: &str) -> String {
    base64::encode(password.as_bytes())
}