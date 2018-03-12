#![crate_type = "dylib"]
#![feature(quote, concat_idents, plugin_registrar, rustc_private)]
#![feature(iterator_for_each)]
#![feature(custom_attribute)]
#![feature(i128_type)]
#![allow(unused_attributes)]
#![allow(deprecated)]

extern crate syntax;
extern crate syntax_ext;
extern crate syntax_pos;
extern crate rustc_plugin;

use syntax::symbol::Symbol;
use syntax::ext::base::SyntaxExtension;
use rustc_plugin::Registry;

macro_rules! register_decorators {
    ($registry:expr, $($name:expr => $func:ident),+) => (
        $($registry.register_syntax_extension(Symbol::intern($name),
                SyntaxExtension::MultiModifier(Box::new(decorators::$func)));
         )+
    )
}

macro_rules! register_derives {
    ($registry:expr, $($name:expr => $func:ident),+) => (
        $($registry.register_custom_derive(Symbol::intern($name),
                SyntaxExtension::MultiDecorator(Box::new(decorators::$func)));
         )+
    )
}

macro_rules! register_macros {
    ($reg:expr, $($n:expr => $f:ident),+) => (
        $($reg.register_macro($n, macros::$f);)+
    )
}

mod decorators {
    use syntax::codemap::Span;
    use syntax::ext::base::{ExtCtxt, Annotatable};
    use syntax::ast::MetaItem;

    pub fn from_form_derive(_: &mut ExtCtxt, _: Span, _: &MetaItem, _: &Annotatable, _: &mut FnMut(Annotatable)) { loop { } }

    pub fn catch_decorator  (_: &mut ExtCtxt, _: Span, _: &MetaItem, _: Annotatable) -> Vec<Annotatable> { loop { } }
    pub fn route_decorator  (_: &mut ExtCtxt, _: Span, _: &MetaItem, _: Annotatable) -> Vec<Annotatable> { loop { } }
    pub fn get_decorator    (_: &mut ExtCtxt, _: Span, _: &MetaItem, _: Annotatable) -> Vec<Annotatable> { loop { } }
    pub fn put_decorator    (_: &mut ExtCtxt, _: Span, _: &MetaItem, _: Annotatable) -> Vec<Annotatable> { loop { } }
    pub fn post_decorator   (_: &mut ExtCtxt, _: Span, _: &MetaItem, _: Annotatable) -> Vec<Annotatable> { loop { } }
    pub fn delete_decorator (_: &mut ExtCtxt, _: Span, _: &MetaItem, _: Annotatable) -> Vec<Annotatable> { loop { } }
    pub fn head_decorator   (_: &mut ExtCtxt, _: Span, _: &MetaItem, _: Annotatable) -> Vec<Annotatable> { loop { } }
    pub fn patch_decorator  (_: &mut ExtCtxt, _: Span, _: &MetaItem, _: Annotatable) -> Vec<Annotatable> { loop { } }
    pub fn options_decorator(_: &mut ExtCtxt, _: Span, _: &MetaItem, _: Annotatable) -> Vec<Annotatable> { loop { } }
}

mod macros {
    use syntax::codemap::Span;
    use syntax::ext::base::{ExtCtxt, MacResult};
    use syntax::tokenstream::TokenTree;

    pub fn routes      (_: &mut ExtCtxt, _: Span, _: &[TokenTree]) -> Box<MacResult + 'static> { loop { } }
    pub fn catchers    (_: &mut ExtCtxt, _: Span, _: &[TokenTree]) -> Box<MacResult + 'static> { loop { } }
    pub fn uri         (_: &mut ExtCtxt, _: Span, _: &[TokenTree]) -> Box<MacResult + 'static> { loop { } }
    pub fn uri_internal(_: &mut ExtCtxt, _: Span, _: &[TokenTree]) -> Box<MacResult + 'static> { loop { } }
}

/// Compiler hook for Rust to register plugins.
#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    register_macros!(reg,
        "routes" => routes,
        "catchers" => catchers,
        "uri" => uri,
        "rocket_internal_uri" => uri_internal
    );

    register_derives!(reg,
        "derive_FromForm" => from_form_derive
    );

    register_decorators!(reg,
        "catch" => catch_decorator,
        "route" => route_decorator,
        "get" => get_decorator,
        "put" => put_decorator,
        "post" => post_decorator,
        "delete" => delete_decorator,
        "head" => head_decorator,
        "patch" => patch_decorator,
        "options" => options_decorator
    );
}
