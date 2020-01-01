use derive_more::Display;
use std::fmt;

pub struct Span<'f> {
    pub lexeme: &'f str,
    pub start: usize,
    pub end: usize,
}

impl<'f> Span<'f> {
    pub fn new(lexeme: &'f str, start: usize, end: usize) -> Self {
        Self { lexeme, start, end }
    }

    pub fn eof(len: usize) -> Self {
        Self {
            lexeme: "",
            start: len,
            end: len,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Display)]
pub enum TokenType {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

pub struct Token<'f> {
    pub ty: TokenType,
    pub span: Span<'f>,
}

impl<'f> Token<'f> {
    pub fn new(ty: TokenType, span: Span<'f>) -> Self {
        Self { ty, span }
    }
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.ty, self.span.lexeme)
    }
}
