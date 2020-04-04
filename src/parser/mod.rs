#[cfg(feature = "nom")]
mod nom_parser;

#[cfg(feature = "custom")]
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

#[cfg(feature = "custom")]
pub fn parse(code: &str) -> Option<Vec<LispValue>> {
    custom_parser::parse(code)
}

#[cfg(feature = "custom")]
pub fn parse_and_print(code: &str) {
    custom_parser::parse_and_print(code);
}

#[cfg(feature = "nom")]
pub fn parse(code: &str) -> Option<Vec<LispValue>> {
    nom_parser::parse(code)
}
