mod interpreter;
mod parser;

use interpreter::run;

use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut args = env::args();
    args.next();

    let mut file = File::open("lisp-code/clisp.lisp")?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;
    if let Some(arg) = args.next() {
        if arg == "-d" {
            parser::parse_and_print(contents.as_str());
        }
    } else {
        run(contents.as_str());
    }
    Ok(())
}
