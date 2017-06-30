#[macro_use] extern crate nom;
#[macro_use] extern crate quick_error;

mod value;
mod parser;
mod error;
#[cfg(test)] mod tests;

use value::Value;
use error::Error;
use nom::IResult;
use nom::verbose_errors::Err;

pub fn from_str(s: &str) -> Result<Value, Error> {
    let input = s.as_bytes();
    match parser::json_value_root(input) {
        IResult::Done(rest, value) => {
            if rest == &b""[..] {
                Ok(value)
            } else {
                Err(Error::ExtraInput(rest.as_ptr() as isize - input.as_ptr() as isize))
            }
        },
        IResult::Incomplete(needed) => {
            Err(Error::IncompleteInput(needed))
        },
        IResult::Error(Err::Code(kind)) => {
            Err(Error::ParseError(format!("parse error: {}", kind.description())))
        },
        IResult::Error(Err::Node(kind, _)) => {
            Err(Error::ParseError(format!("parse error: {}", kind.description())))
        },
        IResult::Error(Err::Position(kind, pos)) => {
            Err(Error::ParseError(format!(
                "parse error '{}' at '{}'",
                kind.description(),
                if pos.len() > 50 {
                    format!("{}...", String::from_utf8_lossy(&pos[..50]))
                } else {
                    format!("{}", String::from_utf8_lossy(&pos[..50]))
                }
            )))
        },
        IResult::Error(Err::NodePosition(kind, pos, _)) => {
            Err(Error::ParseError(format!(
                "parse error '{}' at '{}...'",
                kind.description(),
                if pos.len() > 50 {
                    format!("{}...", String::from_utf8_lossy(&pos[..50]))
                } else {
                    format!("{}", String::from_utf8_lossy(&pos[..50]))
                }
            )))
        }
    }
}
