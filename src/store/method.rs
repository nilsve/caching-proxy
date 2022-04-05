#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Unknown,
}

impl Method {
    pub fn from_awc(method: &awc::http::Method) -> Option<Method> {
        match method.as_str() {
            "GET" => Some(Method::Get),
            "POST" => Some(Method::Post),
            "PUT" => Some(Method::Put),
            "DELETE" => Some(Method::Delete),
            _ => None,
        }
    }
}