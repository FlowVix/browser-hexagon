macro_rules! make_error {
    (

        @kind: $report_type:ident;

        $struct_name:ident {
            $(
                @title: $title:expr;
                @msgs: [
                    $(
                        $span:expr => $fmt:literal $(: $($args:expr),+)?;
                    )*
                ];
                $variant:ident {
                    $(
                        $field:ident: $typ:ty
                    ),* $(,)?
                } $(,)?
            )*
        }
    ) => {

        #[derive(Debug, Clone, PartialEq)]
        pub enum $struct_name {
            $(
                $variant { $($field: $typ),* },
            )*
        }

        impl $struct_name {
            pub fn into_report(self) -> crate::error::Report {

                // use owo_colors::OwoColorize;
                match self {
                    $(
                        Self::$variant { $($field),* } => crate::error::Report {
                            title: String::from($title),
                            typ: crate::error::ReportType::$report_type,
                            messages: Box::new([$(
                                ($span, format!($fmt $( , $( ($args) ),* )? )),
                            )*])
                        },
                    )*
                }
            }
        }

    };
}

pub(crate) use make_error;

use crate::span::Span;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReportType {
    Error,
    Warning,
}
// impl ReportType {
//     pub fn display_str(self) -> String {
//         match self {
//             ReportType::Error => "error:".bright_red().bold().to_string(),
//             ReportType::Warning => "warning:".bright_yellow().bold().to_string(),
//         }
//     }
// }

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Report {
    pub(crate) title: String,
    pub(crate) typ: ReportType,
    pub(crate) messages: Box<[(Span, String)]>,
}

#[wasm_bindgen]
impl Report {
    pub fn get_title(&self) -> String {
        self.title.clone()
    }
    pub fn get_typ(&self) -> ReportType {
        self.typ.clone()
    }
    pub fn get_msg_spans(&self) -> Vec<Span> {
        self.messages.iter().map(|v| v.0).collect()
    }
    pub fn get_msg_strings(&self) -> Vec<String> {
        self.messages.iter().map(|v| v.1.clone()).collect()
    }
}
