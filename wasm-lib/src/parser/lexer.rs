use logos::Logos;

use crate::span::Span;

#[derive(Logos, Debug, PartialEq, Eq, Clone, Copy)]
#[logos(skip r"[ \t\r\n\f]+")] // Ignore this regex pattern between tokens
pub enum Token {
    #[regex("[a-zA-Z_$][a-zA-Z_0-9]*")]
    Ident,
    #[regex("[0-9]+(\\.[0-9]*)?")]
    Number,
    #[regex(r##""(?:[^"]|\\")*""##)]
    String,

    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Mult,
    #[token("/")]
    Div,
    #[token("%")]
    Mod,
    #[token("**")]
    Pow,

    #[token("=")]
    Assign,
    #[token("+=")]
    PlusAssign,
    #[token("-=")]
    MinusAssign,
    #[token("*=")]
    MultAssign,
    #[token("/=")]
    DivAssign,
    #[token("%=")]
    ModAssign,
    #[token("**=")]
    PowAssign,

    #[token("==")]
    Eq,
    #[token("!=")]
    NEq,
    #[token("<")]
    Lt,
    #[token(">")]
    Gt,
    #[token("<=")]
    LtE,
    #[token(">=")]
    GtE,

    #[token("(")]
    OpenParen,
    #[token(")")]
    CloseParen,
    #[token("[")]
    OpenSquare,
    #[token("]")]
    CloseSquare,
    #[token("{")]
    OpenCurly,
    #[token("}")]
    CloseCurly,

    #[token(",")]
    Comma,
    #[token(";")]
    Semicolon,
    #[token(":")]
    Colon,

    #[token("=>")]
    FatArrow,

    #[token("true")]
    True,
    #[token("false")]
    False,

    #[token("var")]
    Var,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("while")]
    While,
    #[token("for")]
    For,

    #[token("dbg")]
    Dbg,

    Unknown,
    Eof,
}

impl Token {
    pub fn name(self) -> &'static str {
        match self {
            Token::Ident => "identifier",
            Token::Number => "number",
            Token::String => "string literal",
            Token::Plus => "+",
            Token::Minus => "-",
            Token::Mult => "*",
            Token::Div => "/",
            Token::Mod => "%",
            Token::Pow => "**",
            Token::Assign => "=",
            Token::Unknown => "unknown",
            Token::Eof => "end of file",
            Token::OpenParen => "(",
            Token::CloseParen => ")",
            Token::OpenSquare => "[",
            Token::CloseSquare => "]",
            Token::OpenCurly => "{",
            Token::CloseCurly => "}",
            Token::Comma => ",",
            Token::Semicolon => ";",
            Token::Colon => ":",
            Token::Var => "var",
            Token::PlusAssign => "+=",
            Token::MinusAssign => "-=",
            Token::MultAssign => "*=",
            Token::DivAssign => "/=",
            Token::ModAssign => "%=",
            Token::PowAssign => "**=",
            Token::If => "if",
            Token::Else => "else",
            Token::While => "while",
            Token::For => "for",
            Token::Eq => "==",
            Token::NEq => "!=",
            Token::Lt => "<",
            Token::Gt => ">",
            Token::LtE => "<=",
            Token::GtE => ">=",
            Token::Dbg => "dbg",
            Token::True => "true",
            Token::False => "false",
            Token::FatArrow => "=>",
        }
    }
}

fn inner_next<'a>(lexer: &mut logos::Lexer<'a, Token>) -> Token {
    lexer
        .next()
        .map(|v| v.unwrap_or(Token::Unknown))
        .unwrap_or(Token::Eof)
}

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    inner: logos::Lexer<'a, Token>,
    peeked: Option<Token>,
}
impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            inner: Token::lexer(src),
            peeked: None,
        }
    }
    pub fn peek(&mut self) -> Token {
        if let Some(p) = self.peeked {
            return p;
        }
        let peek = inner_next(&mut self.inner.clone());
        self.peeked = Some(peek);
        peek
    }
    pub fn span(&self) -> Span {
        let span = self.inner.span();
        Span::new(span.start, span.end)
    }
    pub fn slice(&self) -> &str {
        self.inner.slice()
    }
    pub fn next(&mut self) -> Token {
        match self.peeked.take() {
            Some(v) => v,
            None => inner_next(&mut self.inner),
        }
    }
}
