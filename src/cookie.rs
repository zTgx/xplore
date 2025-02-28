use crate::error::Result;
use reqwest::header::{HeaderMap, HeaderValue, COOKIE};
use std::fs;

pub(crate) struct CookieTracker {
    pub(crate) cookie: String,
    file_path: String,
}

impl CookieTracker {
    pub fn new(cookie: &str) -> Self {
        CookieTracker { cookie: cookie.to_string(), file_path: ".".to_string() }
    }

    pub fn get_cookie(&self) -> &str {
        &self.cookie
    }

    pub fn update_cookie(&mut self, new_cookie: &str) {
        self.cookie = new_cookie.to_string();
    }

    pub fn is_valid(&self) -> bool {
        !self.cookie.is_empty()
    }

    pub fn clear_cookie(&mut self) {
        self.cookie = String::new();
    }

    pub fn add_cookie_to_headers(&self, headers: &mut HeaderMap) {
        if self.is_valid() {
            headers.insert(COOKIE, HeaderValue::from_str(&self.cookie).unwrap());
        }
    }

    pub async fn sync(&self) -> Result<()> {
        fs::write(&self.file_path, &self.cookie)?;
        Ok(())
    }

    pub async fn load_from_file(&mut self) -> Result<()> {
        let cookie = fs::read_to_string(&self.file_path)?;
        self.cookie = cookie;
        Ok(())
    }
}
