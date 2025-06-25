#![feature(rustc_private)]
#![feature(let_chains)]
#![warn(unused_extern_crates)]

extern crate rustc_hir;

use clippy_utils::diagnostics::span_lint_and_help;
use rustc_hir::{Expr, ExprKind, Item, ItemKind, Path, QPath, UseKind, UsePath};
use rustc_lint::{LateContext, LateLintPass};

const HELP_FURTHER_INFO: &str = concat!(
    "For further information visit",
    " https://github.com/mondeja/rs-apps-lints/tree/main/lints/leptos-more/leptos_reexports#readme"
);

dylint_linting::declare_late_lint! {
    /// ### What it does
    ///
    /// Check for usages of third party libraries re-exports from `leptos`.
    ///
    /// ### Why is this bad?
    ///
    /// Uages of re-exports from `leptos` can lead to dependency duplication,
    /// version conflicts and increase mantaintance burden.
    ///
    /// ### Known problems
    ///
    /// N/A
    ///
    /// ### Examples
    ///
    /// Don't use re-exports from `leptos`:
    ///
    /// ```rust
    /// use leptos::wasm_bindgen::JsCast;
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// use wasm_bindgen::JsCast;
    /// ```
    ///
    /// Don't use wildcard imports from `leptos`:
    ///
    /// ```rust
    /// use leptos::*;
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// use leptos::html;
    /// ```
    ///
    /// ### Discussion
    ///
    /// Hygienic re-exports for third party libraries used by leptos' proc-macros
    /// have been proposed in [leptos#4069].
    /// Currently, is a breaking change that probably is postponed or maybe never
    /// merged because the creator of Leptos feel comfortable about using these
    /// re-exports.
    ///
    /// [leptos#4069]: https://github.com/leptos-rs/leptos/pull/4069
    pub LEPTOS_REEXPORTS,
    Warn,
    "Check for usages of third party library re-exports from leptos."
}

static FORBIDDEN_REEXPORTS: [&str; 5] =
    ["wasm_bindgen", "web_sys", "tracing", "serde", "serde_json"];

impl LeptosReexports {
    fn lint_single_path<R>(
        &self,
        cx: &LateContext,
        path: &Path<R>,
        leptos_from_root: bool,
        help_rewrite_prefix: &str,
    ) {
        let second_segmment_index = if leptos_from_root { 2 } else { 1 };
        if let Some(second_segment) = path.segments.get(second_segmment_index) {
            let name = second_segment.ident.name.as_str();
            if FORBIDDEN_REEXPORTS.contains(&name) {
                let span = second_segment.ident.span;
                let second_and_next_segments = &path.segments[1..];
                let rewrite_path = second_and_next_segments
                    .iter()
                    .map(|s| s.ident.name.as_str())
                    .collect::<Vec<_>>();
                let rewrite_path_str = rewrite_path.join("::");
                span_lint_and_help(
                    cx,
                    LEPTOS_REEXPORTS,
                    span,
                    "usage of a third party library re-export from `leptos`",
                    None,
                    format!(
                        "consider using `{help_rewrite_prefix}{rewrite_path_str}` instead. {HELP_FURTHER_INFO}"
                    ),
                );
            }
        }
    }
}

impl<'tctx> LateLintPass<'tctx> for LeptosReexports {
    fn check_item(&mut self, cx: &LateContext, item: &Item) {
        if let Some((path, use_kind, leptos_from_root)) = is_leptos_use_item(item) {
            match use_kind {
                // use leptos::wasm_bindgen::JsCast;
                UseKind::Single(_) => self.lint_single_path(cx, path, leptos_from_root, "use "),
                UseKind::Glob => {
                    let second_segmment_index = if leptos_from_root { 2 } else { 1 };
                    if let Some(second_segment) = path.segments.get(second_segmment_index) {
                        let name = second_segment.ident.name.as_str();
                        // use leptos::web_sys::*;
                        if FORBIDDEN_REEXPORTS.contains(&name) {
                            let span = second_segment.ident.span;
                            let second_and_next_segments = &path.segments[1..];
                            let mut rewrite_path = second_and_next_segments
                                .iter()
                                .map(|s| s.ident.name.as_str())
                                .collect::<Vec<_>>();
                            rewrite_path.push("*");
                            let rewrite_path_str = rewrite_path.join("::");
                            span_lint_and_help(
                                cx,
                                LEPTOS_REEXPORTS,
                                span,
                                "usage of a third party library re-export from `leptos`",
                                None,
                                format!(
                                    "consider using `use {rewrite_path_str}` instead. {HELP_FURTHER_INFO}"
                                ),
                            )
                        }
                    } else {
                        // use leptos::*;
                        span_lint_and_help(
                            cx,
                            LEPTOS_REEXPORTS,
                            item.span,
                            "usage of a third party library re-export from `leptos`",
                            None,
                            format!("do not import using `leptos::*`. {HELP_FURTHER_INFO}"),
                        );
                    }
                }
                // it seems that degenerate list stem is never matching
                _ => {}
            }
        }
    }

    fn check_expr(&mut self, cx: &LateContext, expr: &Expr) {
        if let Some((path, leptos_from_root)) = is_leptos_path_expr(expr) {
            self.lint_single_path(cx, path, leptos_from_root, "");
        }
    }
}

fn is_leptos_use_item<'a>(item: &'a Item) -> Option<(&'a UsePath<'a>, UseKind, bool)> {
    if let ItemKind::Use(path, use_kind) = item.kind {
        match (path.segments.get(0), path.segments.get(1)) {
            (Some(first_segment), Some(second_segment)) => {
                let first_segment_name = first_segment.ident.name.as_str();
                let second_segment_name = second_segment.ident.name.as_str();
                if first_segment_name == "leptos" {
                    return Some((path, use_kind, false));
                } else if first_segment_name == "{{root}}" && second_segment_name == "leptos" {
                    return Some((path, use_kind, true));
                }
            }
            (Some(first_segment), None) => {
                let first_segment_name = first_segment.ident.name.as_str();
                if first_segment_name == "leptos" {
                    return Some((path, use_kind, false));
                }
            }
            _ => {}
        }
    }
    None
}

fn is_leptos_path_expr<'a>(expr: &'a Expr) -> Option<(&'a Path<'a>, bool)> {
    if let ExprKind::Path(qpath) = expr.kind {
        if let QPath::Resolved(_, path) = qpath {
            match (path.segments.get(0), path.segments.get(1)) {
                (Some(first_segment), Some(second_segment)) => {
                    let first_segment_name = first_segment.ident.name.as_str();
                    let second_segment_name = second_segment.ident.name.as_str();
                    if first_segment_name == "leptos" {
                        return Some((path, false));
                    } else if first_segment_name == "{{root}}" && second_segment_name == "leptos" {
                        return Some((path, true));
                    }
                }
                (Some(first_segment), None) => {
                    let first_segment_name = first_segment.ident.name.as_str();
                    if first_segment_name == "leptos" {
                        return Some((path, false));
                    }
                }
                _ => {}
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    #[test]
    fn ui() {
        dylint_testing::ui_test(env!("CARGO_PKG_NAME"), "tests/ui");
    }
}
