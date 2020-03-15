use super::LispValue;

//fn read_fucntion(code: &[char]) {
//}

fn split_functions(code: &[char]) -> Vec<(usize, usize)> {
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
                    functions.push(current_function);
                }
            },
            _ => {},
        };
    }
    return functions;
}

pub fn parse(code: &str) -> Option<Vec<LispValue>> {
    let chars : Vec<char> = code.chars().collect();
    let functions_position = split_functions(&chars[..]);
    
    for f in functions_position {
        println!("{}", &chars[f.0..f.1].into_iter().collect::<String>());
        println!("----------------");
    }

    let values = vec![
        LispValue::Function(vec![LispValue::Name("print".to_string()), LispValue::String("Hello custom parser".to_string())])
    ];
    Some(values) 
}
