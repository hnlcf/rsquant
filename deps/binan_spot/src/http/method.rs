#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
}

impl AsRef<str> for Method {
    fn as_ref(&self) -> &str {
        match self {
            Method::Post => "POST",
            Method::Delete => "DELETE",
            Method::Get => "GET",
            Method::Put => "PUT",
        }
    }
}

impl From<Method> for reqwest::Method {
    fn from(method: Method) -> Self {
        match method {
            Method::Get => reqwest::Method::GET,
            Method::Post => reqwest::Method::POST,
            Method::Delete => reqwest::Method::DELETE,
            Method::Put => reqwest::Method::PUT,
        }
    }
}
