macro_rules! operators {

    (
        Assign: [$($a_tok:ident),*];
        $(
            $( Left => [$($l_tok:ident),*] )?
            $( Right => [$($r_tok:ident),*] )?
            $( Unary => [$($u_tok:ident),*] )?
            ;
        )+
    ) => {

        use crate::parser::lexer::Token;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum Operator {
            Bin(BinOp),
            Unary(UnaryOp),
            Assign(AssignOp),
        }

        impl Operator {
            pub fn name(self) -> &'static str {
                match self {
                    Self::Bin(b) => b.name(),
                    Self::Unary(u) => u.name(),
                    Self::Assign(a) => a.name(),
                }
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum AssignOp {
            $(
                $a_tok,
            )*
        }


        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum OpType {
            Left,
            Right,
            Unary,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum BinOp {
            $(
                $($($l_tok,)+)?
                $($($r_tok,)+)?
            )+
        }
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum UnaryOp {
            $(
                $($($u_tok,)+)?
            )+
        }

        impl BinOp {
            pub fn name(self) -> &'static str {
                match self {
                    $(
                        $($(
                            BinOp::$l_tok => Token::$l_tok.name(),
                        )+)?
                        $($(
                            BinOp::$r_tok => Token::$r_tok.name(),
                        )+)?
                    )+
                }
            }
        }
        impl UnaryOp {
            pub fn name(self) -> &'static str {
                match self {
                    $(
                        $($(
                            UnaryOp::$u_tok => Token::$u_tok.name(),
                        )+)?
                    )+
                }
            }
        }
        impl AssignOp {
            pub fn name(self) -> &'static str {
                match self {
                    $(
                        AssignOp::$a_tok => Token::$a_tok.name(),
                    )*
                }
            }
        }

        impl Token {
            pub fn to_bin_op(self) -> Option<BinOp> {
                Some(match self {
                    $(
                        $($(Token::$l_tok => BinOp::$l_tok,)+)?
                        $($(Token::$r_tok => BinOp::$r_tok,)+)?
                    )+
                    _ => return None,
                })
            }
            pub fn to_unary_op(self) -> Option<UnaryOp> {
                Some(match self {
                    $(
                        $($(Token::$u_tok => UnaryOp::$u_tok,)+)?
                    )+
                    _ => return None,
                })
            }
            pub fn to_assign_op(self) -> Option<AssignOp> {
                Some(match self {
                    $(
                        Token::$a_tok => AssignOp::$a_tok,
                    )*
                    _ => return None,
                })
            }
        }

        const OP_LIST: &[(OpType, &[Token])] = &[
            $(
                $( (OpType::Left, &[$(Token::$l_tok),*]) )?
                $( (OpType::Right, &[$(Token::$r_tok),*]) )?
                $( (OpType::Unary, &[$(Token::$u_tok),*]) )?
            ),*
        ];
        pub const OP_COUNT: usize = OP_LIST.len();

        pub fn next_infix(prec: usize) -> Option<usize> {
            let mut next = prec + 1;
            while next < OP_COUNT {
                if OP_LIST[next].0 != OpType::Unary {
                    return Some(next);
                } else {
                    next += 1;
                }
            }
            None
        }
        pub fn is_infix_prec(op: Token, prec: usize) -> bool {
            for (i, (typ, toks)) in OP_LIST.iter().enumerate() {
                if *typ != OpType::Unary && toks.contains(&op) && i == prec {
                    return true
                }
            }
            return false
        }
        pub fn unary_prec(op: Token) -> Option<usize> {
            for (i, (typ, toks)) in OP_LIST.iter().enumerate() {
                if *typ == OpType::Unary && toks.contains(&op) {
                    return Some(i)
                }
            }
            return None
        }
        pub fn prec_type(prec: usize) -> OpType {
            OP_LIST[prec].0
        }

    };
}

operators! {
    Assign: [Assign, PlusAssign, MinusAssign, MultAssign, DivAssign, ModAssign, PowAssign];

    // lowest precedence
    Left => [Eq, NEq, Gt, Lt, GtE, LtE];
    Left => [Plus, Minus];
    Unary => [Minus];
    Left => [Mult, Div, Mod];
    Right => [Pow];
    // highest precedence

}
