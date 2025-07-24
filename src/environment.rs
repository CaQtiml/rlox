// For keeping variables' state

// src/environment.rs
use crate::value::Value;
use std::collections::HashMap;
use anyhow::{Result, anyhow};
/*
Global Environment          ← Root of the chain
├── global = "I'm global"
├── enclosing = None        ← No parent (this is global scope)
│
│   Block Environment       ← Child environment  
│   ├── local = "I'm local"
│   ├── enclosing = Some(Global Environment)  ← Points to parent
*/
pub struct Environment {
    enclosing: Option<Box<Environment>>, // Parent Environment
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        // TODO: Create empty environment
        Self {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    pub fn new_with_enclosing(enclosing: Environment) -> Self {
        Self {
            enclosing: Some(Box::new(enclosing)),
            values: HashMap::new(),
        }
    }

    pub fn into_enclosing(self) -> Option<Environment> {
        self.enclosing.map(|boxed| *boxed)
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
        } else if let Some(enclosing) = &mut self.enclosing {
            enclosing.assign(name, value)
        } else {
            Err(anyhow!("Undefined variable '{}'.", name))
        }
    }

    pub fn get(&self, name: &str) -> Result<Value> {
        // TODO: Get variable value, return error if undefined
        // Hint: Use anyhow! macro for error
        // println!("DEBUG: Looking up variable '{}', available: {:?}", name, self.values.keys().collect::<Vec<_>>());
        
        if let Some(value) = self.values.get(name) { // don't confuse HashMap's get and Environment's get
            Ok(value.clone())
        }
        else if let Some(enclosing) = &self.enclosing {
            enclosing.get(name)
        }
        else {
            Err(anyhow!("Undefined variable '{}'.", name))
        }
    }
}