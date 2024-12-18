use crate::{
    error::make_error,
    parser::operators::{BinOp, UnaryOp},
    span::Span,
};

use super::value::ValueType;

make_error! {
    @kind: Error;

    RuntimeError {

        @title: format!("Invalid operands");
        @msgs: [
            span => "Cannot apply `{}` to {} and {}": op.name(), type1.name(), type2.name();
        ];
        InvalidOperands {
            type1: ValueType,
            type2: ValueType,
            op: BinOp,
            span: Span,
        }

        @title: format!("Invalid unary operand");
        @msgs: [
            span => "Cannot apply unary `{}` to {}": op.name(), typ.name();
        ];
        InvalidUnaryOperand {
            typ: ValueType,
            op: UnaryOp,
            span: Span,
        }

        @title: format!("Nonexistent variable");
        @msgs: [
            span => "Variable `{}` does not exist": name;
        ];
        NonexistentVariable {
            name: String,
            span: Span,
        }

        @title: format!("Cannot index");
        @msgs: [
            span => "Cannot index {} with {}": type1.name(), type2.name();
        ];
        CannotIndex {
            type1: ValueType,
            type2: ValueType,
            span: Span,
        }

        @title: format!("Fractional index");
        @msgs: [
            span => "Tried to index with non-integer {}": value;
        ];
        FractionalIndex {
            value: f64,
            span: Span,
        }

        @title: format!("Index out of bounds");
        @msgs: [
            span => "Index {} is out of bounds for {} of length {}": idx, typ.name(), length;
        ];
        IndexOutOfBounds {
            idx: i64,
            typ: ValueType,
            length: usize,
            span: Span,
        }

        @title: format!("Invalid expression for assignment");
        @msgs: [
            span => "This expression is not a reference and cannot be assigned to";
        ];
        InvalidAssignExpression {
            span: Span,
        }

        @title: format!("Non boolean condition");
        @msgs: [
            span => "Expected bool for condition, found {}": typ.name();
        ];
        NonBooleanCondition {
            typ: ValueType,
            span: Span,
        }

        @title: format!("Cannot call value");
        @msgs: [
            span => "Cannot call a {}": typ.name();
        ];
        CannotCall {
            typ: ValueType,
            span: Span,
        }

        @title: format!("Incorrect argument count");
        @msgs: [
            span => "This call requires {} arguments, but received {}": correct, bad;
        ];
        IncorrectArgAmount {
            correct: usize,
            bad: usize,
            span: Span,
        }

        @title: format!("Cannot convert");
        @msgs: [
            span => "Cannot convert {} to {}": from.name(), to.name();
        ];
        CannotConvert {
            from: ValueType,
            to: ValueType,
            span: Span,
        }

        // @title: format!("Cannot declare variables with special name");
        // @msgs: [
        //     span => "Variable declared here";
        // ];
        // UserDefinedSpecialIdent {
        //     span: Span,
        // }

        // @title: format!("Invalid expression for assignment");
        // @msgs: [
        //     span => "Cannot assign to this expression";
        // ];
        // InvalidAssignExpression {
        //     span: Span,
        // }

    }
}
