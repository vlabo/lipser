//extern crate jemallocator;
extern crate nom;

use nom::{
    branch::alt,
    bytes::complete::{escaped, tag},
    character::complete::{char, multispace0, newline, one_of},
    combinator::{cut, map, recognize},
    error::{context, convert_error, ParseError, VerboseError},
    multi::many0,
    number::complete::recognize_float,
    sequence::{delimited, preceded, terminated},
    Err, IResult,
};

use super::LispValue;

use nom::error::ErrorKind;
use nom::{AsChar, InputTakeAtPosition};

fn string_allowed<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar + Clone,
{
    input.split_at_position1_complete(
        |item| item.clone().as_char() == '\"' || item.as_char() == '\\',
        ErrorKind::AlphaNumeric,
    )
}

fn comment_allowed<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar + Clone,
{
    input.split_at_position1_complete(|item| item.as_char() == '\n', ErrorKind::AlphaNumeric)
}

fn name_allowed<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar + Clone,
{
    input.split_at_position1_complete(
        |item| match item.as_char() {
            ';' | ',' | '.' | '`' | ' ' | '!' | '(' | ')' => true,
            _ => false,
        },
        ErrorKind::AlphaNumeric,
    )
}

fn parse_str<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    escaped(string_allowed, '\\', one_of("\"\\n"))(i)
}

fn boolean<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, bool, E> {
    alt((map(tag("false"), |_| false), map(tag("true"), |_| true)))(input)
}

fn string<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    context(
        "string",
        preceded(char('\"'), cut(terminated(parse_str, char('\"')))),
    )(i)
}

fn comment<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    context(
        "string",
        preceded(char(';'), terminated(comment_allowed, newline)),
    )(i)
}

fn comments_and_spaces<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    alt((comment, multispace0))(i)
}

fn int<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, i64, E>
where
{
    match recognize_float(i) {
        Err(e) => Err(e),
        Ok((i, s)) => match s.parse::<i64>() {
            Ok(n) => Ok((i, n)),
            _ => Err(Err::Error(E::from_error_kind(i, ErrorKind::Float))),
        },
    }
}

fn float<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, f64, E>
where
{
    match recognize_float(i) {
        Err(e) => Err(e),
        Ok((i, s)) => match s.parse::<f64>() {
            Ok(n) => Ok((i, n)),
            _ => Err(Err::Error(E::from_error_kind(i, ErrorKind::Float))),
        },
    }
}

fn parse_name<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &str, E> {
    recognize(name_allowed)(i)
}

fn value<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, LispValue, E> {
    preceded(
        comments_and_spaces,
        alt((
            map(string, |s| LispValue::String(String::from(s))),
            map(int, |i| LispValue::Int(i)),
            map(float, |f| LispValue::Float(f)),
            map(boolean, |b| LispValue::Boolean(b)),
            map(parse_name, |n| LispValue::Name(String::from(n))),
            map(function, |values| LispValue::Function(values)),
        )),
    )(i)
}

fn function<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Vec<LispValue>, E> {
    context(
        "function",
        preceded(
            comments_and_spaces,
            delimited(
                char('('),
                many0(value),
                context("closing bracket", cut(preceded(multispace0, char(')')))),
            ),
        ),
    )(i)
}

fn parse_function<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, LispValue, E> {
    preceded(
        comments_and_spaces,
        map(function, |values| LispValue::Function(values)),
    )(i)
}

fn root<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Vec<LispValue>, E> {
    many0(preceded(comments_and_spaces, parse_function))(i)
}

pub fn parse(code: &str) -> Option<Vec<LispValue>> {
    match root::<VerboseError<&str>>(code) {
        Err(Err::Error(e)) | Err(Err::Failure(e)) => {
            println!(
                "verbose errors - `root::<VerboseError>(data)`:\n{}",
                convert_error(code, e)
            );
            None
        }
        Ok(s) => Some(s.1),
        _ => None,
    }

}