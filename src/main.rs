use lalrpop_util::lalrpop_mod;  

lalrpop_mod!(pub grammar);      //  Genera grammar.rs

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Uso: {} <expresión>", args[0]);
        process::exit(1);
    }

    match grammar::ExprParser::new().parse(&args[1]) {
        Ok(result) => println!("= {}", result),
        Err(e) => println!("Error: {:?}", e),
    }
}
