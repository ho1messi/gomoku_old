#[derive(PartialEq, Debug)]
pub enum ErrorKind {
    CoordInvalid,
}

#[derive(PartialEq, Debug)]
#[warn(dead_code)]
pub struct Error {
    kind: ErrorKind,
    message: String,
}

impl Error {
    pub fn create_with_detail(kind: ErrorKind, message: &str) -> Error {
        return Error {
            kind,
            message: String::from(message),
        };
    }
}