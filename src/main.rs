use rust_shaders::token::{TokenStream};

// declare fn shade(scene: Scene, material: Material, object: Object) -> vec4;
const SOURCE: &'static str = r#"
uniform scene: Scene;
uniform material: Material;

declare type Material;

use extern::Material; // , Shade};

use root::frag;
"#;

fn main() {
    let buffer = TokenStream::buffer(SOURCE);
    let stream = TokenStream::new(&buffer, SOURCE);

    let (_, module) = match rust_shaders::syntax::module(stream) {
        Err(err) => {
            println!("failed to parse: {}", err);
            return;
        },
        Ok(result) => result,
    };

    println!("Parsing complete!");
    println!("{:#?}", module);

    /*
    generating glsl

    write the version out

    write definitions (hir.definitions())

    write globals (hir.[globals|inputs|outputs]() -> list of fragments)

    write functions in reverse dependency order (hir.functions())
        identify main function (function that writes outputs)
        check the call tree
            check recursive nodes
        walk the call tree

    build symbol table
        read type definitions
        read in/outs

    check functions
        pull in ambient `use` information
        walk function
            declarations => store symbol
            assignments => lookup declarations
            preliminary type check
            evaluate expressions => internal or external dependencies?
                (100% internal expressions can be type checked now)
        check mutability
            if assignment occurs, check if binding was mutable
        check variable usage
            flag anything not used
            flag anything not mutated but is mutable
        linker table
            take note of all unresolved symbols and their kind (dyn/const/static)

    linking
        analyze unresolved symbols
            provide definitions for `use`
            run missing type checks
            record meta variables (optional:) and the types that are valid
            fill in meta variables
            finalize type checking
        linking is successful when all external definitions can be found
        optimizer:
            inline functions
            evaluate constant expressions
            cull dead if statements
            unroll required loops

    generator
        static:
            pull in all meta variables
            send (H)IR to generator
        dynamic (take in a list of optional main functions):
            identify all re-usable stubs, break into parametrized fragments
            identify all main functions (check if arguments are valid ins/outs)
                if they use other globals we can constrain their usage
            write rust code for pipeline construction

    constructing a pipeline
        example:
            let args = { x: foo, y: baz }
            mod::pbr::vert > mod::pbr::geo > mod::pbr::frag
        verify the required outputs for the stages match up
            analyze hir.inputs()/hir.outputs()

    externs, two approaches:
        TODO: define with FQN?
        1) only exists loaded into current scope; other calls must `use` it by importing
            from the containing module; compiler will require FQN if multiple definitions are given
        2) externs get dumped into a single local scope, and multiple definitions is always no bueno
            (unless maybe they are the same definition)
     */
}



// usage in build script
//
// fn do_build() {
//     let generator = generate("./path/", &[("foo", var("bar")), ("baz", var(5))], Some(&["config.json"]))
//         // .static() // throws an error if arguments are underspecified
//         .dynamic();

//     generator.write("module/path")
// }

/*
// tokenizer - turns stream into tokens

read in file -

- update range
    hot patch tokens stream
    expand range until

* tokens are "owned" by an item

struct Compiler {}

compiler.patch(file_id, patch)?
    items()

in input: FooBar;
in baz: Type;

uniform

fn main() -> ReturnType {

}
*/

/*

operating

    module discovery
        traverse directories, auto export submodules by default

    compiling
        refactor
        go to definition
        find references
        find all colors
        list items in document/project
        find signature
        hover
        rename
        compile

    => read file
        tokenize file
        give struct that supports hot patching and pairing with items
    => hold in dynamic structure with all information
        build symbol table
        identify exposed constants
        type check symbols locally
        resolve references
        check mutability
    => drop intermediate structures for compiling
    => generate output

    compile
        determine constant expressions
        reduce into dynamic expressions and static expressions
            if statements of only dynamic constants => become generator fragments
            for loops of constant ranges get unrolled by default
        check against provided instances
        generate dynamic generator if necessary
        generate GLSL stubs for static code
    generator
        contains list of fragments
        type checks the arguments against the valid types
        generates the expressions
 */
