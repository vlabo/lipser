use std::error::Error;
use std::fmt;

mod custom_parser;

#[derive(Debug, PartialEq, Clone)]
pub enum LispValue {
    String(String),
    Boolean(bool),
    Int(i64),
    Float(f64),
    Name(String),
    Function(Vec<LispValue>),
}

impl std::fmt::Display for LispValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LispValue::String(s) => write!(f, "\"{}\"", s),
            LispValue::Boolean(true) => write!(f, "true"),
            LispValue::Boolean(false) => write!(f, "false"),
            LispValue::Int(n) => write!(f, "int({})", n),
            LispValue::Float(n) => write!(f, "float({})", n),
            LispValue::Name(n) => write!(f, "{}", n),
            LispValue::Function(args) => {
                write!(f, "( ")?;
                for a in args {
                    write!(f, "{} ", a)?;
                }
                write!(f, ")")
            }
        }
    }
}

pub trait ToLispValue<T> {
    fn get(t: T) -> LispValue;
}

impl ToLispValue<i64> for LispValue {
    fn get(t: i64) -> LispValue {
        LispValue::Int(t)
    }
}

impl ToLispValue<f64> for LispValue {
    fn get(t: f64) -> LispValue {
        LispValue::Float(t)
    }
}

impl ToLispValue<&str> for LispValue {
    fn get(t: &str) -> LispValue {
        LispValue::String(t.to_string())
    }
}

impl ToLispValue<bool> for LispValue {
    fn get(t: bool) -> LispValue {
        LispValue::Boolean(t)
    }
}

impl ToLispValue<LispValue> for LispValue {
    fn get(t: LispValue) -> LispValue {
        t
    }
}

#[derive(Debug)]
pub struct ParserError {
    description: String,
    position: usize,
}

impl ParserError {
    fn new(description: &str, position: usize) -> Self {
        Self {
            description: description.to_string(),
            position: position,
        }
    }
}

impl Error for ParserError {}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parser Error: {}, {}", self.description, self.position)
    }
}

impl ParserError {
    pub fn print(&self, code: &str) {
        let mut current_position = 0;
        let mut line = String::new();
        let mut line_number : usize = 0;
        for l in code.lines() {
            line = l.to_string();
            line_number += 1;
            current_position += l.len() + 1;
            if current_position > self.position {
                break;
            }
        }
        
        let line_start = current_position - line.len() - 1;
        println!("{} {}", self.position, line_start);
        let line_pos = self.position - line_start;
        let line_number_str = line_number.to_string();
        println!("line_start: {}, line_number: {}, pos: {}", line_start, line_number, self.position);
        let mut pointer = String::new();
        for _ in 0..(line_pos + line_number_str.len() + 2) {
            pointer.push(' ');
        }
        pointer.push('^');
        
        println!("{}: {}", line_number_str, line);
        println!("{} {}", pointer, self.description);
    }
}

pub fn parse(code: &str) -> Result<Vec<LispValue>, ParserError> {
    custom_parser::parse(code)
}

pub fn parse_and_print(code: &str) {
    custom_parser::parse_and_print(code);
}
