
use syntax::codemap::{Span};
use syntax::ast::{MetaItem};
use syntax::ext::base::{Annotatable, ExtCtxt};

pub fn catch_decorator(
    _ecx: &mut ExtCtxt,
    _sp: Span,
    _meta_item: &MetaItem,
    _annotated: Annotatable
) -> Vec<Annotatable> {
    loop { }
}
