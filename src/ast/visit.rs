use super::ast::*;

use crate::error::Error;

// pub type Result<Self::Output, Error> = Result<, Error>;

pub trait Visitor
where
    Self: Sized,
    Self::Output: Default,
{
    type Output;
    // Result<Self::Output, Error>;
    // fn start_expr(&mut self, _e: &mut Expr) -> Result<Self::Output, Error> {
    //     Ok(None)
    // }

    fn visit_expr(&mut self, e: &mut Expr) -> Result<Self::Output, Error> {
        walk_expr(self, e)
    }

    // fn finish_expr(&mut self, _e: &mut Expr, res: Result<Self::Output, Error>) -> Result<Self::Output, Error> {
    //     res
    // }
    // fn start_lit(&mut self, _l: &mut Object) -> Result<Self::Output, Error> {
    //     Ok(None)
    // }

    fn visit_obj(&mut self, _l: &mut Object) -> Result<Self::Output, Error> {
        Ok(Self::Output::default())
    }

    // fn finish_lit(&mut self, _l: &mut Object, res: Result<Self::Output, Error>) -> Result<Self::Output, Error> {
    //     res
    // }

    // fn start_stmt(&mut self, _s: &mut Stmt) -> Result<Self::Output, Error> {
    //     Ok(None)
    // }

    fn visit_stmt(&mut self, s: &mut Stmt) -> Result<Self::Output, Error> {
        walk_stmt(self, s)
    }

    // fn finish_stmt(&mut self, _s: &mut Stmt, res: Result<Self::Output, Error>) -> Result<Self::Output, Error> {
    //     res
    // }

    // fn start_decl(&mut self, _d: &mut Decl) -> Result<Self::Output, Error> {
    //     Ok(None)
    // }

    fn visit_var_decl(
        &mut self,
        ident: &mut Ident,
        init: &mut Option<Expr>,
    ) -> Result<Self::Output, Error> {
        Ok(Self::Output::default())
    }

    fn visit_decl(&mut self, d: &mut Decl) -> Result<Self::Output, Error> {
        walk_decl(self, d)
    }

    // fn finish_decl(&mut self, _d: &mut Decl, res: Result<Self::Output, Error>) -> Result<Self::Output, Error> {
    //     res
    // }

    // fn start_program(&mut self, _p: &mut Program) -> Result<Self::Output, Error> {
    //     Ok(None)
    // }

    fn visit_program(&mut self, p: &mut Program) -> Result<Self::Output, Error> {
        walk_program(self, p)
    }

    // fn finish_program(&mut self, _p: &mut Program, res: Result<Self::Output, Error>) -> Result<Self::Output, Error> {
    //     res
    // }
}

fn walk_expr<V: Visitor>(visitor: &mut V, expr: &mut Expr) -> Result<V::Output, Error> {
    visitor.visit_expr(expr)
}

fn walk_program<V: Visitor>(visitor: &mut V, program: &mut Program) -> Result<V::Output, Error> {
    let mut res = program
        .decls
        .iter_mut()
        .map(|d| visitor.visit_decl(d))
        .collect::<Vec<Result<V::Output, Error>>>();

    res.pop().unwrap()
}

fn walk_decl<V: Visitor>(visitor: &mut V, decl: &mut Decl) -> Result<V::Output, Error> {
    match decl {
        Decl::Stmt(s) => visitor.visit_stmt(s),
        Decl::VarDecl(ident, init) => visitor.visit_var_decl(ident, init),
    }
}

fn walk_stmt<V: Visitor>(visitor: &mut V, stmt: &mut Stmt) -> Result<V::Output, Error> {
    match stmt {
        Stmt::Expr(e) | Stmt::Print(e) => visitor.visit_expr(e),
    }
}

// pub trait Visitable {
//     fn visit<T>(&mut self, v: &mut dyn Visitor<T>) -> Result<Self::Output, Error>;
// }

// impl<V: Visitable> Visitable for Vec<V> {
//     fn visit<T>(&mut self, v: &mut dyn Visitor<T>) -> Result<Self::Output, Error> {
//         for t in self {
//             t.visit(v)?;
//         }
//         Ok(None)
//     }
// }

// impl<V: Visitable> Visitable for Box<V> {
//     fn visit<T>(&mut self, v: &mut dyn Visitor<T>) -> Result<Self::Output, Error> {
//         self.as_mut().visit(v)
//     }
// }

// impl Visitable for Object {
//     fn visit<T>(&mut self, v: &mut dyn Visitor<T>) -> Result<Self::Output, Error> {
//         v.start_lit(self)?;
//         let res = v.visit_obj(self);
//         v.finish_lit(self, res)
//     }
// }

// impl Visitable for Expr {
//     fn visit<T>(&mut self, v: &mut dyn Visitor<T>) -> Result<Self::Output, Error> {
//         v.start_expr(self)?;
//         let res = v.visit_expr(self);
//         v.finish_expr(self, res)
//     }
// }

// impl Visitable for Stmt {
//     fn visit<T>(&mut self, v: &mut dyn Visitor<T>) -> Result<Self::Output, Error> {
//         v.start_stmt(self)?;
//         let res = v.visit_stmt(self);
//         match self {
//             Stmt::Expr(e) => e.visit(v)?,
//             Stmt::Print(e) => e.visit(v)?,
//         };
//         v.finish_stmt(self, res)
//     }
// }

// impl Visitable for Decl {
//     fn visit<T>(&mut self, v: &mut dyn Visitor<T>) -> Result<Self::Output, Error> {
//         v.start_decl(self)?;
//         let res = v.visit_decl(self);
//         match self {
//             Decl::Stmt(s) => s.visit(v)?,
//         };
//         v.finish_decl(self, res)
//     }
// }

// impl Visitable for Program {
//     fn visit<T>(&mut self, v: &mut dyn Visitor<T>) -> Result<Self::Output, Error> {
//         v.start_program(self)?;
//         let res = v.visit_program(self);
//         self.decls.visit(v)?;
//         v.finish_program(self, res)
//     }
// }
