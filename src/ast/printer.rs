use super::ast::{Decl, Expr, Ident, Object, Program, Stmt};
use super::function::{LoxFn, BuiltinFn, UserFn};
use super::visit::Visitor;
use crate::error::Error;

use std::borrow::BorrowMut;

use downcast_rs::Downcast;

pub struct Printer(pub usize);

impl Visitor for Printer {
    type Output = ();
    // fn start_expr(&mut self, _e: &mut Expr) -> Result<Self::Output, Error> {
    //     self.0 += 2;
    //     Ok(None)
    // }

    fn visit_expr(&mut self, e: &mut Expr) -> Result<Self::Output, Error> {
        self.0 += 2;

        // println!("exprexpr");

        match e {
            Expr::Assign(lhs, rhs) => {
                println!("{}[asgn]", " ".repeat(self.0));
                self.visit_expr(lhs)?;
                self.visit_expr(rhs)?;
            }
            Expr::Access(lhs, rhs) => {
                println!("{}[accs]", " ".repeat(self.0));
                self.visit_expr(lhs)?;
                // println!("{}[.]", " ".repeat(self.0));
                self.visit_expr(rhs)?;
            }
            Expr::UnOp(op, rhs) => {
                println!("{}[unop] {}", " ".repeat(self.0), op);
                self.visit_expr(rhs)?;
            }
            Expr::BinOp(lhs, op, rhs) => {
                println!("{}[bnop] {}", " ".repeat(self.0), op);
                self.visit_expr(lhs)?;
                self.visit_expr(rhs)?;
            }
            Expr::Call(ident, args) => {
                print!("{}[call] {} (", " ".repeat(self.0), ident);
                if args.is_empty() {
                    println!(")");
                } else {
                    println!();
                }

                for arg in args {
                    self.visit_expr(arg)?;
                }
            }
            Expr::Object(o) => {
                self.visit_obj(o)?;
            }
        }

        self.0 -= 2;
        Ok(())
    }

    fn visit_obj(&mut self, e: &mut Object) -> Result<Self::Output, Error> {
        // println!("objobj");

        println!("{}[objt]: {:?}", " ".repeat(self.0), e);
        Ok(())
    }

    // fn start_stmt(&mut self, _s: &mut Stmt) -> Result<Self::Output, Error> {
    //     self.0 += 2;
    //     Ok(None)
    // }

    fn visit_stmt(&mut self, e: &mut Stmt) -> Result<Self::Output, Error> {
        self.0 += 2;

        // println!("{:?}", e);
        // println!("stmtmtmt");

        print!("{}[stmt]: ", " ".repeat(self.0));
        match e {
            Stmt::Return(e) => {
                println!("return");
                if let Some(e) = e {
                    self.visit_expr(e)?;
                }
            }
            Stmt::Expr(e) => {
                println!("expr");
                self.visit_expr(e)?;
            }
            Stmt::Print(e) => {
                println!("print");
                self.visit_expr(e)?;
            }
            Stmt::VarDecl(ident, init) => {
                println!("var");
                println!("{}[idnt]: {}", " ".repeat(self.0 + 2), ident);

                if let Some(init) = init {
                    self.visit_expr(init)?;
                }
            }
            Stmt::If(check, good, bad) => {
                println!("if");
                self.visit_expr(check)?;
                self.visit_block(good)?;
                self.visit_block(bad)?;
            }
            Stmt::While(pred, block) => {
                println!("while");
                self.visit_expr(pred)?;
                self.visit_block(block)?;
            }
            Stmt::Block(s) => {
                println!("block");
                for decl in &mut s.0 {
                    self.visit_decl(decl)?;
                }
            }
            Stmt::Func(name, func) => {
                // let func = func.clone();

                if let Some(builtin) = func.clone().borrow().downcast_ref::<BuiltinFn>() {
                    println!("<fn builtin {} ({})>", builtin.name(), builtin.arity());
                } else if let Some(user) = (*func.borrow()).downcast_ref::<UserFn>() {
                    let mut body = user.body.clone();
                    println!("<fn {} ({})>", user.name(), user.arity());
                    self.visit_block(&mut body)?;
                } 
            }
        }

        self.0 -= 2;
        Ok(())
    }

    // fn start_decl(&mut self, _d: &mut Decl) -> Result<Self::Output, Error> {
    //     self.0 += 2;
    //     Ok(None)
    // }

    fn visit_var_decl(
        &mut self,
        ident: &mut Ident,
        init: &mut Option<Expr>,
    ) -> Result<Self::Output, Error> {
        self.0 += 2;

        // println!("vardecl");

        println!("{}[iden]: {}", " ".repeat(self.0), ident);

        if let Some(init) = init {
            self.visit_expr(init)?;
        }

        self.0 -= 2;

        Ok(())
    }

    fn visit_decl(&mut self, e: &mut Decl) -> Result<Self::Output, Error> {
        self.0 += 2;
    
        // println!("decldecl");


        print!("{}[decl]: ", " ".repeat(self.0));
        match e {
            Decl::Stmt(s) => {
                println!("stmt");
                self.visit_stmt(s)?;
            } // Decl::VarDecl(ident, init) => {
              //     println!("var");

              //     self.visit_var_decl(ident, init)?;
              // }
        }

        self.0 -= 2;
        Ok(())
    }

    fn visit_program(&mut self, p: &mut Program) -> Result<Self::Output, Error> {
        println!("[prgm]");

        for decl in p.decls.iter_mut() {
            self.visit_decl(decl)?;
        }

        Ok(())
    }
}
