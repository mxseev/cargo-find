use std::io;
use hyper;
use hyper_native_tls::native_tls;
use serde_json;


#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Http(hyper::Error),
    Tls(native_tls::Error),
    Json(serde_json::Error),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}
impl From<hyper::Error> for Error {
    fn from(e: hyper::Error) -> Self {
        Error::Http(e)
    }
}
impl From<native_tls::Error> for Error {
    fn from(e: native_tls::Error) -> Self {
        Error::Tls(e)
    }
}
impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Json(e)
    }
}
