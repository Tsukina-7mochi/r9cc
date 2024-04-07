use regex::bytes::Regex;

use crate::compiler::token::{Token, TokenKind};

mod re {
    use once_cell::sync::Lazy;
    use regex::bytes::Regex;

    pub const INTEGER: Lazy<Regex> = Lazy::new(|| Regex::new(r"-?(0|[1-9]\d*)").unwrap());
    pub const IDENTIFIER: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"[_a-zA-Z][_a-zA-Z0-9]*").unwrap());
}

#[derive(Debug)]
pub struct Tokenizer<'a> {
    index: usize,
    text: &'a [u8],
}

impl<'a> Tokenizer<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            index: 0,
            text: text.as_bytes(),
        }
    }

    fn skip_whitespaces(&mut self) -> () {
        while self
            .text
            .get(self.index)
            .is_some_and(|x| x.is_ascii_whitespace())
        {
            self.index += 1;
        }
    }

    fn consume_regex(&mut self, regex: &Regex) -> Option<String> {
        let m = regex
            .captures_at(self.text, self.index)?
            .get(0)
            .filter(|x| x.start() == self.index)?;
        self.index += m.len();
        Some(String::from_utf8(m.as_bytes().to_vec()).unwrap())
    }

    pub fn consume_char(&mut self) -> Option<Token> {
        let token = match self.text.get(self.index) {
            None => Some(Token::new(TokenKind::EOF, self.index)),
            Some(v) => match v {
                b'+' => Some(Token::new(TokenKind::SymbolPlus, self.index)),
                b'-' => Some(Token::new(TokenKind::SymbolMinus, self.index)),
                b'*' => Some(Token::new(TokenKind::SymbolStar, self.index)),
                b'/' => Some(Token::new(TokenKind::SymbolSlash, self.index)),
                b'(' => Some(Token::new(TokenKind::SymbolRoundBracketLeft, self.index)),
                b')' => Some(Token::new(TokenKind::SymbolRoundBracketRight, self.index)),
                b'<' => Some(Token::new(TokenKind::SymbolAngleBracketLeft, self.index)),
                b'>' => Some(Token::new(TokenKind::SymbolAngleBracketRight, self.index)),
                b'=' => Some(Token::new(TokenKind::SymbolEqual, self.index)),
                b';' => Some(Token::new(TokenKind::SymbolSemicolon, self.index)),
                _ => None,
            },
        };

        if token.is_some() {
            self.index += 1;
        }

        token
    }

    pub fn consume_2_chars(&mut self) -> Option<Token> {
        let chars = (self.text.get(self.index)?, self.text.get(self.index + 1)?);
        let token = match chars {
            (b'<', b'=') => Some(Token::new(
                TokenKind::SymbolAngleBracketLeftAndEqual,
                self.index,
            )),
            (b'>', b'=') => Some(Token::new(
                TokenKind::SymbolAngleBracketRightAndEqual,
                self.index,
            )),
            (b'=', b'=') => Some(Token::new(TokenKind::SymbolDoubleEqual, self.index)),
            (b'!', b'=') => Some(Token::new(TokenKind::SymbolExclamationAndEqual, self.index)),
            _ => None,
        };

        if token.is_some() {
            self.index += 2;
        }

        token
    }

    pub fn consume_integer(&mut self) -> Option<Token> {
        let index = self.index;
        let value: i32 = self.consume_regex(&*re::INTEGER)?.parse().unwrap();
        Some(Token::new(TokenKind::Integer(value), index))
    }

    pub fn consume_identifier(&mut self) -> Option<Token> {
        let index = self.index;
        let value = self.consume_regex(&*re::IDENTIFIER)?;
        Some(Token::new(TokenKind::Identifier(value), index))
    }

    pub fn consume(&mut self) -> Option<Token> {
        self.skip_whitespaces();

        None.or_else(|| self.consume_2_chars())
            .or_else(|| self.consume_char())
            .or_else(|| self.consume_integer())
            .or_else(|| self.consume_identifier())
    }
}

impl<'a> IntoIterator for Tokenizer<'a> {
    type Item = Token;
    type IntoIter = TokenizerIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        TokenizerIterator { tokenizer: self }
    }
}

pub struct TokenizerIterator<'a> {
    tokenizer: Tokenizer<'a>,
}

impl<'a> Iterator for TokenizerIterator<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokenizer.consume()
    }
}
