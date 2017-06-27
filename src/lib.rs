#[macro_use]
extern crate nom;

use nom::*;
use std::collections::HashMap;
use std::string::String;

#[derive(Debug, PartialEq)]
pub enum JsonValue<'a> {
    Null,
    Boolean(bool),
    Int(i64),
    Float(f64),
    String(String),
    Array(Vec<JsonValue<'a>>),
    Object(HashMap<&'a str, JsonValue<'a>>)
}

named!(
    pub json_value<&[u8], JsonValue>,
    alt_complete!(
        json_null |
        json_boolean |
        json_float |
        json_int |
        json_string
    )
);

named!(
    json_null<&[u8], JsonValue>,
    value!(JsonValue::Null, tag!("null"))
);

named!(
    json_boolean<&[u8], JsonValue>,
    alt!(
        tag!("true") => { |_| JsonValue::Boolean(true) } |
        tag!("false") => { |_| JsonValue::Boolean(false) }
    )
);

named!(
    json_int<&[u8], JsonValue>,
    map!(
        flat_map!(
            recognize!(
                tuple!(
                    opt!(alt!(tag!("+") | tag!("-"))),
                    digit
                )
            ),
            parse_to!(i64)
        ),
        |i:i64| JsonValue::Int(i)
    )
);

named!(
    json_float<&[u8], JsonValue>,
    map!(
        double,
        |i: f64| { JsonValue::Float(i) }
    )
);

fn escaped_string(input: &[u8]) -> IResult<&[u8], Vec<u8>> {
    let len = input.len();
    let mut i = 0;
    let mut s: Vec<u8> = Vec::new();
    while i < len {
        if i < len - 1 && input[i] == b'\\' && input[i+1] == b'"' {
            s.push(b'"');
            i += 2;
        } else if input[i] == b'"' {
            return IResult::Done(&input[i..], s);
        } else {
            s.push(input[i]);
            i += 1;
        }
    }

    return IResult::Incomplete(Needed::Unknown);
}

named!(
    json_string<&[u8], JsonValue>,
    map!(
        map_res!(
            delimited!(
                char!('"'),
                escaped_string,
                char!('"')
            ),
            String::from_utf8
        ),
        |s| JsonValue::String(s)
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom::IResult;

    macro_rules! parse_test(
        ($parser: expr, $input: expr, $output: expr) => (
            assert_eq!($parser($input.as_bytes()), IResult::Done(&b""[..], $output))
        )
    );

    #[test] fn test_json_null() {
        parse_test!(json_value, "null", JsonValue::Null);
    }

    #[test] fn test_json_boolean() {
        parse_test!(json_value, "true", JsonValue::Boolean(true));
        parse_test!(json_value, "false", JsonValue::Boolean(false));
    }

    #[test] fn test_json_int() {
        parse_test!(json_value, "0", JsonValue::Int(0));
        parse_test!(json_value, "1", JsonValue::Int(1));
        parse_test!(json_value, "-2", JsonValue::Int(-2));
        parse_test!(json_value, "42", JsonValue::Int(42));
        parse_test!(json_value, "2834293023", JsonValue::Int(2834293023));
    }

    #[test] fn test_json_float() {
        parse_test!(json_value, "0.0", JsonValue::Float(0.0));
        parse_test!(json_value, "4.2", JsonValue::Float(4.2));
        parse_test!(json_value, "-4.2", JsonValue::Float(-4.2));
        parse_test!(json_value, "-4.2e1", JsonValue::Float(-42.0));
        parse_test!(json_value, "-4.2e-2", JsonValue::Float(-0.042));
    }

    #[test] fn test_json_string() {
        parse_test!(json_value, "\"\"", JsonValue::String(String::from("")));
        parse_test!(json_value, "\"a\"", JsonValue::String(String::from("a")));
        parse_test!(json_value, "\"ab\"", JsonValue::String(String::from("ab")));
        parse_test!(json_value, "\"a b\"", JsonValue::String(String::from("a b")));
        parse_test!(json_value, "\"a\\\"b\"", JsonValue::String(String::from("a\"b")));
    }
}
