use std::cmp::{PartialEq, PartialOrd};
use std::default::Default;
use std::fmt::Display;
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

pub fn perform_add<T>(numbers: &Vec<T>) -> T
where
    T: AddAssign + Default + Copy,
{
    if numbers.is_empty() {
        return T::default();
    }

    let mut result = numbers[0];
    for v in &numbers[1..] {
        result += *v;
    }

    result
}

pub fn perform_sub<T>(numbers: &Vec<T>) -> T
where
    T: SubAssign + Default + Copy,
{
    if numbers.is_empty() {
        return T::default();
    }

    let mut result = numbers[0];
    for v in &numbers[1..] {
        result -= *v;
    }

    result
}

pub fn perform_mul<T>(numbers: &Vec<T>) -> T
where
    T: MulAssign + Default + Copy,
{
    if numbers.is_empty() {
        return T::default();
    }

    let mut result = numbers[0];
    for v in &numbers[1..] {
        result *= *v;
    }

    result
}

pub fn perform_div<T>(numbers: &Vec<T>) -> T
where
    T: DivAssign + Default + Copy,
{
    if numbers.is_empty() {
        return T::default();
    }

    let mut result = numbers[0];
    for v in &numbers[1..] {
        result /= *v;
    }

    result
}

pub fn perform_grater_then<T>(arguments: &Vec<T>) -> bool
where
    T: PartialOrd,
{
    if arguments.is_empty() {
        return false;
    }

    let mut previous = &arguments[0];
    for v in &arguments[1..] {
        if previous <= v {
            return false;
        }
        previous = v;
    }
    return true;
}

pub fn perform_equals<T>(arguments: &Vec<T>) -> bool
where
    T: PartialEq,
{
    if arguments.is_empty() {
        return false;
    }

    let mut previous = &arguments[0];
    for v in &arguments[1..] {
        if previous != v {
            return false;
        }
        previous = v;
    }
    return true;
}

pub fn perform_not_equals<T>(arguments: &Vec<T>) -> bool
where
    T: PartialEq,
{
    if arguments.is_empty() {
        return false;
    }

    for i in 0..(arguments.len() - 1) {
        let val = &arguments[i];
        for v in &arguments[(i + 1)..] {
            if val == v {
                return false;
            }
        }
    }
    return true;
}

pub fn perform_less_then<T>(arguments: &Vec<T>) -> bool
where
    T: PartialOrd,
{
    if arguments.is_empty() {
        return false;
    }

    let mut previous = &arguments[0];
    for v in &arguments[1..] {
        if previous >= v {
            return false;
        }
        previous = v;
    }
    return true;
}

pub fn perform_print<T>(arguments: &Vec<T>)
where
    T: Display,
{
    for v in arguments {
        print!("{} ", v);
    }
}

pub fn perform_println<T>(arguments: &Vec<T>)
where
    T: Display,
{
    for v in arguments {
        println!("{} ", v);
    }
}

pub fn perform_print_string(arguments: &Vec<String>) {
    for v in arguments {
        let b = v.replace("\\n", "\n");
        print!("{}", b);
    }
}

pub fn perform_println_string(arguments: &Vec<String>) {
    for v in arguments {
        let b = v.replace("\\n", "\n");
        print!("{}", b);
    }
    println!();
}
