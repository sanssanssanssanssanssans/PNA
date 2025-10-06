mod ast;
mod codegen_cpp;
mod lexer;
mod parser;
mod token;
mod typeck;

use std::{env, fs, process};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args.is_empty() {
        eprintln!("usage: pna-cc <input.pna> [-o out.cpp]");
        process::exit(2);
    }

    let mut infile: Option<String> = None;
    let mut outfile: Option<String> = None;

    let mut i = 0usize;
    while i < args.len() {
        if args[i] == "-o" {
            if i + 1 >= args.len() {
                eprintln!("-o needs a file");
                process::exit(2);
            }
            outfile = Some(args[i + 1].clone());
            i += 2;
            continue;
        }
        if infile.is_none() {
            infile = Some(args[i].clone());
        }
        i += 1;
    }

    let src_path = infile.unwrap();
    let src = fs::read_to_string(&src_path)?;

    let toks = lexer::lex(&src)?;
    let prog = parser::parse(toks).map_err(|e| format!("Error: {}", e))?;
    typeck::check(&prog)?;

    let cpp = codegen_cpp::compile_to_cpp(&prog)?;
    if let Some(outp) = outfile {
        fs::write(outp, cpp)?;
    } else {
        print!("{}", cpp);
    }

    Ok(())
}
