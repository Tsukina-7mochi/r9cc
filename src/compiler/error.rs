use std::error;
use std::fmt;

use crate::compiler::token::TokenKind;

pub type Result<T = ()> = std::result::Result<T, CompileError>;

impl TokenKind {
    fn token_kind_display(&self) -> &str {
        match self {
            Self::Integer(_) => "integer",
            Self::Identifier(_) => "identifier",
            Self::SymbolPlus => "'+'",
            Self::SymbolMinus => "'-'",
            Self::SymbolStar => "'*'",
            Self::SymbolSlash => "'/'",
            TokenKind::SymbolRoundBracketLeft => "'('",
            Self::SymbolRoundBracketRight => "')'",
            Self::SymbolAngleBracketLeft => "'<'",
            Self::SymbolAngleBracketRight => "'>'",
            Self::SymbolAngleBracketLeftAndEqual => "'<='",
            Self::SymbolAngleBracketRightAndEqual => "'>='",
            Self::SymbolDoubleEqual => "'=='",
            Self::SymbolExclamationAndEqual => "'!='",
            Self::EOF => "EOF",
        }
    }
}

#[derive(Debug)]
pub enum CompileErrorKind {
    UnexpectedToken { expected: Vec<TokenKind> },
    UnexpectedEOF,
}

#[derive(Debug)]
pub struct CompileError {
    kind: CompileErrorKind,
    text: String,
    index_start: usize,
}

impl CompileError {
    pub fn unexpected_token(text: &str, index_start: usize, expected: Vec<TokenKind>) -> Self {
        Self {
            kind: CompileErrorKind::UnexpectedToken { expected },
            text: text.to_owned(),
            index_start,
        }
    }

    pub fn unexpected_eof(text: &str, index_start: usize) -> Self {
        Self {
            kind: CompileErrorKind::UnexpectedEOF,
            text: text.to_owned(),
            index_start,
        }
    }
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Compile error: ")?;

        match self.kind {
            CompileErrorKind::UnexpectedToken { ref expected } => {
                writeln!(f, "unexpected token at {}", self.index_start)?;
                writeln!(
                    f,
                    "expected the one of [{}]",
                    expected
                        .iter()
                        .map(|kind| kind.token_kind_display())
                        .collect::<Vec<_>>()
                        .join(", ")
                )?;
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
