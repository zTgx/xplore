use reqwest::header::{HeaderMap, HeaderValue, COOKIE};

pub(crate) struct CookieTracker {
    pub(crate) cookie: String,
}

impl CookieTracker {
    pub fn new(cookie: &str) -> Self {
        CookieTracker { cookie: cookie.to_string() }
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
}
