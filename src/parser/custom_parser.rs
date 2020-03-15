use super::LispValue;

fn parse_function(code: &[char]){
    let rest: String = code.into_iter().collect();
    println!("{}", rest);
}


pub fn parse(code: &str) -> Option<Vec<LispValue>> {
    let chars: Vec<char> = code.chars().collect();
    for i in 0..chars.len() {
        match chars[i] {
            '(' => parse_function(&chars[i..]),// read function
            _ => (),// do nothing
        }

    }

    let values = vec![LispValue::Function(vec![LispValue::Name("print".to_string()), LispValue::String("Hello custom parser".to_string())])];
    Some(values) 
}
