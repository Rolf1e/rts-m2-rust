use std::fmt::Display;

#[derive(Debug)]
pub enum WebServerException {
    SqlException(String),
    HashPassword,
}

impl Display for WebServerException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            WebServerException::SqlException(m) => write!(f, "{}", m),
            WebServerException::HashPassword => write!(f, "Failed to hash password"),
        }
    }
}
