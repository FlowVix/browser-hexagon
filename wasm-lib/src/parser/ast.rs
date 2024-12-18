use std::rc::Rc;

use lasso::{Rodeo, Spur};

use crate::{
    span::{Span, Spannable, Spanned},
    util::BoxPostfix,
};

use super::{
    error::ParserError,
    operators::{AssignOp, BinOp, UnaryOp},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64),
    String(Rc<str>),
    Bool(bool),

    Ident(Spur),

    BinOp(Box<Spanned<Expr>>, BinOp, Box<Spanned<Expr>>),
    UnaryOp(UnaryOp, Box<Spanned<Expr>>),

    Block(Box<Spanned<Block>>),

    Array(Vec<Spanned<Expr>>),

    Index {
        base: Box<Spanned<Expr>>,
        index: Box<Spanned<Expr>>,
    },
    Call {
        base: Box<Spanned<Expr>>,
        args: Vec<Spanned<Expr>>,
    },

    Declaration(Spur, Box<Spanned<Expr>>),
    Assign {
        op: AssignOp,
        pattern: Box<Spanned<PlacePattern>>,
        value: Box<Spanned<Expr>>,
    },

    Dbg(Box<Spanned<Expr>>),

    If {
        cond: Box<Spanned<Expr>>,
        if_true: Box<Spanned<Expr>>,
        if_false: Option<Box<Spanned<Expr>>>,
    },
    While {
        cond: Box<Spanned<Expr>>,
        body: Box<Spanned<Expr>>,
    },
    For {
        init: Box<Spanned<Expr>>,
        cond: Box<Spanned<Expr>>,
        step: Box<Spanned<Expr>>,
        body: Box<Spanned<Expr>>,
    },

    Function {
        params: Vec<Spanned<Spur>>,
        body: Box<Spanned<Expr>>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expr(Spanned<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub normal: Vec<Spanned<Stmt>>,
    pub ret: Option<Spanned<Stmt>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PlacePattern {
    Var(Spur),
    Index {
        base: Box<Spanned<PlacePattern>>,
        index: Spanned<Expr>,
    },
}
impl PlacePattern {
    pub fn from_expr(expr: Spanned<Expr>, rodeo: &Rodeo) -> Result<Self, ParserError> {
        Ok(match expr.val {
            Expr::Ident(s) => {
                let str = &rodeo[s];

                if str.starts_with("$") {
                    return Err(ParserError::UserDefinedSpecialIdent { span: expr.span });
                }
                Self::Var(s)
            }
            Expr::Index { base, index } => Self::Index {
                base: {
                    let span = base.span;
                    Self::from_expr(*base, rodeo)?.spanned(span).boxed()
                },
                index: *index,
            },
            _ => return Err(ParserError::InvalidAssignExpression { span: expr.span }),
        })
    }
}
