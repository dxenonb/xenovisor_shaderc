use crate::config;
use crate::driver::Driver;
use crate::error::Result;
use crate::hir;
use crate::session::Session;

use std::hash::Hash;
use std::iter::IntoIterator;
use std::path::{Path, PathBuf};
use std::fs;
use std::io::Read;
use std::io;
use std::sync::Arc;

pub trait Writer {
    fn write<R: Read>(&mut self, target: &config::Target, stage: &config::ShaderStage, contents: R) -> io::Result<()>;
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
    fn write<R: Read>(
        &mut self,
        target: &config::Target,
        stage: &config::ShaderStage,
        mut contents: R,
    ) -> io::Result<()> {
        match target {
            config::Target::Glsl => {},
        }

        let suffix = match stage {
            config::ShaderStage::Fragment => "frag.glsl",
            config::ShaderStage::Vertex => "vert.glsl",
        };

        fs::create_dir_all(&self.root)?;
        self.root.push(suffix);
        let mut file = fs::File::create(&self.root)?;
        io::copy(&mut contents, &mut file)?;
        self.root.pop();

        Ok(())
    }
}

pub struct GlslBackend; impl GlslBackend { fn new() -> GlslBackend { GlslBackend } }

/// Entry point for dynamic runtime use.
pub struct Compiler {
    backend: GlslBackend,
    driver: Driver,
    session: Session,
}

impl Compiler {
    /// Begin a new compiler session.
    pub fn open(_config: config::Config) -> Compiler {
        let backend = GlslBackend::new();
        let driver = Driver::new();
        let session = Session::new();
        Compiler {
            backend,
            driver,
            session,
        }
    }

    /// Feed the compiler a set of inputs.
    pub fn feed<I: IntoIterator<Item=config::Input>>(&mut self, inputs: I) -> Result<()> {
        for i in inputs {
            self.session.register_input(&i)?;
        }
        Ok(())
    }

    pub fn declare<S1: AsRef<str>, S2: AsRef<str>, I: IntoIterator<Item=(S1, config::EnvVar<S2>)>>(&self, _env: I) {

    }

    /// Query the compiler to generate code or perform analysis.
    pub fn query(&self) -> Queries {
        Queries {
            compiler: self,
        }
    }

    fn backend(&self) -> &GlslBackend {
        &self.backend
    }
}

pub struct Queries<'c> {
    compiler: &'c Compiler,
}

impl<'c> Queries<'c> {
    pub fn run_code_gen<S: AsRef<str> + Eq + Hash, W: Writer>(&self, pipeline: config::Pipeline<S>, mut w: W) -> Result<()> {
        let pipeline = [
            pipeline.vertex.expect("must declare vertex shader"),
            pipeline.fragment.expect("must declare frag shader"),
        ];
        let asts = self.compiler.driver.ast(&self.compiler.session, &pipeline)?;
        let contents: String = asts.map(|a| format!("{:?}\n", a)).collect();
        w.write(&config::Target::Glsl, &config::ShaderStage::Vertex, contents.as_bytes())?;
        // let valid = self.linker().validate_pipeline(vs, fs);
        // self.validate_pipeline(&vs, &fs);
        // there will be internal HIR info built but the returned structure
        // is such that only the items provided exist
        // let hir = self.hir(&[&*vs, &*fs]);
        // let env = hir.infer_env();
        // let reduced = hir.apply_env();
        // self.backend().code_gen(&hir);
        /*
            verify types of each stage align
                do type check
                    resolve needed types
                        lower AST
                        resolve needed references
            build control flow graphs
                identify call graph
                do backend validation
                type check
            evaluate consts
                build const library
            check consts + substitute
                evaluate const expressions
            apply consts
                unroll loops, handle if expressions
            code gen
                render HIR into target structure code gen units
                link the code gen units
                write out target structure
        */
        Ok(())
    }

    pub fn hir<'a, I: IntoIterator<Item=&'a hir::Function>>(&self, _includes: I) -> Arc<hir::Module> {
        Arc::new(hir::Module::empty())
    }

    pub fn validate_pipeline(&self, _vs: &hir::Function, _fs: &hir::Function) {
        // TODO: self.backend().validate_pipeline(vs, fs);
    }

    pub fn backend(&self) -> &GlslBackend {
        self.compiler.backend()
    }
}

pub struct Generator {
    compiler: Compiler,
}

impl Generator {
    /// Generates a single shader pipeline.
    pub fn glsl<W: Writer, S: AsRef<str> + Eq + Hash>(&self, pipeline: (S, S), w: W) {
        let (vertex, fragment) = pipeline;
        let pipeline = config::Pipeline {
            vertex: Some(vertex),
            fragment: Some(fragment),
        };
        self.compiler.query()
            .run_code_gen(pipeline, w)
            .expect("failed code gen");
    }

    /// Generates Rust code to generate shaders from fragments, without full
    /// compiler runtime.
    pub fn glsl_static_runtime<S: AsRef<str>, I: IntoIterator<Item=S>>(&self, _includes: I) {
        // TODO: refine this
        unimplemented!();

        // let fragments = self.compiler.query()
        //     .identify_fragments(includes);

        /*
        static runtime generation:
        */
    }
}

pub fn generate<P, E, C, S1, S2>(root: P, env: E, config: C)
-> Generator
where P: AsRef<Path>,
    E: IntoIterator<Item=(S1, config::EnvVar<S2>)>,
    C: config::ConfigSource,
    S1: AsRef<str>,
    S2: AsRef<str>,
{
    let config = config.read();
    let mut compiler = Compiler::open(config);
    compiler.declare(env);
    compiler.feed([config::Input::Path(root.as_ref().to_owned())])
        .expect("error feeding file");
    Generator { compiler }
}
