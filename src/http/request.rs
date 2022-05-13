use super::method::Method;
use std::error::Error;
use std::str; 
use std::str::{Utf8Error};
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buf)?;
        unimplemented!();
    }
}

fn get_next_word(request:&str) ->Option<(&str,&str)> {
    for (i,v) in request.chars().enumerate(){
        if v == ' ' {
            return Some((&request[..i],&request[i+1..]));
        }
    }    

    return None;
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidMethod => "Invalid Method",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidRequest => "Invalid Request",
        }
    }
}

impl From<Utf8Error> for ParseError{
    fn from(_: Utf8Error) -> Self{
        Self::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}
