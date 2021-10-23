use crate::config;

use std::iter::IntoIterator;
use std::path::{Path, PathBuf};
use std::fs;
use std::io::Read;
use std::io;

pub trait Writer {
    fn write(&mut self, target: &config::Target, stage: &config::ShaderStage, contents: &mut dyn Read) -> io::Result<()>;
}

pub struct DefaultWriter {
    pub root: PathBuf,
}

impl DefaultWriter {
    pub fn new(root: PathBuf) -> DefaultWriter {
        DefaultWriter {
            root,
        }
    }
}

impl Writer for DefaultWriter {
    fn write(
        &mut self,
        target: &config::Target,
        stage: &config::ShaderStage,
        contents: &mut dyn Read,
    ) -> io::Result<()> {
        match target {
            Glsl => {},
        }

        let suffix = match stage {
            config::ShaderStage::Fragment => "frag.glsl",
            config::ShaderStage::Vertex => "vert.glsl",
        };

        fs::create_dir_all(&self.root)?;
        self.root.push(suffix);
        let mut file = fs::File::create(&self.root)?;
        io::copy(contents, &mut file)?;
        self.root.pop();

        Ok(())
    }
}

pub struct Compiler {

}

impl Compiler {
    /// Begin a new compiler session.
    pub fn open(_config: config::Config) -> Compiler {
        Compiler {}
    }

    /// Feed the compiler a set of inputs.
    pub fn feed<I: IntoIterator<Item=config::Input>>(&self, _inputs: I) {

    }

    pub fn declare<S1: AsRef<str>, S2: AsRef<str>, I: IntoIterator<Item=(S1, config::EnvVar<S2>)>>(&self, env: I) {

    }

    /// Query the compiler to generate code or perform analysis.
    pub fn query(&self) -> Queries {
        Queries {
        }
    }
}

pub struct Queries {

}

impl Queries {
    pub fn run_code_gen<S: AsRef<str>, W: Writer>(&self, pipeline: config::Pipeline<S>, mut w: W) {

    }
}

pub struct Generator {
    compiler: Compiler,
}

impl Generator {
    /// Generates a single shader pipeline.
    pub fn glsl<W: Writer, S: AsRef<str>>(&self, pipeline: (S, S), w: W) {
        let (vertex, fragment) = pipeline;
        let pipeline = config::Pipeline {
            vertex: Some(vertex),
            fragment: Some(fragment),
        };
        self.compiler.query()
            .run_code_gen(pipeline, w)
    }

    /// Generates Rust code to generate shaders from fragments, without full
    /// compiler runtime.
    pub fn glsl_static_runtime<S: AsRef<str>, I: IntoIterator<Item=S>>(&self, includes: I) {
        // TODO: refine this

        // let fragments = self.compiler.query()
        //     .identify_fragments(includes);

        /*
        static runtime generation:
        */
    }
}

pub fn generate<P1, P2, E, C, S1, S2>(root: P1, env: E, config: Option<C>)
-> Generator
where P1: AsRef<Path>,
    P2: AsRef<Path>,
    E: IntoIterator<Item=(S1, config::EnvVar<S2>)>,
    C: config::ConfigSource,
    S1: AsRef<str>,
    S2: AsRef<str>,
{
    let config = match config {
        Some(source) => source.read(),
        None => config::Config::default()
    };
    let compiler = Compiler::open(config);
    compiler.declare(env);
    compiler.feed([config::Input::Path(root.as_ref().to_owned())]);
    Generator { compiler }
}
