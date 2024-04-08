use std::error;
use std::fmt;

use crate::compiler::token::TokenKind;

pub type Result<T = ()> = std::result::Result<T, CompileError>;

impl TokenKind {
    fn token_kind_display(&self) -> &str {
        match self {
            Self::Integer(_) => "integer",
            Self::Identifier(_) => "identifier",
            Self::KeywordReturn => "return",
            Self::KeywordIf => "if",
            Self::KeywordElse => "else",
            Self::KeywordWhile => "while",
            Self::KeywordFor => "for",
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
            Self::SymbolCurlyBracketLeft => "'{'",
            Self::SymbolCurlyBracketRight => "'}'",
            Self::SymbolDoubleEqual => "'=='",
            Self::SymbolExclamationAndEqual => "'!='",
            Self::SymbolEqual => "'='",
            Self::SymbolSemicolon => "';'",
            Self::SymbolComma => "','",
            Self::EOF => "EOF",
        }
    }
}

#[derive(Debug)]
pub enum CompileErrorKind {
    UnexpectedToken { expected: Vec<TokenKind> },
    UnexpectedEOF,
    NotALeftValue,
}

#[derive(Debug)]
pub struct CompileError {
    kind: CompileErrorKind,
    index_start: usize,
}

impl CompileError {
    pub fn unexpected_token(index_start: usize, expected: Vec<TokenKind>) -> Self {
        Self {
            kind: CompileErrorKind::UnexpectedToken { expected },
            index_start,
        }
    }

    pub fn unexpected_eof(index_start: usize) -> Self {
        Self {
            kind: CompileErrorKind::UnexpectedEOF,
            index_start,
        }
    }

    pub fn not_a_left_value(index_start: usize) -> Self {
        Self {
            kind: CompileErrorKind::NotALeftValue,
            index_start,
        }
    }

    pub fn into_formatter<'a>(self, text: &'a str) -> CompileErrorFormatter<'a> {
        CompileErrorFormatter::new(self, text)
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
            CompileErrorKind::NotALeftValue => {
                writeln!(f, "left value expected at {}", self.index_start)?;
            }
        }

        Ok(())
    }
}

impl error::Error for CompileError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

pub struct CompileErrorFormatter<'a> {
    error: CompileError,
    text: &'a str,
}

impl<'a> CompileErrorFormatter<'a> {
    pub fn new(error: CompileError, text: &'a str) -> Self {
        Self { error, text }
    }
}

impl<'a> fmt::Display for CompileErrorFormatter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.error)?;

        write!(f, "{}", self.text)?;
        writeln!(f, "{}^", " ".repeat(self.error.index_start))?;

        Ok(())
    }
}
