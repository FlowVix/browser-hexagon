// use ahash::AHashMap;

use std::{collections::HashMap, rc::Rc};

use error::RuntimeError;
use lasso::{Rodeo, Spur};
use value::{ops::IndexResult, FunctionData, Value, ValueType};

use crate::{
    console_log,
    parser::{
        ast::{Block, Expr, PlacePattern, Stmt},
        operators::{AssignOp, BinOp, UnaryOp},
    },
    span::{Spannable, Spanned},
};

pub mod error;
pub mod value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScopeType {
    Normal,
    FuncStart,
}

#[derive(Debug, Clone)]
pub struct Scope {
    vars: HashMap<Spur, Value>,
    typ: ScopeType,
}
pub struct Vm {
    scopes: Vec<Scope>,
}

impl Vm {
    pub fn new() -> Self {
        Self { scopes: vec![] }
    }

    pub fn get_var(&mut self, name: Spur) -> Option<&mut Value> {
        for i in self.scopes.iter_mut().rev() {
            let v = i.vars.get_mut(&name);
            if v.is_some() {
                return v;
            }
            if i.typ == ScopeType::FuncStart {
                break;
            }
        }
        None
    }

    pub fn get_assign_pattern(
        &mut self,
        pattern: &Spanned<PlacePattern>,
        rodeo: &mut Rodeo,
    ) -> Result<&mut Value, RuntimeError> {
        Ok(match &pattern.val {
            PlacePattern::Var(name) => {
                self.get_var(*name)
                    .ok_or_else(|| RuntimeError::NonexistentVariable {
                        name: rodeo[*name].into(),
                        span: pattern.span,
                    })?
            }
            PlacePattern::Index { base, index } => {
                let index_v = self.run_expr(index, rodeo)?;
                let base_v = self.get_assign_pattern(&base, rodeo)?;
                match value::ops::index(base_v, &index_v, pattern.span)? {
                    IndexResult::Created(value) => {
                        return Err(RuntimeError::InvalidAssignExpression { span: pattern.span })
                    }
                    IndexResult::Ref(v) => v,
                }
            }
        })
    }

    pub fn run_expr(
        &mut self,
        expr: &Spanned<Expr>,
        rodeo: &mut Rodeo,
    ) -> Result<Value, RuntimeError> {
        Ok(match &expr.val {
            Expr::Number(n) => Value::Number(*n),
            Expr::String(s) => Value::String(s.clone()),
            Expr::Bool(b) => Value::Bool(*b),
            Expr::Ident(name) => self
                .get_var(*name)
                .ok_or_else(|| RuntimeError::NonexistentVariable {
                    name: rodeo[*name].into(),
                    span: expr.span,
                })?
                .clone(),
            Expr::BinOp(a, op, b) => {
                let a_v = self.run_expr(&a, rodeo)?;
                let b_v = self.run_expr(&b, rodeo)?;
                match op {
                    BinOp::Plus => value::ops::plus(&a_v, &b_v),
                    BinOp::Minus => value::ops::minus(&a_v, &b_v),
                    BinOp::Mult => value::ops::mult(&a_v, &b_v),
                    BinOp::Div => value::ops::div(&a_v, &b_v),
                    BinOp::Mod => value::ops::modulo(&a_v, &b_v),
                    BinOp::Pow => value::ops::pow(&a_v, &b_v),
                    BinOp::Eq => value::ops::eq(&a_v, &b_v),
                    BinOp::NEq => value::ops::neq(&a_v, &b_v),
                    BinOp::Gt => value::ops::gt(&a_v, &b_v),
                    BinOp::Lt => value::ops::lt(&a_v, &b_v),
                    BinOp::GtE => value::ops::gte(&a_v, &b_v),
                    BinOp::LtE => value::ops::lte(&a_v, &b_v),
                }
                .ok_or_else(|| RuntimeError::InvalidOperands {
                    type1: a_v.get_type(),
                    type2: b_v.get_type(),
                    op: *op,
                    span: expr.span,
                })?
            }
            Expr::UnaryOp(op, v) => {
                let v_v = self.run_expr(&v, rodeo)?;
                match op {
                    UnaryOp::Minus => value::ops::unary_minus(&v_v),
                }
                .ok_or_else(|| RuntimeError::InvalidUnaryOperand {
                    typ: v_v.get_type(),
                    op: *op,
                    span: expr.span,
                })?
            }
            Expr::Block(spanned) => self.run_block(&spanned, false, rodeo)?,
            Expr::Array(vec) => Value::Array(
                vec.iter()
                    .map(|v| self.run_expr(v, rodeo))
                    .collect::<Result<_, _>>()?,
            ),
            Expr::Index { base, index } => {
                let mut base_v = self.run_expr(&base, rodeo)?;
                let index_v = self.run_expr(&index, rodeo)?;
                value::ops::index(&mut base_v, &index_v, expr.span)?.to_owned()
            }
            Expr::Assign { op, pattern, value } => {
                let value_v = self.run_expr(&value, rodeo)?;
                let value_type = value_v.get_type();
                let p_ref = self.get_assign_pattern(&pattern, rodeo)?;

                let set = match op {
                    AssignOp::Assign => Some(value_v),
                    AssignOp::PlusAssign => value::ops::plus(p_ref, &value_v),
                    AssignOp::MinusAssign => value::ops::minus(p_ref, &value_v),
                    AssignOp::MultAssign => value::ops::mult(p_ref, &value_v),
                    AssignOp::DivAssign => value::ops::div(p_ref, &value_v),
                    AssignOp::ModAssign => value::ops::modulo(p_ref, &value_v),
                    AssignOp::PowAssign => value::ops::pow(p_ref, &value_v),
                }
                .ok_or_else(|| RuntimeError::InvalidOperands {
                    type1: p_ref.get_type(),
                    type2: value_type,
                    op: match op {
                        AssignOp::Assign => unreachable!(),
                        AssignOp::PlusAssign => BinOp::Plus,
                        AssignOp::MinusAssign => BinOp::Minus,
                        AssignOp::MultAssign => BinOp::Mult,
                        AssignOp::DivAssign => BinOp::Div,
                        AssignOp::ModAssign => BinOp::Mod,
                        AssignOp::PowAssign => BinOp::Pow,
                    },
                    span: expr.span,
                })?;

                *p_ref = set;

                Value::Null
            }
            Expr::Dbg(v) => {
                let v = self.run_expr(&v, rodeo)?;
                console_log!("{}", v.to_str());
                v
            }
            Expr::If {
                cond,
                if_true,
                if_false,
            } => {
                let cond_v = self.run_expr(&cond, rodeo)?;
                if cond_v.as_bool(cond.span)? {
                    self.run_expr(&if_true, rodeo)?
                } else {
                    match if_false {
                        Some(v) => self.run_expr(&v, rodeo)?,
                        None => Value::Null,
                    }
                }
            }
            Expr::While { cond, body } => {
                let mut out = Value::Null;
                while self.run_expr(&cond, rodeo)?.as_bool(cond.span)? {
                    out = self.run_expr(&body, rodeo)?;
                }
                out
            }
            Expr::Declaration(name, value) => {
                let value = self.run_expr(value, rodeo)?;
                self.scopes.last_mut().unwrap().vars.insert(*name, value);
                Value::Null
            }
            Expr::For {
                init,
                cond,
                step,
                body,
            } => {
                self.scopes.push(Scope {
                    vars: HashMap::new(),
                    typ: ScopeType::Normal,
                });
                self.run_expr(&init, rodeo)?;

                let mut out = Value::Null;
                while self.run_expr(&cond, rodeo)?.as_bool(cond.span)? {
                    out = self.run_expr(&body, rodeo)?;
                    self.run_expr(&step, rodeo)?;
                }

                self.scopes.pop();
                out
            }
            Expr::Function { params, body } => {
                //
                Value::Function(Rc::new(FunctionData {
                    body: body.clone(),
                    params: params.iter().map(|v| v.val).collect(),
                }))
            }
            Expr::Call { base, args } => {
                let base_v = self.run_expr(&base, rodeo)?;
                match base_v {
                    Value::Function(v) => {
                        if v.params.len() != args.len() {
                            return Err(RuntimeError::IncorrectArgAmount {
                                correct: v.params.len(),
                                bad: args.len(),
                                span: expr.span,
                            });
                        }
                        let mut new_scope = Scope {
                            vars: HashMap::new(),
                            typ: ScopeType::FuncStart,
                        };
                        for (param, arg) in v.params.iter().zip(args) {
                            let val = self.run_expr(arg, rodeo)?;
                            new_scope.vars.insert(*param, val);
                        }
                        self.scopes.push(new_scope);
                        let out = self.run_expr(&v.body, rodeo)?;
                        self.scopes.pop();
                        out
                    }
                    Value::Type(t) => {
                        if args.len() != 1 {
                            return Err(RuntimeError::IncorrectArgAmount {
                                correct: 1,
                                bad: args.len(),
                                span: expr.span,
                            });
                        }
                        let value = self.run_expr(&args[0], rodeo)?;
                        value::ops::convert(&value, &t, expr.span).ok_or_else(|| {
                            RuntimeError::CannotConvert {
                                from: value.get_type(),
                                to: t,
                                span: expr.span,
                            }
                        })?
                    }
                    _ => {
                        return Err(RuntimeError::CannotCall {
                            typ: base_v.get_type(),
                            span: base.span,
                        })
                    }
                }
            }
        })
    }
    pub fn run_stmt(
        &mut self,
        stmt: &Spanned<Stmt>,
        rodeo: &mut Rodeo,
    ) -> Result<Value, RuntimeError> {
        Ok(match &stmt.val {
            Stmt::Expr(expr) => self.run_expr(expr, rodeo)?,
        })
    }
    pub fn run_block(
        &mut self,
        block: &Spanned<Block>,
        root: bool,
        rodeo: &mut Rodeo,
    ) -> Result<Value, RuntimeError> {
        let mut out = Value::Null;
        self.scopes.push(Scope {
            vars: HashMap::new(),
            typ: if root {
                ScopeType::FuncStart
            } else {
                ScopeType::Normal
            },
        });

        if root {
            for t in ValueType::TYPES {
                self.scopes.last_mut().unwrap().vars.insert(
                    rodeo.get_or_intern(format!("${}", t.name())),
                    Value::Type(*t),
                );
            }
        }

        for i in block.normal.iter().chain(block.ret.iter()) {
            out = self.run_stmt(i, rodeo)?;
        }
        self.scopes.pop();
        Ok(if block.ret.is_some() {
            out
        } else {
            Value::Null
        })
    }
}
