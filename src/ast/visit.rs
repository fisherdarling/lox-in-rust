use std::rc::Rc;

use super::ast::*;
use super::function::LoxFn;

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

    fn visit_func_call(
        &mut self,
        _f: Func,
        _args: &mut [Object],
    ) -> Result<Self::Output, Error> {
        // println!("vfc");

        Ok(Self::Output::default())
    }

    fn visit_func(
        &mut self,
        name: &mut Ident,
        func: Func,
    ) -> Result<Self::Output, Error> {
        // println!("vf");
        walk_func(self, name, func)
    }

    fn visit_if(
        &mut self,
        check: &mut Expr,
        good: &mut Block,
        bad: &mut Block,
    ) -> Result<Self::Output, Error> {
        // println!("vi");
        walk_if(self, check, good, bad)
    }

    fn visit_expr(&mut self, e: &mut Expr) -> Result<Self::Output, Error> {
        // println!("ve");
        walk_expr(self, e)
    }

    fn visit_block(&mut self, block: &mut Block) -> Result<Self::Output, Error> {
        // println!("vb");
        walk_block(self, block)
    }

    fn visit_while(&mut self, pred: &mut Expr, block: &mut Block) -> Result<Self::Output, Error> {
        // println!("vw");
        walk_while(self, pred, block)
    }

    // fn finish_expr(&mut self, _e: &mut Expr, res: Result<Self::Output, Error>) -> Result<Self::Output, Error> {
    //     res
    // }
    // fn start_lit(&mut self, _l: &mut Object) -> Result<Self::Output, Error> {
    //     Ok(None)
    // }

    fn visit_obj(&mut self, _l: &mut Object) -> Result<Self::Output, Error> {
        // println!("vo");
        Ok(Self::Output::default())
    }

    // fn finish_lit(&mut self, _l: &mut Object, res: Result<Self::Output, Error>) -> Result<Self::Output, Error> {
    //     res
    // }

    // fn start_stmt(&mut self, _s: &mut Stmt) -> Result<Self::Output, Error> {
    //     Ok(None)
    // }

    fn visit_stmt(&mut self, s: &mut Stmt) -> Result<Self::Output, Error> {
        // println!("vs");
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
        _ident: &mut Ident,
        _init: &mut Option<Expr>,
    ) -> Result<Self::Output, Error> {
        // println!("vvd");
        Ok(Self::Output::default())
    }

    fn visit_decl(&mut self, d: &mut Decl) -> Result<Self::Output, Error> {
        // println!("vd");
        walk_decl(self, d)
    }

    // fn finish_decl(&mut self, _d: &mut Decl, res: Result<Self::Output, Error>) -> Result<Self::Output, Error> {
    //     res
    // }

    // fn start_program(&mut self, _p: &mut Program) -> Result<Self::Output, Error> {
    //     Ok(None)
    // }

    fn visit_program(&mut self, p: &mut Program) -> Result<Self::Output, Error> {
        // println!("vp");
        walk_program(self, p)
    }

    // fn finish_program(&mut self, _p: &mut Program, res: Result<Self::Output, Error>) -> Result<Self::Output, Error> {
    //     res
    // }
}

pub fn walk_expr<V: Visitor>(visitor: &mut V, expr: &mut Expr) -> Result<V::Output, Error> {
    // println!("we");
    visitor.visit_expr(expr)
}

pub fn walk_program<V: Visitor>(
    visitor: &mut V,
    program: &mut Program,
) -> Result<V::Output, Error> {
    let mut res = program
        .decls
        .iter_mut()
        .map(|d| visitor.visit_decl(d))
        .collect::<Vec<Result<V::Output, Error>>>();

    res.pop().unwrap()
}

pub fn walk_decl<V: Visitor>(visitor: &mut V, decl: &mut Decl) -> Result<V::Output, Error> {
    match decl {
        Decl::Stmt(s) => visitor.visit_stmt(s),
    }
}

pub fn walk_stmt<V: Visitor>(visitor: &mut V, stmt: &mut Stmt) -> Result<V::Output, Error> {
    // println!()
    
    match stmt {
        Stmt::Expr(e) | Stmt::Print(e) => visitor.visit_expr(e),
        Stmt::Block(decls) => visitor.visit_block(decls),
        Stmt::VarDecl(ident, init) => visitor.visit_var_decl(ident, init),
        Stmt::If(c, g, b) => visitor.visit_if(c, g, b),
        Stmt::While(pred, block) => visitor.visit_while(pred, block),
        Stmt::Func(name, func) => visitor.visit_func(name, func.clone()),
    }
}

pub fn walk_block<V: Visitor>(visitor: &mut V, block: &mut Block) -> Result<V::Output, Error> {
    match &mut block.0[..] {
        [first] => walk_decl(visitor, first),
        [first @ .., last] => {
            for decl in first {
                walk_decl(visitor, decl)?;
            }

            walk_decl(visitor, last)
        }
        [] => Ok(V::Output::default()),
    }
}

pub fn walk_if<V: Visitor>(
    visitor: &mut V,
    check: &mut Expr,
    good: &mut Block,
    bad: &mut Block,
) -> Result<V::Output, Error> {
    visitor.visit_block(good)?;
    visitor.visit_block(bad)?;
    visitor.visit_expr(check)
}

pub fn walk_while<V: Visitor>(
    visitor: &mut V,
    pred: &mut Expr,
    block: &mut Block,
) -> Result<V::Output, Error> {
    visitor.visit_expr(pred)?;
    visitor.visit_block(block)
}

pub fn walk_func<V: Visitor>(
    visitor: &mut V,
    name: &mut Ident,
    func: Func,
) -> Result<V::Output, Error> {
    Ok(V::Output::default())
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
