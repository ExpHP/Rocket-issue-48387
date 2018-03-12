
use syntax::codemap::{Span};
use syntax::ast::{MetaItem};
use syntax::ext::base::{Annotatable, ExtCtxt};

pub fn route_decorator(
    _ecx: &mut ExtCtxt, _: Span, _: &MetaItem, _: Annotatable
) -> Vec<Annotatable> {
    loop { }
}

macro_rules! method_decorator {
    ($name:ident, $method:ident) => (
        pub fn $name(
            _: &mut ExtCtxt, _: Span, _: &MetaItem, _: Annotatable
        ) -> Vec<Annotatable> {
            loop { }
        }
    )
}

method_decorator!(get_decorator, Get);
method_decorator!(put_decorator, Put);
method_decorator!(post_decorator, Post);
method_decorator!(delete_decorator, Delete);
method_decorator!(head_decorator, Head);
method_decorator!(patch_decorator, Patch);
method_decorator!(options_decorator, Options);
