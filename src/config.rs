use std::default::Default;
use std::path::{Path, PathBuf};

pub trait ConfigSource {
    fn read(self) -> Config;
}

impl ConfigSource for Config {
    fn read(self) -> Config {
        self
    }
}

impl<T: AsRef<Path>> ConfigSource for T {
    fn read(self) -> Config {
        unimplemented!("TODO: use serde feature!")
    }
}

pub struct Config {
}

impl Default for Config {
    fn default() -> Self {
        Config {}
    }
}

pub enum Input {
    Path(PathBuf),
}

pub enum EnvVar<S> {
    String(S),
    Integer(i32),
    Bool(bool),
}

pub enum Target {
    Glsl,
}

pub enum ShaderStage {
    Vertex,
    Fragment,
}

pub struct Pipeline<S> {
    pub vertex: Option<S>,
    pub fragment: Option<S>,
}

pub struct Fragment {
    
}
