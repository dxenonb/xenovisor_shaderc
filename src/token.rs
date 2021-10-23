use logos::{Logos, Span};

pub type Lexer<'source> = logos::Lexer<'source, Token>;

#[derive(Logos, Debug, Clone, PartialEq)]
pub enum Token {
    // modules
    #[token("declare")]
    Declare,
    #[token("type")]
    Type,
    #[token("const")]
    Const,
    #[token("use")]
    Use,

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
    #[token(":")]
    Colon,
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
    #[token("::")]
    PathSeparator,

    // operators
    #[token("=")]
    Equals,

    // comments
    #[regex(r"//.*")]
    LineComment,
    #[token("/*")]
    OpenComment,
    #[token("*/")]
    CloseComment,

    // misc
    #[regex("[a-zA-Z]+")]
    Text,
    #[error]
    #[regex(r"[ \t\r\n\f]+", logos::skip)]
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
        let skipped = self.skip_comments();
        self.rem.get(skipped).map(|(token, _)| token)
    }

    pub fn next(&mut self) -> Option<&Token> {
        self.rem = &self.rem[self.skip_comments()..];
        let next = self.rem.get(0).map(|(token, _)| token);
        self.rem = &self.rem[1..];
        next
    }

    pub fn slice(&self) -> Option<&str> {
        let (_, span) = self.rem.get(0)?;
        self.source.get(span.clone())
    }

    pub fn skip_line_comments(&self) -> usize {
        TokenStream::skip_line_comments_impl(&self.rem)
    }

    fn skip_line_comments_impl(mut rem: &[(Token, Span)]) -> usize {
        let mut skipped = 0;
        while let Some((Token::LineComment, _)) = rem.get(0) {
            skipped += 1;
            rem = &rem[1..];
        }
        skipped
    }

    /// Determine how many tokens to skip due to comments
    pub fn skip_comments(&self) -> usize {
        let mut rem = self.rem;
        rem = &rem[TokenStream::skip_line_comments_impl(rem)..];
        let skipped = if let Some((Token::OpenComment, _)) = rem.get(0) {
            rem = &rem[1..];
            let mut depth = 0;
            let mut exited = false;
            while !exited {
                match rem.get(0) {
                    Some((Token::OpenComment, _)) => {
                        depth += 1;
                    },
                    Some((Token::CloseComment, _)) => {
                        if depth == 0 {
                            exited = true;
                        } else {
                            depth -= 1;
                        }
                    },
                    Some(_) => {},
                    None => {
                        panic!("encountered EOF inside comment");
                    },
                }
                rem = &rem[1..];
            }
            // TODO: need to recurse/loop, as this won't handle all sequential comments
            rem = &rem[TokenStream::skip_line_comments_impl(rem)..];
            rem
        } else {
            rem
        };
        self.rem.len() - skipped.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::iter::Iterator;

    #[test]
    fn parses_tokens() {
        let buffer: Vec<Token> = TokenStream::buffer("
            uniform boop : //
        ").iter().map(|(token, _span)| token.clone()).collect();

        assert_eq!(buffer, vec![
            Token::Uniform,
            Token::Text,
            Token::Colon,
            Token::LineComment,
        ]);
    }

    #[test]
    fn skips_line_comments() {
        const SOURCE: &str = "
            uniform
            //
            foo
        ";
        let buffer = TokenStream::buffer(SOURCE);
        let mut stream = TokenStream::new(&buffer, SOURCE);

        assert!(Iterator::eq(buffer.iter().map(|(t, _)| t), [
            Token::Uniform,
            Token::LineComment,
            Token::Text,
        ].iter()));
        assert_eq!(stream.next().unwrap(), &Token::Uniform);
        assert_eq!(stream.skip_line_comments(), 1);
        assert_eq!(stream.skip_comments(), 1);
        assert_eq!(stream.next().unwrap(), &Token::Text);
    }

    #[test]
    fn skips_block_comments() {
        const SOURCE: &str = "
            uniform
            /*
             uniform
             foobarbaz
            */
            let
        ";
        let buffer = TokenStream::buffer(SOURCE);
        let mut stream = TokenStream::new(&buffer, SOURCE);

        assert!(Iterator::eq(buffer.iter().map(|(t, _)| t), [
            Token::Uniform,
            Token::OpenComment,
            Token::Uniform,      // 1 space ahead
            Token::Text,         // 2 spaces ahead
            Token::CloseComment, // 3 spaces ahead
            Token::Let,          // 4 spaces ahead
        ].iter()));
        assert_eq!(stream.next().unwrap(), &Token::Uniform);
        assert_eq!(stream.skip_line_comments(), 0);
        // when at "open comment", skip ahead 4 spaces
        assert_eq!(stream.skip_comments(), 4);
        assert_eq!(stream.next().unwrap(), &Token::Let);
    }

    #[test]
    fn skips_nested_block_comments() {
        const SOURCE: &str = "
            uniform
            /*
             /*
              */
             uniform
             foobarbaz
            */
            let
        ";
        let buffer = TokenStream::buffer(SOURCE);
        let mut stream = TokenStream::new(&buffer, SOURCE);

        assert!(Iterator::eq(buffer.iter().map(|(t, _)| t), [
            Token::Uniform,
            Token::OpenComment,
            Token::OpenComment,
            Token::CloseComment,
            Token::Uniform,
            Token::Text,
            Token::CloseComment,
            Token::Let,
        ].iter()));
        assert_eq!(stream.next().unwrap(), &Token::Uniform);
        assert_eq!(stream.skip_line_comments(), 0);
        assert_eq!(stream.skip_comments(), 6);
        assert_eq!(stream.next().unwrap(), &Token::Let);
    }
}
