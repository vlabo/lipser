use super::basic::*;
use super::types::*;
use crate::parser::LispValue;

fn add(state: &mut State, mut args: Vec<LispValue>) -> Option<LispValue> {
    prepare_execution(state, &mut args);
    let unified_type = get_args_unified_type(&args);
    match unified_type {
        Type::Int => Some(LispValue::Int(perform_add(&convert_to_int_array(args)))),
        Type::Float => Some(LispValue::Float(perform_add(&convert_to_float_array(args)))),
        _ => None,
    }
}

fn sub(state: &mut State, mut args: Vec<LispValue>) -> Option<LispValue> {
    prepare_execution(state, &mut args);
    let unified_type = get_args_unified_type(&args);
    match unified_type {
        Type::Int => Some(LispValue::Int(perform_sub(&convert_to_int_array(args)))),
        Type::Float => Some(LispValue::Float(perform_sub(&convert_to_float_array(args)))),
        _ => None,
    }
}

fn mul(state: &mut State, mut args: Vec<LispValue>) -> Option<LispValue> {
    prepare_execution(state, &mut args);
    let unified_type = get_args_unified_type(&args);
    match unified_type {
        Type::Int => Some(LispValue::Int(perform_mul(&convert_to_int_array(args)))),
        Type::Float => Some(LispValue::Float(perform_mul(&convert_to_float_array(args)))),
        _ => None,
    }
}

fn div(state: &mut State, mut args: Vec<LispValue>) -> Option<LispValue> {
    replace_variables_with_values(state, &mut args);
    execute_functions(state, &mut args);

    let unified_type = get_args_unified_type(&args);
    match unified_type {
        Type::Int => Some(LispValue::Int(perform_div(&convert_to_int_array(args)))),
        Type::Float => Some(LispValue::Float(perform_div(&convert_to_float_array(args)))),
        _ => None,
    }
}

#[allow(dead_code)]
fn print(state: &mut State, mut args: Vec<LispValue>) -> Option<LispValue> {
    prepare_execution(state, &mut args);

    let unified_type = get_args_unified_type(&args);
    match unified_type {
        Type::Int => perform_print(&convert_to_int_array(args)),
        Type::Float => perform_print(&convert_to_float_array(args)),
        Type::Boolean => perform_print(&convert_to_boolean_array(args)),
        Type::String => perform_print_string(&convert_to_string_array(args)),
        _ => {}
    };

    None
}

fn println(state: &mut State, mut args: Vec<LispValue>) -> Option<LispValue> {
    prepare_execution(state, &mut args);

    let unified_type = get_args_unified_type(&args);
    match unified_type {
        Type::Int => perform_println(&convert_to_int_array(args)),
        Type::Float => perform_println(&convert_to_float_array(args)),
        Type::Boolean => perform_println(&convert_to_boolean_array(args)),
        Type::String => perform_println_string(&convert_to_string_array(args)),
        _ => {}
    };

    None
}

fn create_custom_function(state: &mut State, mut args: Vec<LispValue>) -> Option<LispValue> {
    if args.len() < 3 {
        return None;
    }

    let name;
    if let LispValue::Name(n) = args.remove(0) {
        name = n;
    } else {
        return None;
    }

    let mut f_args = Vec::new();
    if let LispValue::Function(args) = args.remove(0) {
        for arg in args {
            if let LispValue::Name(n) = arg {
                f_args.push(n);
            }
        }
    } else {
        return None;
    }

    let mut functions = Vec::new();

    for f in args.drain(..) {
        functions.push(f);
    }

    state.add_function(name, Function::Custom(f_args, functions));
    None
}

fn execute_custom_function(
    state: &mut State,
    mut args: Vec<LispValue>,
    parameters: Vec<String>,
    code: Vec<LispValue>,
) -> Option<LispValue> {
    if args.len() != parameters.len() {
        return None;
    }

    let mut local_state = State::create_child(state);
    for param in parameters {
        local_state.add_variable(param.to_string(), args.remove(0));
    }

    let mut result = None;
    for element in code {
        if let LispValue::Function(mut arguments) = element {
            if let LispValue::Name(name) = arguments.remove(0) {
                result = execute_function(&mut local_state, name.as_str(), arguments);
            }
        }
    }

    result
}

fn define_var(state: &mut State, mut args: Vec<LispValue>) -> Option<LispValue> {
    if args.len() != 2 {
        return None;
    }

    if let LispValue::Name(name) = args.remove(0) {
        let value = args.remove(0);

        if let LispValue::Function(mut function_args) = value {
            if let LispValue::Name(function_name) = function_args.remove(0) {
                if let Some(value) = execute_function(state, function_name.as_str(), function_args)
                {
                    state.add_variable(name, value);
                }
            }
        } else if let LispValue::Name(name) = value {
            let mut state_value = None;
            if let Some(value) = state.get_value(name.as_str()) {
                state_value = Some(value.clone());
            }

            if let Some(value) = state_value {
                state.add_variable(name, value);
            }
        } else {
            state.add_variable(name, value);
        }
    }
    None
}

pub fn equals(state: &mut State, mut args: Vec<LispValue>) -> Option<LispValue> {
    prepare_execution(state, &mut args);
    let unified_type = get_args_unified_type(&args);

    match unified_type {
        Type::Int => Some(LispValue::Boolean(perform_equals(&convert_to_int_array(
            args,
        )))),
        Type::Float => Some(LispValue::Boolean(perform_equals(&convert_to_float_array(
            args,
        )))),
        Type::Boolean => Some(LispValue::Boolean(perform_equals(
            &convert_to_boolean_array(args),
        ))),
        Type::String => Some(LispValue::Boolean(perform_equals(
            &convert_to_string_array(args),
        ))),
        Type::None => None,
    }
}

pub fn not_equals(state: &mut State, mut args: Vec<LispValue>) -> Option<LispValue> {
    prepare_execution(state, &mut args);
    let unified_type = get_args_unified_type(&args);

    match unified_type {
        Type::Int => Some(LispValue::Boolean(perform_not_equals(
            &convert_to_int_array(args),
        ))),
        Type::Float => Some(LispValue::Boolean(perform_not_equals(
            &convert_to_float_array(args),
        ))),
        Type::Boolean => Some(LispValue::Boolean(perform_not_equals(
            &convert_to_boolean_array(args),
        ))),
        Type::String => Some(LispValue::Boolean(perform_not_equals(
            &convert_to_string_array(args),
        ))),
        Type::None => None,
    }
}

pub fn grater_then(state: &mut State, mut args: Vec<LispValue>) -> Option<LispValue> {
    prepare_execution(state, &mut args);
    let unified_type = get_args_unified_type(&args);

    match unified_type {
        Type::Int => Some(LispValue::Boolean(perform_grater_then(
            &convert_to_int_array(args),
        ))),
        Type::Float => Some(LispValue::Boolean(perform_grater_then(
            &convert_to_float_array(args),
        ))),
        _ => None,
    }
}

pub fn grater_or_equals_then(state: &mut State, mut args: Vec<LispValue>) -> Option<LispValue> {
    prepare_execution(state, &mut args);
    let unified_type = get_args_unified_type(&args);

    match unified_type {
        Type::Int => {
            let array = convert_to_int_array(args);
            Some(LispValue::Boolean(
                perform_grater_then(&array) | perform_equals(&array),
            ))
        }
        Type::Float => {
            let array = convert_to_float_array(args);
            Some(LispValue::Boolean(
                perform_grater_then(&array) | perform_equals(&array),
            ))
        }
        _ => None,
    }
}

pub fn less_or_equals_then(state: &mut State, mut args: Vec<LispValue>) -> Option<LispValue> {
    prepare_execution(state, &mut args);
    let unified_type = get_args_unified_type(&args);

    match unified_type {
        Type::Int => {
            let array = convert_to_int_array(args);
            Some(LispValue::Boolean(
                perform_less_then(&array) | perform_equals(&array),
            ))
        }
        Type::Float => {
            let array = convert_to_float_array(args);
            Some(LispValue::Boolean(
                perform_less_then(&array) | perform_equals(&array),
            ))
        }
        _ => None,
    }
}

pub fn less_then(state: &mut State, mut args: Vec<LispValue>) -> Option<LispValue> {
    prepare_execution(state, &mut args);
    let unified_type = get_args_unified_type(&args);

    match unified_type {
        Type::Int => Some(LispValue::Boolean(perform_less_then(
            &convert_to_int_array(args),
        ))),
        Type::Float => Some(LispValue::Boolean(perform_less_then(
            &convert_to_float_array(args),
        ))),
        _ => None,
    }
}

pub fn or(state: &mut State, mut args: Vec<LispValue>) -> Option<LispValue> {
    prepare_execution(state, &mut args);
    let unified_type = get_args_unified_type(&args);

    match unified_type {
        Type::Boolean => {
            let array = convert_to_boolean_array(args);
            let mut result = false;
            for val in array {
                result |= val;
            }
            Some(LispValue::Boolean(result))
        }
        _ => None,
    }
}

pub fn and(state: &mut State, mut args: Vec<LispValue>) -> Option<LispValue> {
    prepare_execution(state, &mut args);
    let unified_type = get_args_unified_type(&args);

    match unified_type {
        Type::Boolean => {
            let array = convert_to_boolean_array(args);
            let mut result = true;
            for val in array {
                result &= val;
            }
            Some(LispValue::Boolean(result))
        }
        _ => None,
    }
}

pub fn not(state: &mut State, mut args: Vec<LispValue>) -> Option<LispValue> {
    if args.len() != 1 {
        return None;
    }

    prepare_execution(state, &mut args);

    if let LispValue::Boolean(value) = args[0] {
        return Some(LispValue::Boolean(!value));
    }
    None
}

pub fn execute(state: &mut State, function: Function, args: Vec<LispValue>) -> Option<LispValue> {
    match function {
        Function::Add => add(state, args),
        Function::Sub => sub(state, args),
        Function::Mul => mul(state, args),
        Function::Div => div(state, args),
        Function::Grater => grater_then(state, args),
        Function::GraterOrEquals => grater_or_equals_then(state, args),
        Function::Less => less_then(state, args),
        Function::LessOrEquals => less_or_equals_then(state, args),
        Function::Equals => equals(state, args),
        Function::NotEquals => not_equals(state, args),
        Function::Or => or(state, args),
        Function::And => and(state, args),
        Function::Not => not(state, args),
        //        Function::Print => print(state, args),
        Function::Println => println(state, args),
        Function::Defvar => define_var(state, args),
        Function::Defun => create_custom_function(state, args),
        // Function::Setq => None,
        Function::Custom(parameters, code) => {
            execute_custom_function(state, args, parameters, code)
        }
    }
}
