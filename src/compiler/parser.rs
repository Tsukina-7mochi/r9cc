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
            node = if self.consume_symbol_plus().is_ok() {
                Node::OperatorAdd {
                    lhs: node.into(),
                    rhs: self.consume_mul()?.into(),
                }
            } else if self.consume_symbol_minus().is_ok() {
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
        let mut node = self.consume_unary()?;

        loop {
            node = if self.consume_symbol_star().is_ok() {
                Node::OperatorMul {
                    lhs: node.into(),
                    rhs: self.consume_unary()?.into(),
                }
            } else if self.consume_symbol_slash().is_ok() {
                Node::OperatorDiv {
                    lhs: node.into(),
                    rhs: self.consume_unary()?.into(),
                }
            } else {
                break Ok(node);
            }
        }
    }

    fn consume_unary(&mut self) -> Result<Node> {
        if self.consume_symbol_plus().is_ok() {
            return self.consume_primary();
        } else if self.consume_symbol_minus().is_ok() {
            let rhs = self.consume_primary()?;
            return Ok(Node::OperatorSub {
                lhs: Node::Integer { value: 0 }.into(),
                rhs: rhs.into(),
            });
        }

        self.consume_primary()
    }

    fn consume_primary(&mut self) -> Result<Node> {
        if let Ok(value) = self.consume_numeric() {
            Ok(Node::Integer { value })
        } else if self.consume_symbol_round_bracket_left().is_ok() {
            let node = self.consume_expr()?;
            self.consume_symbol_round_bracket_right()?;
            Ok(node)
        } else {
            Err(self.error_unexpected_token(vec![
                TokenKind::Integer(0),
                TokenKind::SymbolRoundBracketLeft,
            ]))
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

    fn consume_symbol_plus(&mut self) -> Result<()> {
        self.tokens
            .next_if(|token| token.kind == TokenKind::SymbolPlus)
            .map(|_| ())
            .ok_or(self.error_unexpected_token(vec![TokenKind::SymbolPlus]))
    }

    fn consume_symbol_minus(&mut self) -> Result<()> {
        self.tokens
            .next_if(|token| token.kind == TokenKind::SymbolMinus)
            .map(|_| ())
            .ok_or(self.error_unexpected_token(vec![TokenKind::SymbolMinus]))
    }

    fn consume_symbol_star(&mut self) -> Result<()> {
        self.tokens
            .next_if(|token| token.kind == TokenKind::SymbolStar)
            .map(|_| ())
            .ok_or(self.error_unexpected_token(vec![TokenKind::SymbolStar]))
    }

    fn consume_symbol_slash(&mut self) -> Result<()> {
        self.tokens
            .next_if(|token| token.kind == TokenKind::SymbolSlash)
            .map(|_| ())
            .ok_or(self.error_unexpected_token(vec![TokenKind::SymbolSlash]))
    }

    fn consume_symbol_round_bracket_left(&mut self) -> Result<()> {
        self.tokens
            .next_if(|token| token.kind == TokenKind::SymbolRoundBracketLeft)
            .map(|_| ())
            .ok_or(self.error_unexpected_token(vec![TokenKind::SymbolRoundBracketLeft]))
    }

    fn consume_symbol_round_bracket_right(&mut self) -> Result<()> {
        self.tokens
            .next_if(|token| token.kind == TokenKind::SymbolRoundBracketRight)
            .map(|_| ())
            .ok_or(self.error_unexpected_token(vec![TokenKind::SymbolRoundBracketRight]))
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

    #[test]
    fn unary_plus() {
        let mut parser = Parser::new("  1 * + 2  ");
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
    fn unary_minus() {
        let mut parser = Parser::new("  1 * - 2  ");
        assert_eq!(
            parser.consume_expr().unwrap(),
            Node::OperatorMul {
                lhs: Box::new(Node::Integer { value: 1 }),
                rhs: Box::new(Node::OperatorSub {
                    lhs: Box::new(Node::Integer { value: 0 }),
                    rhs: Box::new(Node::Integer { value: 2 }),
                }),
            }
        );
        assert!(parser.consume_eof().is_ok());
    }
}
