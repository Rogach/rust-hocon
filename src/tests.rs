use super::from_str;
use ::value::Value;
use ::error::Error;
use std::collections::HashMap;
use std::string::String;
use nom;

#[test] fn test_full_parse() {
    assert_eq!(
        from_str("a = 2"),
        Ok(Value::Object({
            let mut m = HashMap::new();
            m.insert(String::from("a"), Value::Int(2));
            m
        }))
    );
}

#[test] fn test_incomplete_parse() {
    assert_eq!(
        from_str("a ="),
        Err(Error::ExtraInput(0))
    );
}
