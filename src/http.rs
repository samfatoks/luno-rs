use crate::credential::Credential;
use crate::error::{Error, LunoError};
use crate::middleware::Logger;
use async_std::future::timeout;
use serde::de::DeserializeOwned;
use std::time::Duration;
use surf::http::mime;
use surf::{StatusCode, Url};
pub struct Http {
    basic_auth: String,
    timeout: Duration,
    client: surf::Client,
}

impl Http {
    pub fn new(key_id: String, secret: String) -> Result<Self, Error> {
        let credential = Credential::new(key_id, secret);
        let timeout = Duration::from_millis(60000);
        Http::new_with_features(credential, timeout, false)
    }

    pub fn new_with_features(
        credential: Credential,
        timeout: Duration,
        enable_debug_middleware: bool,
    ) -> Result<Self, Error> {
        let mut client = surf::client();
        if enable_debug_middleware {
            client = client.with(Logger);
        }

        client.set_base_url(Url::parse("https://api.luno.com")?);
        let http = Http {
            basic_auth: credential.get_basic_auth(),
            client,
            timeout,
        };
        Ok(http)
    }

    pub async fn process_request<T: DeserializeOwned, S: AsRef<str>>(
        &self,
        path: S,
    ) -> Result<T, Error> {
        let request = self
            .client
            .get(path)
            .header("Authorization", format!("Basic {}", self.basic_auth))
            .content_type(mime::JSON)
            .build();

        let mut response = timeout(self.timeout, self.client.send(request)).await??;
        if response.status() == StatusCode::Ok {
            Ok(response.take_body().into_json().await?)
        } else {
            let luno_error: LunoError = response.take_body().into_json().await?;
            Err(Error::ApiError(luno_error))
        }
    }
}
