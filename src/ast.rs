use crate::{error::CompilerError, syntax::use_item, token::TokenStream};

mod path;
pub use path::Path;

pub fn parse_path(source: &str) -> std::result::Result<Path, CompilerError> {
    let buffer = TokenStream::buffer(source);
    let tokens = TokenStream::new(&buffer, source);
    let (_, path) = use_item(tokens)?;

    Ok(path.path.iter().into())
}
