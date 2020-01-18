use crate::ast::visit::Visitor;
use crate::ast::{Block, Ident, Object};
use crate::error::Error;
use crate::interpreter::{Interpreter, Exec};
use crate::env::Closure;
use downcast_rs::{Downcast, impl_downcast};

use std::cell::Cell;
use std::fmt;
use std::rc::Rc;

pub trait LoxFn: Downcast {
    fn arity(&self) -> usize;
    // fn args(&self) -> &[Ident];
    fn name(&self) -> &str;
    // fn body(&self) -> Block;
    // fn closure(&self) -> &Closure;
    fn call(&self, interpreter: &mut Interpreter, args: &[Object]) -> Result<Exec, Error>;
}
impl_downcast!(LoxFn);

pub struct BuiltinFn {
    pub arity: usize,
    pub args: Vec<Ident>,
    pub name: Ident,
    pub closure: Closure,
    pub body: fn(&mut Interpreter, &[Object]) -> Result<Exec, Error>,
}

impl BuiltinFn {
    pub fn new<F>(
        arity: usize,
        args: Vec<Ident>,
        name: Ident,
        closure: Closure,
        body: fn(&mut Interpreter, &[Object]) -> Result<Exec, Error>,
    ) -> Self {
        Self {
            arity,
            args,
            name,
            closure,
            body,
        }
    }
}

impl LoxFn for BuiltinFn {
    fn arity(&self) -> usize {
        self.arity
    }

    // fn args(&self) -> &[Ident] {
    //     &self.args
    // }

    fn name(&self) -> &str {
        &self.name.0
    }

    // fn closure(&self) -> &Closure {
    //     &self.closure
    // }

    fn call(&self, interpreter: &mut Interpreter, args: &[Object]) -> Result<Exec, Error> {
        (self.body)(interpreter, args)
    }
}

pub struct UserFn {
    pub arity: usize,
    pub args: Vec<Ident>,
    pub name: Ident,
    pub closure: Cell<Closure>,
    pub body: Block,
}

impl UserFn {
    pub fn new(name: Ident, args: Vec<Ident>, closure: Closure, body: Block) -> Self {
        Self {
            arity: args.len(),
            args,
            name,
            closure: Cell::new(closure),
            body,
        }
    }

    pub fn set_closure(&self, closure: Closure) {
        self.closure.set(closure);
    }
}

impl LoxFn for UserFn {
    fn arity(&self) -> usize {
        self.arity
    }

    fn name(&self) -> &str {
        &self.name.0
    }

    // fn closure(&self) -> &Closure {
    //     // &self.closure.
    // }

    fn call(&self, interpreter: &mut Interpreter, args: &[Object]) -> Result<Exec, Error> {
        // printn
        
        if self.arity() != args.len() {
            return Err(Error::ArgumentArity(self.arity(), args.len()));
        }

        let closure = self.closure.take();
        interpreter.push_closure(closure);

        for (i, arg_name) in self.args.iter().enumerate() {
            interpreter.define(arg_name.clone(), args[i].clone());
        }

        let res = interpreter.visit_block(&mut self.body.clone());

        let new_closure = interpreter.pop_scope();
        self.closure.set(new_closure);

        res
    }
}

impl PartialEq for dyn LoxFn {
    fn eq(&self, other: &Self ) -> bool {
        self.arity() == other.arity() && self.name() == other.name()
    }
}

use std::cmp::Ordering;
impl PartialOrd for dyn LoxFn {
    fn partial_cmp(&self, _: &Self) -> Option<Ordering> {
        None
    }
}

// impl PartialOrd for Rc<dyn LoxFn> {
//     fn partial_cmp(&self, _: &Self) -> Option<Ordering> {
//         None
//     }
// }

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
