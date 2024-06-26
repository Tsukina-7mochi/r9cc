#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    Integer(i32),
    Identifier(String),
    KeywordReturn,
    KeywordIf,
    KeywordElse,
    KeywordWhile,
    KeywordFor,
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
    SymbolCurlyBracketLeft,
    SymbolCurlyBracketRight,
    SymbolDoubleEqual,
    SymbolExclamationAndEqual,
    SymbolEqual,
    SymbolSemicolon,
    SymbolComma,
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
