use crate::{error::make_error, span::Span};

use super::lexer::Token;

make_error! {
    @kind: Error;

    ParserError {

        @title: format!("Expected {}, found `{}`", expected, found.name());
        @msgs: [
            span => "Expected {}": expected;
        ];
        Expected {
            expected: String,
            found: Token,
            span: Span,
        }

        @title: format!("Cannot declare or modify variables with special name");
        @msgs: [
            span => "Variable name used here";
        ];
        UserDefinedSpecialIdent {
            span: Span,
        }

        @title: format!("Invalid expression for assignment");
        @msgs: [
            span => "Cannot assign to this expression";
        ];
        InvalidAssignExpression {
            span: Span,
        }

        @title: format!("No matching parenthesis");
        @msgs: [
            span => "Cannot find a matching `)` for this `(`";
        ];
        NoMatchingParen {
            span: Span,
        }

    }
}
