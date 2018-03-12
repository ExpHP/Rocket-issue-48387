mod meta_item_ext;
mod parser_ext;
mod ident_ext;
mod span_ext;
mod expr_ext;

pub use self::meta_item_ext::MetaItemExt;
pub use self::parser_ext::ParserExt;
pub use self::ident_ext::IdentExt;
pub use self::span_ext::SpanExt;
pub use self::expr_ext::ExprExt;

use syntax::parse::token::Token;
use syntax::tokenstream::TokenTree;
use syntax::ast::{Ident};
use syntax::ext::base::{ExtCtxt};
use syntax::codemap::{DUMMY_SP};
use syntax::ext::quote::rt::ToTokens;
use syntax::ptr::P;
use syntax::fold::Folder;
use syntax::ast::{Lifetime, LifetimeDef, Ty};

pub fn sep_by_tok<T>(ecx: &ExtCtxt, things: &[T], token: Token) -> Vec<TokenTree>
    where T: ToTokens
{
    let mut output: Vec<TokenTree> = vec![];
    for (i, thing) in things.iter().enumerate() {
        output.extend(thing.to_tokens(ecx));
        if i < things.len() - 1 {
            output.push(TokenTree::Token(DUMMY_SP, token.clone()));
        }
    }

    output
}


pub struct TyLifetimeRemover;

// FIXME: Doesn't work for T + whatever.
impl Folder for TyLifetimeRemover {
    fn fold_opt_lifetime(&mut self, _: Option<Lifetime>) -> Option<Lifetime> {
        None
    }

    fn fold_lifetime_defs(&mut self, _: Vec<LifetimeDef>) -> Vec<LifetimeDef> {
        vec![]
    }

    fn fold_lifetimes(&mut self, _: Vec<Lifetime>) -> Vec<Lifetime> {
        vec![]
    }
}

pub fn strip_ty_lifetimes(ty: P<Ty>) -> P<Ty> {
    TyLifetimeRemover.fold_ty(ty)
}

pub fn split_idents(path: &str) -> Vec<Ident> {
    path.split("::").map(|segment| Ident::from_str(segment)).collect()
}

macro_rules! try_parse {
    ($sp:expr, $parse:expr) => (
        match $parse {
            Ok(v) => v,
            Err(mut e) => { e.emit(); return DummyResult::expr($sp); }
        }
    )
}

macro_rules! p {
    ("parameter", $num:expr) => (
        if $num == 1 { "parameter" } else { "parameters" }
    );

    ($num:expr, "was") => (
        if $num == 1 { "1 was".into() } else { format!("{} were", $num) }
    );

    ($num:expr, "parameter") => (
        if $num == 1 { "1 parameter".into() } else { format!("{} parameters", $num) }
    )
}
