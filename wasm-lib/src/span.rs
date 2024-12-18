use std::{
    fmt::Debug,
    ops::{Deref, DerefMut, Range},
};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
    pub fn extended(self, other: Span) -> Self {
        Self {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }
}

impl Debug for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}
impl From<Range<usize>> for Span {
    fn from(value: Range<usize>) -> Self {
        Self::new(value.start, value.end)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Spanned<T> {
    pub val: T,
    pub span: Span,
}
impl<T> Deref for Spanned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.val
    }
}
impl<T> DerefMut for Spanned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.val
    }
}
impl<T: Debug> Debug for Spanned<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{:?} {:#?}", self.span, self.val)
        } else {
            write!(f, "{:?} {:?}", self.span, self.val)
        }
    }
}
impl<T> Spanned<T> {
    pub fn map<R, F: FnOnce(T) -> R>(self, f: F) -> Spanned<R> {
        Spanned {
            val: f(self.val),
            span: self.span,
        }
    }
}

pub trait Spannable: Sized {
    fn spanned(self, span: impl Into<Span>) -> Spanned<Self>;
}
impl<T> Spannable for T {
    fn spanned(self, span: impl Into<Span>) -> Spanned<Self> {
        Spanned {
            val: self,
            span: span.into(),
        }
    }
}
