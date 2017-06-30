#[cfg(test)] mod tests;

use ::value::Value;
use nom::*;
use std::collections::HashMap;
use std::string::String;
use std::str;

named!(
    pub json_value_root<&[u8], Value, u32>,
    delimited!(
        json_whitespace,
        alt!(json_object | json_object_root),
        json_whitespace
    )
);

named!(
    json_value<&[u8], Value>,
    alt_complete!(
        json_null |
        json_boolean |
        json_float |
        json_int |
        json_string |
        json_array |
        json_object
    )
);

fn json_whitespace(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let len = input.len();
    let mut i = 0;
    while i < len {
        let c = input[i];
        if c == b' ' || c == b'\t' || c == b'\n' {
            i += 1;
        } else if c == b'#' {
            i += 1;
            while i < len && input[i] != b'\n' {
                i += 1;
            }
        } else if c == b'/' && i < len - 1 && input[i+1] == b'/' {
            i += 2;
            while i < len && input[i] != b'\n' {
                i += 1;
            }
        } else {
            break;
        }
    }
    return IResult::Done(&input[i..], &input[..i]);
}

fn inferrable_comma(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let len = input.len();
    let mut i = 0;
    let mut got_newline = false;
    let mut got_comma = false;
    while i < len {
        let c = input[i];
        if c == b' ' || c == b'\t' {
            i += 1;
        } else if c == b'\n' {
            got_newline = true;
            i += 1;
        } else if c == b'#' {
            i += 1;
            while i < len && input[i] != b'\n' {
                i += 1;
            }
        } else if c == b'/' && i < len - 1 && input[i+1] == b'/' {
            i += 2;
            while i < len && input[i] != b'\n' {
                i += 1;
            }
        } else if c == b',' && !got_comma {
            got_comma = true;
            i += 1;
        } else {
            break;
        }
    }
    if got_comma || got_newline {
        return IResult::Done(&input[i..], &input[..i]);
    } else {
        return IResult::Error(error_position!(ErrorKind::Char, &input[i..]));
    }
}

named!(
    json_null<&[u8], Value>,
    value!(Value::Null, tag!("null"))
);

named!(
    json_boolean<&[u8], Value>,
    alt!(
        tag!("true") => { |_| Value::Boolean(true) } |
        tag!("false") => { |_| Value::Boolean(false) }
    )
);

named!(
    json_int<&[u8], Value>,
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
        |i:i64| Value::Int(i)
    )
);

named!(
    json_float<&[u8], Value>,
    map!(
        double,
        |i: f64| { Value::Float(i) }
    )
);

fn escaped_string(input: &[u8]) -> IResult<&[u8], Vec<u8>> {
    let len = input.len();
    let mut i = 0;
    let mut s: Vec<u8> = Vec::new();
    while i < len {
        let c = input[i];
        if c == b'\\' && i < len - 1 && input[i+1] == b'"' {
            s.push(b'"');
            i += 2;
        } else if c == b'"' {
            return IResult::Done(&input[i..], s);
        } else if c == b'\n' {
            return IResult::Error(error_position!(ErrorKind::IsNot, &input[i..]));
        } else {
            s.push(c);
            i += 1;
        }
    }

    return IResult::Incomplete(Needed::Unknown);
}

named!(
    multiline_string<&[u8], &[u8]>,
    delimited!(
        tag!("\"\"\""),
        take_until!("\"\"\""),
        tag!("\"\"\"")
    )
);

fn unquoted_string(input: &[u8], allow_dot: bool) -> IResult<&[u8], &[u8]> {
    let len = input.len();
    let mut i = 0;
    while i < len {
        let c = input[i];
        if c == b'/' && i < len - 1 && input[i+1] == b'/' {
            break;
        } else if b"$\"{}[]:=,+#`^?!@*&\\ \t\n\r'".iter().any(|b| b == &c) {
            break;
        } else if c == b'.' && !allow_dot {
            break;
        } else {
            i += 1;
        }
    }
    if i > 0 {
        return IResult::Done(&input[i..], &input[..i]);
    } else {
        return IResult::Incomplete(Needed::Size(1));
    }
}

named!(
    json_string<&[u8], Value>,
    map!(
        alt_complete!(
            map!(map_res!(multiline_string, str::from_utf8), String::from) |
            delimited!(
                char!('"'),
                map_res!(escaped_string, String::from_utf8),
                char!('"')
            ) |
            map!(map_res!(apply!(unquoted_string, true), str::from_utf8), String::from)
        ),
        |s| Value::String(s)
    )
);

named!(
    json_array<&[u8], Value>,
    map!(
        delimited!(
            tuple!(char!('['), json_whitespace),
            separated_list_complete!(
                inferrable_comma,
                json_value
            ),
            tuple!(json_whitespace, char!(']'))
        ),
        |elems| Value::Array(elems)
    )
);

named!(
    json_object<&[u8], Value>,
    delimited!(
        tuple!(char!('{'), json_whitespace),
        json_object_root,
        tuple!(json_whitespace, char!('}'))
    )
);

fn merge_json(
    old: Value,
    new: Value
) -> Value {
    match (old, new) {
        (Value::Object(mut obj_prev), Value::Object(mut obj_new)) => {
            for (key, value) in obj_new.drain() {
                let new_value = match obj_prev.remove(&key) {
                    Some(old_value) => merge_json(old_value, value),
                    _ => value
                };
                obj_prev.insert(key, new_value);
            }
            Value::Object(obj_prev)
        },
        (_, new) => {
            new
        }
    }
}

named!(
    json_object_path<&[u8], Vec<String>>,
    separated_list!(
        tag!("."),
        alt!(
            delimited!(
                char!('"'),
                map_res!(escaped_string, String::from_utf8),
                char!('"')
            ) |
            map!(map_res!(apply!(unquoted_string, false), str::from_utf8), String::from)
        )
    )
);

named!(
    json_object_root<&[u8], Value>,
    map!(
        separated_list_complete!(
            inferrable_comma,
            tuple!(
                json_object_path,
                alt!(
                    preceded!(json_whitespace, json_object) |
                    preceded!(
                        tuple!(json_whitespace, alt!(char!(':') | char!('=')), json_whitespace),
                        json_value
                    )
                )
            )
        ),
        |pairs| {
            let mut obj = Value::Object(HashMap::new());

            for (path, value) in pairs {
                let next_pair = path.into_iter().rev().fold(value, |v, key| {
                    let mut m = HashMap::new();
                    m.insert(key, v);
                    Value::Object(m)
                });

                obj = merge_json(obj, next_pair);
            }

            obj
        }
    )
);
