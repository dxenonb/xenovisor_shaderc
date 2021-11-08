use crate::{error::CompilerError, token::{Token, TokenStream}};

macro_rules! expect {
    ($lex:ident, $err:ident) => {
        if let Some(result) = $lex.next() {
            result
        } else {
            return Err(ParseError::$err($lex));
        }
    };
    ($lex:ident, $err:ident, $msg:expr) => {
        if let Some(result) = $lex.next() {
            result
        } else {
            return Err(ParseError::$err($lex, $msg));
        }
    };
}

macro_rules! expect_sequence {
    ($tokens:ident, $($token:pat),+) => {{
        fn gather(mut tokens: TokenStream) -> std::result::Result<TokenStream, ParseError> {
            $(
                match tokens.next() {
                    Some($token) => {},
                    _ => return Err(ParseError::syntax(tokens, &format!("expected {}", stringify!($token)))),
                }
            )+
            Ok(tokens)
        }
        gather($tokens)
    }};
}

macro_rules! expect_identifier {
    ($lex:ident) => {
        if let Some(Token::Text) = $lex.next() {
            Identifier($lex.slice_prev().unwrap().to_owned())
        } else {
            return Err(ParseError::identifier($lex));
        }
    };
}

macro_rules! return_if {
    ($result:expr) => {
        match $result {
            Ok(result) => return Ok(result),
            Err(err) => err,
        }
    };
}

pub type Result<'source, T> = std::result::Result<(TokenStream<'source>, T), ParseError<'source>>;

#[derive(Debug, Clone)]
pub struct ParseError<'source> {
    stream: TokenStream<'source>,
    detail: ParseErrorDetail,
}

impl<'source> ParseError<'source> {
    // TODO

    fn syntax<'a>(stream: TokenStream<'source>, message: &'a str) -> ParseError<'source> {
        ParseError {
            stream,
            detail: ParseErrorDetail::Syntax(message.to_string()),
        }
    }

    fn identifier(stream: TokenStream) -> ParseError {
        ParseError::syntax(stream, "expected identifier")
    }
}

#[derive(Debug, Clone)]
enum ParseErrorDetail {
    Syntax(String),
}

impl<'source> std::fmt::Display for ParseError<'source> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "ParseError <undefined> at \"")?;
        match self.stream.slice() {
            Some(text) => write!(fmt, "{}\"; detail: {:?}", text, &self.detail),
            None => write!(fmt, "<eof>")
        }

    }
}

impl<'source> std::error::Error for ParseError<'source> {}

#[derive(Debug, Clone)]
pub struct Module {
    items: Vec<Item>,
}

#[derive(Debug, Clone)]
pub enum Item {
    Use(Use),
    Declare(Declare),
    Global(Global),
    Function(Function),
    Struct(Struct),
}

#[derive(Debug, Clone)]
pub struct Use {
    pub path: Vec<Identifier>,
}

#[derive(Debug, Clone)]
pub enum Declare {
    Function(DeclareFunction),
    Type(DeclareType),
    Const(DeclareConst),
}

#[derive(Debug, Clone)]
pub struct DeclareFunction(Identifier, Arguments, TypeName);

#[derive(Debug, Clone)]
pub struct DeclareType(Identifier);

#[derive(Debug, Clone)]
pub struct DeclareConst(Identifier, TypeName);

#[derive(Debug,Clone)]
pub struct Arguments(Vec<(Identifier, TypeName)>);

#[derive(Debug,Clone)]
pub enum TypeName {
    Identifier(Identifier),
    Tuple(Vec<TypeName>),
    Literal(Literal),
}

#[derive(Debug,Clone)]
pub struct Struct {
    fields: Vec<(Identifier, TypeName)>,
}

#[derive(Debug, Clone)]
pub struct Global {
    qualifier: GlobalQualifier,
    identifier: Identifier,
    definition: TypeName,
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

impl Identifier {
    pub fn str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Identifier {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    name: Identifier,
    return_type: Option<TypeName>,
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

pub fn parse_module(source: &str) -> std::result::Result<Module, CompilerError> {
    let buffer = TokenStream::buffer(source);
    let stream = TokenStream::new(&buffer, source);

    let (_, module) = module(stream)?;

    Ok(module)
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
    let upcoming = tokens.peek();
    if let Some(intention) = upcoming {
        match intention {
            Token::Use => {
                return use_item(tokens.clone()).map(|(s, u)| (s, Item::Use(u)));
            },
            Token::Declare => {
                return declare_item(tokens.clone());
            },
            Token::Uniform | Token::In | Token::Out => {
                return global(tokens.clone());
            },
            Token::Function => {
                return function(tokens.clone());
            },
            _ => {},
        }
    }
    Err(ParseError::syntax(tokens, "expected item"))
}

pub fn use_item(mut tokens: TokenStream) -> Result<Use> {
    tokens = expect_sequence!(tokens, Token::Use)?;
    let mut path = Vec::new();
    loop {
        let component = expect_identifier!(tokens);
        path.push(component);

        match tokens.next() {
            Some(Token::Semicolon) => {
                break;
            },
            Some(Token::PathSeparator) => {

            },
            _ => {
                return Err(ParseError::syntax(tokens, "expected path"));
            }
        }
    }
    Ok((tokens, Use { path }))
}

pub fn declare_item(tokens: TokenStream) -> Result<Item> {
    let into_item = |(tokens, d)| (tokens, Item::Declare(d));
    return_if!(declare_type(tokens.clone()).map(into_item));
    return_if!(declare_const(tokens.clone()).map(into_item));
    Err(ParseError::syntax(tokens, "expected declaration"))
}

pub fn declare_type(mut tokens: TokenStream) -> Result<Declare> {
    tokens = expect_sequence!(tokens, Token::Declare, Token::Type)?;
    let name = expect_identifier!(tokens);
    // let (mut tokens, name) = match type_name(tokens)? {
    //     (tokens, TypeName::Identifier(ident)) => (tokens, ident),
    //     _ => unimplemented!("whoops"),
    // };
    tokens = expect_sequence!(tokens, Token::Semicolon)?;
    Ok((tokens, Declare::Type(DeclareType(name))))
}

pub fn declare_const(mut tokens: TokenStream) -> Result<Declare> {
    tokens = expect_sequence!(tokens, Token::Declare, Token::Const)?;
    let name = expect_identifier!(tokens);

    tokens = expect_sequence!(tokens, Token::Colon)?;
    let (mut tokens, def) = type_name(tokens)?;
    tokens = expect_sequence!(tokens, Token::Semicolon)?;

    Ok((tokens, Declare::Const(DeclareConst(name, def))))
}

pub fn type_name(mut tokens: TokenStream) -> Result<TypeName> {
    // read type
    let name = expect_identifier!(tokens);
    Ok((tokens, TypeName::Identifier(name)))
}

pub fn global(mut tokens: TokenStream) -> Result<Item> {
    const MSG: &str = "expected global qualifier (in, out, or uniform)";
    let token = expect!(tokens, syntax, MSG);

    let qualifier = match token {
        Token::In => GlobalQualifier::In,
        Token::Out => GlobalQualifier::Out,
        Token::Uniform => GlobalQualifier::Uniform,
        _ => {
            return Err(ParseError::syntax(tokens, MSG))
        },
    };

    let identifier = expect_identifier!(tokens);
    tokens = expect_sequence!(tokens, Token::Colon)?;

    let (mut tokens, definition) = type_name(tokens)?;
    tokens = expect_sequence!(tokens, Token::Semicolon)?;

    let global = Global {
        qualifier,
        identifier,
        definition,
    };

    Ok((tokens, Item::Global(global)))
}

pub fn function(mut tokens: TokenStream) -> Result<Item> {
    tokens = expect_sequence!(tokens, Token::Function)?;

    let name = expect_identifier!(tokens);

    tokens = expect_sequence!(tokens, Token::LeftParen, Token::RightParen)?;

    let mut return_type = None;
    match tokens.peek() {
        Some(Token::LeftBrace) => {},
        _ => {
            tokens = expect_sequence!(tokens, Token::RightArrow)?;
            let result = type_name(tokens)?;
            tokens = result.0;
            return_type = Some(result.1);
        },
    }

    // TODO: blocks, let (tokens, body) = block(tokens)?;
    tokens = expect_sequence!(tokens, Token::LeftBrace, Token::RightBrace)?;

    let function = Function {
        name,
        body: Block { statements: Vec::new() },
        return_type,
    };

    Ok((tokens, Item::Function(function)))
}
