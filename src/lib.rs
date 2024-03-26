mod compiler;

use compiler::{token::TokenKind, tokenizer::Tokenizer};

pub fn compile(text: &str) -> String {
    let text = text.trim();
    let mut token_iter = Tokenizer::new(text).into_iter();

    let mut result = String::new();

    result.push_str(".intel_syntax noprefix\n");
    result.push_str(".global main\n");
    result.push_str("main:\n");

    result.push_str(&format!(
        "   mov rax, {}\n",
        token_iter
            .next()
            .unwrap()
            .as_integer()
            .expect("Expected number")
    ));

    loop {
        let token = token_iter.next();
        let token = match token {
            Some(token) => token,
            _ => break,
        };

        match token.kind {
            TokenKind::EOF => break,
            TokenKind::OperatorPlus => result.push_str(&format!(
                "   add rax, {}\n",
                token_iter.next().unwrap().as_integer().unwrap()
            )),
            TokenKind::OperatorMinus => result.push_str(&format!(
                "   sub rax, {}\n",
                token_iter.next().unwrap().as_integer().unwrap()
            )),
            _ => panic!("Unexpected token {:?}", token.kind),
        }
    }

    result.push_str("ret\n");

    result
}
