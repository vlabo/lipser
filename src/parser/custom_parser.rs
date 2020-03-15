use super::LispValue;

fn parse_value(value: &str) -> LispValue {
    if let Ok(i) = value.parse::<i64>() {
        return LispValue::Int(i);
    }

    if let Ok(f) = value.parse::<f64>() {
        return LispValue::Float(f);
    }
    
    if value == "false" {
        return LispValue::Boolean(false);
    } else if value == "true" {
        return LispValue::Boolean(true);
    }


    LispValue::String(value.to_string())
}

fn parse_function(code: &[char]) -> (LispValue, usize) {
    let mut is_open = false;
    let mut i = 0;
    let mut current_argument = String::new();
    let mut arguments = Vec::new();
    while i < code.len() {
        match code[i] {
            '(' => {
                if is_open {
                    let (value, count) = parse_function(&code[i..]);
                    arguments.push(value);
                    i += count;
                }
                is_open = true;
            },
            ')' => {
                if current_argument.len() > 0 {
                    if arguments.is_empty() {
                        let value = LispValue::Name(current_argument);
                        arguments.push(value);
                    } else {
                        let value = parse_value(&current_argument);
                        arguments.push(value);
                    }
                }
                break;
            },
            ' ' => {
                if current_argument.len() > 0 {
                    if arguments.is_empty() {
                        let value = LispValue::Name(current_argument);
                        arguments.push(value);
                    } else {
                        let value = parse_value(&current_argument);
                        arguments.push(value);
                    }
                    current_argument = String::new();
                }
            },
            _ => {
                current_argument.push(code[i]);
            }
        }
        i += 1;
    }

    (LispValue::Function(arguments), i)
}

fn split_functions(code: &[char]) -> Vec<&[char]> {
    let mut open_count = 0;
    let mut current_function = (0, 0);
    let mut functions = Vec::new();
    for i in 0..code.len() {
        match code[i] {
            '(' => {
                open_count += 1;
                if open_count == 1 {
                    current_function.0 = i;
                }
            },
            ')' => {
                open_count -= 1;
                if open_count == 0 {
                    current_function.1 = i + 1;
                    functions.push(&code[current_function.0..current_function.1]);
                }
            },
            _ => {},
        };
    }
    return functions;
}

pub fn parse(code: &str) -> Option<Vec<LispValue>> {
    let chars : Vec<char> = code.chars().collect();
    let functions = split_functions(&chars[..]);
    let mut lisp_functions = Vec::new();

    for f in functions {
        let (value, _) = parse_function(f);
        lisp_functions.push(value);
    }
    // println!("{:?}", lisp_functions);
    Some(lisp_functions) 
}
