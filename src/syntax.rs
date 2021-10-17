use crate::token::{Token, TokenStream};

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
            Identifier($lex.slice().unwrap().to_owned())
        } else {
            return Err(ParseError::identifier($lex));
        }
    };
}

pub type Result<'source, T> = std::result::Result<(TokenStream<'source>, T), ParseError<'source>>;

#[derive(Debug, Clone)]
pub struct ParseError<'source> {
    stream: TokenStream<'source>,
}

impl<'source> ParseError<'source> {
    // TODO

    fn syntax(stream: TokenStream) -> ParseError {
        ParseError {
            stream,
        }
    }

    fn identifier(stream: TokenStream) -> ParseError {
        ParseError::syntax(stream)
    }
}

impl<'source> std::fmt::Display for ParseError<'source> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "ParseError <undefined>")
    }
}

impl<'source> std::error::Error for ParseError<'source> {}

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

pub fn module(mut tokens: TokenStream) -> Result<Module> {
    let mut module = Module {
        items: Vec::new(),
    };

    loop {
        if tokens.eof() {
            break;
        }

        let result = item(tokens)?;
        tokens = result.0;
        module.items.push(result.1);
    }

    Ok((tokens, module))
}

pub fn item(tokens: TokenStream) -> Result<Item> {
    if let Ok((tokens, global)) = global(tokens.clone()) {
        return Ok((tokens, global))
    }
    if let Ok((tokens, function)) = function(tokens.clone()) {
        return Ok((tokens, function))
    }

    Err(ParseError::syntax(tokens))
}

pub fn global(mut tokens: TokenStream) -> Result<Item> {
    let token = expect!(tokens, syntax);

    let qualifier = match token {
        Token::In => GlobalQualifier::In,
        Token::Out => GlobalQualifier::Out,
        Token::Uniform => GlobalQualifier::Uniform,
        _ => return Err(ParseError::syntax(tokens)),
    };

    let identifier = expect_identifier!(tokens);

    match expect!(tokens, syntax) {
        Token::Semicolon => {},
        _ => return Err(ParseError::syntax(tokens)),
    }

    let global = Global {
        qualifier,
        identifier,
    };

    Ok((tokens, Item::Global(global)))
}

pub fn function(mut tokens: TokenStream) -> Result<Item> {
    match expect!(tokens, syntax) {
        Token::Function => {},
        _ => return Err(ParseError::syntax(tokens)),
    }

    let name = expect_identifier!(tokens);

    match expect!(tokens, syntax) {
        Token::LeftParen => {},
        _ => return Err(ParseError::syntax(tokens)),
    }
    match expect!(tokens, syntax) {
        Token::RightParen => {},
        _ => return Err(ParseError::syntax(tokens)),
    }

    let (tokens, body) = block(tokens)?;

    let function = Function {
        name,
        body,
    };

    Ok((tokens, Item::Function(function)))
}

pub fn block(mut tokens: TokenStream) -> Result<Block> {
    match expect!(tokens, syntax) {
        Token::LeftBrace => {},
        _ => return Err(ParseError::syntax(tokens)),
    }
    match expect!(tokens, syntax) {
        Token::RightBrace => {},
        _ => return Err(ParseError::syntax(tokens)),
    }

    let block = Block { statements: Vec::new() };
    Ok((tokens, block))
}
