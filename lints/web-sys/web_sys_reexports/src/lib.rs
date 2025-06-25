#![feature(rustc_private)]
#![feature(let_chains)]
#![warn(unused_extern_crates)]

extern crate rustc_hir;

use clippy_utils::diagnostics::span_lint_and_help;
use rustc_hir::{HirId, Item, ItemKind, Path, UseKind, UsePath};
use rustc_lint::{LateContext, LateLintPass};

const HELP_FURTHER_INFO: &str = concat!(
    "For further information visit",
    " https://github.com/mondeja/rs-apps-lints/tree/main/lints/web-sys/web_sys_reexports#readme"
);

dylint_linting::declare_late_lint! {
    /// ### What it does
    ///
    /// Check for usages of third party libraries re-exports from `web-sys`.
    ///
    /// ### Why is this bad?
    ///
    /// Uages of re-exports from `web-sys` can lead to dependency duplication,
    /// version conflicts and increase mantaintance burden.
    ///
    /// ### Known problems
    ///
    /// N/A
    ///
    /// ### Examples
    ///
    /// Don't use re-exports from `web-sys`:
    ///
    /// ```rust
    /// use web_sys::wasm_bindgen::JsCast;
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// use wasm_bindgen::JsCast;
    /// ```
    ///
    /// Don't use wildcard imports from `web-sys`:
    ///
    /// ```rust
    /// use web_sys::*;
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust
    /// use web_sys::HtmlElement;
    /// ```
    ///
    /// ### Discussion
    ///
    /// `web-sys` reexports the libraries `wasm-bindgen` and `js-sys`.
    /// This behaviour may has sense since `js-sys` and `wasm-bindgen` can be seen
    /// as subsets of the `web-sys` ecosystem. But re-exports should be avoided
    /// because it can lead to dependency duplication, version conflicts,
    /// increase mantaintance burden and reduce modularity in the codebase.
    pub WEB_SYS_REEXPORTS,
    Warn,
    "Check for usages of third party library re-exports from web-sys."
}

static FORBIDDEN_REEXPORTS: [&str; 2] = ["wasm_bindgen", "js_sys"];

impl WebSysReexports {
    fn lint_single_path<R>(cx: &LateContext, path: &Path<R>, web_sys_from_root: bool) {
        let second_segmment_index = if web_sys_from_root { 2 } else { 1 };
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
                    WEB_SYS_REEXPORTS,
                    span,
                    "usage of a third party library re-export from `web_sys`",
                    None,
                    format!("consider using `{rewrite_path_str}` instead. {HELP_FURTHER_INFO}"),
                );
            }
        }
    }
}

impl LateLintPass<'_> for WebSysReexports {
    fn check_item(&mut self, cx: &LateContext, item: &Item) {
        if let Some((path, use_kind, web_sys_from_root)) = is_web_sys_use_item(item) {
            match use_kind {
                UseKind::Glob => {
                    let second_segmment_index = if web_sys_from_root { 2 } else { 1 };
                    if path.segments.get(second_segmment_index).is_none() {
                        // use web_sys::*;
                        span_lint_and_help(
                            cx,
                            WEB_SYS_REEXPORTS,
                            item.span,
                            "usage of a third party library re-export from `web_sys`",
                            None,
                            format!("do not import using `web_sys::*`. {HELP_FURTHER_INFO}"),
                        );
                    }
                }
                UseKind::ListStem | UseKind::Single(_) => {}
            }
        }
    }

    fn check_path(&mut self, cx: &LateContext, path: &Path, _: HirId) {
        if let Some(web_sys_from_root) = is_web_sys_path(path) {
            WebSysReexports::lint_single_path(cx, path, web_sys_from_root);
        }
    }
}

fn is_web_sys_use_item<'a>(item: &'a Item) -> Option<(&'a UsePath<'a>, UseKind, bool)> {
    if let ItemKind::Use(path, use_kind) = item.kind {
        match (path.segments.first(), path.segments.get(1)) {
            (Some(first_segment), Some(second_segment)) => {
                let first_segment_name = first_segment.ident.name.as_str();
                let second_segment_name = second_segment.ident.name.as_str();
                if first_segment_name == "web_sys" {
                    return Some((path, use_kind, false));
                } else if first_segment_name == "{{root}}" && second_segment_name == "web_sys" {
                    return Some((path, use_kind, true));
                }
            }
            (Some(first_segment), None) => {
                let first_segment_name = first_segment.ident.name.as_str();
                if first_segment_name == "web_sys" {
                    return Some((path, use_kind, false));
                }
            }
            _ => {}
        }
    }
    None
}

fn is_web_sys_path(path: &Path) -> Option<bool> {
    match (path.segments.first(), path.segments.get(1)) {
        (Some(first_segment), Some(second_segment)) => {
            let first_segment_name = first_segment.ident.name.as_str();
            let second_segment_name = second_segment.ident.name.as_str();
            if first_segment_name == "web_sys" {
                return Some(false);
            } else if first_segment_name == "{{root}}" && second_segment_name == "web_sys" {
                return Some(true);
            }
        }
        (Some(first_segment), None) => {
            let first_segment_name = first_segment.ident.name.as_str();
            if first_segment_name == "web_sys" {
                return Some(false);
            }
        }
        _ => {}
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
