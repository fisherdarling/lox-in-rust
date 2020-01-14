use std::collections::HashMap;

use derive_more::{Deref, DerefMut};

use crate::ast::{Ident, Object};

#[derive(Default, Debug, Clone, PartialEq, Deref, DerefMut)]
pub struct Environment {
    pub vars: HashMap<Ident, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn vars(&self) -> &HashMap<Ident, Object> {
        &self.vars
    }

    pub fn vars_mut(&mut self) -> &mut HashMap<Ident, Object> {
        &mut self.vars
    }
}
