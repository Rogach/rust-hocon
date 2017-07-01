use super::from_str;
use ::value::Value;
use ::error::Error;
use std::collections::HashMap;
use std::string::String;

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

#[test] fn test_value_extraction() {
    assert_eq!(from_str("a = 42").unwrap().get("a"), Ok(Value::Int(42)));
    assert_eq!(from_str("a.b = 42").unwrap().get("a.b"), Ok(Value::Int(42)));
    assert_eq!(from_str("a.b.c = 42").unwrap().get("a.b.c"), Ok(Value::Int(42)));
    assert_eq!(from_str("a = 42").unwrap().get("b"), Err(Error::NotFound(String::from("b"))));

    assert_eq!(from_str("a = true").unwrap().get_bool("a"), Ok(true));
    assert_eq!(from_str("a = false").unwrap().get_bool("a"), Ok(false));
    assert_eq!(from_str("a = \"true\"").unwrap().get_bool("a"), Ok(true));
    assert_eq!(from_str("a = \"yes\"").unwrap().get_bool("a"), Ok(true));
    assert_eq!(from_str("a = \"on\"").unwrap().get_bool("a"), Ok(true));
    assert_eq!(from_str("a = \"false\"").unwrap().get_bool("a"), Ok(false));
    assert_eq!(from_str("a = \"no\"").unwrap().get_bool("a"), Ok(false));
    assert_eq!(from_str("a = \"off\"").unwrap().get_bool("a"), Ok(false));

    assert_eq!(from_str("a = \"true\"").unwrap().get_bool_or("b", true), true);
}
