pub mod ast;
pub mod error;
pub mod lexer;
pub mod operators;

use std::{mem, rc::Rc};

use ast::{Block, Expr, PlacePattern, Stmt};
use error::ParserError;
use lasso::{Rodeo, Spur};
use lexer::{Lexer, Token};

use crate::{
    console_log,
    span::{Span, Spannable, Spanned},
    util::BoxPostfix,
};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    rodeo: &'a mut Rodeo,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a str, rodeo: &'a mut Rodeo) -> Self {
        Self {
            lexer: Lexer::new(src),
            rodeo,
        }
    }

    fn next_tok(&mut self) -> Token {
        self.lexer.next()
    }
    fn peek_tok(&self) -> Token {
        self.lexer.clone().next()
    }
    fn peek_toks<const N: usize>(&self) -> [Token; N] {
        let mut l = self.lexer.clone();
        std::array::from_fn(|_| l.next())
    }
    fn next_is(&self, tok: Token) -> bool {
        self.peek_tok() == tok
    }
    fn skip_tok(&mut self, tok: Token) -> bool {
        if self.next_is(tok) {
            self.next_tok();
            true
        } else {
            false
        }
    }
    fn expect_tok_named(&mut self, tok: Token, name: &str) -> Result<(), ParserError> {
        let peek = self.peek_tok();
        if peek == tok {
            self.next_tok();
        } else {
            return Err(ParserError::Expected {
                expected: name.to_string(),
                found: peek,
                span: self.peek_span(),
            });
        }
        Ok(())
    }
    fn expect_tok(&mut self, tok: Token) -> Result<(), ParserError> {
        self.expect_tok_named(tok, &format!("`{}`", tok.name()))
    }

    fn span(&self) -> Span {
        self.lexer.span()
    }
    fn slice(&self) -> &str {
        self.lexer.slice()
    }
    fn slice_intern(&mut self) -> Spur {
        self.rodeo.get_or_intern(self.slice_rc())
    }
    fn slice_rc(&self) -> Rc<str> {
        self.slice().into()
    }

    fn peek_span(&self) -> Span {
        let mut l = self.lexer.clone();
        l.next();
        l.span()
    }

    /// meant to be called after passing the opening token
    pub fn list_parse<F: FnMut(&mut Self) -> Result<(), ParserError>>(
        &mut self,
        delim: Token,
        end: Token,
        mut cb: F,
    ) -> Result<(), ParserError> {
        loop {
            if self.skip_tok(end) {
                break;
            }
            cb(self)?;
            if !self.skip_tok(delim) {
                self.expect_tok(end)?;
                break;
            }
        }
        Ok(())
    }

    pub fn parse_unit(&mut self) -> Result<Spanned<Expr>, ParserError> {
        let unary;

        Ok(match self.peek_tok() {
            Token::Number => {
                self.next_tok();
                Expr::Number(self.slice().parse().unwrap()).spanned(self.span())
            }
            Token::Ident => {
                self.next_tok();
                Expr::Ident(self.slice_intern()).spanned(self.span())
            }
            Token::String => {
                self.next_tok();
                let s = self.slice();
                let s = s[1..s.len() - 1].replace("\\\"", "\"");
                Expr::String(s.into()).spanned(self.span())
            }
            Token::True => {
                self.next_tok();
                Expr::Bool(true).spanned(self.span())
            }
            Token::False => {
                self.next_tok();
                Expr::Bool(false).spanned(self.span())
            }
            Token::OpenParen => {
                self.next_tok();
                let start = self.span();

                let new_lexer = self.lexer.clone();
                let old_lexer = mem::replace(&mut self.lexer, new_lexer);

                let mut depth = 1;
                let is_func = loop {
                    match self.next_tok() {
                        Token::OpenParen => {
                            depth += 1;
                        }
                        Token::CloseParen => {
                            depth -= 1;
                            if depth == 0 {
                                break self.next_tok() == Token::FatArrow;
                            }
                        }
                        Token::Eof => return Err(ParserError::NoMatchingParen { span: start }),
                        _ => {}
                    }
                };
                self.lexer = old_lexer;

                if !is_func {
                    let inner = self.parse_expr()?;

                    self.expect_tok(Token::CloseParen)?;
                    inner.val.spanned(start.extended(self.span()))
                } else {
                    let mut params = vec![];

                    self.list_parse(Token::Comma, Token::CloseParen, |slef| {
                        slef.expect_tok(Token::Ident)?;
                        params.push(slef.slice_intern().spanned(slef.span()));
                        Ok(())
                    })?;
                    self.expect_tok(Token::FatArrow)?;

                    let body = self.parse_expr()?;

                    Expr::Function {
                        params,
                        body: body.boxed(),
                    }
                    .spanned(start.extended(self.span()))
                }
            }
            Token::OpenSquare => {
                self.next_tok();
                let start = self.span();

                let mut v = vec![];

                self.list_parse(Token::Comma, Token::CloseSquare, |slef| {
                    v.push(slef.parse_expr()?);
                    Ok(())
                })?;

                Expr::Array(v).spanned(start.extended(self.span()))
            }
            Token::OpenCurly => {
                self.next_tok();
                let start = self.span();
                let block = self.parse_block(false)?;
                Expr::Block(block.boxed()).spanned(start.extended(self.span()))
            }
            Token::Dbg => {
                self.next_tok();
                let start = self.span();
                let v = self.parse_expr()?;
                Expr::Dbg(v.boxed()).spanned(start.extended(self.span()))
            }
            Token::If => {
                self.next_tok();
                let start = self.span();
                let cond = self.parse_expr()?;
                let if_true = self.parse_expr()?;
                let if_false = if self.skip_tok(Token::Else) {
                    Some(self.parse_expr()?)
                } else {
                    None
                };
                Expr::If {
                    cond: cond.boxed(),
                    if_true: if_true.boxed(),
                    if_false: if_false.map(Box::new),
                }
                .spanned(start.extended(self.span()))
            }
            Token::While => {
                self.next_tok();
                let start = self.span();
                let cond = self.parse_expr()?;
                let body = self.parse_expr()?;
                Expr::While {
                    cond: cond.boxed(),
                    body: body.boxed(),
                }
                .spanned(start.extended(self.span()))
            }
            Token::For => {
                self.next_tok();
                let start = self.span();
                let init = self.parse_expr()?;
                self.expect_tok(Token::Comma)?;
                let cond = self.parse_expr()?;
                self.expect_tok(Token::Comma)?;
                let step = self.parse_expr()?;
                let body = self.parse_expr()?;
                Expr::For {
                    init: init.boxed(),
                    cond: cond.boxed(),
                    step: step.boxed(),
                    body: body.boxed(),
                }
                .spanned(start.extended(self.span()))
            }
            Token::Var => {
                self.next_tok();
                let start = self.span();
                self.expect_tok_named(Token::Ident, "variable name")?;

                if self.slice().starts_with("$") {
                    return Err(ParserError::UserDefinedSpecialIdent { span: self.span() });
                }
                let name = self.slice_intern();

                self.expect_tok(Token::Assign)?;
                let val = self.parse_expr()?;
                Expr::Declaration(name, val.boxed()).spanned(start.extended(self.span()))
            }
            unary_op
                if {
                    unary = operators::unary_prec(unary_op);
                    unary.is_some()
                } =>
            {
                self.next_tok();
                let start = self.span();
                let unary_prec = unary.unwrap();
                let next_prec = operators::next_infix(unary_prec);
                let val = match next_prec {
                    Some(next_prec) => self.parse_op(next_prec)?,
                    None => self.parse_value()?,
                };

                Expr::UnaryOp(unary_op.to_unary_op().unwrap(), val.boxed())
                    .spanned(start.extended(self.span()))
            }
            t => {
                return Err(ParserError::Expected {
                    expected: "expression".into(),
                    found: t,
                    span: self.peek_span(),
                })
            }
        })
    }
    pub fn parse_value(&mut self) -> Result<Spanned<Expr>, ParserError> {
        let mut out = self.parse_unit()?;

        loop {
            let start_span = out.span;
            match self.peek_tok() {
                Token::OpenSquare => {
                    self.next_tok();

                    let idx = self.parse_expr()?;
                    self.expect_tok(Token::CloseSquare)?;

                    out = Expr::Index {
                        base: out.boxed(),
                        index: idx.boxed(),
                    }
                    .spanned(start_span.extended(self.span()));
                }
                Token::OpenParen => {
                    self.next_tok();

                    let mut args = vec![];

                    self.list_parse(Token::Comma, Token::CloseParen, |slef| {
                        args.push(slef.parse_expr()?);
                        Ok(())
                    })?;

                    out = Expr::Call {
                        base: out.boxed(),
                        args,
                    }
                    .spanned(start_span.extended(self.span()));
                }
                t if t.to_assign_op().is_some() => {
                    self.next_tok();

                    let pattern = PlacePattern::from_expr(out, self.rodeo)?;
                    let value = self.parse_expr()?;

                    out = Expr::Assign {
                        op: t.to_assign_op().unwrap(),
                        pattern: pattern.spanned(start_span).boxed(),
                        value: value.boxed(),
                    }
                    .spanned(start_span.extended(self.span()));
                }
                _ => break,
            }
        }

        Ok(out)
    }
    pub fn parse_op(&mut self, prec: usize) -> Result<Spanned<Expr>, ParserError> {
        let next_prec = operators::next_infix(prec);

        let mut left = match next_prec {
            Some(next_prec) => self.parse_op(next_prec)?,
            None => self.parse_value()?,
        };

        while operators::is_infix_prec(self.peek_tok(), prec) {
            let op = self.next_tok();

            let right = if operators::prec_type(prec) == operators::OpType::Left {
                match next_prec {
                    Some(next_prec) => self.parse_op(next_prec)?,
                    None => self.parse_value()?,
                }
            } else {
                self.parse_op(prec)?
            };
            let new_span = left.span.extended(right.span);
            left =
                Expr::BinOp(left.boxed(), op.to_bin_op().unwrap(), right.boxed()).spanned(new_span)
        }

        Ok(left)
    }
    pub fn parse_expr(&mut self) -> Result<Spanned<Expr>, ParserError> {
        self.parse_op(0)
    }
    /// meant to be called after passing the opening brace
    pub fn parse_block(&mut self, root: bool) -> Result<Spanned<Block>, ParserError> {
        let start = self.span();
        let mut block = Block {
            normal: vec![],
            ret: None,
        };

        let end_tok = if root { Token::Eof } else { Token::CloseCurly };

        loop {
            let stmt = match self.peek_tok() {
                _ => {
                    let expr = self.parse_expr()?;
                    let span = expr.span;
                    Stmt::Expr(expr).spanned(span)
                }
            };

            if !self.skip_tok(Token::Semicolon) {
                self.expect_tok(end_tok)?;
                block.ret = Some(stmt);
                return Ok(block.spanned(start.extended(self.span())));
            }
            block.normal.push(stmt);
            if self.skip_tok(end_tok) {
                return Ok(block.spanned(start.extended(self.span())));
            }
        }
    }
    // pub fn parse_root(&mut self) -> Result<Spanned<Block>, ParserError> {
    //     let out = self.parse_block(true)?;
    //     Ok(out)
    // }
}
