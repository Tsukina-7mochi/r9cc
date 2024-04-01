use std::iter::Peekable;

use crate::compiler::ast::Node;
use crate::compiler::error::{CompileError, Result};
use crate::compiler::token::TokenKind;
use crate::compiler::tokenizer::{Tokenizer, TokenizerIterator};

pub struct Parser<'a> {
    text: &'a str,
    tokens: Peekable<TokenizerIterator<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(text: &'a str) -> Self {
        let tokenizer = Tokenizer::new(text);
        Self {
            text,
            tokens: tokenizer.into_iter().peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<Node> {
        let node = self.consume_expr()?;
        self.consume_eof()?;

        Ok(node)
    }

    fn current_index_in_text(&mut self) -> Option<usize> {
        self.tokens.peek().map(|token| token.index_start)
    }

    fn error_unexpected_token(&mut self, expected: Vec<TokenKind>) -> CompileError {
        CompileError::unexpected_token(self.text, self.current_index_in_text().unwrap(), expected)
    }

    fn consume_expr(&mut self) -> Result<Node> {
        let mut node = self.consume_mul()?;

        loop {
            node = if self.consume_operator_add().is_ok() {
                Node::OperatorAdd {
                    lhs: node.into(),
                    rhs: self.consume_mul()?.into(),
                }
            } else if self.consume_operator_sub().is_ok() {
                Node::OperatorSub {
                    lhs: node.into(),
                    rhs: self.consume_mul()?.into(),
                }
            } else {
                break Ok(node);
            }
        }
    }

    fn consume_mul(&mut self) -> Result<Node> {
        let mut node = self.consume_primary()?;

        loop {
            node = if self.consume_operator_mul().is_ok() {
                Node::OperatorMul {
                    lhs: node.into(),
                    rhs: self.consume_primary()?.into(),
                }
            } else if self.consume_operator_div().is_ok() {
                Node::OperatorDiv {
                    lhs: node.into(),
                    rhs: self.consume_primary()?.into(),
                }
            } else {
                break Ok(node);
            }
        }
    }

    fn consume_primary(&mut self) -> Result<Node> {
        if let Ok(value) = self.consume_numeric() {
            Ok(Node::Integer { value })
        } else if self.consume_round_bracket_left().is_ok() {
            let node = self.consume_expr()?;
            self.consume_round_bracket_right()?;
            Ok(node)
        } else {
            Err(self
                .error_unexpected_token(vec![TokenKind::Integer(0), TokenKind::RoundBracketLeft]))
        }
    }

    fn consume_numeric(&mut self) -> Result<i32> {
        let token = match self.tokens.peek() {
            Some(t) => t,
            None => return Err(self.error_unexpected_token(vec![TokenKind::Integer(0)])),
        };

        if let TokenKind::Integer(v) = token.kind {
            self.tokens.next();
            Ok(v)
        } else {
            Err(self.error_unexpected_token(vec![TokenKind::Integer(0)]))
        }
    }

    fn consume_operator_add(&mut self) -> Result<()> {
        self.tokens
            .next_if(|token| token.kind == TokenKind::OperatorAdd)
            .map(|_| ())
            .ok_or(self.error_unexpected_token(vec![TokenKind::OperatorAdd]))
    }

    fn consume_operator_sub(&mut self) -> Result<()> {
        self.tokens
            .next_if(|token| token.kind == TokenKind::OperatorSub)
            .map(|_| ())
            .ok_or(self.error_unexpected_token(vec![TokenKind::OperatorSub]))
    }

    fn consume_operator_mul(&mut self) -> Result<()> {
        self.tokens
            .next_if(|token| token.kind == TokenKind::OperatorMul)
            .map(|_| ())
            .ok_or(self.error_unexpected_token(vec![TokenKind::OperatorMul]))
    }

    fn consume_operator_div(&mut self) -> Result<()> {
        self.tokens
            .next_if(|token| token.kind == TokenKind::OperatorDiv)
            .map(|_| ())
            .ok_or(self.error_unexpected_token(vec![TokenKind::OperatorDiv]))
    }

    fn consume_round_bracket_left(&mut self) -> Result<()> {
        self.tokens
            .next_if(|token| token.kind == TokenKind::RoundBracketLeft)
            .map(|_| ())
            .ok_or(self.error_unexpected_token(vec![TokenKind::RoundBracketLeft]))
    }

    fn consume_round_bracket_right(&mut self) -> Result<()> {
        self.tokens
            .next_if(|token| token.kind == TokenKind::RoundBracketRight)
            .map(|_| ())
            .ok_or(self.error_unexpected_token(vec![TokenKind::RoundBracketRight]))
    }

    fn consume_eof(&mut self) -> Result<()> {
        self.tokens
            .next_if(|token| token.kind == TokenKind::EOF)
            .map(|_| ())
            .ok_or(self.error_unexpected_token(vec![TokenKind::EOF]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number() {
        let mut parser = Parser::new("  1234567890  ");
        assert_eq!(
            parser.consume_expr().unwrap(),
            Node::Integer { value: 1234567890 }
        );
        assert!(parser.consume_eof().is_ok());
    }

    #[test]
    fn add() {
        let mut parser = Parser::new("  1 + 2  ");
        assert_eq!(
            parser.consume_expr().unwrap(),
            Node::OperatorAdd {
                lhs: Box::new(Node::Integer { value: 1 }),
                rhs: Box::new(Node::Integer { value: 2 }),
            }
        );
        assert!(parser.consume_eof().is_ok());
    }

    #[test]
    fn sub() {
        let mut parser = Parser::new("  1 - 2  ");
        assert_eq!(
            parser.consume_expr().unwrap(),
            Node::OperatorSub {
                lhs: Box::new(Node::Integer { value: 1 }),
                rhs: Box::new(Node::Integer { value: 2 }),
            }
        );
        assert!(parser.consume_eof().is_ok());
    }

    #[test]
    fn mul() {
        let mut parser = Parser::new("  1 * 2  ");
        assert_eq!(
            parser.consume_expr().unwrap(),
            Node::OperatorMul {
                lhs: Box::new(Node::Integer { value: 1 }),
                rhs: Box::new(Node::Integer { value: 2 }),
            }
        );
        assert!(parser.consume_eof().is_ok());
    }

    #[test]
    fn div() {
        let mut parser = Parser::new("  1 / 2  ");
        assert_eq!(
            parser.consume_expr().unwrap(),
            Node::OperatorDiv {
                lhs: Box::new(Node::Integer { value: 1 }),
                rhs: Box::new(Node::Integer { value: 2 }),
            }
        );
        assert!(parser.consume_eof().is_ok());
    }

    #[test]
    fn mul_and_add() {
        let mut parser = Parser::new("  1 * 2 + 3 / 4  ");
        assert_eq!(
            parser.consume_expr().unwrap(),
            Node::OperatorAdd {
                lhs: Box::new(Node::OperatorMul {
                    lhs: Box::new(Node::Integer { value: 1 }),
                    rhs: Box::new(Node::Integer { value: 2 }),
                }),
                rhs: Box::new(Node::OperatorDiv {
                    lhs: Box::new(Node::Integer { value: 3 }),
                    rhs: Box::new(Node::Integer { value: 4 }),
                }),
            }
        );
        assert!(parser.consume_eof().is_ok());
    }

    #[test]
    fn nested_expression() {
        let mut parser = Parser::new("  (1 + 2 * 3) / (4 - 5) + 6  ");
        assert_eq!(
            parser.consume_expr().unwrap(),
            Node::OperatorAdd {
                lhs: Box::new(Node::OperatorDiv {
                    lhs: Box::new(Node::OperatorAdd {
                        lhs: Box::new(Node::Integer { value: 1 }),
                        rhs: Box::new(Node::OperatorMul {
                            lhs: Box::new(Node::Integer { value: 2 }),
                            rhs: Box::new(Node::Integer { value: 3 }),
                        }),
                    }),
                    rhs: Box::new(Node::OperatorSub {
                        lhs: Box::new(Node::Integer { value: 4 }),
                        rhs: Box::new(Node::Integer { value: 5 }),
                    })
                }),
                rhs: Box::new(Node::Integer { value: 6 })
            }
        );
        assert!(parser.consume_eof().is_ok());
    }
}
