use super::method::Method;
use std::{
    convert::TryFrom,
    error::Error,
    str,
    str::Utf8Error,
    fmt::{
        Display,
        Formatter,
        Result as FMTResult,
        Debug
    }
};

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl Request {
    fn from_byte_array(buf: &[u8]) -> Result<Self, String> {
        unimplemented!();
    }
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> { 
        let request = str::from_utf8(buf)?;
        unimplemented!()
    }   
}

fn get_net_word(request: &str) -> Option<(&str, &str)> {
    let mut iter = request.chars();
    for (i, c) in request.chars().enumerate() {
        if c == ' ' {
            return Some((&request[..i], &request[i+1..]));
        }
    }

    None
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FMTResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FMTResult {
        write!(f, "{}", self.message())
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "InvalidRequest",
            Self::InvalidEncoding => "InvalidEncoding",
            Self::InvalidProtocol => "InvalidProtocol",
            Self::InvalidMethod => "InvalidMethod",
        }
    }
}
impl Error for ParseError {}