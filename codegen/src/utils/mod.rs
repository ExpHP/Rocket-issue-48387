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
use syntax::ext::base::{ExtCtxt};
use syntax::codemap::{DUMMY_SP};
use syntax::ext::quote::rt::ToTokens;

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

