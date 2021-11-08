use std::{error::Error as StdError, fmt::{self}};
use thiserror::Error;
use std::io;

use crate::{span::{Annotations, Span}, syntax::ParseError};

pub type Result<T> = std::result::Result<T, CompilerError>;

#[derive(Error, Debug)]
pub enum CompilerError {
    #[error("An internal compiler error occurred: {0}")]
    Internal(#[from] InternalError),
    #[error("There was a problem handling compiler input: {0}")]
    Input(#[from] InputError),
    #[error("There was a problem compiling the shader: {0}")]
    Shader(#[from] ShaderError),
    #[error("Resource unavailable, would block ({0})")]
    WouldBlock(&'static str),
}

impl<'a> From<io::Error> for CompilerError {
    fn from(error: io::Error) -> Self {
        CompilerError::Input(error.into())
    }
}

// TODO: refactor away the parse error to use annotations
impl<'a> From<ParseError<'a>> for CompilerError {
    fn from(error: ParseError) -> Self {
        CompilerError::Shader(ShaderError {
            annotations: Annotations::from_error_message(format!("{}", error))
        })
    }
}

impl CompilerError {
    pub fn include_error(item: &str) -> CompilerError {
        CompilerError::Input(InputError::Include(item.to_string()))
    }

    pub fn resource_unavailable(resource: &'static str) -> CompilerError {
        CompilerError::WouldBlock(resource)
    }

    pub fn ice(message: String, during: CompilerStage) -> CompilerError {
        CompilerError::Internal(InternalError {
            during,
            involving: None,
            error: message.into(),
        })
    }
}

#[derive(Error, Debug)]
pub enum InputError {
    #[error("{0} is not a valid include")]
    Include(String),
    #[error("IO Error: {source}")]
    Io {
        #[from]
        source: io::Error,
    },
}

#[derive(Error, Debug)]
pub struct InternalError {
    error: Box<dyn StdError>,
    during: CompilerStage,
    involving: Option<Span>,
}

impl fmt::Display for InternalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "During {}, encountered error:\n{}", &self.during, &self.error)?;
        if let Some(span) = &self.involving {
            write!(f, "See {:?}", span)?;
        }
        Ok(())
    }
}


#[derive(Error, Debug, Clone)]
#[error("TODO: an error occurred at {:?}", .annotations.primary)]
pub struct ShaderError {
    annotations: Annotations,
}

#[derive(Debug, Clone)]
pub enum CompilerStage {
    Parsing,
    Lowering,
}

impl fmt::Display for CompilerStage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            CompilerStage::Parsing => "parsing",
            CompilerStage::Lowering => "lowering",
        })
    }
}
