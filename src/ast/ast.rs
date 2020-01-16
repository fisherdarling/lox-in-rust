use derive_more::{Add, AsMut, AsRef, Deref, DerefMut, Display, From, Mul, Sub};

use pest::iterators::{Pair, Pairs};
use pest::prec_climber::{Assoc, Operator, PrecClimber};

use super::operator::{is_binop, BinOp, UnOp};
use crate::error::Error;
use crate::parser::Rule;
use crate::{impl_from, impl_try_from};

use std::fmt;

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

#[derive(Debug, Clone, PartialEq, PartialOrd, Display)]
pub enum Object {
    Int(isize),
    Float(f32),
    // #[display(fmt = "{}", "_0")]
    Str(String),
    Ident(Ident),
    Bool(bool),
    // #[display(fmt = "{:?}", "_0")]
    // Path(Path),
    #[display(fmt = "()")]
    Unit,
}

impl Default for Object {
    fn default() -> Self {
        Object::Unit
    }
}

impl Object {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::int => Object::Int(pair.as_str().parse().unwrap()),
            Rule::float => Object::Float(pair.as_str().parse().unwrap()),
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

    pub fn from_pair(pair: Pair<Rule>) -> Self {
        let climber = PrecClimber::new(create_operators());

        climber.climb(pair.into_inner(), Expr::primary, Expr::infix)
    }

    fn handle_term(pair: Pair<Rule>) -> Expr {
        let pairs: Vec<Pair<Rule>> = pair.into_inner().collect();

        match &pairs[..] {
            [unary, rhs] => {
                let op = match unary.as_rule() {
                    Rule::op_unary_not => UnOp::Not,
                    Rule::op_unary_minus => UnOp::Minus,
                    _ => unreachable!(),
                };

                let rhs = Expr::primary(rhs.clone());
                Expr::UnOp(op, Box::new(rhs))
            }
            [term] => Expr::primary(term.clone()),
            _ => todo!(),
        }
    }

    fn primary(pair: Pair<Rule>) -> Expr {
        match pair.as_rule() {
            Rule::float | Rule::int | Rule::string => Object::from_pair(pair).into(),
            Rule::ident => Object::Ident(Ident(pair.as_str().into())).into(),
            Rule::value => Expr::primary(pair.into_inner().next().unwrap()),
            Rule::term => Expr::handle_term(pair),
            Rule::rtrue => Expr::Object(Object::from(true)),
            Rule::rfalse => Expr::Object(Object::from(false)),
            Rule::expr => Expr::from_pair(pair),
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
                    .into_iter()
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
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        let inner_decls: Vec<Decl> = pair.into_inner().map(Decl::from_pair).collect();
        Self(inner_decls)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Stmt {
    Expr(Expr),
    Print(Expr),
    Block(Block),
    VarDecl(Ident, Option<Expr>),
    If(Expr, Block, Block),
    While(Expr, Block),
}

impl Stmt {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        // println!("[stmt] {:?}", pair.as_rule());

        let pair = pair.into_inner().next().unwrap();

        match pair.as_rule() {
            Rule::statement => Stmt::from_pair(pair.into_inner().next().unwrap()),
            Rule::expr_stmt => {
                // println!("[expr]");
                let inner_expr = pair.into_inner().next().unwrap();
                let expr = Expr::from_pair(inner_expr);
                Stmt::Expr(expr)
            }
            Rule::print_stmt => {
                // println!("[print]");
                let inner_expr = pair.into_inner().next().unwrap();
                let expr = Expr::from_pair(inner_expr);
                Stmt::Print(expr)
            }
            Rule::var_decl => {
                let pairs: Vec<Pair<Rule>> = pair.into_inner().collect();
                let ident = Ident(pairs[0].as_str().to_string());
                let initializer = pairs.last().map(|p| Expr::from_pair(p.clone()));

                Stmt::VarDecl(ident, initializer)
            }
            Rule::while_stmt => {
                let pairs: Vec<Pair<Rule>> = pair.into_inner().collect();
                let pred = Expr::from_pair(pairs[0].clone());
                let block = Block::from_pair(pairs[1].clone());

                Stmt::While(pred, block)
            }
            Rule::for_stmt => {
                let pairs: Vec<Pair<Rule>> = pair.into_inner().collect();
                let rules: Vec<Rule> = pairs.iter().map(|p| p.as_rule()).collect();

                let var_decl;
                let pred;
                let inc;

                match &rules[..] {
                    [Rule::var_decl, Rule::expr, Rule::semi, Rule::expr, Rule::block] => {
                        let var_pairs: Vec<Pair<Rule>> = pairs[0].clone().into_inner().collect();
                        let ident = Ident(var_pairs[0].as_str().to_string());
                        let initializer = var_pairs.last().map(|p| Expr::from_pair(p.clone()));

                        var_decl = Stmt::VarDecl(ident, initializer);
                        pred = Expr::from_pair(pairs[1].clone());
                        inc = Expr::from_pair(pairs[3].clone());
                    }
                    _ => todo!(),
                }

                let mut block = Block::from_pair(pairs.last().unwrap().clone());
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
                        let pred = Expr::from_pair(pred.clone());
                        let good = Block::from_pair(good.clone());
                        let bad = Block::from_pair(bad.clone());
                        Stmt::If(pred, good, bad)
                    }
                    [pred, good] => {
                        let pred = Expr::from_pair(pred.clone());
                        let good = Block::from_pair(good.clone());
                        let bad = Block::default();

                        Stmt::If(pred, good, bad)
                    }
                    _ => unreachable!(),
                }
            }
            Rule::block => {
                // let inner_decls: Vec<Decl> = pair.into_inner().map(Decl::from_pair).collect();
                Stmt::Block(Block::from_pair(pair))
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
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        // println!("[decl] {:?}", pair.as_rule());
        // assert_eq!(pair.as_rule(), Rule::declaration);

        let pair = pair.into_inner().next().unwrap();

        match pair.as_rule() {
            // Rule::declaration => {
            //     Decl::Stmt(Stmt::from_pair(pair.into_inner().next().unwrap()))
            // }
            Rule::statement => {
                // println!("[stmt]");
                let stmt = Stmt::from_pair(pair);
                Self::from(stmt)
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
                    let decl = Decl::from_pair(pair);
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
    isize < Object::Int,
    f32 < Object::Float,
    String < Object::Str,
    Ident < Object::Ident,
    bool < Object::Bool,
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
