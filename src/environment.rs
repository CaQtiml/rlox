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

pub type EnvId = usize;

#[derive(Debug, Clone)]
pub struct Environment {
    enclosing: Option<EnvId>, // Parent Environment
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

    pub fn new_with_enclosing(enclosing: EnvId) -> Self {
        Self {
            enclosing: Some(enclosing),
            values: HashMap::new(),
        }
    }
}

// This is the "parking lot" that holds all environments
#[derive(Debug)]
pub struct EnvironmentArena {
    environments: Vec<Environment>, // All environments stored here
}

impl EnvironmentArena {
    pub fn new() -> Self {
        Self {
            environments: Vec::new(),
        }
    }

    // Create a new environment and return its ID (parking spot number)
    pub fn create_env(&mut self) -> EnvId {
        let id = self.environments.len(); // Next available spot
        self.environments.push(Environment::new());
        id // Return the spot number
    }

    // Create a new environment with a parent, return its ID
    pub fn create_env_with_enclosing(&mut self, enclosing: EnvId) -> EnvId {
        let id = self.environments.len();
        self.environments.push(Environment::new_with_enclosing(enclosing));
        id
    }

    // Define a variable in a specific environment (by ID)
    pub fn define(&mut self, env_id: EnvId, name: String, value: Value) {
        self.environments[env_id].values.insert(name, value);
    }

    // Assign to a variable, walking up the chain if needed
    pub fn assign(&mut self, env_id: EnvId, name: &str, value: Value) -> Result<()> {
        let mut current = env_id;
        loop {
            // Check current environment
            if self.environments[current].values.contains_key(name) {
                self.environments[current].values.insert(name.to_string(), value);
                return Ok(());
            }
            
            // Move to parent environment
            if let Some(parent) = self.environments[current].enclosing {
                current = parent;
            } else {
                return Err(anyhow!("Undefined variable '{}'.", name));
            }
        }
    }

    // Get a variable's value, walking up the chain if needed
    pub fn get(&self, env_id: EnvId, name: &str) -> Result<Value> {
        let mut current = env_id;
        loop {
            // Check current environment
            if let Some(value) = self.environments[current].values.get(name) {
                return Ok(value.clone());
            }
            
            // Move to parent environment
            if let Some(parent) = self.environments[current].enclosing {
                current = parent;
            } else {
                return Err(anyhow!("Undefined variable '{}'.", name));
            }
        }
    }
}