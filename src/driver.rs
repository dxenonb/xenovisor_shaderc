use std::{hash::Hash, sync::Arc};

use crate::{error::Result, session::Session, syntax};

pub struct Driver;

impl Driver {
    pub fn new() -> Driver {
        Driver
    }

    pub fn ast<S, I>(
        &self,
        session: &Session,
        includes: I,
    ) -> Result<impl Iterator<Item=Arc<syntax::Module>>>
    where S: AsRef<str> + Eq + Hash,
          I: IntoIterator<Item=S>,
    {
        let iter = includes.into_iter();
        let size_estimate = iter.size_hint();
        let size_estimate = size_estimate.1.unwrap_or(size_estimate.0);

        let includes = session.parse_references(iter)?;

        let mut handles = Vec::with_capacity(size_estimate);

        for module in includes.modules() {
            handles.push(session.parse_module(module)?);
        }
        session.errors()?;

        Ok(handles.into_iter())
    }

    // pub fn hir<'a, I: IntoIterator<Item=&'a hir::Function>>(&self, session: &Session, includes: I) -> Result<()> {
    //     let includes = self.ast(includes)?;

    //     // lowering
    //     let items = includes.all();
    //     for item in &items {
    //         session.lower(item);
    //     }
    //     session.errors()?;

    //     Ok(())
    // }

    // pub fn typed_hir<'a, I: IntoIterator<Item=&'a hir::Function>>(&self, session: &Session, includes: I) -> Result<()> {
    //     let includes = self.hir(includes)?;

    //     for item in includes.all() {
    //         session.type_check(item);
    //     }

    //     Ok(())
    // }
}
