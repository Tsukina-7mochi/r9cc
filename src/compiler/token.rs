#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    Integer(i32),
    Identifier(String),
    SymbolPlus,
    SymbolMinus,
    SymbolStar,
    SymbolSlash,
    SymbolRoundBracketLeft,
    SymbolRoundBracketRight,
    SymbolAngleBracketLeft,
    SymbolAngleBracketRight,
    SymbolAngleBracketLeftAndEqual,
    SymbolAngleBracketRightAndEqual,
    SymbolDoubleEqual,
    SymbolExclamationAndEqual,
    SymbolEqual,
    SymbolSemicolon,
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
}
