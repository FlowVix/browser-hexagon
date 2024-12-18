use std::{rc::Rc, sync::Arc};

use itertools::Itertools;
use lasso::Spur;

use crate::{
    parser::ast::Expr,
    span::{Span, Spanned},
};

use super::error::RuntimeError;

macro_rules! values {
    (
        $(
            $variant:ident
                $( ( $($t_t:ty),* $(,)? ) )?
                $( { $($field:ident: $s_t:ty),* $(,)? } )?
        ),* $(,)?
    ) => {

        #[derive(Debug, Clone)]
        pub enum Value {
            $(
                $variant
                    $( ( $($t_t),* ) )?
                    $( { $($field: $s_t),* } )?
                ,
            )*
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum ValueType {
            $(
                $variant,
            )*
        }

        impl Value {
            pub fn get_type(&self) -> ValueType {
                match self {
                    $(
                        Self::$variant {..} => ValueType::$variant,
                    )*
                }
            }
        }
        impl ValueType {
            pub fn name(&self) -> &str {
                paste::paste! {
                    match self {
                        $(
                            Self::$variant => stringify!([< $variant:lower >]),
                        )*
                    }
                }
            }
            pub const TYPES: &[Self] = &[$(Self::$variant),*];
        }

    };
}

#[derive(Debug)]
pub struct FunctionData {
    pub body: Box<Spanned<Expr>>,
    pub params: Box<[Spur]>,
}

values! {
    Number(f64),
    Bool(bool),

    String(Rc<str>),

    Array(Rc<[Value]>),

    Null,

    Function(Rc<FunctionData>),

    Type(ValueType),
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::Bool(l0), Self::Bool(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Array(l0), Self::Array(r0)) => l0 == r0,
            (Self::Null, Self::Null) => true,
            (Self::Function(l0), Self::Function(r0)) => Rc::ptr_eq(&l0, &r0),
            (Self::Type(l0), Self::Type(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl Value {
    pub fn to_str(&self) -> String {
        match self {
            Value::Number(v) => v.to_string(),
            Value::Bool(v) => v.to_string(),
            Value::String(v) => v.to_string(),
            Value::Array(v) => format!("[{}]", v.iter().map(|v| v.to_str()).join(", ")),
            Value::Null => "null".into(),
            Value::Function(data) => format!("<{}-param func>", data.params.len()),
            Value::Type(value_type) => format!("<type '{}'>", value_type.name()),
        }
    }
    pub fn as_bool(&self, span: Span) -> Result<bool, RuntimeError> {
        if let Value::Bool(v) = self {
            Ok(*v)
        } else {
            Err(RuntimeError::NonBooleanCondition {
                typ: self.get_type(),
                span,
            })
        }
    }
}

pub mod ops {
    use std::rc::Rc;

    use crate::{parser::operators::BinOp, span::Span, vm::error::RuntimeError};

    use super::{Value, ValueType};

    fn repeat_int(n: f64) -> usize {
        (n as i64).max(0) as usize
    }

    fn op_err(op: BinOp, a: &Value, b: &Value, span: Span) -> RuntimeError {
        RuntimeError::InvalidOperands {
            type1: a.get_type(),
            type2: b.get_type(),
            op,
            span,
        }
    }

    pub fn plus(a: &Value, b: &Value) -> Option<Value> {
        Some(match (a, b) {
            (Value::Number(a), Value::Number(b)) => Value::Number(*a + *b),
            (Value::String(a), Value::String(b)) => Value::String(format!("{}{}", a, b).into()),
            (Value::Array(a), Value::Array(b)) => {
                Value::Array(a.iter().cloned().chain(b.iter().cloned()).collect())
            }
            _ => return None,
        })
    }
    pub fn minus(a: &Value, b: &Value) -> Option<Value> {
        Some(match (a, b) {
            (Value::Number(a), Value::Number(b)) => Value::Number(*a - *b),
            _ => return None,
        })
    }
    pub fn mult(a: &Value, b: &Value) -> Option<Value> {
        Some(match (a, b) {
            (Value::Number(a), Value::Number(b)) => Value::Number(*a * *b),
            (Value::Number(n), Value::String(s)) | (Value::String(s), Value::Number(n)) => {
                Value::String(s.repeat(repeat_int(*n)).into())
            }
            (Value::Number(n), Value::Array(arr)) | (Value::Array(arr), Value::Number(n)) => {
                Value::Array(
                    arr.iter()
                        .cloned()
                        .cycle()
                        .take(arr.len() * repeat_int(*n))
                        .collect::<Vec<_>>()
                        .into(),
                )
            }
            _ => return None,
        })
    }
    pub fn div(a: &Value, b: &Value) -> Option<Value> {
        Some(match (a, b) {
            (Value::Number(a), Value::Number(b)) => Value::Number(*a / *b),
            _ => return None,
        })
    }
    pub fn modulo(a: &Value, b: &Value) -> Option<Value> {
        Some(match (a, b) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a.rem_euclid(*b)),
            _ => return None,
        })
    }
    pub fn pow(a: &Value, b: &Value) -> Option<Value> {
        Some(match (a, b) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a.powf(*b)),
            _ => return None,
        })
    }
    pub fn eq(a: &Value, b: &Value) -> Option<Value> {
        Some(Value::Bool(a == b))
    }
    pub fn neq(a: &Value, b: &Value) -> Option<Value> {
        Some(Value::Bool(a != b))
    }
    pub fn lt(a: &Value, b: &Value) -> Option<Value> {
        Some(match (a, b) {
            (Value::Number(a), Value::Number(b)) => Value::Bool(*a < *b),
            _ => return None,
        })
    }
    pub fn lte(a: &Value, b: &Value) -> Option<Value> {
        Some(match (a, b) {
            (Value::Number(a), Value::Number(b)) => Value::Bool(*a <= *b),
            _ => return None,
        })
    }
    pub fn gt(a: &Value, b: &Value) -> Option<Value> {
        Some(match (a, b) {
            (Value::Number(a), Value::Number(b)) => Value::Bool(*a > *b),
            _ => return None,
        })
    }
    pub fn gte(a: &Value, b: &Value) -> Option<Value> {
        Some(match (a, b) {
            (Value::Number(a), Value::Number(b)) => Value::Bool(*a >= *b),
            _ => return None,
        })
    }

    pub fn unary_minus(v: &Value) -> Option<Value> {
        Some(match v {
            Value::Number(a) => Value::Number(-a),
            _ => return None,
        })
    }

    #[derive(Debug)]
    pub enum IndexResult<'a> {
        Created(Value),
        Ref(&'a mut Value),
    }
    impl<'a> IndexResult<'a> {
        pub fn to_owned(self) -> Value {
            match self {
                IndexResult::Created(value) => value,
                IndexResult::Ref(v) => v.clone(),
            }
        }
    }
    pub fn index<'a, 'b>(
        base: &'a mut Value,
        idx: &'b Value,
        span: Span,
    ) -> Result<IndexResult<'a>, RuntimeError> {
        Ok(match (base, idx) {
            (Value::String(s), Value::Number(idx)) => {
                if idx.fract() != 0.0 {
                    return Err(RuntimeError::FractionalIndex { value: *idx, span });
                }
                if *idx >= 0.0 {
                    let idx = *idx as usize;
                    if let Some(c) = s.chars().nth(idx) {
                        return Ok(IndexResult::Created(Value::String(c.to_string().into())));
                    }
                }
                return Err(RuntimeError::IndexOutOfBounds {
                    idx: *idx as i64,
                    typ: ValueType::String,
                    length: s.chars().count(),
                    span,
                });
            }
            (Value::Array(v), Value::Number(idx)) => {
                if idx.fract() != 0.0 {
                    return Err(RuntimeError::FractionalIndex { value: *idx, span });
                }
                let len = v.len();
                if *idx >= 0.0 {
                    let idx = *idx as usize;

                    if let Some(c) = Rc::make_mut(v).get_mut(idx) {
                        return Ok(IndexResult::Ref(c));
                    }
                }
                return Err(RuntimeError::IndexOutOfBounds {
                    idx: *idx as i64,
                    typ: ValueType::Array,
                    length: len,
                    span,
                });
            }
            (a, b) => {
                return Err(RuntimeError::CannotIndex {
                    type1: a.get_type(),
                    type2: b.get_type(),
                    span,
                })
            }
        })
    }

    pub fn convert<'a, 'b>(value: &Value, to: &ValueType, span: Span) -> Option<Value> {
        Some(match (value, to) {
            (_, _) if value.get_type() == *to => value.clone(),
            (_, ValueType::String) => Value::String(value.to_str().into()),
            (_, ValueType::Type) => Value::Type(value.get_type()),
            (Value::String(s), ValueType::Number) => match s.parse::<f64>() {
                Ok(v) => Value::Number(v),
                Err(_) => Value::Null,
            },
            (Value::String(s), ValueType::Bool) => match &s[..] {
                "true" => Value::Bool(true),
                "false" => Value::Bool(false),
                _ => Value::Null,
            },
            (Value::Number(n), ValueType::Bool) => Value::Bool(*n != 0.0),
            (Value::Bool(b), ValueType::Number) => Value::Number(if *b { 1.0 } else { 0.0 }),
            _ => return None,
        })
    }
}
