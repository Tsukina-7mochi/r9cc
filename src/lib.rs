mod compiler;

use std::error;
use std::fmt;

use compiler::token::TokenKind;
use compiler::tokenizer::Tokenizer;

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
    fn unexpected_token(text: String, index_start: usize) -> Self {
        Self {
            kind: CompileErrorKind::UnexpectedToken,
            text,
            index_start,
        }
    }

    fn unexpected_eof(text: String, index_start: usize) -> Self {
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

pub fn compile(text: &str) -> Result<String, CompileError> {
    let text = text.trim();
    let mut token_iter = Tokenizer::new(text).into_iter();

    let mut result = String::new();

    result.push_str(".intel_syntax noprefix\n");
    result.push_str(".global main\n");
    result.push_str("main:\n");

    let next_number = token_iter
        .next()
        .ok_or(CompileError::unexpected_eof(text.to_owned(), 0))
        .and_then(|token| {
            token.integer_value().ok_or(CompileError::unexpected_token(
                text.to_owned(),
                token.index_start,
            ))
        })?;
    result.push_str(&format!("   mov rax, {}\n", next_number));

    loop {
        let token = token_iter.next();
        let token = match token {
            Some(token) => token,
            _ => {
                eprintln!("{}", text);
                eprintln!("{}^", " ".repeat(text.len()));
                break;
            }
        };

        match token.kind {
            TokenKind::EOF => break,
            TokenKind::OperatorPlus => {
                let next_number = token_iter
                    .next()
                    .ok_or(CompileError::unexpected_eof(
                        text.to_owned(),
                        token.index_start,
                    ))
                    .and_then(|token| {
                        token.integer_value().ok_or(CompileError::unexpected_token(
                            text.to_owned(),
                            token.index_start,
                        ))
                    })?;
                result.push_str(&format!("   add rax, {}\n", next_number))
            }
            TokenKind::OperatorMinus => {
                let next_number = token_iter
                    .next()
                    .ok_or(CompileError::unexpected_eof(
                        text.to_owned(),
                        token.index_start,
                    ))
                    .and_then(|token| {
                        token.integer_value().ok_or(CompileError::unexpected_token(
                            text.to_owned(),
                            token.index_start,
                        ))
                    })?;
                result.push_str(&format!("   sub rax, {}\n", next_number))
            }
            _ => panic!("Unexpected token {:?}", token.kind),
        }
    }

    result.push_str("ret\n");

    Ok(result)
}
