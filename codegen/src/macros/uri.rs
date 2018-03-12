
use syntax::codemap::Span;
use syntax::ext::base::{ExtCtxt, MacResult};
use syntax::tokenstream::{TokenTree};

pub fn uri(
    _: &mut ExtCtxt,
    _: Span,
    _: &[TokenTree],
) -> Box<MacResult + 'static> {
    loop { }
}

#[allow(unused_imports)]
pub fn uri_internal(
    _: &mut ExtCtxt,
    _: Span,
    _: &[TokenTree],
) -> Box<MacResult + 'static> {
    loop { }
}
