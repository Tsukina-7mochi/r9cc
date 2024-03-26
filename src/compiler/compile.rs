use super::error::{CompileError, Result};
use super::token::TokenKind;
use super::tokenizer::Tokenizer;

pub fn compile(text: &str) -> Result<String> {
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
            TokenKind::OperatorAdd => {
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
            TokenKind::OperatorSub => {
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
