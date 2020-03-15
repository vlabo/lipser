mod parser;
mod interpreter;

use interpreter::run;

use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("lisp-code/test.lisp")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    run(contents.as_str());
    Ok(())
}
