pub struct Credential {
    pub key_id: String,
    pub key_secret: String,
}

impl Credential {
    pub fn new(key_id: String, key_secret: String) -> Self {
        Credential { key_id, key_secret }
    }

    pub fn get_basic_auth(&self) -> String {
        base64::encode(format!("{}:{}", self.key_id, self.key_secret))
    }
}
