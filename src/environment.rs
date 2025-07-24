// For keeping variables' state

// src/environment.rs
use crate::value::Value;
use std::collections::HashMap;
use anyhow::{Result, anyhow};

pub struct Environment {
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        // TODO: Create empty environment
        Self {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        // TODO: Store variable (no error if already exists - globals can be redefined)
        // println!("DEBUG: Storing variable '{}' with value {:?}", name, value);
        self.values.insert(name, value);
    }

    pub fn assign(&mut self, name: &str, value: Value) -> Result<()> {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value);
            Ok(())
        } else {
            Err(anyhow!("Undefined variable '{}'.", name))
        }
    }

    pub fn get(&self, name: &str) -> Result<Value> {
        // TODO: Get variable value, return error if undefined
        // Hint: Use anyhow! macro for error
        // println!("DEBUG: Looking up variable '{}', available: {:?}", name, self.values.keys().collect::<Vec<_>>());
        match self.values.get(name) { // don't confuse HashMap's get and Environment's get
            Some(value) => Ok(value.clone()),
            None => Err(anyhow!("Undefined variable '{}'.", name)),
        }
    }
}