use super::method::{Method, MethodError};
use super::QueryString;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;

/* ---------- Request ---------- */

#[derive(Debug)]
pub struct Request<'a> {
	path: &'a str,
	query_string: Option<QueryString<'a>>,
	method: Method
}

impl<'a> Request<'a> {
	pub fn path(&self) -> &str {
		&self.path
	}
	pub fn query_string(&self) -> Option<&QueryString> {
		self.query_string.as_ref()
	}
	pub fn method(&self) -> &Method {
		&self.method
	}
}

impl<'a> TryFrom<&'a [u8]> for Request<'a> {
	type Error = ParseError;

	fn try_from(buf: &'a [u8]) -> Result<Self, Self::Error> {
		let request = str::from_utf8(buf)?;

		let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
		let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
		let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

		if protocol != "HTTP/1.1" {
			return Err(ParseError::InvalidProtocol);
		}

		let method: Method = method.parse()?;

		let mut query_string = None;
		if let Some(index) = path.find('?') {
			query_string = Some(QueryString::from(&path[index + 1..]));
			path = &path[..index];
		}
		Ok(Self { path, query_string, method })
	}
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
	
	for (i, val) in request.chars().enumerate() {
		if val.is_whitespace() {
			return Some((&request[..i], &request[i + 1..]));
		}
	}
	None
}

/* ---------- ParseError ---------- */

pub enum ParseError {
	InvalidRequest,
	InvalidEncoding,
	InvalidProtocol,
	InvalidMethod,
}

impl ParseError {
	fn message(&self) -> &str {
		match self {
			Self::InvalidRequest => "Invalid request",
			Self::InvalidEncoding => "Invalid encoding",
			Self::InvalidProtocol => "Invalid protocol",
			Self::InvalidMethod => "Invalid method",
		}
	}
}

impl From<Utf8Error> for ParseError {
	fn from(_: Utf8Error) -> Self {
		Self::InvalidEncoding
	}
}

impl From<MethodError> for ParseError {
	fn from(_: MethodError) -> Self {
		Self::InvalidMethod
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

impl Error for ParseError {

}