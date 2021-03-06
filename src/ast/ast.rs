use derive_more::{Add, AsMut, AsRef, Deref, DerefMut, Display, From, Mul, Sub};

use pest::iterators::{Pair, Pairs};
use pest::prec_climber::{Assoc, Operator, PrecClimber};


use super::operator::{is_binop, BinOp, UnOp};
use super::function::{LoxFn, BuiltinFn, UserFn};
use crate::error::Error;
use crate::parser::Rule;
use crate::{impl_from, impl_try_from};

use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;

pub type Func = Rc<RefCell<Box<dyn LoxFn>>>;

lazy_static::lazy_static! {
    static ref PREC_CLIMBER: PrecClimber<Rule> = PrecClimber::new(create_operators());
}

#[derive(Clone, PartialEq, PartialOrd, Hash)]
pub struct Path {
    pub items: Vec<String>,
}

impl fmt::Debug for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.items.join("."))
    }
}

impl Path {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn from_str(s: &str) -> Self {
        let items = s.split('.').map(Into::into).collect();
        Self { items }
    }
}

#[derive(
    Hash,
    Default,
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Display,
    AsRef,
    AsMut,
    Deref,
    DerefMut,
)]
pub struct Ident(pub String);

impl Ident {
    pub fn from_pair(pair: &Pair<Rule>) -> Self {
        Ident(pair.as_str().to_string())
    }
}

#[derive(Clone, PartialEq, PartialOrd, Display)]
pub enum Object {
    Int(isize),
    Float(f32),
    Str(String),
    Ident(Ident),
    Bool(bool),
    #[display(fmt = "<func {}>", "_0.borrow().name()")]
    Func(Func),
    #[display(fmt = "()")]
    Unit,
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Func(func) => write!(f, "<func {}({})>", func.borrow().name(), func.borrow().arity()),
            Object::Int(e) => write!(f, "{:?}", e),
            Object::Float(e) => write!(f, "{:?}", e),
            Object::Str(e) => write!(f, "{:?}", e),
            Object::Ident(e) => write!(f, "{:?}", e),
            Object::Bool(e) => write!(f, "{:?}", e),
            Object::Unit => write!(f, "()"),
        }
    }
}

impl Default for Object {
    fn default() -> Self {
        Object::Unit
    }
}

impl Object {
    pub fn from_pair(pair: &Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::int => Object::Int(pair.as_str().parse().unwrap()),
            Rule::float => Object::Float(pair.as_str().parse().unwrap()),
            Rule::ident => Object::Ident(Ident(pair.as_str().to_string())),
            Rule::string => Object::Str(pair.as_str()[1..pair.as_str().len() - 1].into()),
            _ => todo!(),
        }
    }

    pub fn is_truthy(&self) -> Result<bool, Error> {
        match self {
            Object::Bool(b) => Ok(*b),
            Object::Int(i) => Ok(*i > 0),
            Object::Float(f) => Ok(*f > 0.0),
            Object::Str(s) => Ok(!s.is_empty()),
            Object::Unit => Ok(false),
            _ => Err(Error::UnsupportedTruthiness("".into())),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Expr {
    Object(Object),
    UnOp(UnOp, Box<Expr>),
    BinOp(Box<Expr>, BinOp, Box<Expr>),
    Access(Box<Expr>, Box<Expr>),
    Assign(Box<Expr>, Box<Expr>),
    Call(Ident, Vec<Expr>),
}

impl Expr {
    pub fn binop(lhs: Expr, op: BinOp, rhs: Expr) -> Self {
        Self::BinOp(Box::new(lhs), op, Box::new(rhs))
    }

    pub fn from_pair(pair: &Pair<Rule>) -> Self {
        PREC_CLIMBER.climb(pair.clone().into_inner(), Expr::primary, Expr::infix)
    }

    fn handle_term(pair: &Pair<Rule>) -> Expr {
        let pairs: Vec<Pair<Rule>> = pair.clone().into_inner().collect();

        if pairs[0].as_rule() == Rule::call {
            let pairs: Vec<Pair<_>> = pairs[0].clone().into_inner().collect();
            let ident = Ident::from_pair(&pairs[0]);
            let args: Vec<Expr> = pairs.iter().skip(1).map(Expr::from_pair).collect();

            return Expr::Call(ident, args);
        }
        // println!("pairs: {:?}", pairs);
        
        match &pairs[..] {
            [unary @ .., rhs] => {
                unary.iter().fold(Expr::from_pair(rhs), |inner: Expr, op| {
                    match op.as_rule() {
                        Rule::op_unary_not => Expr::UnOp(UnOp::Not, Box::new(inner)),
                        Rule::op_unary_minus => Expr::UnOp(UnOp::Minus, Box::new(inner)),
                        _ => unreachable!(),
                    }
                })
            }
            [] => unreachable!(),
        }
    }

    fn primary(pair: Pair<Rule>) -> Expr {
        match pair.as_rule() {
            Rule::float | Rule::int | Rule::string => Object::from_pair(&pair).into(),
            Rule::ident => Object::Ident(Ident(pair.as_str().into())).into(),
            Rule::value => Expr::primary(pair.into_inner().next().unwrap()),
            Rule::term => Expr::handle_term(&pair),
            Rule::rtrue => Expr::Object(Object::from(true)),
            Rule::rfalse => Expr::Object(Object::from(false)),
            Rule::expr => Expr::from_pair(&pair),
            Rule::op_unary_not | Rule::op_unary_minus => {
                // let rhs = Expr::primary()
                println!("Not Pair: {:?}", pair);
                todo!()
            }
            Rule::call => {
                let pairs: Vec<Pair<Rule>> = pair.into_inner().collect();

                let ident = Ident(
                    pairs
                        .first()
                        .into_iter()
                        .next()
                        .unwrap()
                        .as_str()
                        .to_string(),
                );
                let args: Vec<Expr> = pairs
                    .iter()
                    .skip(1)
                    .map(|p| Expr::from_pair(p))
                    .collect();

                Expr::Call(ident, args)
            }
            _ => {
                println!("{:?}", pair.as_rule());
                todo!()
            }
        }
    }

    fn infix(lhs: Expr, op: Pair<Rule>, rhs: Expr) -> Expr {
        match op.as_rule() {
            o if is_binop(o) => Expr::binop(lhs, BinOp::from(o), rhs),
            Rule::op_dot => Expr::Access(Box::new(lhs), Box::new(rhs)),
            Rule::op_assign => Expr::Assign(Box::new(lhs), Box::new(rhs)),
            other => {
                println!("{:?}", other);
                todo!()
            }
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, PartialOrd, AsRef, AsMut, Deref, DerefMut)]
pub struct Block(pub Vec<Decl>);

impl Block {
    pub fn from_pair(pair: &Pair<Rule>) -> Self {
        let inner_decls: Vec<Decl> = pair.clone().into_inner().map(|p| Decl::from_pair(&p)).collect();
        Self(inner_decls)
    }
}

#[derive(Clone, PartialEq, PartialOrd)]
pub enum Stmt {
    Expr(Expr),
    Print(Expr),
    Block(Block),
    VarDecl(Ident, Option<Expr>),
    If(Expr, Block, Block),
    While(Expr, Block),
    Func(Ident, Func),
    Return(Option<Expr>),
}

impl fmt::Debug for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Stmt::Func(_, func) => write!(f, "<func {}({})>", func.borrow().name(), func.borrow().arity()),
            Stmt::Expr(e) | Stmt::Print(e) => write!(f, "{:?}", e),
            Stmt::Block(b) => write!(f, "{:?}", b),
            Stmt::VarDecl(i, e) => write!(f, "{:?} = {:?}", i, e),
            Stmt::If(c, g, b) => write!(f, "[if] {:?} {{ {:?} }} else {{ {:?} }}", c, g, b),
            Stmt::While(e, b) => write!(f, "[while] {:?} {{ {:?} }}", e, b),
            Stmt::Return(e) => write!(f, "[return] {:?}", e),
        }
    }
}

impl Stmt {
    pub fn from_pair(pair: &Pair<Rule>) -> Self {
        // println!("[stmt] {:?}", pair.as_rule());

        let pair = pair.clone().into_inner().next().unwrap();

        match pair.as_rule() {
            Rule::statement => Stmt::from_pair(pair.clone().into_inner().next().as_ref().unwrap()),
            Rule::expr_stmt => {
                // println!("[expr]");
                let inner_expr = pair.clone().into_inner().next().unwrap();
                let expr = Expr::from_pair(&inner_expr);
                Stmt::Expr(expr)
            }
            Rule::print_stmt => {
                // println!("[print]");
                let inner_expr = pair.clone().into_inner().next().unwrap();
                let expr = Expr::from_pair(&inner_expr);
                Stmt::Print(expr)
            }
            Rule::var_decl => {
                let pairs: Vec<Pair<Rule>> = pair.clone().into_inner().collect();
                let ident = Ident(pairs[0].as_str().to_string());
                let initializer = pairs.last().map(|p| Expr::from_pair(p));

                Stmt::VarDecl(ident, initializer)
            }
            Rule::while_stmt => {
                let pairs: Vec<Pair<Rule>> = pair.clone().into_inner().collect();
                let pred = Expr::from_pair(&pairs[0]);
                let block = Block::from_pair(&pairs[1]);

                Stmt::While(pred, block)
            }
            Rule::for_stmt => {
                let pairs: Vec<Pair<Rule>> = pair.clone().into_inner().collect();
                let rules: Vec<Rule> = pairs.iter().map(|p| p.as_rule()).collect();

                let var_decl;
                let pred;
                let inc;

                match &rules[..] {
                    [Rule::var_decl, Rule::expr, Rule::semi, Rule::expr, Rule::block] => {
                        let var_pairs: Vec<Pair<Rule>> = pairs[0].clone().into_inner().collect();
                        let ident = Ident(var_pairs[0].as_str().to_string());
                        let initializer = var_pairs.last().map(Expr::from_pair);

                        var_decl = Stmt::VarDecl(ident, initializer);
                        pred = Expr::from_pair(&pairs[1]);
                        inc = Expr::from_pair(&pairs[3]);
                    }
                    _ => todo!(),
                }

                let mut block = Block::from_pair(pairs.last().unwrap());
                block.0.push(Decl::Stmt(Stmt::Expr(inc)));

                let while_stmt = Stmt::While(pred, block);

                let mut desugared = Block::default();
                desugared.0.push(Decl::Stmt(var_decl));
                desugared.0.push(Decl::Stmt(while_stmt));

                Stmt::Block(desugared)
            }
            Rule::if_stmt => {
                let pairs: Vec<Pair<Rule>> = pair.into_inner().collect();

                match &pairs[..] {
                    [pred, good, bad] => {
                        let pred = Expr::from_pair(pred);
                        let good = Block::from_pair(good);
                        let bad = Block::from_pair(bad);
                        Stmt::If(pred, good, bad)
                    }
                    [pred, good] => {
                        let pred = Expr::from_pair(pred);
                        let good = Block::from_pair(good);
                        let bad = Block::default();

                        Stmt::If(pred, good, bad)
                    }
                    _ => unreachable!(),
                }
            }
            Rule::return_stmt => {
                let expr = pair.into_inner().next().map(|e| Expr::from_pair(&e));
                Stmt::Return(expr)
            }
            Rule::block => {
                // let inner_decls: Vec<Decl> = pair.into_inner().map(Decl::from_pair).collect();
                Stmt::Block(Block::from_pair(&pair))
            }
            _ => {
                println!("{:?}", pair.as_rule());
                todo!()
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Decl {
    Stmt(Stmt),
}

impl Decl {
    pub fn from_pair(pair: &Pair<Rule>) -> Self {
        let pair = pair.clone().into_inner().next().unwrap();

        match pair.as_rule() {
            Rule::statement => {
                let stmt = Stmt::from_pair(&pair);
                Self::from(stmt)
            }
            Rule::fun_decl => {
                let pairs: Vec<Pair<Rule>> = pair.into_inner().next().unwrap().into_inner().collect();

                let func_name: Ident = Ident::from_pair(&pairs[0]);
                let parameters: Vec<Ident> = if pairs.len() == 3 {
                    pairs[1].clone().into_inner().map(|p| Ident::from_pair(&p)).collect()
                } else {
                    vec![]
                };

                let body = if pairs.len() == 3 { 
                    Block::from_pair(&pairs[2])
                } else {
                    Block::from_pair(&pairs[1])    
                };

                let user_fn = UserFn::new(func_name.clone(), parameters, Default::default(), body);
                Decl::Stmt(Stmt::Func(func_name, Rc::new(RefCell::new(Box::new(user_fn)))))
            }
            _ => {
                println!("{:?}, {:#?}", pair.as_rule(), pair);
                todo!()
            }
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct Program {
    pub decls: Vec<Decl>,
}

impl Program {
    pub fn from_pairs(mut pairs: Pairs<Rule>) -> Self {
        // println!("Parsing program");
        // println!("{:#?}", pairs);
        let mut program = Program::default();
        let root = pairs.next().unwrap();

        // let root_id = ast.push(Node::root());

        for pair in root.into_inner() {
            match pair.as_rule() {
                Rule::declaration => {
                    // println!("[decl]");
                    let decl = Decl::from_pair(&pair);
                    program.decls.push(decl)
                }
                Rule::EOI => (),
                _ => {
                    println!("{:?}", pair.as_rule());
                    todo!()
                }
            }
        }

        program
    }

    // pub fn pretty_print
}

impl_from!(
    isize > Object::Int,
    f32 > Object::Float,
    String > Object::Str,
    Ident > Object::Ident,
    // Path > Object::Path,
    bool > Object::Bool,
    Object > Expr::Object,
    Expr > Stmt::Expr,
    Stmt > Decl::Stmt,
);

impl_try_from!(
    isize as Object::Int,
    f32 as Object::Float,
    String as Object::Str,
    Ident as Object::Ident,
    bool as Object::Bool,
    Func as Object::Func,
);

pub fn create_operators() -> Vec<Operator<Rule>> {
    vec![
        Operator::new(Rule::op_assign, Assoc::Left),
        Operator::new(Rule::op_or, Assoc::Left),
        Operator::new(Rule::op_and, Assoc::Left),
        Operator::new(Rule::op_equal, Assoc::Left) | Operator::new(Rule::op_not_equal, Assoc::Left),
        Operator::new(Rule::op_greater, Assoc::Left)
            | Operator::new(Rule::op_greater_equal, Assoc::Left)
            | Operator::new(Rule::op_lower, Assoc::Left)
            | Operator::new(Rule::op_lower_equal, Assoc::Left),
        Operator::new(Rule::op_plus, Assoc::Left) | Operator::new(Rule::op_minus, Assoc::Left),
        Operator::new(Rule::op_times, Assoc::Left) | Operator::new(Rule::op_divide, Assoc::Left),
        Operator::new(Rule::op_dot, Assoc::Left),
        // Operator::new(Rule::op_unary_not, Assoc::Left)
    ]
}
