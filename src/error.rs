use std::fmt::{Display, Formatter, Result};
use std::error::Error;

#[derive(Debug)]
pub enum CustomErrorKind {
    FailedDownload,
    UnsupportedOS,
    WgetUnavaible
}

#[derive(Debug)]
pub struct CustomError {
    kind: CustomErrorKind,
    msg: String
}

impl CustomError {
    pub fn new(kind: CustomErrorKind, msg: &str) -> Self {
        CustomError {
            kind,
            msg: msg.to_string()
        }
    }
}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}: {}", self.kind, self.msg)
    }
}

impl Error for CustomError {}
