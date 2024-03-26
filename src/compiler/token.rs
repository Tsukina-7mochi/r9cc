#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    Integer(i32),
    OperatorPlus,
    OperatorMinus,
    EOF,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub index_start: usize,
}

impl Token {
    pub fn integer(value: i32, index_start: usize) -> Self {
        Self {
            kind: TokenKind::Integer(value),
            index_start,
        }
    }

    pub fn operator_plus(index_start: usize) -> Self {
        Self {
            kind: TokenKind::OperatorPlus,
            index_start,
        }
    }

    pub fn operator_minus(index_start: usize) -> Self {
        Self {
            kind: TokenKind::OperatorMinus,
            index_start,
        }
    }

    pub fn eof(index_start: usize) -> Self {
        Self {
            kind: TokenKind::EOF,
            index_start,
        }
    }

    pub fn as_integer(self) -> Option<i32> {
        match self.kind {
            TokenKind::Integer(value) => Some(value),
            _ => None,
        }
    }
}
