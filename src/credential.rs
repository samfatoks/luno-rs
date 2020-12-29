pub struct Credential {
    pub api_id: String,
    pub api_secret: String,
}

impl Credential {
    pub fn new(api_id: String, api_secret: String) -> Self {
        Credential { api_id, api_secret }
    }

    pub fn get_basic_auth(&self) -> String {
        base64::encode(format!("{}:{}", self.api_id, self.api_secret))
    }
}
