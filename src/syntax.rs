use crate::token::{Lexer, Token};

macro_rules! expect {
    ($lex:ident, $err:ident) => {
        if let Some(result) = $lex.next() {
            result
        } else {
            return Err(ParseError::$err($lex));
        }
    };
}

macro_rules! expect_identifier {
    ($lex:ident) => {
        if let Some(Token::Text) = $lex.next() {
            Identifier($lex.slice().to_owned())
        } else {
            return Err(ParseError::identifier($lex));
        }
    };
}

pub type Result<'source, T> = std::result::Result<(Lexer<'source>, T), ParseError<'source>>;

#[derive(Debug, Clone)]
pub struct ParseError<'source> {
    source: WrappedLexer<'source>,
}

impl<'source> ParseError<'source> {
    // TODO

    fn syntax(lex: Lexer) -> ParseError {
        ParseError {
            source: WrappedLexer(lex),
        }
    }

    fn identifier(lex: Lexer) -> ParseError {
        ParseError::syntax(lex)
    }
}

impl<'source> std::fmt::Display for ParseError<'source> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "ParseError <undefined>")
    }
}

impl<'source> std::error::Error for ParseError<'source> {}

#[derive(Clone)]
struct WrappedLexer<'source>(Lexer<'source>);

impl<'source> std::fmt::Debug for WrappedLexer<'source> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "<lexer>")
    }
}

#[derive(Debug, Clone)]
pub struct Module {
    items: Vec<Item>,
}

#[derive(Debug, Clone)]
pub enum Item {
    Global(Global),
    Function(Function),
}

#[derive(Debug, Clone)]
pub struct Global {
    qualifier: GlobalQualifier,
    identifier: Identifier,
}

#[derive(Debug, Clone)]
pub enum GlobalQualifier {
    In,
    Out,
    Uniform,
    Const,
}

#[derive(Debug, Clone)]
pub struct Identifier(String);

#[derive(Debug, Clone)]
pub struct Function {
    name: Identifier,
    // return_type,
    // arguments,
    body: Block,
}

#[derive(Debug, Clone)]
pub struct Block {
    statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Assignment(Assignment),
    Block(Block),
    Expr(Expr),
}

#[derive(Debug, Clone)]
pub struct Assignment {
    binding: Identifier,
    expression: Expr,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
}

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Number(String),
}

pub fn module(mut lex: Lexer) -> Result<Module> {
    let mut module = Module {
        items: Vec::new(),
    };

    loop {
        if lex.clone().next().is_none() {
            break;
        }

        let result = item(lex)?;
        lex = result.0;
        module.items.push(result.1);
    }

    Ok((lex, module))
}

pub fn item(mut lex: Lexer) -> Result<Item> {
    if let Ok((lex, global)) = global(lex.clone()) {
        return Ok((lex, global))
    }
    if let Ok((lex, function)) = function(lex.clone()) {
        return Ok((lex, function))
    }

    Err(ParseError::syntax(lex))
}

pub fn global(mut lex: Lexer) -> Result<Item> {
    let token = expect!(lex, syntax);

    let mut qualifier = match token {
        Token::In => GlobalQualifier::In,
        Token::Out => GlobalQualifier::Out,
        Token::Uniform => GlobalQualifier::Uniform,
        _ => return Err(ParseError::syntax(lex)),
    };

    let identifier = expect_identifier!(lex);

    match expect!(lex, syntax) {
        Token::Semicolon => {},
        _ => return Err(ParseError::syntax(lex)),
    }

    let global = Global {
        qualifier,
        identifier,
    };

    Ok((lex, Item::Global(global)))
}

pub fn function(mut lex: Lexer) -> Result<Item> {
    match expect!(lex, syntax) {
        Token::Function => {},
        _ => return Err(ParseError::syntax(lex)),
    }

    let name = expect_identifier!(lex);

    match expect!(lex, syntax) {
        Token::LeftParen => {},
        _ => return Err(ParseError::syntax(lex)),
    }
    match expect!(lex, syntax) {
        Token::RightParen => {},
        _ => return Err(ParseError::syntax(lex)),
    }

    let (lex, body) = block(lex)?;

    let function = Function {
        name,
        body,
    };

    Ok((lex, Item::Function(function)))
}

pub fn block(mut lex: Lexer) -> Result<Block> {
    match expect!(lex, syntax) {
        Token::LeftBrace => {},
        _ => return Err(ParseError::syntax(lex)),
    }
    match expect!(lex, syntax) {
        Token::RightBrace => {},
        _ => return Err(ParseError::syntax(lex)),
    }

    let block = Block { statements: Vec::new() };
    Ok((lex, block))
}
