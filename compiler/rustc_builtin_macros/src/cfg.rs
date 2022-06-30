//! Implementation of the `#[cfg(...)]` attribute macro.

#![allow(unused_imports)]

use rustc_ast as ast;
use rustc_errors::Applicability;
use rustc_expand::base::{Annotatable, ExpandResult, ExtCtxt, Indeterminate, MultiItemModifier};
use rustc_feature::AttributeTemplate;
use rustc_parse::validate_attr;
use rustc_session::Session;
use rustc_span::symbol::sym;
use rustc_span::Span;

pub(crate) struct Expander;

pub fn parse_cfg<'a>(meta_item: &'a ast::MetaItem, sess: &Session) -> Option<&'a ast::MetaItem> {
    let error = |span, msg, suggestion: &str| {
        let mut err = sess.parse_sess.span_diagnostic.struct_span_err(span, msg);
        if !suggestion.is_empty() {
            err.span_suggestion(
                span,
                "expected syntax is",
                suggestion,
                Applicability::HasPlaceholders,
            );
        }
        err.emit();
        None
    };
    let span = meta_item.span;
    match meta_item.meta_item_list() {
        None => error(span, "`cfg` is not followed by parentheses", "cfg(/* predicate */)"),
        Some([]) => error(span, "`cfg` predicate is not specified", ""),
        Some([_, .., l]) => error(l.span(), "multiple `cfg` predicates are specified", ""),
        Some([single]) => match single.meta_item() {
            Some(meta_item) => Some(meta_item),
            None => error(single.span(), "`cfg` predicate key cannot be a literal", ""),
        },
    }
}

impl MultiItemModifier for Expander {
    fn expand(
        &self,
        ecx: &mut ExtCtxt<'_>,
        _span: Span,
        meta_item: &ast::MetaItem,
        item: Annotatable,
    ) -> ExpandResult<Vec<Annotatable>, Annotatable> {
        let template = AttributeTemplate { list: Some("predicate"), ..Default::default() };
        let attr = &ecx.attribute(meta_item.clone());
        validate_attr::check_builtin_attribute(&ecx.sess.parse_sess, attr, sym::cfg, template);

        dbg!(&meta_item, &item);
        let Some(mi) = parse_cfg(meta_item, &ecx.sess) else {
            return ExpandResult::Ready(Vec::new());
        };

        // // Normal version
        // match rustc_attr::cfg_matches(&mi, &ecx.sess.parse_sess, ecx.current_expansion.lint_node_id, ecx.ecfg.features) {
        //     true => ExpandResult::Ready(vec![item]),
        //     false => ExpandResult::Ready(Vec::new()),
        // }

        // Extanded version (ie with accessible support)
        match rustc_attr::cfg_matches_extanded(
            &mi,
            &ecx.sess.parse_sess,
            ecx.current_expansion.lint_node_id,
            ecx.ecfg.features,
            &mut |path| ecx.resolver.cfg_accessible(ecx.current_expansion.id, path),
        ) {
            Ok(true) => ExpandResult::Ready(vec![item]),
            Ok(false) => ExpandResult::Ready(Vec::new()),
            Err(Indeterminate) if ecx.force_mode => {
                ecx.span_err(_span, "cannot determine whether the path is accessible or not");
                ExpandResult::Ready(vec![item])
            }
            Err(Indeterminate) => ExpandResult::Retry(item),
        }
    }
}
