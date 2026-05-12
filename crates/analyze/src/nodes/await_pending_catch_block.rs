//! `{#await}` / `{:then}` / `{:catch}` analyze pass — mirrors
//! upstream `htmlxtojsx_v2/nodes/AwaitPendingCatchBlock.ts`.
//!
//! Currently empty; `visit_await_block` plus the await-block
//! `enter_scope` arms migrate from `template_walker.rs` per
//! Steps 2 and 3 of `notes/TEMPLATE_WALKER_SPLIT.md`.
