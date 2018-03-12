#![allow(unused_imports)] // FIXME: Why is this coming from quote_tokens?

use std::mem::transmute;
use std::collections::HashMap;

use syntax::ext::base::{Annotatable, ExtCtxt};
use syntax::print::pprust::{stmt_to_string};
use syntax::ast::{ItemKind, Expr, MetaItem, Mutability, VariantData, Ident};
use syntax::ast::{StructField, GenericParam};
use syntax::codemap::Span;
use syntax::ext::build::AstBuilder;
use syntax::ptr::P;

use syntax_ext::deriving::generic::MethodDef;
use syntax_ext::deriving::generic::{StaticStruct, Substructure, TraitDef, ty};
use syntax_ext::deriving::generic::combine_substructure as c_s;

use utils::{strip_ty_lifetimes, SpanExt};

// TODO: Use proper logging to emit the error messages.
pub fn from_form_derive(
    _ecx: &mut ExtCtxt,
    _span: Span,
    _meta_item: &MetaItem,
    _annotated: &Annotatable,
    _push: &mut FnMut(Annotatable)
) {
    loop { }
}
