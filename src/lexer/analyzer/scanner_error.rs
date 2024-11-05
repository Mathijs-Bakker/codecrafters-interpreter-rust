use std::fmt;

#[derive(Debug)]
pub struct SingleTokenError {
    pub token: char,

    pub source_code: String,
    pub source_code_idx: usize,
}

impl SingleTokenError {
    pub fn line(&self) -> usize {
        let line_start = 1;

        let code_span_to_error = &self.source_code[..self.source_code_idx + line_start];
        code_span_to_error.lines().count()
    }
}

impl fmt::Display for SingleTokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[line {}] Error: Unexpected character: {}",
            self.line(),
            self.token,
        )
    }
}

impl std::error::Error for SingleTokenError {}

#[derive(Debug)]
pub struct UnterminatedStringError {
    pub source_code: String,
    pub source_code_idx: usize,
}

impl UnterminatedStringError {
    pub fn line(&self) -> usize {
        let line_start = 1;

        let code_span_to_error = &self.source_code[..self.source_code_idx + line_start];
        code_span_to_error.lines().count()
    }
}

impl fmt::Display for UnterminatedStringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[line {}] Error: Unterminated string.", self.line(),)
    }
}

impl std::error::Error for UnterminatedStringError {}
