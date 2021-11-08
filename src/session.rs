use std::collections::HashSet;
use std::hash::Hash;
use std::path::{Component, Path};
use std::sync::{Arc, RwLock, TryLockError};
use std::{collections::HashMap, path::PathBuf};
use std::fs::{read_dir, read_to_string};
use std::io;

use crate::ast::{self, parse_path};
use crate::syntax::parse_module;
use crate::{config, syntax};
use crate::error::{CompilerError, CompilerStage};
use crate::{error::Result};

pub struct Session {
    source_store: SourceStore,
    parsed: RwLock<HashMap<PathBuf, Arc<syntax::Module>>>,
}

impl Session {
    pub fn new() -> Session {
        let source_store = SourceStore::new();
        let parsed = RwLock::new(HashMap::new());
        Session {
            source_store,
            parsed,
        }
    }

    pub fn register_input(&mut self, input: &config::Input) -> io::Result<()> {
        match input {
            config::Input::Path(path) => {
                self.source_store.discover_tree(path)?;
            },
        }
        Ok(())
    }

    pub fn parse_references<'a: 'i, 'i, S: AsRef<str> + Eq + Hash + 'i, I: Iterator<Item=S>>(&'a self, includes: I) -> Result<References<'i, S>> {
        let mut references = References {
            explicit_modules: HashSet::new(),
            items: HashSet::new(),
        };

        for reference in includes {
            let item = reference.as_ref();
            let path = parse_path(item)?;
            match self.source_store.identify_potential_source(&path) {
                Some(path) => {
                    references.items.insert(reference);
                    references.explicit_modules.insert(path);
                },
                None => return Err(CompilerError::include_error(item)),
            }
        }

        Ok(references)
    }

    pub fn parse_module<P: AsRef<Path>>(&self, p: P) -> Result<Arc<syntax::Module>> {
        let contents = read_to_string(&p)?;
        let module = Arc::new(parse_module(&contents)?);
        let mut parsed = match self.parsed.try_write() {
            Ok(parsed) => parsed,
            Err(TryLockError::WouldBlock) => return Err(CompilerError::resource_unavailable("session.parsed")),
            Err(TryLockError::Poisoned(_)) => return Err(CompilerError::ice("a parsing thread panicked".to_string(), CompilerStage::Parsing)),
        };
        let handle = parsed.entry(p.as_ref().to_owned())
            .or_insert(module);
        Ok(handle.clone())
    }

    pub fn lower_functions(&self) {

    }

    pub fn errors(&self) -> Result<()> {
        Ok(())
    }
}

pub struct SourceStore {
    roots: Vec<PathBuf>,
    modules: HashMap<ast::Path, PathBuf>,
}

pub struct SourceInfo {

}

impl SourceStore {
    fn new() -> SourceStore {
        SourceStore {
            roots: Vec::new(),
            modules: HashMap::new(),
        }
    }

    fn discover_tree<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        if self.roots.iter().find(|p| *p == path.as_ref()).is_some() {
            // the path has already been traversed
            return Ok(());
        }
        let root_id = self.roots.len();
        self.roots.push(path.as_ref().into());
        self.discover_tree_inner(path, root_id)?;

        Ok(())
    }

    fn discover_tree_inner<P: AsRef<Path>>(&mut self, path: P, root_id: usize) -> io::Result<()> {
        for entry in read_dir(path)? {
            let entry = entry?;
            let kind = entry.file_type()?;
            if kind.is_dir() {
                self.discover_tree(entry.path())?;
            } else if kind.is_file() {
                let fs_path = entry.path();
                let mut module_path = fs_path.clone();
                // convert PathBuf into a Path, stripping extension and root
                module_path.set_extension("");
                let prefix = &self.roots[root_id];
                let mut temp_path = Vec::new();
                let module_path = module_path.strip_prefix(prefix)
                    .map_err(|err| io::Error::new(io::ErrorKind::Other, format!("StripPrefixError: {}", err)))?
                    .components();
                for component in module_path {
                    if let Component::Normal(component) = component {
                        if let Some(component) = component.to_str() {
                            temp_path.push(component);
                        } else {
                            return Err(io::Error::new(io::ErrorKind::InvalidData, "path was not utf8"));
                        }
                    }
                }
                let module_path = ast::Path::from(temp_path.iter());
                self.modules.insert(module_path, fs_path);
            }
        }
        Ok(())
    }

    fn identify_potential_source<'a>(&'a self, path: &ast::Path) -> Option<&'a Path> {
        let mut slice = path.clone();
        while slice.len() > 0 {
            let matching_module_path = self.modules.get(&slice);
            if let Some(matching_module_path) = matching_module_path {
                return Some(matching_module_path);
            }
            slice.parent();
        }
        None
    }
}

pub struct References<'a, S> {
    explicit_modules: HashSet<&'a Path>,
    items: HashSet<S>,
}

impl<'a, S> References<'a, S> {
    pub fn modules(&'a self) -> impl Iterator<Item=impl AsRef<Path> + 'a> + 'a {
        self.explicit_modules.iter()
    }
}
