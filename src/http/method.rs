use std::str::FromStr;

pub enum Method {
    GET,
    PUT,
    POST,
    DELETE,
    HEAD,
    CONNECT,
    OPTION,
    PATCH,
    TRACE,
}

impl FromStr for Method {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::GET),
            "PUT" => Ok(Self::PUT),
            "POST" => Ok(Self::POST),
            "DELETE" => Ok(Self::DELETE),
            "HEAD" => Ok(Self::HEAD),
            "CONNECT" => Ok(Self::CONNECT),
            "OPTION" => Ok(Self::OPTION),
            "PATCH" => Ok(Self::PATCH),
            "TRACE" => Ok(Self::TRACE),
            _ => Err(MethodError),
        }
    }
}
pub struct MethodError;
