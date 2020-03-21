use super::LispValue;
use super::ParserError;

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

fn read_line_comment(full_code: &[char], pos: usize) -> Result<usize, ParserError> {
    let code = &full_code[pos..];

    let mut count = 0;
    for c in code {
        if *c == '\n' {
            count += 1;
            return Ok(count);
        }
        count += 1;
    }
    return Ok(count);
}

fn read_argument(full_code: &[char], pos: usize) -> (LispValue, usize) {
    let code = &full_code[pos..];
    let mut arg = String::new();
    let mut quate_open = false;
    let mut is_string = false;
    let mut size = 0;
    for c in code {
        match c {
            '(' | ')' => {
                if quate_open {
                    arg.push(*c);
                    arg.push(*c);
                } else {
                    size -= 1;
                    break;
                }
            }
            '"' => {
                if quate_open {
                    break;
                }
                quate_open = true;
                is_string = true;
            }
            '\n' => {
                break;
            }
            ' ' | '\t' => {
                if quate_open {
                    arg.push(*c);
                } else {
                    break;
                }
            }
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
            }
        }
        size += 1;
    }

    if is_string {
        return (LispValue::String(arg), size);
    }

    (parse_value(&arg), size)
}

fn parse_function(code: &[char], pos: usize) -> Result<(LispValue, usize), ParserError> {
    let mut is_open = false;
    let mut read_chars_count = pos;
    let mut arguments = Vec::new();
    while read_chars_count < code.len() {
        match code[read_chars_count] {
            '(' => {
                if is_open {
                    let (value, count) = parse_function(code, read_chars_count)?;
                    arguments.push(value);
                    read_chars_count += count;
                } else {
                    is_open = true;
                    read_chars_count += 1;
                }
            }
            ')' => {
                read_chars_count += 1;
                is_open = false;
                break;
            }
            ';' => {
                let count = read_line_comment(code, read_chars_count)?;
                read_chars_count += count;
            }
            ' ' | '\t' | '\n' => {
                read_chars_count += 1;
            }
            _ => {
                let (value, count) = read_argument(code, read_chars_count);
                arguments.push(value);
                read_chars_count += count;
                read_chars_count += 1;
            }
        }
    }

    if is_open {
        return Err(ParserError::new("Function not closed", pos));
    }

    Ok((LispValue::Function(arguments), read_chars_count - pos))
}

fn parse_functions(code: &[char]) -> Result<Vec<LispValue>, ParserError> {
    let mut read_chars_count = 0;
    let mut functions = Vec::new();
    while read_chars_count < code.len() {
        match code[read_chars_count] {
            '(' => {
                let (value, count) = parse_function(code, read_chars_count)?;
                functions.push(value);
                read_chars_count += count;
            }
            ';' => {
                read_chars_count += read_line_comment(code, read_chars_count)?;
            }
            '\n' | ' ' | '\t' => {
                read_chars_count += 1;
            }
            _ => {
                return Err(ParserError::new(
                    &format!("Unexpected symbol: {}", code[read_chars_count]),
                    read_chars_count,
                ));
            }
        }
    }
    Ok(functions)
}

pub fn parse(code: &str) -> Result<Vec<LispValue>, ParserError> {
    let chars: Vec<char> = code.chars().collect();
    let functions = parse_functions(&chars[..])?;

    Ok(functions)
}

pub fn parse_and_print(code: &str) {
    let chars: Vec<char> = code.chars().collect();
    match parse_functions(&chars[..]) {
        Ok(functions) => {
            for f in &functions {
                println!("{}", f);
            }
        }
        Err(err) => println!("{}", err),
    }
}

#[cfg(test)]
mod custom_parser_test {

    use super::super::{LispValue, ParserError, ToLispValue};
    use super::parse;

    macro_rules! lf {
        ($n:tt $($a:expr) *) => {
            LispValue::Function(vec![ln!($n),
            $(
                    LispValue::get($a),
            )*
            ])
        };
    }

    macro_rules! vlf {
        ($n:tt $($a:expr) *) => {
            vec!(lf!($n $($a)*))
        }
    }

    macro_rules! ln {
        ($n:tt) => {
            LispValue::Name(stringify!($n).to_string())
        };
    }

    #[test]
    fn function_call() -> Result<(), ParserError> {
        let result = parse("(+ 3 4 5)")?;
        assert_eq!(result, vlf!(+ 3 4 5));

        let result = parse("(/= 3.0 4.0 5.0)")?;
        assert_eq!(result, vlf!(/= 3.0 4.0 5.0));

        let result = parse("(print \"Test\")")?;
        assert_eq!(result, vlf!(print "Test"));
        Ok(())
    }

    #[test]
    fn function_inside_function() -> Result<(), ParserError> {
        let result = parse("(+ 3 (- 6 5))")?;
        assert_eq!(result, vlf!(+ 3 lf!(- 6 5)));

        let result = parse("(+ (- 6 5.0) (+ 3.0 4))")?;
        assert_eq!(result, vlf!(+ lf!(- 6 5.0) lf!(+ 3.0 4)));

        Ok(())
    }

    #[test]
    fn function_definition() -> Result<(), ParserError> {
        let result = parse(
            r#"(defun println ()
                                       (print " "))"#,
        )?;
        assert_eq!(
            result,
            vlf!(defun ln!(println) LispValue::Function(vec![]) lf!(print " "))
        );

        let result = parse(
            r#"(defun square (n) 
                                       (print "squaring")
                                       (* n n))"#,
        )?;
        assert_eq!(
            result,
            vlf!(defun ln!(square) lf!(n) lf!(print "squaring") lf!(* ln!(n) ln!(n)))
        );
        Ok(())
    }
}
