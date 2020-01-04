use super::ast::{Decl, Expr, Object, Program, Stmt};
use super::{visit::VResult, visit::Visitable, visit::Visitor};

pub struct Printer(pub usize);

impl Visitor<()> for Printer {
    fn start_expr(&mut self, _e: &mut Expr) -> VResult<()> {
        self.0 += 2;
        Ok(None)
    }

    fn visit_expr(&mut self, e: &mut Expr) -> VResult<()> {
        match e {
            Expr::Object(o) => {
                o.visit(self)?;
            }
            Expr::Call(p, a) => {
                println!("[call]: {:?}(", p);
                a.visit(self)?;
                println!(")");
            }
            Expr::BinOp(lhs, op, rhs) => {
                println!("{}[bnop]: {}", " ".repeat(self.0), op);
                lhs.visit(self)?;
                rhs.visit(self)?;
            }
            _ => (),
        };
        Ok(None)
    }

    fn finish_expr(&mut self, _e: &mut Expr, _r: VResult<()>) -> VResult<()> {
        self.0 -= 2;
        Ok(None)
    }

    fn visit_lit(&mut self, e: &mut Object) -> VResult<()> {
        println!("{}[ltrl]: {}", " ".repeat(self.0), e);
        Ok(None)
    }

    fn start_stmt(&mut self, _s: &mut Stmt) -> VResult<()> {
        self.0 += 2;
        Ok(None)
    }

    fn visit_stmt(&mut self, e: &mut Stmt) -> VResult<()> {
        print!("{}[stmt]: ", " ".repeat(self.0));
        match e {
            Stmt::Expr(_) => {
                println!("expr");
            }
            Stmt::Print(_) => {
                println!("print");
            }
        }

        Ok(None)
    }

    fn finish_stmt(&mut self, _s: &mut Stmt, _r: VResult<()>) -> VResult<()> {
        self.0 -= 2;
        Ok(None)
    }

    fn start_decl(&mut self, _d: &mut Decl) -> VResult<()> {
        self.0 += 2;
        Ok(None)
    }

    fn visit_decl(&mut self, e: &mut Decl) -> VResult<()> {
        print!("{}[decl]: ", " ".repeat(self.0));
        match e {
            Decl::Stmt(_) => {
                println!("stmt");
            }
        }

        Ok(None)
    }

    fn finish_decl(&mut self, _d: &mut Decl, _r: VResult<()>) -> VResult<()> {
        self.0 -= 2;
        Ok(None)
    }

    fn visit_program(&mut self, _e: &mut Program) -> VResult<()> {
        println!("[prgm]");
        Ok(None)
    }
}
