use std::collections::HashMap;
use ::error::Error;
use ::parser::json_object_path;
use nom::IResult;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Null,
    Boolean(bool),
    Int(i64),
    Float(f64),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>)
}

impl Value {

    pub fn get(&self, path: &str) -> Result<Value, Error> {
        match json_object_path(path.as_bytes()) {
            IResult::Done(ref rest, ref path_parts) if rest.len() == 0 => {
                let v: Option<&Value> = path_parts.iter().fold(Some(self), |v, key| {
                    match v {
                        Some(&Value::Object(ref obj)) => {
                            obj.get(key)
                        },
                        _ => None
                    }
                });
                v.map(|vr| (*vr).clone()).ok_or(Error::NotFound(String::from(path)))
            },
            r => {
                println!("{:?}", r);
                Err(Error::PathError(String::from(path)))
            }
        }
    }

}
