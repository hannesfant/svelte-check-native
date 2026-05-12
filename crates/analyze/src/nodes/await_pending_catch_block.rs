//! `{#await}` / `{:then}` / `{:catch}` analyze pass — mirrors
//! upstream `htmlxtojsx_v2/nodes/AwaitPendingCatchBlock.ts`.

use crate::template_walker::AnalyzeVisitor;

pub(crate) fn visit(v: &mut AnalyzeVisitor<'_>, b: &svn_parser::AwaitBlock) {
    v.pending_await_promise_range = Some(b.expression_range);
}
