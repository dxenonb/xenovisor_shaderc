use logos::Logos;

pub type Lexer<'source> = logos::Lexer<'source, Token>;

#[derive(Logos, Debug, Clone, PartialEq)]
pub enum Token {
    // modules

    // variables
    #[token("in")]
    In,
    #[token("out")]
    Out,
    #[token("uniform")]
    Uniform,
    #[token("let")]
    Let,
    #[token("mut")]
    Mut,

    // types

    // functions
    #[token("fn")]
    Function,

    // control flow
    #[token("if")]
    If,
    #[token("for")]
    For,

    // symbols
    #[token(".")]
    Period,
    #[token(";")]
    Semicolon,
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,

    // operators

    // misc
    #[regex("[a-zA-Z]+")]
    Text,
    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}
