use super::ast::{Decl, Expr, Ident, Object, Program, Stmt};
use super::visit::Visitor;
use crate::error::Error;

pub struct Printer(pub usize);

impl Visitor for Printer {
    type Output = ();
    // fn start_expr(&mut self, _e: &mut Expr) -> Result<Self::Output, Error> {
    //     self.0 += 2;
    //     Ok(None)
    // }

    fn visit_expr(&mut self, e: &mut Expr) -> Result<Self::Output, Error> {
        self.0 += 2;

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
        println!("{}[objt]: {}", " ".repeat(self.0), e);
        Ok(())
    }

    // fn start_stmt(&mut self, _s: &mut Stmt) -> Result<Self::Output, Error> {
    //     self.0 += 2;
    //     Ok(None)
    // }

    fn visit_stmt(&mut self, e: &mut Stmt) -> Result<Self::Output, Error> {
        self.0 += 2;

        print!("{}[stmt]: ", " ".repeat(self.0));
        match e {
            Stmt::Expr(e) => {
                println!("expr");
                self.visit_expr(e)?;
            }
            Stmt::Print(e) => {
                println!("print");
                self.visit_expr(e)?;
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
        println!("{}[iden]: {}", " ".repeat(self.0), ident);

        if let Some(init) = init {
            self.visit_expr(init)?;
        }

        self.0 -= 2;

        Ok(())
    }

    fn visit_decl(&mut self, e: &mut Decl) -> Result<Self::Output, Error> {
        self.0 += 2;

        print!("{}[decl]: ", " ".repeat(self.0));
        match e {
            Decl::Stmt(s) => {
                println!("stmt");
                self.visit_stmt(s)?;
            }
            Decl::VarDecl(ident, init) => {
                println!("var");

                self.visit_var_decl(ident, init)?;
            }
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
