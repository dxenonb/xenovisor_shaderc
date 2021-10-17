use logos::{Logos, Span};

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

#[derive(Debug, Clone)]
pub struct TokenStream<'a> {
    rem: &'a [(Token, Span)],
    source: &'a str,
}

impl<'a> TokenStream<'a> {
    pub fn buffer(source: &'a str) -> Vec<(Token, Span)> {
        let lexer = Token::lexer(source);
        lexer.spanned().collect()
    }

    pub fn new(buffer: &'a [(Token, Span)], source: &'a str) -> TokenStream<'a> {
        TokenStream {
            rem: buffer,
            source,
        }
    }

    pub fn eof(&self) -> bool {
        self.rem.len() == 0
    }

    pub fn peek(&self) -> Option<&Token> {
        self.rem.get(0).map(|(token, _)| token)
    }

    pub fn next(&mut self) -> Option<&Token> {
        let next = self.rem.get(0).map(|(token, _)| token);
        self.rem = &self.rem[1..];
        next
    }

    pub fn slice(&self) -> Option<&str> {
        let (_, span) = self.rem.get(0)?;
        self.source.get(span.clone())
    }
}
