mod basic;
mod functions;
mod types;

use crate::parser::{parse, LispValue};
use types::*;

#[cfg_attr(rustfmt, rustfmt_skip)]
fn initialize_functions(state: &mut State) {
    state.add_function("+".to_string(), Function::Add);
    state.add_function("-".to_string(), Function::Sub);
    state.add_function("*".to_string(), Function::Mul);
    state.add_function("/".to_string(), Function::Div);
    state.add_function(">".to_string(), Function::Grater);
    state.add_function(">=".to_string(), Function::GraterOrEquals);
    state.add_function("<".to_string(), Function::Less);
    state.add_function("<=".to_string(), Function::LessOrEquals);
    state.add_function("=".to_string(), Function::Equals);
    state.add_function("/=".to_string(), Function::NotEquals);
    state.add_function("or".to_string(), Function::Or);
    state.add_function("and".to_string(), Function::And);
    state.add_function("not".to_string(), Function::Not);
    state.add_function("add".to_string(), Function::Add);
    state.add_function("sub".to_string(), Function::Sub);
    state.add_function("div".to_string(), Function::Mul);
    state.add_function("mul".to_string(), Function::Div);
    state.add_function("print".to_string(), Function::Println);
    state.add_function("defvar".to_string(), Function::Defvar);
	state.add_function("defun".to_string(), Function::Defun);
}

pub fn run(code: &str) {
    let mut state = State::new();
    initialize_functions(&mut state);

    if let Some(functions) = parse(code) {
        for f in functions {
            if let LispValue::Function(mut arguments) = f {
                if let LispValue::Name(name) = arguments.remove(0) {
                    execute_function(&mut state, name.as_str(), arguments);
                }
            }
        }
    }
}
