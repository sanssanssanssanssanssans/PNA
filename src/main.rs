mod ast;
mod codegen_cpp;
mod lexer;
mod parser;
mod token;
use std::{env, fs, process};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args.is_empty() {
        eprintln!("usage: pna-cc <input.pna> -o <out.cpp>");
        process::exit(1);
    }

    let mut input = String::new();
    let mut out = String::from("out.cpp");
    let mut i = 0;
    while i < args.len() {
        if args[i] == "-o" && i + 1 < args.len() {
            out = args[i + 1].clone();
            i += 2;
            continue;
        }
        if input.is_empty() {
            input = args[i].clone();
            i += 1;
            continue;
        }
        i += 1;
    }
    if input.is_empty() {
        eprintln!("missing input");
        process::exit(1);
    }

    let src = fs::read_to_string(&input)?;
    let toks = lexer::lex(&src)?;
    let program = parser::parse(toks)?;
    let cpp = codegen_cpp::emit_cpp(&program)?;
    fs::write(&out, cpp)?;
    Ok(())
}
