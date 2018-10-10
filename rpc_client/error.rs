extern crate jsonrpc;

use std::fmt;

#[derive(Debug)]
pub enum Error {
    JsonRpcError(jsonrpc::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::JsonRpcError(ref e) => writeln!(f, "{}", e),
        }
    }
}


impl From<jsonrpc::Error> for Error {
    fn from(e: jsonrpc::Error) -> Error {
        Error::JsonRpcError(e)
    }
}
