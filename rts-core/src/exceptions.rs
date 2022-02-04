use std::fmt::Display;

#[derive(Debug)]
pub enum RtsException {
    GeneralException(String)
}

impl Display for RtsException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            RtsException::GeneralException(m) => write!(f, "Rts Game: {}", m),
        }
        
    }
}
