use derive_more::{Add, AddAssign, AsMut, AsRef, Deref, DerefMut, From, Index, IndexMut, Into};

use crate::parser::Rule;
use pest::iterators::{Pair, Pairs};
use pest::prec_climber::{Operator, PrecClimber};

use std::fmt;

#[derive(Clone, Deref, DerefMut, Index, IndexMut, PartialEq, PartialOrd)]
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
    EqEq,
    NotEq,
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
            Rule::op_equal => Op::EqEq,
            Rule::op_not_equal => Op::NotEq,
            Rule::op_greater => Op::Gt,
            Rule::op_greater_equal => Op::Ge,
            Rule::op_lower => Op::Lt,
            Rule::op_lower_equal => Op::Le,
            // Rule::op_plus => Op::Plus,
            _ => todo!(),
        }
    }
}

pub fn is_op(rule: Rule) -> bool {
    match rule {
        Rule::op_plus
        | Rule::op_minus
        | Rule::op_greater
        | Rule::op_times
        | Rule::op_greater_equal
        | Rule::op_divide
        | Rule::op_lower
        | Rule::op_equal
        | Rule::op_lower_equal
        | Rule::op_not_equal => true,
        // Rule::op_plus => Op::Plus,
        _ => false,
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum NodeType {
    Root,
    Assign,
    Path(String),
    Op(Op),
    Unary(Op),
    Call(Path),
    NPath(Path),
    Ident(String),
    Literal,
    Int(isize),
    Float(f64),
    Variable,
    Declaration,
    Statement,
    Expression,
    Arguments,
    Print,
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
    AddAssign,
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
        let before_length = self.nodes.len();

        for node in other.nodes.into_iter() {
            let node_id = self.push(node);
            for child_id in self[*node_id].children_mut() {
                *child_id = *child_id + before_length.into();
            }
        }

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
                Rule::var_decl => {
                    println!("[vdcl]");
                    self.create_var_decl(node_id, pair);
                }
                Rule::class_decl => {
                    println!("[cdcl]");
                    // self.create_var_decl(node_id, pair);
                }
                _ => {
                    println!("{:?}", pair.as_rule());
                    todo!()
                }
            }
        }

        self[*parent].push_child(node_id);
    }

    pub fn create_var_decl(&mut self, parent: NodeId, stmt: Pair<Rule>) {
        assert_eq!(stmt.as_rule(), Rule::var_decl);
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
                Rule::print_stmt => {
                    println!("[prnt]");
                    let expr = pair.into_inner().next().unwrap();

                    self.create_print(node_id, expr);
                }
                _ => todo!(),
            }
        }

        self[*parent].push_child(node_id);
    }

    pub fn create_print(&mut self, parent: NodeId, expr: Pair<Rule>) {
        let node = Node::new(Print);
        let node_id = self.push(node);

        self.create_expr(node_id, expr);
        self[*parent].push_child(node_id);
    }

    pub fn create_expr(&mut self, parent: NodeId, expr: Pair<Rule>) {
        println!("[expr] {:?}", parent);

        assert_eq!(expr.as_rule(), Rule::expr);

        let climber = PrecClimber::new(create_operators());

        let infix = |lhs, op: Pair<Rule>, rhs| {
            println!("{:?}", op.as_rule());

            match op.as_rule() {
                o if is_op(o) => {
                    let mut new_ast = Ast::with_root_node(Node::new(Op(Op::from(o))));

                    let root_left = new_ast.push_tree(lhs);
                    new_ast[0].push_child(root_left);

                    let root_right = new_ast.push_tree(rhs);
                    new_ast[0].push_child(root_right);

                    new_ast
                }
                _ => {
                    // println!("{}")
                    todo!()
                }
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
        println!(
            "[{:2}] {}{:?}",
            *self[*parent].id,
            " ".repeat(depth),
            self[*parent].ty
        );

        for child in self[*parent].children() {
            self.pretty_print(*child, depth + 2);
        }
    }
}

pub fn create_operators() -> Vec<Operator<Rule>> {
    // use Rule::*;
    use pest::prec_climber::Assoc;

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
            let node = Node::new(Int(value.into()));
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
        Rule::path => {
            let value = Path::from_str(pair.as_str());
            let node = Node::new(NPath(value));
            Ast::with_root_node(node)
        }
        Rule::value | Rule::term => primary(pair.into_inner().next().unwrap()),
        Rule::expr => {
            let mut ast = Ast::with_root_node(Node::new(Expression));
            ast.create_expr(NodeId(0), pair);
            ast
        }
        Rule::call => {
            let mut pairs = pair.into_inner();

            let mut ast = Ast::new();
            let path = pairs.next().unwrap().as_str();
            let node = Node::new(Call(Path::from_str(path)));
            let node_id = ast.push(node);

            for arg_expr in pairs {
                ast.create_expr(node_id, arg_expr);
            }

            ast
        }
        _ => {
            println!("{:?}", pair.as_rule());
            todo!()
        }
    }
}
