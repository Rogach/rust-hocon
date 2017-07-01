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

    pub fn get_bool(&self, path: &str) -> Result<bool, Error> {
        self.get(path).and_then(|v| {
            match v {
                Value::Boolean(b) => Ok(b),
                Value::String(s) => {
                    if &s == "true" || &s == "yes" || &s == "on" {
                        Ok(true)
                    } else if &s == "false" || &s == "no" || &s == "off" {
                        Ok(false)
                    } else {
                        Err(Error::IncompatibleType)
                    }
                },
                _ => Err(Error::IncompatibleType)
            }
        })
    }

    pub fn get_bool_or(&self, path: &str, default: bool) -> bool {
        self.get_bool(path).unwrap_or(default)
    }



}
