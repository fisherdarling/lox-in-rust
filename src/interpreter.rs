use std::convert::TryFrom;

use crate::ast::visit::*;
use crate::ast::{
    operator::{BinOp, BinaryOp},
    Decl, Expr, Object, Program, Stmt,
};

use crate::error::Error;


macro_rules! same_type {
    ($lhs:ident $rhs:ident) => {
        match (&$lhs, &$rhs) {
            (Lit::Int(_), Lit::Int(_)) => Ok(()),
            (Lit::Float(_), Lit::Float(_)) => Ok(()),
            (Lit::Str(_), Lit::Str(_)) => Ok(()),
            (Lit::Ident(_), Lit::Ident(_)) => Ok(()),
            (Lit::Path(_), Lit::Path(_)) => Ok(()),
            (Lit::Bool(_), Lit::Bool(_)) => Ok(()),
            (Lit::Unit, Lit::Unit) => Ok(()),
            _ => Err(Error::TypeMismatch($lhs.clone(), $rhs.clone())),
        }
    };
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Interpreter;

impl Interpreter {
    pub fn eval_expr(&self, expr: &Expr) -> Result<Object, Error> {
        match expr {
            Expr::Call(_p, _a) => panic!(),
            Expr::Object(l) => Ok(l.clone()),
            Expr::BinOp(lhs, op, rhs) => {
                let lhs = self.eval_expr(lhs.as_ref())?;
                let rhs = self.eval_expr(rhs.as_ref())?;

                match lhs {
                    Object::Int(_) => exec_op::<isize>(lhs, op.clone(), rhs),
                    Object::Float(_) => exec_op::<f32>(lhs, op.clone(), rhs),
                    Object::Bool(_) => exec_op::<bool>(lhs, op.clone(), rhs),
                    _ => Err(Error::InvalidOperator(
                        lhs.to_string(),
                        op.clone(),
                        rhs.to_string(),
                    )),
                }

                // exec_op(lhs, op.clone(), rhs)
            }
        }
    }
}

impl Visitor<Object> for Interpreter {
    fn visit_expr(&mut self, e: &mut Expr) -> VResult<Object> {
        let res = match e {
            Expr::Call(_p, _a) => panic!(),
            Expr::Object(l) => Ok(Some(l.clone())),
            Expr::BinOp(lhs, op, rhs) => {
                let lhs = lhs.visit(self)?.ok_or(Error::ExpectedValue)?;
                let rhs = rhs.visit(self)?.ok_or(Error::ExpectedValue)?;

                match lhs {
                    Object::Int(_) => exec_op::<isize>(lhs, op.clone(), rhs),
                    Object::Float(_) => exec_op::<f32>(lhs, op.clone(), rhs),
                    Object::Bool(_) => exec_op::<bool>(lhs, op.clone(), rhs),
                    _ => Err(Error::InvalidOperator(
                        lhs.to_string(),
                        op.clone(),
                        rhs.to_string(),
                    )),
                }
                .map(|l| Some(l))
            }
        };

        res
    }

    fn visit_stmt(&mut self, s: &mut Stmt) -> VResult<Object> {
        match s {
            Stmt::Print(e) => {
                let v = e.visit(self)?.unwrap_or_default();
                println!("{}", v);
                Ok(None)
            }
            _ => Ok(None),
        }
    }
}

fn exec_op<T: BinaryOp + TryFrom<Object, Error = Error> + ToString>(
    lhs: Object,
    op: BinOp,
    rhs: Object,
) -> Result<Object, Error> {
    let (lhs, rhs) = (T::try_from(lhs)?, T::try_from(rhs)?);
    lhs.binop(op, &rhs)
}
