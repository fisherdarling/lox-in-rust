use std::collections::HashMap;

use derive_more::{Deref, DerefMut};

use crate::ast::{Ident, Object};
use crate::error::Error;

pub type Closure = HashMap<Ident, Object>;

#[derive(Default, Debug, Clone, PartialEq, Deref, DerefMut)]
pub struct Environment {
    // pub globals: HashMap<Ident, Object>,
    pub vars: Vec<HashMap<Ident, Object>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            vars: vec![HashMap::default()],
        }
    }

    pub fn last(&self) -> &HashMap<Ident, Object> {
        &self.vars.last().unwrap()
    }

    pub fn last_mut(&mut self) -> &mut HashMap<Ident, Object> {
        self.vars.last_mut().unwrap()
    }

    pub fn define(&mut self, ident: Ident, value: Object) {
        self.last_mut().insert(ident, value);
    }

    pub fn get(&self, ident: &Ident) -> Result<Object, Error> {
        self.vars
            .iter()
            .rev()
            .find(|e| e.contains_key(ident))
            .map(|e| e[ident].clone())
            .ok_or(Error::UndefinedVariable(ident.clone()))
    }

    pub fn set(&mut self, ident: &Ident, value: Object) -> Result<Object, Error> {
        let v: &mut HashMap<Ident, Object> = self
            .vars
            .iter_mut()
            .rev()
            .find(|e| e.contains_key(ident))
            .ok_or(Error::UndefinedVariable(ident.clone()))?;
        *v.get_mut(ident).unwrap() = value.clone();
        Ok(value)
    }

    pub fn push_scope(&mut self) {
        self.vars.push(Default::default());
    }

    pub fn push_closure(&mut self, closure: Closure) {
        self.vars.push(closure);
    }

    pub fn pop_scope(&mut self) -> Closure {
        self.vars.pop().unwrap()
    }
}
