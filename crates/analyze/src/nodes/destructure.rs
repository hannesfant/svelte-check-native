//! Destructure / typeof / narrowing helpers — pure free functions
//! shared by the per-node passes. No direct upstream equivalent;
//! upstream inlines these inside each node-handler file.
//!
//! Currently empty; `items_typeof_expr`, `default_typeof_expr`,
//! `project_destructure_path`, `is_destructure`,
//! `is_typeof_safe_chain`, and the related helpers migrate from
//! `template_walker.rs` per Step 1 of
//! `notes/TEMPLATE_WALKER_SPLIT.md`.
