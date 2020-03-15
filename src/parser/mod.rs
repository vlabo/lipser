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

#[cfg(feature = "custom")]
pub fn parse(code: &str) -> Option<Vec<LispValue>> {
    custom_parser::parse(code)
}

#[cfg(feature = "nom")]
pub fn parse(code: &str) -> Option<Vec<LispValue>> {
    nom_parser::parse(code)
}