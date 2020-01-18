use std::convert::TryFrom;
use std::rc::Rc;
use std::convert::TryInto;

use crate::ast::function::LoxFn;
use crate::ast::visit::*;
use crate::ast::{
    operator::{BinOp, BinaryOp, UnOp, UnaryOp},
    Block, Decl, Expr, Ident, Object, Program, Stmt, Func,
};
use crate::env::{Environment, Closure};
use crate::ast::function::{BuiltinFn, UserFn};

use crate::error::Error;

macro_rules! value {
    ($name:expr) => {
        match $name {
            Exec::Value(o) => o,
            r @ Exec::Return(_) => return Ok(r),
            _ => panic!(),
        }
    };
}

macro_rules! catch {
    ($name:expr) => {
        match $name {
            Exec::Return(c) | Exec::Value(c) => Exec::Value(c),
            _ => panic!(),
        }
    }
}


// macro_rules! same_type {
//     ($lhs:ident $rhs:ident) => {
//         match (&$lhs, &$rhs) {
//             (Lit::Int(_), Lit::Int(_)) => Ok(()),
//             (Lit::Float(_), Lit::Float(_)) => Ok(()),
//             (Lit::Str(_), Lit::Str(_)) => Ok(()),
//             (Lit::Ident(_), Lit::Ident(_)) => Ok(()),
//             (Lit::Path(_), Lit::Path(_)) => Ok(()),
//             (Lit::Bool(_), Lit::Bool(_)) => Ok(()),
//             (Lit::Unit, Lit::Unit) => Ok(()),
//             _ => Err(Error::TypeMismatch($lhs.clone(), $rhs.clone())),
//         }
//     };
// }

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Interpreter {
    env: Environment,
    call_stack: Vec<Ident>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
            call_stack: Vec::new(),
        }
    }

    pub fn define_global(&mut self, _name: Ident, _value: Object) {}

    pub fn push_closure(&mut self, closure: Closure) {
        self.env.push_closure(closure)
    }

    pub fn push_scope(&mut self) {
        self.env.push_scope();
    }

    pub fn pop_scope(&mut self) -> Closure {
        self.env.pop_scope()
    }

    pub fn define(&mut self, name: Ident, value: Object) {
        self.env.define(name, value);
    }
}

impl Visitor for Interpreter {
    type Output = Exec;

    fn visit_func_call(
        &mut self,
        f: Func,
        args: &mut [Object],
    ) -> Result<Self::Output, Error> {
        // self.call_stack.push(Ident(f.borrow().name().to_string()));

        f.borrow().call(self, args)
    }

    fn visit_func(
        &mut self,
        name: &mut Ident,
        func: Func,
    ) -> Result<Self::Output, Error> {
        if let Some(user) = (*func.borrow()).downcast_ref::<UserFn>() {
            user.set_closure(self.env.last().clone());
        } 
        // println!("[FUNC] {:?}", func);
        self.env.define(name.clone(), Object::Func(func.clone()));
        Ok(Exec::None)
    }

    fn visit_expr(&mut self, e: &mut Expr) -> Result<Self::Output, Error> {
        // println!("[ENV] {:#?}", self.env);
        // println!("[EXPR] {:?}", e);

        match e {
            Expr::Assign(lhs, rhs) => {
                let rhs = value!(self.visit_expr(rhs)?);

                if let box Expr::Object(Object::Ident(ident)) = lhs {
                    self.env.set(&ident, rhs)
                } else {
                    Err(Error::UnsupportedOperation(
                        "Currently only identifiers can be newly assigned.".to_string(),
                    ))
                }?;

                Ok(Exec::None)
            }
            Expr::Access(_, _) => panic!(),
            Expr::Call(p, a) => {
                // println!("Executing a function");

                let func: Func = self.env.get(p)?.try_into()?;
                let mut args = Vec::new();

                for mut arg in a.iter_mut() {
                    let object: Object = value!(self.visit_expr(&mut arg)?);
                    args.push(object);
                }

                let catch = catch!(self.visit_func_call(func.clone(), args.as_mut_slice())?);
                // println!("Caught: {:?}", catch);
                Ok(catch)
            },
            Expr::Object(l) => {
                if let Object::Ident(ident) = l {
                    Ok(Exec::Value(self.env.get(&ident)?))
                } else {
                    Ok(Exec::Value(l.clone()))
                }
            }
            Expr::UnOp(op, rhs) => {
                let rhs = value!(self.visit_expr(rhs)?);
                match rhs {
                    Object::Int(_) => exec_unop::<isize>(op.clone(), rhs),
                    Object::Float(_) => exec_unop::<f32>(op.clone(), rhs),
                    Object::Bool(_) => exec_unop::<bool>(op.clone(), rhs),
                    _ => Err(Error::InvalidUnaryOperator(op.clone(), rhs.to_string())),
                }.map(Into::into)
            }
            Expr::BinOp(lhs, op, rhs) => {
                let lhs = value!(self.visit_expr(lhs)?);

                // Short circuiting logic
                match op {
                    BinOp::And => {
                        if !lhs.is_truthy()? {
                            return Ok(Object::from(false).into());
                        }
                    }
                    BinOp::Or => {
                        if lhs.is_truthy()? {
                            return Ok(Object::from(true).into());
                        }
                    }
                    _ => (),
                };

                let rhs = value!(self.visit_expr(rhs)?);

                match lhs {
                    Object::Int(_) => exec_binop::<isize>(lhs, op.clone(), rhs),
                    Object::Float(_) => exec_binop::<f32>(lhs, op.clone(), rhs),
                    Object::Bool(_) => exec_binop::<bool>(lhs, op.clone(), rhs),
                    _ => Err(Error::InvalidBinaryOperator(
                        lhs.to_string(),
                        op.clone(),
                        rhs.to_string(),
                    )),
                }.map(Into::into)
            }
        }
    }

    fn visit_if(
        &mut self,
        check: &mut Expr,
        good: &mut Block,
        bad: &mut Block,
    ) -> Result<Self::Output, Error> {
        let check = self.visit_expr(check)?;
        let check: Object = value!(check); 
        if check.is_truthy()? {
            self.visit_block(good)
        } else {
            self.visit_block(bad)
        }
    }

    fn visit_block(&mut self, block: &mut Block) -> Result<Self::Output, Error> {
        // println!("[BLOCK] {:?}", block);

        self.env.push_scope();

        let mut last = Self::Output::default();
        for decl in &mut block.0 {
            last = self.visit_decl(decl)?;
        }

        self.env.pop_scope();

        Ok(last)
    }

    fn visit_while(&mut self, pred: &mut Expr, block: &mut Block) -> Result<Self::Output, Error> {
        let mut last = Self::Output::default();

        while value!(self.visit_expr(pred)?).is_truthy()? {
            last = self.visit_block(block)?;
        }

        Ok(last)
    }

    fn visit_var_decl(
        &mut self,
        ident: &mut Ident,
        init: &mut Option<Expr>,
    ) -> Result<Self::Output, Error> {
        let value: Object = if let Some(init) = init {
            value!(self.visit_expr(init)?)
        } else {
            Object::Unit
        };

        self.env.define(ident.clone(), value);
        Ok(Exec::None)
    }

    fn visit_stmt(&mut self, s: &mut Stmt) -> Result<Self::Output, Error> {
        // println!("[STMT]: {:?}", s);

        match s {
            Stmt::Print(e) => {
                let v = self.visit_expr(e)?;
                println!("{}", value!(v));
                Ok(Object::Unit.into())
            }
            Stmt::Return(e) => {
                let value = if let Some(e) = e {
                    value!(self.visit_expr(e)?)
                } else {
                    Object::Unit
                };

                // println!("Returning: {:?}", value);

                Ok(Exec::Return(value))
            }
            Stmt::VarDecl(ident, init) => self.visit_var_decl(ident, init),
            Stmt::Block(decls) => self.visit_block(decls),
            Stmt::Expr(e) => self.visit_expr(e),
            Stmt::If(c, g, b) => self.visit_if(c, g, b),
            Stmt::While(pred, block) => self.visit_while(pred, block), // _ => Ok(Object::Unit),
            Stmt::Func(name, func) => self.visit_func(name, func.clone()),
        }
    }
}

fn exec_binop<T: BinaryOp + TryFrom<Object, Error = Error> + ToString>(
    lhs: Object,
    op: BinOp,
    rhs: Object,
) -> Result<Object, Error> {
    let (lhs, rhs) = (T::try_from(lhs)?, T::try_from(rhs)?);
    lhs.binop(op, &rhs)
}

fn exec_unop<T: UnaryOp + TryFrom<Object, Error = Error> + ToString>(
    op: UnOp,
    rhs: Object,
) -> Result<Object, Error> {
    let rhs = T::try_from(rhs)?;
    rhs.unop(op)
}


#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Exec {
    Return(Object),
    Value(Object),
    Continue,
    Break,
    None
}

impl Default for Exec {
    fn default() -> Self {
        Exec::None
    }
}

impl From<Object> for Exec {
    fn from(o: Object) -> Self {
        Exec::Value(o)
    }
}

// s