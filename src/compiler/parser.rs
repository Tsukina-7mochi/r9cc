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
        let mut statements = Vec::<Node>::new();
        while self.next_eof().is_none() {
            statements.push(self.consume_statement()?);
        }

        Ok(Node::Block {
            statements: statements,
        })
    }

    fn current_index_in_text(&mut self) -> Option<usize> {
        self.tokens.peek().map(|token| token.index_start)
    }

    fn error_unexpected_token(&mut self, expected: Vec<TokenKind>) -> CompileError {
        CompileError::unexpected_token(
            self.current_index_in_text()
                .unwrap_or_else(|| self.text.len()),
            expected,
        )
    }

    fn consume_statement(&mut self) -> Result<Node> {
        let expr = self.consume_expr()?;
        if self.next_symbol_semicolon().is_none() {
            return Err(self.error_unexpected_token(vec![TokenKind::SymbolSemicolon]));
        }

        Ok(expr)
    }

    fn consume_expr(&mut self) -> Result<Node> {
        self.consume_assign()
    }

    fn consume_assign(&mut self) -> Result<Node> {
        let index = self
            .current_index_in_text()
            .unwrap_or_else(|| self.text.len());
        let equality = self.consume_equality()?;

        if self.next_symbol_equal().is_some() {
            if !equality.is_left_value() {
                return Err(CompileError::not_a_left_value(index));
            }

            let assign = self.consume_assign()?;
            Ok(Node::OperatorAssign {
                lhs: equality.into(),
                rhs: assign.into(),
            })
        } else {
            Ok(equality)
        }
    }

    fn consume_equality(&mut self) -> Result<Node> {
        let mut node = self.consume_relational()?;

        loop {
            node = if self.next_symbol_double_equal().is_some() {
                Node::OperatorEq {
                    lhs: node.into(),
                    rhs: self.consume_relational()?.into(),
                }
            } else if self.next_symbol_exclamation_and_equal().is_some() {
                Node::OperatorNe {
                    lhs: node.into(),
                    rhs: self.consume_relational()?.into(),
                }
            } else {
                break Ok(node);
            }
        }
    }

    fn consume_relational(&mut self) -> Result<Node> {
        let mut node = self.consume_add()?;

        loop {
            node = if self.next_symbol_angle_bracket_left().is_some() {
                Node::OperatorLt {
                    lhs: node.into(),
                    rhs: self.consume_add()?.into(),
                }
            } else if self.next_symbol_angle_bracket_right().is_some() {
                Node::OperatorLt {
                    lhs: self.consume_add()?.into(),
                    rhs: node.into(),
                }
            } else if self.next_symbol_angle_bracket_left_and_equal().is_some() {
                Node::OperatorLtEq {
                    lhs: node.into(),
                    rhs: self.consume_add()?.into(),
                }
            } else if self.next_symbol_angle_bracket_right_and_equal().is_some() {
                Node::OperatorLtEq {
                    lhs: self.consume_add()?.into(),
                    rhs: node.into(),
                }
            } else {
                break Ok(node);
            }
        }
    }

    fn consume_add(&mut self) -> Result<Node> {
        let mut node = self.consume_mul()?;

        loop {
            node = if self.next_symbol_plus().is_some() {
                Node::OperatorAdd {
                    lhs: node.into(),
                    rhs: self.consume_mul()?.into(),
                }
            } else if self.next_symbol_minus().is_some() {
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
            node = if self.next_symbol_star().is_some() {
                Node::OperatorMul {
                    lhs: node.into(),
                    rhs: self.consume_unary()?.into(),
                }
            } else if self.next_symbol_slash().is_some() {
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
        if self.next_symbol_plus().is_some() {
            return self.consume_primary();
        } else if self.next_symbol_minus().is_some() {
            let rhs = self.consume_primary()?;
            return Ok(Node::OperatorSub {
                lhs: Node::Integer { value: 0 }.into(),
                rhs: rhs.into(),
            });
        }

        self.consume_primary()
    }

    fn consume_primary(&mut self) -> Result<Node> {
        if let Some(value) = self.next_numeric_value() {
            Ok(Node::Integer { value })
        } else if let Some(value) = self.next_identifier() {
            Ok(Node::LocalVariable {
                identifier: value,
                offset: (value - b'a') as usize,
            })
        } else if self.next_symbol_round_bracket_left().is_some() {
            let node = self.consume_expr()?;
            self.next_symbol_round_bracket_right().ok_or_else(|| {
                self.error_unexpected_token(vec![TokenKind::SymbolRoundBracketRight])
            })?;
            Ok(node)
        } else {
            Err(self.error_unexpected_token(vec![
                TokenKind::Integer(0),
                TokenKind::SymbolRoundBracketLeft,
            ]))
        }
    }

    fn next_numeric_value(&mut self) -> Option<i32> {
        let token = self.tokens.peek()?;

        if let TokenKind::Integer(v) = token.kind {
            self.tokens.next();
            Some(v)
        } else {
            None
        }
    }

    fn next_identifier(&mut self) -> Option<u8> {
        let token = self.tokens.peek()?;

        if let TokenKind::Identifier(v) = token.kind {
            self.tokens.next();
            Some(v)
        } else {
            None
        }
    }

    fn next_symbol_plus(&mut self) -> Option<()> {
        self.tokens
            .next_if(|token| token.kind == TokenKind::SymbolPlus)
            .map(|_| ())
    }

    fn next_symbol_minus(&mut self) -> Option<()> {
        self.tokens
            .next_if(|token| token.kind == TokenKind::SymbolMinus)
            .map(|_| ())
    }

    fn next_symbol_star(&mut self) -> Option<()> {
        self.tokens
            .next_if(|token| token.kind == TokenKind::SymbolStar)
            .map(|_| ())
    }

    fn next_symbol_slash(&mut self) -> Option<()> {
        self.tokens
            .next_if(|token| token.kind == TokenKind::SymbolSlash)
            .map(|_| ())
    }

    fn next_symbol_round_bracket_left(&mut self) -> Option<()> {
        self.tokens
            .next_if(|token| token.kind == TokenKind::SymbolRoundBracketLeft)
            .map(|_| ())
    }

    fn next_symbol_round_bracket_right(&mut self) -> Option<()> {
        self.tokens
            .next_if(|token| token.kind == TokenKind::SymbolRoundBracketRight)
            .map(|_| ())
    }

    fn next_symbol_angle_bracket_left(&mut self) -> Option<()> {
        self.tokens
            .next_if(|token| token.kind == TokenKind::SymbolAngleBracketLeft)
            .map(|_| ())
    }

    fn next_symbol_angle_bracket_right(&mut self) -> Option<()> {
        self.tokens
            .next_if(|token| token.kind == TokenKind::SymbolAngleBracketRight)
            .map(|_| ())
    }

    fn next_symbol_angle_bracket_left_and_equal(&mut self) -> Option<()> {
        self.tokens
            .next_if(|token| token.kind == TokenKind::SymbolAngleBracketLeftAndEqual)
            .map(|_| ())
    }

    fn next_symbol_angle_bracket_right_and_equal(&mut self) -> Option<()> {
        self.tokens
            .next_if(|token| token.kind == TokenKind::SymbolAngleBracketRightAndEqual)
            .map(|_| ())
    }

    fn next_symbol_double_equal(&mut self) -> Option<()> {
        self.tokens
            .next_if(|token| token.kind == TokenKind::SymbolDoubleEqual)
            .map(|_| ())
    }

    fn next_symbol_exclamation_and_equal(&mut self) -> Option<()> {
        self.tokens
            .next_if(|token| token.kind == TokenKind::SymbolExclamationAndEqual)
            .map(|_| ())
    }

    fn next_symbol_equal(&mut self) -> Option<()> {
        self.tokens
            .next_if(|token| token.kind == TokenKind::SymbolEqual)
            .map(|_| ())
    }

    fn next_symbol_semicolon(&mut self) -> Option<()> {
        self.tokens
            .next_if(|token| token.kind == TokenKind::SymbolSemicolon)
            .map(|_| ())
    }

    fn next_eof(&mut self) -> Option<()> {
        self.tokens
            .next_if(|token| token.kind == TokenKind::EOF)
            .map(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number() {
        let mut parser = Parser::new("  1234567890 ;  ");
        assert_eq!(
            parser.consume_statement().unwrap(),
            Node::Integer { value: 1234567890 }
        );
        assert!(parser.next_eof().is_some());
    }

    #[test]
    fn add() {
        let mut parser = Parser::new("  1 + 2 ;  ");
        assert_eq!(
            parser.consume_statement().unwrap(),
            Node::OperatorAdd {
                lhs: Box::new(Node::Integer { value: 1 }),
                rhs: Box::new(Node::Integer { value: 2 }),
            }
        );
        assert!(parser.next_eof().is_some());
    }

    #[test]
    fn sub() {
        let mut parser = Parser::new("  1 - 2 ;  ");
        assert_eq!(
            parser.consume_statement().unwrap(),
            Node::OperatorSub {
                lhs: Box::new(Node::Integer { value: 1 }),
                rhs: Box::new(Node::Integer { value: 2 }),
            }
        );
        assert!(parser.next_eof().is_some());
    }

    #[test]
    fn mul() {
        let mut parser = Parser::new("  1 * 2 ;  ");
        assert_eq!(
            parser.consume_statement().unwrap(),
            Node::OperatorMul {
                lhs: Box::new(Node::Integer { value: 1 }),
                rhs: Box::new(Node::Integer { value: 2 }),
            }
        );
        assert!(parser.next_eof().is_some());
    }

    #[test]
    fn div() {
        let mut parser = Parser::new("  1 / 2 ;  ");
        assert_eq!(
            parser.consume_statement().unwrap(),
            Node::OperatorDiv {
                lhs: Box::new(Node::Integer { value: 1 }),
                rhs: Box::new(Node::Integer { value: 2 }),
            }
        );
        assert!(parser.next_eof().is_some());
    }

    #[test]
    fn mul_and_add() {
        let mut parser = Parser::new("  1 * 2 + 3 / 4 ;  ");
        assert_eq!(
            parser.consume_statement().unwrap(),
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
        assert!(parser.next_eof().is_some());
    }

    #[test]
    fn nested_expression() {
        let mut parser = Parser::new("  (1 + 2 * 3) / (4 - 5) + 6 ;  ");
        assert_eq!(
            parser.consume_statement().unwrap(),
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
        assert!(parser.next_eof().is_some());
    }

    #[test]
    fn unary_plus() {
        let mut parser = Parser::new("  1 * + 2 ;  ");
        assert_eq!(
            parser.consume_statement().unwrap(),
            Node::OperatorMul {
                lhs: Box::new(Node::Integer { value: 1 }),
                rhs: Box::new(Node::Integer { value: 2 }),
            }
        );
        assert!(parser.next_eof().is_some());
    }

    #[test]
    fn unary_minus() {
        let mut parser = Parser::new("  1 * - 2 ;  ");
        assert_eq!(
            parser.consume_statement().unwrap(),
            Node::OperatorMul {
                lhs: Box::new(Node::Integer { value: 1 }),
                rhs: Box::new(Node::OperatorSub {
                    lhs: Box::new(Node::Integer { value: 0 }),
                    rhs: Box::new(Node::Integer { value: 2 }),
                }),
            }
        );
        assert!(parser.next_eof().is_some());
    }

    #[test]
    fn lt_gt() {
        let mut parser = Parser::new("  1 < 2 > 3 ;  ");
        assert_eq!(
            parser.consume_statement().unwrap(),
            Node::OperatorLt {
                lhs: Box::new(Node::Integer { value: 3 }),
                rhs: Box::new(Node::OperatorLt {
                    lhs: Box::new(Node::Integer { value: 1 }),
                    rhs: Box::new(Node::Integer { value: 2 }),
                }),
            }
        )
    }

    #[test]
    fn lteq_gteq() {
        let mut parser = Parser::new("  1 <= 2 >= 3 ;  ");
        assert_eq!(
            parser.consume_statement().unwrap(),
            Node::OperatorLtEq {
                lhs: Box::new(Node::Integer { value: 3 }),
                rhs: Box::new(Node::OperatorLtEq {
                    lhs: Box::new(Node::Integer { value: 1 }),
                    rhs: Box::new(Node::Integer { value: 2 }),
                }),
            }
        )
    }

    #[test]
    fn eq_ne() {
        let mut parser = Parser::new("  1 == 2 != 3 ;  ");
        assert_eq!(
            parser.consume_statement().unwrap(),
            Node::OperatorNe {
                lhs: Box::new(Node::OperatorEq {
                    lhs: Box::new(Node::Integer { value: 1 }),
                    rhs: Box::new(Node::Integer { value: 2 }),
                }),
                rhs: Box::new(Node::Integer { value: 3 })
            }
        )
    }

    #[test]
    fn assignment() {
        let mut parser = Parser::new("  c = 1 + 2 ;  ");
        assert_eq!(
            parser.consume_statement().unwrap(),
            Node::OperatorAssign {
                lhs: Box::new(Node::LocalVariable {
                    identifier: b'c',
                    offset: 2
                }),
                rhs: Box::new(Node::OperatorAdd {
                    lhs: Box::new(Node::Integer { value: 1 }),
                    rhs: Box::new(Node::Integer { value: 2 }),
                }),
            }
        )
    }

    #[test]
    fn multiple_statements() {
        let mut parser = Parser::new("  1 + 2; a;  ");
        assert_eq!(
            parser.parse().unwrap(),
            Node::Block {
                statements: vec![
                    Node::OperatorAdd {
                        lhs: Box::new(Node::Integer { value: 1 }),
                        rhs: Box::new(Node::Integer { value: 2 })
                    },
                    Node::LocalVariable {
                        identifier: b'a',
                        offset: 0
                    }
                ]
            }
        )
    }
}
