use derive_more::{AsMut, AsRef, Deref, DerefMut, From, Index, IndexMut, Into, Add, AddAssign};

use crate::parser::Rule;
use pest::iterators::{Pair, Pairs};
use pest::prec_climber::{Operator, PrecClimber};
use std::cell::RefCell;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Op {
    Plus,
    Minus,
    Times,
    Divide,
    // Mod,
    Gt,
    Ge,
    Lt,
    Le,
    Ne,
    Eq,
    Not,
    And,
    Or,
    Dot,
}

impl From<Rule> for Op {
    fn from(rule: Rule) -> Self {
        match rule {
            Rule::op_plus => Op::Plus,
            Rule::op_minus => Op::Minus,
            Rule::op_times => Op::Times,
            Rule::op_divide => Op::Divide,
            // Rule::op_md => Op::Mod,
            // Rule::op_plus => Op::Plus,
            // Rule::op_plus => Op::Plus,
            _ => todo!(),
        }
    }

}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum NodeType {
    Root,
    Assign,
    Path(String),
    Binary(Op),
    Unary(Op),
    Call,
    Ident(String),
    Literal,
    Integer(isize),
    Float(f64),
    Variable,
    Declaration,
    Statement,
    Expression,
    Arguments,
}

use NodeType::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Node {
    pub ty: NodeType,
    pub id: NodeId,
    pub children: Vec<NodeId>,
}

impl Node {
    pub fn new(ty: NodeType) -> Self {
        Node {
            ty: ty,
            id: 0.into(),
            children: Vec::new(),
        }
    }

    pub fn root() -> Self {
        Self {
            ty: NodeType::Root,
            id: 0.into(),
            children: Vec::new(),
        }
    }

    pub fn push_child(&mut self, child: NodeId) {
        self.children.push(child);
    }

    pub fn children(&self) -> &[NodeId] {
        &self.children
    }

    pub fn children_mut(&mut self) -> &mut [NodeId] {
        &mut self.children
    }
}

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Deref,
    DerefMut,
    AsRef,
    AsMut,
    From,
    Into,
    Add,
    AddAssign
)]
pub struct NodeId(usize);

#[derive(Debug, Clone, Index, IndexMut, PartialEq, PartialOrd)]
pub struct Ast {
    #[index]
    #[index_mut]
    pub nodes: Vec<Node>,
}

impl Ast {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn with_root_node(node: Node) -> Self {
        let mut ast = Ast::new();
        ast.push(node);
        ast
    }

    pub fn with_root() -> Self {
        let mut ast = Ast::new();
        ast.push(Node::root());
        ast
    }

    pub fn push_tree(&mut self, other: Ast) -> NodeId {
        // println!("\n=== Pushing ===");
        // other.pretty_print(0.into(), 0);

        let before_length = self.nodes.len();

        // println!("Before Length: {}", before_length);

        for node in other.nodes.into_iter() {
            let node_id = self.push(node);
            // println!("New Id: {:?}", node_id);
            for child_id in self[*node_id].children_mut() {
                *child_id = *child_id + before_length.into();
            }
        }

        // println!("++++++++++++++++++++");
        // self.pretty_print(before_length.into(), 0);

        // println!("====================");

        before_length.into()
    }

    pub fn from_program(mut pairs: Pairs<Rule>) -> Self {
        println!("Parsing program");
        println!("{:#?}", pairs);
        let mut ast = Ast::new();
        let program = pairs.next().unwrap();

        let root_id = ast.push(Node::root());

        for pair in program.into_inner() {
            match pair.as_rule() {
                Rule::declaration => {
                    println!("[decl]");
                    ast.create_decl(root_id, pair);
                }
                Rule::EOI => println!("[done]"),
                _ => todo!(),
            }
        }

        // println!("{:#?}", pairs);

        ast
    }

    pub fn create_decl(&mut self, parent: NodeId, decl: Pair<Rule>) {
        assert_eq!(decl.as_rule(), Rule::declaration);

        let node = Node::new(Declaration);
        let node_id = self.push(node);

        for pair in decl.into_inner() {
            match pair.as_rule() {
                Rule::statement => {
                    println!("[stmt]");
                    self.create_stmt(node_id, pair);
                }
                _ => todo!(),
            }
        }

        self[*parent].push_child(node_id);
    }

    pub fn create_stmt(&mut self, parent: NodeId, stmt: Pair<Rule>) {
        assert_eq!(stmt.as_rule(), Rule::statement);

        let node = Node::new(Statement);
        let node_id = self.push(node);

        for pair in stmt.into_inner() {
            match pair.as_rule() {
                Rule::expr_stmt => {
                    println!("[expr]");

                    let expr = pair.into_inner().next().unwrap(); // // let e

                    self.create_expr(node_id, expr);
                }
                _ => todo!(),
            }
        }

        self[*parent].push_child(node_id);
    }

    pub fn create_expr(&mut self, parent: NodeId, expr: Pair<Rule>) {
        // use Rule::*;

        println!("[expr] {:?}", parent);

        assert_eq!(expr.as_rule(), Rule::expr);

        // let node = Node::new(Expression);
        // let node_id = self.push(node);

        let climber = PrecClimber::new(create_operators());

        let infix = |lhs, op: Pair<Rule>, rhs| {
            println!("{:?}", op.as_rule());

            match op.as_rule() {
                Rule::op_plus | Rule::op_times | Rule::op_minus | Rule::op_divide => {
                    let mut new_ast = Ast::with_root_node(Node::new(Binary(Op::from(op.as_rule()))));
                    let root_left = new_ast.push_tree(lhs);
                    new_ast[0].push_child(root_left);
                    let root_right = new_ast.push_tree(rhs);
                    new_ast[0].push_child(root_right);
                    new_ast
                }
                _ => {
                    // println!("{}")
                    todo!()
                },
            }
        };

        let result = climber.climb(expr.into_inner(), primary, infix);
        let result_id = self.push_tree(result);
        self[*parent].push_child(result_id);
    }

    pub fn push(&mut self, node: Node) -> NodeId {
        self.nodes.push(node);
        let id: NodeId = (self.nodes.len() - 1).into();
        self[*id].id = id;
        id
    }

    pub fn nodes(&self) -> &[Node] {
        &self.nodes
    }

    pub fn pretty_print(&self, parent: NodeId, depth: usize) {
        println!("[{:2}] {}{:?}", *self[*parent].id, " ".repeat(depth), self[*parent].ty);

        for child in self[*parent].children() {
            self.pretty_print(*child, depth + 2);
        }
    }
}

pub fn create_operators() -> Vec<Operator<Rule>> {
    // use Rule::*;
    use pest::prec_climber::Assoc;

    vec![
        Operator::new(Rule::op_plus, Assoc::Left) | Operator::new(Rule::op_minus, Assoc::Left),
        Operator::new(Rule::op_times, Assoc::Left) | Operator::new(Rule::op_divide, Assoc::Left),
        // Operator::new(Rule::call)
    ]
}

fn primary(pair: Pair<Rule>) -> Ast {
    println!("{:?}", pair.as_rule());

    match pair.as_rule() {
        Rule::float => {
            let value: f64 = pair.as_str().parse().unwrap();
            let node = Node::new(Float(value.into()));
            Ast::with_root_node(node)
        }
        Rule::int => {
            let value: isize = pair.as_str().parse().unwrap();
            let node = Node::new(Integer(value.into()));
            Ast::with_root_node(node)
        }
        Rule::ident => {
            let value = pair.as_str();
            let node = Node::new(Ident(value.into()));
            Ast::with_root_node(node)
        }
        Rule::string => {
            let value = pair.as_str();
            let node = Node::new(Ident(value.into()));
            Ast::with_root_node(node)
        }
        Rule::value | Rule::term => {
            primary(pair.into_inner().next().unwrap())
        }
        Rule::expr => {
            let mut ast = Ast::with_root_node(Node::new(Expression));
            ast.create_expr(NodeId(0), pair);
            ast
        }
        _ => todo!()
    }
}

// pub fn infix(ast: &mut Ast) -> impl FnMut(Node, Pair<Rule>, Node) -> Node {
//     |lhs: Node, op: Pair<Rule>, rhs: Node| {
//         Node::new(Ident("".into()))
//     }
// }