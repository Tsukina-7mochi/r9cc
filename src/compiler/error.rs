use std::error;
use std::fmt;

pub type Result<T = ()> = std::result::Result<T, CompileError>;

#[derive(Debug)]
pub enum CompileErrorKind {
    UnexpectedToken,
    UnexpectedEOF,
}

#[derive(Debug)]
pub struct CompileError {
    kind: CompileErrorKind,
    text: String,
    index_start: usize,
}

impl CompileError {
    pub fn unexpected_token(text: String, index_start: usize) -> Self {
        Self {
            kind: CompileErrorKind::UnexpectedToken,
            text,
            index_start,
        }
    }

    pub fn unexpected_eof(text: String, index_start: usize) -> Self {
        Self {
            kind: CompileErrorKind::UnexpectedEOF,
            text,
            index_start,
        }
    }
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Compile error: ")?;

        match self.kind {
            CompileErrorKind::UnexpectedToken => {
                writeln!(f, "unexpected token at {}", self.index_start)?;
            }
            CompileErrorKind::UnexpectedEOF => {
                writeln!(f, "unexpected EOF at {}", self.index_start)?;
            }
        }

        writeln!(f, "{}", self.text)?;
        writeln!(f, "{}^", " ".repeat(self.index_start))?;

        Ok(())
    }
}

impl error::Error for CompileError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
