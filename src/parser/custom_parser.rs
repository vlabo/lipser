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

    LispValue::Name(value.to_string())
}

fn find_endline(code: &[char]) -> usize {
    let mut count = 0;
    for c in code {
        if *c == '\n' {
            return count;
        }
        count += 1;
    }
    return count;
}

fn read_argument(code: &[char]) -> (LispValue, usize) {
    let mut arg = String::new();
    let mut quate_open = false;
    let mut is_string = false;
    let mut size = 0;
    for c in code {
        match c {
            '(' | ')' => {
                    if quate_open {
                        arg.push(*c);
                    } else {
                        size -= 1;
                        break;
                    }
                },
            '"' => {
                if quate_open {
                    break;
                }
                quate_open = true;
                is_string = true;
            },
            '\n' => {
                break;
            },
            ' ' | '\t' => {
                if quate_open {
                    arg.push(*c);
                } else {
                    break;
                }
            },
            ';' => {
                if quate_open {
                    arg.push(*c);
                } else {
                    size -= 1;
                    break;
                }
            }
            _ => {
                arg.push(*c);
            },
        }
        size += 1;
    }
    if is_string {
        return (LispValue::String(arg), size);
    }

    (parse_value(&arg), size)
}

fn parse_function(code: &[char]) -> (LispValue, usize) {
    let mut is_open = false;
    let mut read_chars_count = 0;
    let mut arguments = Vec::new();
    while read_chars_count < code.len() {
        match code[read_chars_count] {
            '(' => {
                if is_open {
                    let (value, count) = parse_function(&code[read_chars_count..]);
                    arguments.push(value);
                    read_chars_count += count;
                }
                is_open = true;
            },
            ')' => {
                break;
            },
            ';' => {
                read_chars_count += 1;
                if code[read_chars_count] == ';' {
                    let count = find_endline(code);
                    read_chars_count += count;
                }
            },
            ' ' | '\t' | '\n' => {},
            _ => {
                let (value, count) = read_argument(&code[read_chars_count..]);
                arguments.push(value);
                read_chars_count += count;
            }
        }
        read_chars_count += 1;
    }

    (LispValue::Function(arguments), read_chars_count)
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
    
    Some(lisp_functions) 
}

pub fn parse_and_print(code: &str) {
    let chars : Vec<char> = code.chars().collect();
    let functions = split_functions(&chars[..]);
    let mut lisp_functions = Vec::new();

    for f in functions {
        let (value, _) = parse_function(f);
        lisp_functions.push(value);
    }
    
    for f in &lisp_functions {
        println!("{}", f);
    }
}

#[cfg(test)]
mod custom_parser_test {

    use super::super::{LispValue, ToLispValue};
    
    macro_rules! lisp {
        ($n:tt $($a:expr) *) => {
            LispValue::Function(vec![LispValue::Name(stringify!($n).to_string()),
            $(
                {
                    LispValue::get($a)
                }, 
            )*
            ])
        }
    }

    #[test]
    fn function_call_test() {
        if let Some(result) = super::parse("(+ 3 4 5)") {
            assert_eq!(result, vec![lisp!(+ 3 4 5)]);
        } else {
            panic!("Parse retern None")
        }

        if let Some(result) = super::parse("(+ 3.0 4.0 5.0)") {
            assert_eq!(result, vec![lisp!(+ 3.0 4.0 5.0)]);
        } else {
            panic!("Parse retern None")
        }

        if let Some(result) = super::parse("(print \"Test\")") {
            assert_eq!(result, vec![lisp!(print "Test")]);
        } else {
            panic!("Parse retern None")
        }
    }
}
