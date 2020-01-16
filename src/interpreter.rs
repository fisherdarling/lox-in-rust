use std::convert::TryFrom;

use crate::ast::visit::*;
use crate::ast::function::LoxFn;
use crate::ast::{
    operator::{BinOp, BinaryOp, UnOp, UnaryOp},
    Block, Decl, Expr, Ident, Object, Program, Stmt,
};
use crate::env::Environment;

use crate::error::Error;

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
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
        }
    }

    pub fn define_global(&mut self, name: Ident, value: Object) {
        
    }

    pub fn push_scope(&mut self) {
        self.env.push_scope();
    }

    pub fn pop_scope(&mut self) {
        self.env.push_scope();
    }

    pub fn define(&mut self, name: Ident, value: Object) {
        self.env.define(name, value);
    }
}

impl Visitor for Interpreter {
    type Output = Object;

    fn visit_func_call(&mut self, f: &mut Box<dyn LoxFn>, args: &mut [Object]) -> Result<Self::Output, Error> {
       f.call(self, args)
    } 

    fn visit_expr(&mut self, e: &mut Expr) -> Result<Self::Output, Error> {
        // println!("[ENV] {:#?}", self.env);

        match e {
            Expr::Assign(lhs, rhs) => {
                let rhs = self.visit_expr(rhs)?;

                if let box Expr::Object(Object::Ident(ident)) = lhs {
                    self.env.set(&ident, rhs)
                } else {
                    Err(Error::UnsupportedOperation(
                        "Currently only identifiers can be newly assigned.".to_string(),
                    ))
                }
            }
            Expr::Access(_, _) => panic!(),
            Expr::Call(_p, _a) => panic!(),
            Expr::Object(l) => {
                if let Object::Ident(ident) = l {
                    self.env.get(&ident)
                } else {
                    Ok(l.clone())
                }
            }
            Expr::UnOp(op, rhs) => {
                let rhs = self.visit_expr(rhs)?;
                match rhs {
                    Object::Int(_) => exec_unop::<isize>(op.clone(), rhs),
                    Object::Float(_) => exec_unop::<f32>(op.clone(), rhs),
                    Object::Bool(_) => exec_unop::<bool>(op.clone(), rhs),
                    _ => Err(Error::InvalidUnaryOperator(op.clone(), rhs.to_string())),
                }
            }
            Expr::BinOp(lhs, op, rhs) => {
                let lhs = self.visit_expr(lhs)?;

                // Short circuiting logic
                match op {
                    BinOp::And => {
                        if !lhs.is_truthy()? {
                            return Ok(false.into())
                        }
                    }
                    BinOp::Or => {
                        if lhs.is_truthy()? {
                            return Ok(true.into())
                        }
                    }
                    _ => (),
                };

                let rhs = self.visit_expr(rhs)?;

                match lhs {
                    Object::Int(_) => exec_binop::<isize>(lhs, op.clone(), rhs),
                    Object::Float(_) => exec_binop::<f32>(lhs, op.clone(), rhs),
                    Object::Bool(_) => exec_binop::<bool>(lhs, op.clone(), rhs),
                    _ => Err(Error::InvalidBinaryOperator(
                        lhs.to_string(),
                        op.clone(),
                        rhs.to_string(),
                    )),
                }
            }
        }
    }

    fn visit_if(&mut self, check: &mut Expr, good: &mut Block, bad: &mut Block) -> Result<Self::Output, Error> {
        if self.visit_expr(check)?.is_truthy()? {
            self.visit_block(good)
        } else {
            self.visit_block(bad)
        }
    }

    fn visit_block(&mut self, block: &mut Block) -> Result<Self::Output, Error> {
        // println!("[BLOCK] {:?}", decls);

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
        
        while self.visit_expr(pred)?.is_truthy()? {
            last = self.visit_block(block)?;
        }

        Ok(last)
    }

    fn visit_var_decl(
        &mut self,
        ident: &mut Ident,
        init: &mut Option<Expr>,
    ) -> Result<Self::Output, Error> {
        // println!("[VAR DECL] {} {:?}", ident, init);

        let value: Object = if let Some(init) = init {
            self.visit_expr(init)?
        } else {
            Object::Unit
        };

        self.env.define(ident.clone(), value);
        Ok(Object::Unit)
    }

    fn visit_stmt(&mut self, s: &mut Stmt) -> Result<Self::Output, Error> {
        // println!("[STMT]: {:?}", s);

        match s {
            Stmt::Print(e) => {
                let v = self.visit_expr(e)?;
                println!("{}", v);
                Ok(Object::Unit)
            }
            Stmt::VarDecl(ident, init) => self.visit_var_decl(ident, init),
            Stmt::Block(decls) => self.visit_block(decls),
            Stmt::Expr(e) => self.visit_expr(e),
            Stmt::If(c, g, b) => {
                self.visit_if(c, g, b)
            }
            Stmt::While(pred, block) => {
                self.visit_while(pred, block)
            }
            // _ => Ok(Object::Unit),
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
