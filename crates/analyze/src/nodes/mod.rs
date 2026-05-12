//! Per-node analyze passes — one file per upstream `htmlxtojsx_v2`
//! node kind, matching the emit-side `crates/emit/src/nodes/` layout
//! file-for-file so a developer can navigate either side by basename.
//!
//! Scaffolded as part of `notes/TEMPLATE_WALKER_SPLIT.md` Step 0.
//! Modules are currently empty — content migrates in over the
//! subsequent commits described in that plan. Nothing in
//! `template_walker.rs` has moved yet.

pub mod action;
pub mod animation;
pub mod attribute;
pub mod await_pending_catch_block;
pub mod binding;
pub mod const_tag;
pub mod destructure;
pub mod each_block;
pub mod element;
pub mod event_handler;
pub mod if_else_block;
pub mod inline_component;
pub mod let_directive;
pub mod snippet_block;
pub mod svelte_element;
pub mod transition;
