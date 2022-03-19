use std::fmt::Display;

#[derive(Debug)]
pub enum WebServerException {
    Sql(String),
    HashPassword,
    User(String),
}

impl Display for WebServerException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            WebServerException::Sql(m) | WebServerException::User(m) => write!(f, "{}", m),
            WebServerException::HashPassword => write!(f, "Failed to hash password"),
        }
    }
}
