use syntax::codemap::Span;
use syntax::tokenstream::TokenTree;
use syntax::ext::base::{ExtCtxt, MacResult};

#[rustfmt_skip]
pub fn routes(_: &mut ExtCtxt, _: Span, _: &[TokenTree])
        -> Box<MacResult + 'static> {
    loop { }
}

#[rustfmt_skip]
pub fn catchers(_: &mut ExtCtxt, _: Span, _: &[TokenTree])
        -> Box<MacResult + 'static> {
    loop { }
}

pub fn uri(
    _: &mut ExtCtxt,
    _: Span,
    _: &[TokenTree],
) -> Box<MacResult + 'static> {
    loop { }
}

pub fn uri_internal(
    _: &mut ExtCtxt,
    _: Span,
    _: &[TokenTree],
) -> Box<MacResult + 'static> {
    loop { }
}
