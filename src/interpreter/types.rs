use crate::parser::LispValue;
use std::collections::HashMap;

use super::functions::*;

pub enum Type {
    None,
    Int,
    Float,
    Boolean,
    String,
}
#[warn(dead_code)]
#[derive(Clone)]
pub enum Function {
    Add,
    Sub,
    Mul,
    Div,
    //    Print,
    Println,
    Defvar,
    Defun,
    // Setq,
    Equals,
    NotEquals,
    Grater,
    GraterOrEquals,
    Less,
    LessOrEquals,
    Or,
    And,
    Not,
    Custom(Vec<String>, Vec<LispValue>),
}

pub struct State<'a> {
    parent: Option<&'a State<'a>>,
    functions: HashMap<String, Function>,
    pub variables: HashMap<String, LispValue>,
}

impl<'a> State<'a> {
    pub fn new() -> Self {
        let parent = None;
        let functions = HashMap::new();
        let variables = HashMap::new();
        Self {
            parent,
            functions,
            variables,
        }
    }

    pub fn create_child(&'a self) -> State<'a> {
        let parent = Some(self);
        let functions = HashMap::new();
        let variables = HashMap::new();
        Self {
            parent,
            functions,
            variables,
        }
    }

    pub fn get_function(&self, name: &str) -> Option<&Function> {
        let mut function = self.functions.get(name);
        if let None = function {
            if let Some(parent_state) = &self.parent {
                function = parent_state.get_function(name);
            }
        }
        function
    }

    pub fn get_value(&self, name: &str) -> Option<&LispValue> {
        let mut value = self.variables.get(name);
        if let None = value {
            if let Some(parent_state) = &self.parent {
                value = parent_state.get_value(name);
            }
        }
        value
    }

    pub fn add_function(&mut self, name: String, function: Function) {
        self.functions.insert(name, function);
    }

    pub fn add_variable(&mut self, name: String, value: LispValue) {
        self.variables.insert(name, value);
    }
}

pub fn convert_to_int_array(array: Vec<LispValue>) -> Vec<i64> {
    let mut result_array = Vec::new();
    for value in array {
        match value {
            LispValue::Int(i) => {
                result_array.push(i);
            }
            value => {
                println!("{:?}", value);
            }
        }
    }
    return result_array;
}

pub fn convert_to_float_array(array: Vec<LispValue>) -> Vec<f64> {
    let mut result_array = Vec::new();
    for value in array {
        match value {
            LispValue::Int(i) => {
                result_array.push(i as f64);
            }
            LispValue::Float(f) => {
                result_array.push(f);
            }
            _ => {}
        }
    }
    return result_array;
}

pub fn convert_to_boolean_array(array: Vec<LispValue>) -> Vec<bool> {
    let mut result_array = Vec::new();
    for value in array {
        match value {
            LispValue::Boolean(b) => {
                result_array.push(b);
            }
            _ => {}
        }
    }
    return result_array;
}

pub fn convert_to_string_array(array: Vec<LispValue>) -> Vec<String> {
    let mut result_array = Vec::new();
    for value in array {
        match value {
            LispValue::Int(i) => {
                result_array.push(i.to_string());
            }
            LispValue::Float(f) => {
                result_array.push(f.to_string());
            }
            LispValue::Boolean(b) => {
                result_array.push(b.to_string());
            }
            LispValue::String(s) => {
                result_array.push(s);
            }
            LispValue::Name(_) | LispValue::Function(_) => {}
        }
    }

    return result_array;
}

pub fn get_args_unified_type(args: &Vec<LispValue>) -> Type {
    let mut value_type = Type::None;

    for value in args {
        value_type = get_value_type(value_type, value);
    }
    value_type
}

fn get_value_type(last_type: Type, value: &LispValue) -> Type {
    match value {
        LispValue::Int(_) => match last_type {
            Type::None | Type::Int => Type::Int,
            Type::Boolean | Type::String => Type::String,
            _ => last_type,
        },
        LispValue::Float(_) => match last_type {
            Type::Int | Type::Float | Type::None => Type::Float,
            _ => Type::String,
        },
        LispValue::Boolean(_) => match last_type {
            Type::None | Type::Boolean => Type::Boolean,
            _ => Type::String,
        },
        LispValue::String(_) => Type::String,
        LispValue::Name(_) | LispValue::Function(_) => last_type,
    }
}

pub fn execute_function(
    state: &mut State,
    function_name: &str,
    arguments: Vec<LispValue>,
) -> Option<LispValue> {
    let mut function = None;

    if let Some(f) = state.get_function(function_name) {
        function = Some(f.clone());
    }

    if let Some(f) = function {
        return execute(state, f, arguments);
    }

    None
}

pub fn replace_variables_with_values(state: &State, args: &mut Vec<LispValue>) {
    for i in 0..args.len() {
        let value = &args[i];
        match value {
            LispValue::Name(name) => {
                if let Some(value) = state.get_value(name.as_str()) {
                    args[i] = value.clone();
                }
            }
            _ => {}
        }
    }
}

pub fn execute_functions(state: &mut State, parent_arguments: &mut Vec<LispValue>) {
    for i in 0..parent_arguments.len() {
        let mut is_function = false;
        {
            let value = &parent_arguments[i];
            if let LispValue::Function(_) = value {
                is_function = true;
            }
        }

        if is_function {
            let value = parent_arguments.remove(i);
            if let LispValue::Function(mut args) = value {
                if !args.is_empty() {
                    if let LispValue::Name(name) = args.remove(0) {
                        if let Some(returned_value) = execute_function(state, name.as_str(), args) {
                            parent_arguments.insert(i, returned_value);
                        }
                    }
                }
            }
        }
    }
}

pub fn prepare_execution(state: &mut State, args: &mut Vec<LispValue>) {
    replace_variables_with_values(state, args);
    execute_functions(state, args);
}
