#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    Integer(i32),
    OperatorAdd,
    OperatorSub,
    OperatorMul,
    OperatorDiv,
    EOF,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub index_start: usize,
}

impl Token {
    pub fn new(kind: TokenKind, index_start: usize) -> Self {
        Self { kind, index_start }
    }

    pub fn integer_value(&self) -> Option<i32> {
        match self.kind {
            TokenKind::Integer(value) => Some(value),
            _ => None,
        }
    }
}
