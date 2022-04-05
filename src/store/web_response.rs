#[derive(Debug, Clone)]
pub struct WebResponse {
    pub code: u16,
    pub body: String,
}

impl WebResponse {
    pub fn new(code: u16, body: String) -> WebResponse {
        WebResponse {
            code,
            body,
        }
    }
}
