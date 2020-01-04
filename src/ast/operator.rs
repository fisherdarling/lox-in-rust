use super::Lit;
use crate::error::Error;
use crate::parser::Rule;
use derive_more::Display;

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
        | Rule::op_not_equal => true,
        // Rule::op_plus => Op::Plus,
        _ => false,
    }
}

pub trait BinaryOp {
    fn binop(&self, op: BinOp, rhs: &Self) -> Result<Lit, Error>
    where
        Self: Sized + ToString,
    {
        Err(Error::InvalidOperator(
            self.to_string(),
            op,
            rhs.to_string(),
        ))
    }
}

impl BinaryOp for isize {
    fn binop(&self, op: BinOp, rhs: &Self) -> Result<Lit, Error> {
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
            _ => Err(Error::InvalidOperator(
                self.to_string(),
                op,
                rhs.to_string(),
            ))?,
        })
    }
}

impl BinaryOp for f32 {
    fn binop(&self, op: BinOp, rhs: &Self) -> Result<Lit, Error> {
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
            _ => Err(Error::InvalidOperator(
                self.to_string(),
                op,
                rhs.to_string(),
            ))?,
        })
    }
}

impl BinaryOp for bool {
    fn binop(&self, op: BinOp, rhs: &Self) -> Result<Lit, Error> {
        Ok(match op {
            BinOp::Gt => (self > rhs).into(),
            BinOp::Ge => (self >= rhs).into(),
            BinOp::Lt => (self < rhs).into(),
            BinOp::Le => (self <= rhs).into(),
            BinOp::Ne => (self != rhs).into(),
            BinOp::EqEq => (self == rhs).into(),
            BinOp::NotEq => (self != rhs).into(),
            _ => Err(Error::InvalidOperator(
                self.to_string(),
                op,
                rhs.to_string(),
            ))?,
        })
    }
}
