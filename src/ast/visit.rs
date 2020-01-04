use super::ast::*;

use crate::error::Error;

pub type VResult<T> = Result<Option<T>, Error>;

pub trait Visitor<T> {
    fn start_expr(&mut self, _e: &mut Expr) -> VResult<T> {
        Ok(None)
    }

    fn visit_expr(&mut self, _e: &mut Expr) -> VResult<T> {
        Ok(None)
    }

    fn finish_expr(&mut self, _e: &mut Expr, res: VResult<T>) -> VResult<T> {
        res
    }

    fn start_lit(&mut self, _l: &mut Object) -> VResult<T> {
        Ok(None)
    }

    fn visit_lit(&mut self, _l: &mut Object) -> VResult<T> {
        println!("Default Lit");
        Ok(None)
    }

    fn finish_lit(&mut self, _l: &mut Object, res: VResult<T>) -> VResult<T> {
        res
    }

    fn start_stmt(&mut self, _s: &mut Stmt) -> VResult<T> {
        Ok(None)
    }

    fn visit_stmt(&mut self, _s: &mut Stmt) -> VResult<T> {
        Ok(None)
    }

    fn finish_stmt(&mut self, _s: &mut Stmt, res: VResult<T>) -> VResult<T> {
        res
    }

    fn start_decl(&mut self, _d: &mut Decl) -> VResult<T> {
        Ok(None)
    }

    fn visit_decl(&mut self, _d: &mut Decl) -> VResult<T> {
        Ok(None)
    }

    fn finish_decl(&mut self, _d: &mut Decl, res: VResult<T>) -> VResult<T> {
        res
    }

    fn start_program(&mut self, _p: &mut Program) -> VResult<T> {
        Ok(None)
    }

    fn visit_program(&mut self, _p: &mut Program) -> VResult<T> {
        Ok(None)
    }

    fn finish_program(&mut self, _p: &mut Program, res: VResult<T>) -> VResult<T> {
        res
    }
}

pub trait Visitable {
    fn visit<T>(&mut self, v: &mut dyn Visitor<T>) -> VResult<T>;
}

impl<V: Visitable> Visitable for Vec<V> {
    fn visit<T>(&mut self, v: &mut dyn Visitor<T>) -> VResult<T> {
        for t in self {
            t.visit(v)?;
        }
        Ok(None)
    }
}

impl<V: Visitable> Visitable for Box<V> {
    fn visit<T>(&mut self, v: &mut dyn Visitor<T>) -> VResult<T> {
        self.as_mut().visit(v)
    }
}

impl Visitable for Object {
    fn visit<T>(&mut self, v: &mut dyn Visitor<T>) -> VResult<T> {
        v.start_lit(self)?;
        let res = v.visit_lit(self);
        v.finish_lit(self, res)
    }
}

impl Visitable for Expr {
    fn visit<T>(&mut self, v: &mut dyn Visitor<T>) -> VResult<T> {
        v.start_expr(self)?;
        let res = v.visit_expr(self);
        v.finish_expr(self, res)
    }
}

impl Visitable for Stmt {
    fn visit<T>(&mut self, v: &mut dyn Visitor<T>) -> VResult<T> {
        v.start_stmt(self)?;
        let res = v.visit_stmt(self);
        match self {
            Stmt::Expr(e) => e.visit(v)?,
            Stmt::Print(e) => e.visit(v)?,
        };
        v.finish_stmt(self, res)
    }
}

impl Visitable for Decl {
    fn visit<T>(&mut self, v: &mut dyn Visitor<T>) -> VResult<T> {
        v.start_decl(self)?;
        let res = v.visit_decl(self);
        match self {
            Decl::Stmt(s) => s.visit(v)?,
        };
        v.finish_decl(self, res)
    }
}

impl Visitable for Program {
    fn visit<T>(&mut self, v: &mut dyn Visitor<T>) -> VResult<T> {
        v.start_program(self)?;
        let res = v.visit_program(self);
        self.decls.visit(v)?;
        v.finish_program(self, res)
    }
}
