#[derive(Debug)]
pub enum Error {
    BufferTooShort,
    InvalidHeader,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::BufferTooShort =>
                write!(f, "Buffer too short"),
            Error::InvalidHeader =>
                write!(f, "Invalid header"),
        }
    }
}