use proc_macro2::TokenStream;
use quote::quote;

use yugolang_parser::{Scope, Statement, Expression, Literal};

pub trait ToTokens {
    fn to_tokens(&self) -> TokenStream;
}

impl ToTokens for Scope {
    fn to_tokens(&self) -> TokenStream {
        let statements = &self.statements.to_tokens();
        let result = &self.result.to_tokens();
        quote! {
            yugolang_parser::Scope {
                statements: #statements,
                result: #result,
            }
        }
    }
}

impl ToTokens for Statement {
    fn to_tokens(&self) -> TokenStream {
        let inner = &self.0.to_tokens();
        quote! { yugolang_parser::Statement(#inner) }
    }
}

impl ToTokens for Expression {
    fn to_tokens(&self) -> TokenStream {
        match self {
            Expression::Literal(inner) => {
                let inner = inner.to_tokens();
                quote! { yugolang_parser::Expression::Literal(#inner) }
            },
            Expression::Scope(inner) => {
                let inner = inner.to_tokens();
                quote! { yugolang_parser::Expression::Scope(#inner) }
            },
            Expression::Identifier(inner) => {
                let inner = inner.to_tokens();
                quote! { yugolang_parser::Expression::Identifier(#inner) }
            },
        }
    }
}

impl ToTokens for Literal {
    fn to_tokens(&self) -> TokenStream {
        match self {
            Literal::Int(inner) => {
                quote! { yugolang_parser::Literal::Int(#inner) }
            },
            Literal::Float(inner) => {
                quote! { yugolang_parser::Literal::Float(#inner) }
            },
            Literal::String(inner) => {
                let inner = inner.to_tokens();
                quote! { yugolang_parser::Literal::String(#inner) }
            },
        }
    }
}

impl<T:ToTokens> ToTokens for Vec<T> {
    fn to_tokens(&self) -> TokenStream {
        let elements = self.iter().map(|e| e.to_tokens());
        quote! { vec![#(#elements,)*] }
    }
}

impl<T:ToTokens> ToTokens for Option<T> {
    fn to_tokens(&self) -> TokenStream {
        match self {
            Some(inner) => {
                let inner = inner.to_tokens();
                quote! { Some(#inner) }
            },
            None => quote! { None }
        }
    }
}

impl<T:ToTokens> ToTokens for Box<T> {
    fn to_tokens(&self) -> TokenStream {
        let inner = self.as_ref().to_tokens();
        quote! { Box::new(#inner) }
    }
}

impl ToTokens for String {
    fn to_tokens(&self) -> TokenStream {
        quote! { String::new(#self) }
    }
}

