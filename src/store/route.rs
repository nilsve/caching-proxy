use crate::Method;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Route {
    pub method: Method,
    pub url: String,
    pub params: Vec<(String, String)>, // Vec instead of more convenient HashMap, because HashMap doesn't implement Hash
}

impl Route {
    pub fn new(method: Method, url: String) -> Route {
        let (parsed_url, params) = Route::decode_params(&url);
        Route {
            method,
            url: parsed_url.to_owned(),
            params,
        }
    }

    fn decode_params(url: &str) -> (&str, Vec<(String, String)>) {
        (
            url,
            Vec::new() // Todo
        )
    }
}