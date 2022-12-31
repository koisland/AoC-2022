
#[derive(Debug)]
pub struct ParserError {
    pub reason: String,
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unable to parse prompt. {}", self.reason)
    }
}
impl std::error::Error for ParserError {}