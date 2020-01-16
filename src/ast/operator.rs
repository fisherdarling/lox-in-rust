use super::Object;
use crate::error::Error;
use crate::parser::Rule;
use derive_more::Display;

#[derive(Debug, Clone, PartialEq, PartialOrd, Display)]
pub enum UnOp {
    #[display(fmt = "!")]
    Not,
    #[display(fmt = "~")]
    Tilde,
    #[display(fmt = "-")]
    Minus,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Display)]
pub enum BinOp {
    #[display(fmt = "+")]
    Plus,
    #[display(fmt = "-")]
    Minus,
    #[display(fmt = "*")]
    Times,
    #[display(fmt = "/")]
    Divide,
    #[display(fmt = ">")]
    // Mod,
    Gt,
    #[display(fmt = ">=")]
    Ge,
    #[display(fmt = "<")]
    Lt,
    #[display(fmt = "<=")]
    Le,
    #[display(fmt = "!=")]
    Ne,
    #[display(fmt = "==")]
    EqEq,
    #[display(fmt = "!=")]
    NotEq,
    // Not,
    #[display(fmt = "and")]
    And,
    #[display(fmt = "or")]
    Or,
    #[display(fmt = ".")]
    Dot,
}

impl From<Rule> for BinOp {
    fn from(rule: Rule) -> Self {
        match rule {
            Rule::op_plus => BinOp::Plus,
            Rule::op_minus => BinOp::Minus,
            Rule::op_times => BinOp::Times,
            Rule::op_divide => BinOp::Divide,
            Rule::op_equal => BinOp::EqEq,
            Rule::op_not_equal => BinOp::NotEq,
            Rule::op_greater => BinOp::Gt,
            Rule::op_greater_equal => BinOp::Ge,
            Rule::op_lower => BinOp::Lt,
            Rule::op_lower_equal => BinOp::Le,
            Rule::op_and => BinOp::And,
            Rule::op_or => BinOp::Or,
            // Rule::op_plus => Op::Plus,
            _ => todo!(),
        }
    }
}

pub fn is_binop(rule: Rule) -> bool {
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
        | Rule::op_not_equal
        | Rule::op_and
        | Rule::op_or => true,
        // Rule::op_plus => Op::Plus,
        _ => false,
    }
}

pub fn is_unop(rule: Rule) -> bool {
    match rule {
        Rule::op_minus | Rule::op_unary_not => true,
        _ => false,
    }
}

pub trait BinaryOp {
    fn binop(&self, op: BinOp, rhs: &Self) -> Result<Object, Error>
    where
        Self: Sized + ToString,
    {
        Err(Error::InvalidBinaryOperator(
            self.to_string(),
            op,
            rhs.to_string(),
        ))
    }
}

impl BinaryOp for isize {
    fn binop(&self, op: BinOp, rhs: &Self) -> Result<Object, Error> {
        Ok(match op {
            BinOp::Plus => (self + rhs).into(),
            BinOp::Minus => (self - rhs).into(),
            BinOp::Times => (self * rhs).into(),
            BinOp::Divide => (self / rhs).into(),
            BinOp::Gt => (self > rhs).into(),
            BinOp::Ge => (self >= rhs).into(),
            BinOp::Lt => (self < rhs).into(),
            BinOp::Le => (self <= rhs).into(),
            BinOp::Ne => (self != rhs).into(),
            BinOp::EqEq => (self == rhs).into(),
            BinOp::NotEq => (self != rhs).into(),
            _ => Err(Error::InvalidBinaryOperator(
                self.to_string(),
                op,
                rhs.to_string(),
            ))?,
        })
    }
}

impl BinaryOp for f32 {
    fn binop(&self, op: BinOp, rhs: &Self) -> Result<Object, Error> {
        Ok(match op {
            BinOp::Plus => (self + rhs).into(),
            BinOp::Minus => (self - rhs).into(),
            BinOp::Times => (self * rhs).into(),
            BinOp::Divide => (self / rhs).into(),
            BinOp::Gt => (self > rhs).into(),
            BinOp::Ge => (self >= rhs).into(),
            BinOp::Lt => (self < rhs).into(),
            BinOp::Le => (self <= rhs).into(),
            BinOp::Ne => (self != rhs).into(),
            BinOp::EqEq => (self == rhs).into(),
            BinOp::NotEq => (self != rhs).into(),
            _ => Err(Error::InvalidBinaryOperator(
                self.to_string(),
                op,
                rhs.to_string(),
            ))?,
        })
    }
}

impl BinaryOp for bool {
    fn binop(&self, op: BinOp, rhs: &Self) -> Result<Object, Error> {
        Ok(match op {
            BinOp::Gt => (self > rhs).into(),
            BinOp::Ge => (self >= rhs).into(),
            BinOp::Lt => (self < rhs).into(),
            BinOp::Le => (self <= rhs).into(),
            BinOp::Ne => (self != rhs).into(),
            BinOp::EqEq => (self == rhs).into(),
            BinOp::NotEq => (self != rhs).into(),
            BinOp::And => (*self && *rhs).into(),
            BinOp::Or => (*self || *rhs).into(),
            _ => Err(Error::InvalidBinaryOperator(
                self.to_string(),
                op,
                rhs.to_string(),
            ))?,
        })
    }
}

pub trait UnaryOp {
    fn unop(&self, op: UnOp) -> Result<Object, Error>
    where
        Self: Sized + ToString,
    {
        Err(Error::InvalidUnaryOperator(op, self.to_string()))
    }
}

impl UnaryOp for isize {
    fn unop(&self, op: UnOp) -> Result<Object, Error> {
        Ok(match op {
            UnOp::Not | UnOp::Tilde => (!self).into(),
            UnOp::Minus => (-1 * self).into(),
        })
    }
}

impl UnaryOp for f32 {
    fn unop(&self, op: UnOp) -> Result<Object, Error> {
        Ok(match op {
            UnOp::Minus => (-1.0 * self).into(),
            _ => Err(Error::InvalidUnaryOperator(op, self.to_string()))?,
        })
    }
}

impl UnaryOp for bool {
    fn unop(&self, op: UnOp) -> Result<Object, Error> {
        Ok(match op {
            UnOp::Not | UnOp::Tilde => (!self).into(),
            _ => Err(Error::InvalidUnaryOperator(op, self.to_string()))?,
        })
    }
}
