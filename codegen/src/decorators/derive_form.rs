use syntax::ext::base::{Annotatable, ExtCtxt};
use syntax::ast::{MetaItem};
use syntax::codemap::Span;

// TODO: Use proper logging to emit the error messages.
pub fn from_form_derive(
    _: &mut ExtCtxt,
    _: Span,
    _: &MetaItem,
    _: &Annotatable,
    _: &mut FnMut(Annotatable)
) {
    loop { }
}
