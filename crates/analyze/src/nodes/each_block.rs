//! `{#each}` analyze pass — mirrors upstream
//! `htmlxtojsx_v2/nodes/EachBlock.ts`.

use crate::template_walker::AnalyzeVisitor;

pub(crate) fn visit(v: &mut AnalyzeVisitor<'_>, b: &svn_parser::EachBlock) {
    v.summary.each_block_count += 1;
    v.pending_each_items_range = Some(b.expression_range);
}
