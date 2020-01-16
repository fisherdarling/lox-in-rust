use crate::ast::{Block, Object, Ident};
use crate::interpreter::Interpreter;
use crate::error::Error;
use crate::ast::visit::Visitor;
use std::fmt;

pub trait LoxFn {
    fn arity(&self) -> usize;
    fn args(&self) -> &[Ident];
    fn name(&self) -> &str;
    // fn body(&self) -> Block;
    fn call(&self, interpreter: &mut Interpreter, args: &[Object]) -> Result<Object, Error>;
}

pub struct BuiltinFn {
    pub arity: usize,
    pub args: Vec<Ident>,
    pub name: Ident,
    pub code: fn(&mut Interpreter, &[Object]) -> Result<Object, Error>,
}

impl BuiltinFn {
    pub fn new<F>(arity: usize, args: Vec<Ident>, name: Ident, code: fn(&mut Interpreter, &[Object]) -> Result<Object, Error>) -> Self {
        Self {
            arity,
            args,
            name,
            code,
        }
    }
}

impl LoxFn for BuiltinFn {
    fn arity(&self) -> usize {
        self.arity
    }
    
    fn args(&self) -> &[Ident] {
        &self.args
    }

    fn name(&self) -> &str {
        &self.name.0
    }

    fn call(&self, interpreter: &mut Interpreter, args: &[Object]) -> Result<Object, Error> {
        (self.code)(interpreter, args)
    }
}

// if self.arity() != args.len() {
//     return Err(Error::ArgumentArity(self.arity(), args.len()));
// }

// interpreter.push_scope();

// for (i, arg_name) in f.args().iter().enumerate() {
//     interpreter.define(arg_name.clone(), args[i].clone());
// }
// let res = interpreter.visit_block(&mut f.body().clone());

// interpreter.pop_scope();

// res