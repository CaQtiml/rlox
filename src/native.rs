use crate::value::Value;
use crate::interpreter::Interpreter;
use anyhow::Result;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, PartialEq)]
pub enum NativeFunction {
    Clock,
}

impl NativeFunction {
    pub fn arity(&self) -> usize {
        match self {
            NativeFunction::Clock => 0,
        }
    }

    pub fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<Value>) -> Result<Value> {
        match self {
            NativeFunction::Clock => {
                let duration = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap();
                Ok(Value::Number(duration.as_millis() as f64 / 1000.0))
            }
        }
    }

    pub fn name(&self) -> &str {
        match self {
            NativeFunction::Clock => "clock",
        }
    }
}