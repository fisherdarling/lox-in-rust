use derive_more::{Add, Display, Mul, Sub};

use pest::iterators::{Pair, Pairs};
use pest::prec_climber::{Assoc, Operator, PrecClimber};

use super::operator::{is_binop, BinOp};
use crate::impl_try_from;
use crate::parser::Rule;

use std::fmt;

macro_rules! impl_from {
    ($($ty:ty > $name:ident :: $inner:ident),+$(,)?) => {
        $(
            impl From<$ty> for $name {
                fn from(f: $ty) -> Self {
                    Self::$inner(f)
                }
            }
        )+
    };
}

#[derive(Clone, PartialEq, PartialOrd)]
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

#[derive(Debug, Clone, PartialEq, PartialOrd, Display)]
pub enum Object {
    Int(isize),
    Float(f32),
    Str(String),
    Ident(String),
    Bool(bool),
    #[display(fmt = "{:?}", "_0")]
    Path(Path),
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
            Rule::string => Object::Str(pair.as_str().into()),
            _ => todo!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Expr {
 Object(Object),
    // #[display(fmt = "+")]
    BinOp(Box<Expr>, BinOp, Box<Expr>),
    // #[display(fmt = "")]
    Call(Path, Vec<Expr>),
}

impl Expr {
    pub fn binop(lhs: Expr, op: BinOp, rhs: Expr) -> Self {
        Self::BinOp(Box::new(lhs), op, Box::new(rhs))
    }

    pub fn from_pair(pair: Pair<Rule>) -> Self {
        let climber = PrecClimber::new(create_operators());

        climber.climb(pair.into_inner(), Expr::primary, Expr::infix)
    }

    fn primary(pair: Pair<Rule>) -> Expr {
        match pair.as_rule() {
            Rule::float | Rule::int | Rule::string => Object::from_pair(pair).into(),
            Rule::ident => Object::Ident(pair.as_str().into()).into(),
            Rule::path => Object::from(Path::from_str(pair.as_str())).into(),
            Rule::value | Rule::term => Expr::primary(pair.into_inner().next().unwrap()),
            Rule::expr => Expr::from_pair(pair),
            Rule::call => {
                let pairs: Vec<Pair<Rule>> = pair.into_inner().collect();

                let path = Path::from_str(pairs.first().unwrap().as_str());
                let args: Vec<Expr> = pairs
                    .into_iter()
                    .skip(1)
                    .map(|p| Expr::from_pair(p))
                    .collect();

                Expr::Call(path, args)
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
            _ => todo!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Stmt {
    Expr(Expr),
    Print(Expr),
}

impl Stmt {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        let pair = pair.into_inner().next().unwrap();

        match pair.as_rule() {
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
        let pair = pair.into_inner().next().unwrap();

        match pair.as_rule() {
            Rule::statement => {
                // println!("[stmt]");
                let stmt = Stmt::from_pair(pair);
                Self::from(stmt)
            }
            _ => {
                println!("{:?}", pair.as_rule());
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
    Path > Object::Path,
    bool > Object::Bool,
    Object > Expr::Object,
    Expr > Stmt::Expr,
    Stmt > Decl::Stmt,
);

impl_try_from!(
    isize < Object::Int,
    f32 < Object::Float,
    String < Object::Str,
    Path < Object::Path,
    bool < Object::Bool,
);

pub fn create_operators() -> Vec<Operator<Rule>> {
    vec![
        Operator::new(Rule::op_or, Assoc::Left),
        Operator::new(Rule::op_and, Assoc::Left),
        Operator::new(Rule::op_equal, Assoc::Left) | Operator::new(Rule::op_not_equal, Assoc::Left),
        Operator::new(Rule::op_greater, Assoc::Left)
            | Operator::new(Rule::op_greater_equal, Assoc::Left)
            | Operator::new(Rule::op_lower, Assoc::Left)
            | Operator::new(Rule::op_lower_equal, Assoc::Left),
        Operator::new(Rule::op_plus, Assoc::Left) | Operator::new(Rule::op_minus, Assoc::Left),
        Operator::new(Rule::op_times, Assoc::Left) | Operator::new(Rule::op_divide, Assoc::Left),
    ]
}
