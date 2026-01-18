mod token;
mod lexer;
mod ast;
mod parser;
mod object;
mod evaluator;
mod environment; // <--- NEW MODULE

use std::io::{self, Write};
use lexer::Lexer;
use parser::Parser;
use evaluator::eval_program;
use environment::Environment; // <--- NEW IMPORT

fn main() {
    println!("Welcome to Nova (v0.1)");
    println!("Now supports VARIABLES! Try 'let x = 10;' then 'x * 2'");
    println!("-----------------------------------------------------");

    // Create memory ONCE, outside the loop
    let mut env = Environment::new();

    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        let bytes_read = io::stdin().read_line(&mut line).unwrap();
        if bytes_read == 0 { break; }

        let l = Lexer::new(line);
        let mut p = Parser::new(l);
        let program = p.parse_program();

        if p.errors.len() > 0 {
            for msg in p.errors {
                println!("\t{}", msg);
            }
            continue;
        }

        // Pass the PERSISTENT env to the evaluator
        let evaluated = eval_program(&program, &mut env);
        println!("{}", evaluated);
    }
}