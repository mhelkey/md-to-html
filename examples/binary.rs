use clap::Parser;

use md_to_html::MarkDownCompiler;

fn main() {
    tracing_subscriber::fmt().init();

    let mut compiler = MarkDownCompiler::parse();

    if compiler.get_default_compiler_dir().is_none() {
        compiler.set_default_compile_dir(std::env::current_dir().unwrap());
    }

    compiler.compile_default().unwrap();
}
