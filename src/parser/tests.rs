use super::*;
use super::Value::*;
use nom::IResult;
use std::string::String as Str;

macro_rules! parse_test(
    ($parser: expr, $input: expr, $output: expr) => (
        assert_eq!($parser($input.as_bytes()), IResult::Done(&b""[..], $output))
    )
);

#[test] fn test_json_null() {
    parse_test!(json_value, "null", Null);
}

#[test] fn test_json_boolean() {
    parse_test!(json_value, "true", Boolean(true));
    parse_test!(json_value, "false", Boolean(false));
}

#[test] fn test_json_int() {
    parse_test!(json_value, "0", Int(0));
    parse_test!(json_value, "1", Int(1));
    parse_test!(json_value, "-2", Int(-2));
    parse_test!(json_value, "42", Int(42));
    parse_test!(json_value, "2834293023", Int(2834293023));
}

#[test] fn test_json_float() {
    parse_test!(json_value, "0.0", Float(0.0));
    parse_test!(json_value, "4.2", Float(4.2));
    parse_test!(json_value, "-4.2", Float(-4.2));
    parse_test!(json_value, "-4.2e1", Float(-42.0));
    parse_test!(json_value, "-4.2e-2", Float(-0.042));
}

#[test] fn test_json_string() {
    parse_test!(json_value, "\"\"", String(Str::from("")));
    parse_test!(json_value, "\"a\"", String(Str::from("a")));
    parse_test!(json_value, "\"ab\"", String(Str::from("ab")));
    parse_test!(json_value, "\"a b\"", String(Str::from("a b")));
    parse_test!(json_value, "\"a\\\"b\"", String(Str::from("a\"b")));
}

#[test] fn test_json_array() {
    parse_test!(json_value, "[]", Array(vec![]));
    parse_test!(json_value, "[null]", Array(vec![Null]));
    parse_test!(json_value, "[1,2]", Array(vec![Int(1), Int(2)]));
    parse_test!(json_value, "[1,[2,3]]", Array(vec![Int(1), Array(vec![Int(2), Int(3)])]));
}

#[test] fn test_json_object() {
    parse_test!(json_value, "{}", Object(HashMap::new()));
    parse_test!(json_value, "{\"a\":42}", Object({
        let mut m = HashMap::new();
        m.insert(Str::from("a"), Int(42));
        m
    }));
    parse_test!(json_value, "{\"a\":42,\"b\":43}", Object({
        let mut m = HashMap::new();
        m.insert(Str::from("a"), Int(42));
        m.insert(Str::from("b"), Int(43));
        m
    }));
}

macro_rules! parse_test_eq(
    ($parser: expr, $input: expr) => (
        assert_eq!($parser($input.as_bytes()), IResult::Done(&b""[..], $input.as_bytes()))
    )
);

#[test] fn test_comments() {
    parse_test_eq!(json_whitespace, "");
    parse_test_eq!(json_whitespace, "\n");
    parse_test_eq!(json_whitespace, "\n#");
    parse_test_eq!(json_whitespace, "#\n");
    parse_test_eq!(json_whitespace, " ");
    parse_test_eq!(json_whitespace, " #");
    parse_test_eq!(json_whitespace, " # c");
    parse_test_eq!(json_whitespace, " # c\n");
    parse_test_eq!(json_whitespace, " # c\n ");
    parse_test_eq!(json_whitespace, " # c\n  ");
    parse_test_eq!(json_whitespace, " # c\n  ");
    parse_test_eq!(json_whitespace, " # c\n  //");
    parse_test_eq!(json_whitespace, " # c\n  //\n");
    parse_test_eq!(json_whitespace, " # c\n  //\n////");
    parse_test!(json_value, "[ ]", Array(vec![]));
    parse_test!(json_value, "[ 1]", Array(vec![Int(1)]));
    parse_test!(json_value, "[1 ]", Array(vec![Int(1)]));
    parse_test!(json_value, "[ 1 ]", Array(vec![Int(1)]));
    parse_test!(json_value, "[ 1,2 ]", Array(vec![Int(1), Int(2)]));
    parse_test!(json_value, "[ 1 ,2 ]", Array(vec![Int(1), Int(2)]));
    parse_test!(json_value, "[ 1, 2 ]", Array(vec![Int(1), Int(2)]));
    parse_test!(json_value, "[ 1 , 2 ]", Array(vec![Int(1), Int(2)]));
    parse_test!(json_value, "[ 1 , 2,3 ]", Array(vec![Int(1), Int(2), Int(3)]));
    parse_test!(json_value, "[ 1 , 2,3]", Array(vec![Int(1), Int(2), Int(3)]));
    parse_test!(json_value, "[ 1 , 2, 3]", Array(vec![Int(1), Int(2), Int(3)]));
    parse_test!(json_value, "[ 1 , 2 , 3]", Array(vec![Int(1), Int(2), Int(3)]));
    parse_test!(json_value, "[1 , 2 , 3 ]", Array(vec![Int(1), Int(2), Int(3)]));
    parse_test!(json_value, "[ 1 , 2 , 3 ]", Array(vec![Int(1), Int(2), Int(3)]));
    parse_test!(json_value, "[ 1 , #s\n 2 , 3 ]", Array(vec![Int(1), Int(2), Int(3)]));
    parse_test!(json_value, "[ 1 , #s\n\n 2 , 3 ]", Array(vec![Int(1), Int(2), Int(3)]));

    let m0 = || Object(HashMap::new());
    parse_test!(json_value_root, "{}", m0());
    parse_test!(json_value_root, " {} ", m0());
    parse_test!(json_value_root, " { } ", m0());
    parse_test!(json_value_root, " { \n} ", m0());
    parse_test!(json_value_root, " {\n } ", m0());
    parse_test!(json_value_root, " { \n } ", m0());

    let m1 = || {
        let mut m = HashMap::new();
        m.insert(Str::from("a"), Int(1));
        Object(m)
    };
    parse_test!(json_value_root, "{\"a\":1}", m1());
    parse_test!(json_value_root, " {\"a\":1} ", m1());
    parse_test!(json_value_root, " { \"a\":1} ", m1());
    parse_test!(json_value_root, " {\"a\" :1} ", m1());
    parse_test!(json_value_root, " {\"a\": 1} ", m1());
    parse_test!(json_value_root, " {\"a\":1 } ", m1());
    parse_test!(json_value_root, " { \"a\" : 1 } ", m1());
    parse_test!(json_value_root, "\n{\n\"a\"\n:\n1\n}\n", m1());
    parse_test!(json_value_root, "\n\n{\n\n\"a\"\n\n:\n\n1\n\n}\n\n", m1());
    parse_test!(json_value_root, "\n{\n\"a\"\n:# cmt \n1\n}\n", m1());

    let m2 = || {
        let mut m = HashMap::new();
        m.insert(Str::from("a"), Int(1));
        m.insert(Str::from("b"), Int(2));
        Object(m)
    };
    parse_test!(json_value_root, "{\"a\":1,\"b\":2}", m2());
    parse_test!(json_value_root, "{\"a\":1 ,\"b\":2}", m2());
    parse_test!(json_value_root, "{\"a\":1, \"b\":2}", m2());
    parse_test!(json_value_root, "{\"a\":1 , \"b\":2}", m2());
    parse_test!(json_value_root, "{\"a\":1 ,\n \"b\":2}", m2());
    parse_test!(json_value_root, "{\"a\":1 ,\n\n \"b\":2}", m2());
}

#[test] fn test_comma_inference() {
    parse_test!(json_value, "[1\n2]", Array(vec![Int(1), Int(2)]));
    parse_test!(json_value, "[1#a\n2]", Array(vec![Int(1), Int(2)]));
    parse_test!(json_value, "[1 , 2 \n, 3]", Array(vec![Int(1), Int(2), Int(3)]));
    parse_test!(json_value, "[1 , 2 \n\n\n, 3]", Array(vec![Int(1), Int(2), Int(3)]));
    parse_test!(json_value, "[1 , 2 \n# s\n\n, 3]", Array(vec![Int(1), Int(2), Int(3)]));

    let m2 = || {
        let mut m = HashMap::new();
        m.insert(Str::from("a"), Int(1));
        m.insert(Str::from("b"), Int(2));
        Object(m)
    };
    parse_test!(json_value_root, "{ \"a\":1\n\"b\":2 }", m2());
    parse_test!(json_value_root, "{ \"a\":1,\n\"b\":2 }", m2());
}

#[test] fn test_equals_instead_of_colon() {
    parse_test!(json_value, "{\"a\" = 42}", Object({
        let mut m = HashMap::new();
        m.insert(Str::from("a"), Int(42));
        m
    }));
    parse_test!(json_value, "{\"a\" = 42,\"b\":43}", Object({
        let mut m = HashMap::new();
        m.insert(Str::from("a"), Int(42));
        m.insert(Str::from("b"), Int(43));
        m
    }));
}

#[test] fn test_skipping_colon_before_object_values() {
    parse_test!(json_value, "{\"a\" = { \"b\":43 }}", Object({
        let mut m1 = HashMap::new();
        m1.insert(Str::from("b"), Int(43));
        let mut m2 = HashMap::new();
        m2.insert(Str::from("a"), Object(m1));
        m2
    }));
    parse_test!(json_value, "{\"a\" { \"b\":43 }}", Object({
        let mut m1 = HashMap::new();
        m1.insert(Str::from("b"), Int(43));
        let mut m2 = HashMap::new();
        m2.insert(Str::from("a"), Object(m1));
        m2
    }));
}

#[test] fn test_dropping_braces_on_root_object() {
    parse_test!(json_value_root, "\"a\" = 42", Object({
        let mut m = HashMap::new();
        m.insert(Str::from("a"), Int(42));
        m
    }));
    parse_test!(json_value_root, "\"a\" = 42\n", Object({
        let mut m = HashMap::new();
        m.insert(Str::from("a"), Int(42));
        m
    }));
    parse_test!(json_value_root, "\"a\" = 42,\"b\":43", Object({
        let mut m = HashMap::new();
        m.insert(Str::from("a"), Int(42));
        m.insert(Str::from("b"), Int(43));
        m
    }));
    parse_test!(json_value_root, "\"a\" = 42\n\"b\":43", Object({
        let mut m = HashMap::new();
        m.insert(Str::from("a"), Int(42));
        m.insert(Str::from("b"), Int(43));
        m
    }));
}

#[test] fn test_object_merging() {
    parse_test!(
        json_value_root,
        r#"
"a" { "b": 1 }
"a" { "c": 2 }
"#,
        Object({
            let mut m1 = HashMap::new();
            m1.insert(Str::from("b"), Int(1));
            m1.insert(Str::from("c"), Int(2));
            let mut m2 = HashMap::new();
            m2.insert(Str::from("a"), Object(m1));
            m2
        })
    );

    parse_test!(
        json_value_root,
        r#"
"a" { "b": { "c": 1 } }
"a" { "b": { "d": 2 } }
"#,
        Object({
            let mut m1 = HashMap::new();
            m1.insert(Str::from("c"), Int(1));
            m1.insert(Str::from("d"), Int(2));
            let mut m2 = HashMap::new();
            m2.insert(Str::from("b"), Object(m1));
            let mut m3 = HashMap::new();
            m3.insert(Str::from("a"), Object(m2));
            m3
        })
    );

    parse_test!(
        json_value_root,
        r#"
"a" { "b": { "c": 1 }, "e": 3 }
"a" { "b": { "d": 2 } }
"#,
        Object({
            let mut m1 = HashMap::new();
            m1.insert(Str::from("c"), Int(1));
            m1.insert(Str::from("d"), Int(2));
            let mut m2 = HashMap::new();
            m2.insert(Str::from("b"), Object(m1));
            m2.insert(Str::from("e"), Int(3));
            let mut m3 = HashMap::new();
            m3.insert(Str::from("a"), Object(m2));
            m3
        })
    );
}

#[test] fn test_unquoted_strings() {
    parse_test!(json_value, "{a = 42}", Object({
        let mut m = HashMap::new();
        m.insert(Str::from("a"), Int(42));
        m
    }));

    parse_test!(json_value, "{a = bc}", Object({
        let mut m = HashMap::new();
        m.insert(Str::from("a"), String(Str::from("bc")));
        m
    }));

    parse_test!(json_value, "{a = b/c}", Object({
        let mut m = HashMap::new();
        m.insert(Str::from("a"), String(Str::from("b/c")));
        m
    }));
}

#[test] fn test_object_paths() {
    parse_test!(json_object_path, "ab", vec![Str::from("ab")]);
    parse_test!(json_object_path, "a.b", vec![Str::from("a"), Str::from("b")]);
    parse_test!(json_object_path, "a", vec![Str::from("a")]);

    parse_test!(json_value_root, "a.b = 43", Object({
        let mut m1 = HashMap::new();
        m1.insert(Str::from("b"), Int(43));
        let mut m2 = HashMap::new();
        m2.insert(Str::from("a"), Object(m1));
        m2
    }));

    parse_test!(json_value_root, "a.b.c = 43", Object({
        let mut m1 = HashMap::new();
        m1.insert(Str::from("c"), Int(43));
        let mut m2 = HashMap::new();
        m2.insert(Str::from("b"), Object(m1));
        let mut m3 = HashMap::new();
        m3.insert(Str::from("a"), Object(m2));
        m3
    }));

    parse_test!(json_value_root, "a.\"b\".c = 43", Object({
        let mut m1 = HashMap::new();
        m1.insert(Str::from("c"), Int(43));
        let mut m2 = HashMap::new();
        m2.insert(Str::from("b"), Object(m1));
        let mut m3 = HashMap::new();
        m3.insert(Str::from("a"), Object(m2));
        m3
    }));

    parse_test!(json_value_root, "a.\"b.2\".c = 43", Object({
        let mut m1 = HashMap::new();
        m1.insert(Str::from("c"), Int(43));
        let mut m2 = HashMap::new();
        m2.insert(Str::from("b.2"), Object(m1));
        let mut m3 = HashMap::new();
        m3.insert(Str::from("a"), Object(m2));
        m3
    }));
}

#[test] fn test_no_newlines_in_normal_strings() {
    assert_eq!(
        escaped_string(&b"a\n"[..]),
        IResult::Error(Err::Position(ErrorKind::IsNot, &b"\n"[..]))
    );
}

#[test] fn test_multiline_strings() {
    parse_test!(json_value_root, r#"
a = """
b
"""
"#, Object({
    let mut m = HashMap::new();
    m.insert(Str::from("a"), String(Str::from("\nb\n")));
    m
}));
}
