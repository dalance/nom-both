#![recursion_limit = "128"]

extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::{quote, ToTokens};
use std::str::FromStr;
use syn::{self, parse_macro_input, parse_quote, AttributeArgs, FnArg, ItemFn, Stmt};

#[proc_macro_attribute]
pub fn both_parser(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as AttributeArgs);
    let item = parse_macro_input!(item as ItemFn);
    impl_both_parser(&attr, &item)
}

fn impl_both_parser(_attr: &AttributeArgs, item: &ItemFn) -> TokenStream {
    let body = impl_both_parser_body(&item);
    let body = parse_macro_input!(body as Stmt);

    let mut item = item.clone();

    item.block.stmts.clear();
    item.block.stmts.push(body);

    item.into_token_stream().into()
}

fn impl_both_parser_body(item: &ItemFn) -> TokenStream {
    let input = if let Some(x) = &item.decl.inputs.first() {
        match x.value() {
            FnArg::Captured(arg) => &arg.pat,
            _ => panic!("function with #[both_parser] must have an argument"),
        }
    } else {
        panic!("function with #[both_parser] must have an argument");
    };

    let body = item.block.as_ref();
    let mut body_token = body.into_token_stream().to_string();

    let both_cnt: Vec<&str> = body_token.matches("both").collect();
    let both_cnt = both_cnt.len();

    let mut replace_parsers = Vec::new();
    for i in 0..both_cnt {
        let pos = body_token.find("both").unwrap();
        let (head, rest) = body_token.split_at(pos);
        if rest.starts_with("both_opt") {
            let rest = rest.replacen("both_opt", &format!("b_temporary{}", i), 1);
            body_token = format!("{}{}", head, rest);
            replace_parsers.push(("nom_both::some", "nom_both::none"));
        } else if rest.starts_with("both_alt") {
            let rest = rest.replacen("both_alt", &format!("b_temporary{}", i), 1);
            body_token = format!("{}{}", head, rest);
            replace_parsers.push(("nom_both::alt_left", "nom_both::alt_right"));
        }
    }

    let mut gen = quote! {};
    for i in 0..2_u32.pow(both_cnt as u32) {
        let mut body_token = body_token.clone();
        for j in 0..both_cnt {
            let (p0, p1) = replace_parsers[j];
            let repl = if ((i >> j) & 1) == 0 { p0 } else { p1 };
            body_token = body_token.replace(&format!("b_temporary{}", j), repl);
        }
        let body_token = format!("{{ {} }}", body_token);
        let body_token = TokenStream::from_str(&body_token).unwrap();
        let body_token = parse_macro_input!(body_token as Stmt);
        gen = quote! {
            #gen
            |#input| #body_token,
        };
    }

    let gen = quote! {
        {
            alt((
                #gen
            ))(s)
        }
    };
    gen.into()
}
