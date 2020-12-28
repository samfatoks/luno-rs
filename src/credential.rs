pub struct Credential {
    pub key_id: String,
    pub secret: String
}

impl Credential {
    pub fn new(key_id: String, secret: String) -> Self {
        Credential { key_id, secret }
    }

    pub fn get_basic_auth(&self) -> String {
        base64::encode(format!("{}:{}", self.key_id, self.secret))
    }
}
