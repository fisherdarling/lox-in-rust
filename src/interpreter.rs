use std::convert::TryFrom;

use crate::ast::visit::*;
use crate::ast::{
    operator::{BinOp, BinaryOp, UnOp, UnaryOp},
    Decl, Expr, Ident, Object, Program, Stmt,
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
    // pub fn eval_expr(&self, expr: &Expr) -> Result<Object, Error> {
    //     match expr {
    //         Expr::Call(_p, _a) => panic!(),
    //         Expr::Object(l) => Ok(l.clone()),
    //         Expr::BinOp(lhs, op, rhs) => {
    //             let lhs = self.eval_expr(lhs.as_ref())?;
    //             let rhs = self.eval_expr(rhs.as_ref())?;

    //             match lhs {
    //                 Object::Int(_) => exec_binop::<isize>(lhs, op.clone(), rhs),
    //                 Object::Float(_) => exec_binop::<f32>(lhs, op.clone(), rhs),
    //                 Object::Bool(_) => exec_binop::<bool>(lhs, op.clone(), rhs),
    //                 _ => Err(Error::InvalidBinaryOperator(
    //                     lhs.to_string(),
    //                     op.clone(),
    //                     rhs.to_string(),
    //                 )),
    //             }

    //             // exec_op(lhs, op.clone(), rhs)
    //         }
    //     }
    // }
}

impl Visitor for Interpreter {
    type Output = Object;

    fn visit_expr(&mut self, e: &mut Expr) -> Result<Self::Output, Error> {
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

    fn visit_var_decl(
        &mut self,
        ident: &mut Ident,
        init: &mut Option<Expr>,
    ) -> Result<Self::Output, Error> {
        let value: Object = if let Some(init) = init {
            self.visit_expr(init)?
        } else {
            Object::Unit
        };

        self.env.define(ident.clone(), value);
        Ok(Object::Unit)
    }

    fn visit_stmt(&mut self, s: &mut Stmt) -> Result<Self::Output, Error> {
        match s {
            Stmt::Print(e) => {
                let v = self.visit_expr(e)?;
                println!("{}", v);
                Ok(Object::Unit)
            }
            Stmt::Expr(e) => self.visit_expr(e),
            _ => Ok(Object::Unit),
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
