#[derive(Debug, PartialEq)]
pub enum TokenTypes {
    SUM,
    MIN,
    MUL,
    DIV,
    EOF,
    INTEGER,
    // STRING,
    LPAREN,
    RPAREN,
}

pub struct Token<T> {
    pub ttype: TokenTypes,
    pub value: T,
}
