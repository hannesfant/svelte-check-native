# svn-lint validator-fixture coverage gaps

This file enumerates upstream `packages/svelte/tests/validator/`
fixtures that `cargo test -p svn-lint --test upstream_validator`
**explicitly skips with reason** rather than enforces. The aim is
to keep the skip set small, every entry traceable to either a
future feature gate or an upstream-side waiver.

A fixture is enforced only when (a) it has an `input.svelte`
source (not `input.svelte.js` — module-mode lint pass is Phase C),
(b) its `_config.js` doesn't reference unsupported compile options,
and (c) every code in its `warnings.json` is on `PORTED_CODES`.
This file documents the (b) bucket — (a) and (c) decay
automatically as JS-pass and individual rules land.

## Compile-option-gated (4 fixtures)

The runner currently skips fixtures whose `_config.js` references:
`skip: true` (upstream's own waiver), `warningFilter`,
`customElement`, `immutable`. Reasoning per fixture:

### `error-mode-warn`

```js
export default test({ skip: true });
```

Upstream itself doesn't run this fixture. It exercises the legacy
"warn, don't error" mode for `invalid_const_assignment` and
`bind_invalid_value` — both compile-stop errors in modern Svelte 5
runes mode, which is why upstream gated it off. Mirroring that
waiver is correct; this fixture should stay skipped permanently.

### `general-siblings-combinator-in-custom-element-selects-slot-fallback` and `siblings-combinator-in-custom-element-selects-slot-fallback`

Both gate on `compileOptions: { customElement: true }`. They assert
that `css_unused_selector` fires on a `h1 ~ slot > p` (resp.
`h1 + slot > span`) selector when the component compiles as a
custom element. Two things missing:

1. **`css_unused_selector` rule itself.** Listed in
   `crates/svn-lint/src/codes.rs` but not on `PORTED_CODES` — the
   CSS unused-selector analysis is a separate workstream from the
   per-element/per-attribute rules currently ported.
2. **`compileOptions.customElement` plumbing.** The lint pass
   takes `CompatFeatures` today, not a structured compile-options
   record. Custom-element mode changes which CSS selectors are
   considered "used" (slot fallback content is reachable); without
   that bit we can't assert the upstream behaviour.

Future work: when the CSS-unused-selector rule lands, plumb a
`custom_element` flag into `LintContext` and remove the
`customElement` gate from `_config.js` filtering.

### `ignore-warning-compiler-option`

Gates on `compileOptions: { warningFilter: (w) => !['a11y_missing_attribute', 'a11y_misplaced_scope'].includes(w.code) }`.
Tests that user-provided filter functions suppress warnings
upstream's compiler would otherwise emit.

We don't expose a programmatic warning-filter surface (and may
never — `svelte-check`'s CLI consumes the filter in JS, but we
run as a standalone Rust binary). The natural mapping is the CLI
`--ignore` family if/when we add it; until then this fixture
stays skipped.

## Module-mode (input.svelte.js)

Currently 0 fixtures hit this bucket because the gating happens
upstream of warnings.json (module-only fixtures are typically
errors-only). If a future fixture lands with both
`input.svelte.js` + `warnings.json`, it'll surface in the
`module-only` skip bucket and need re-triage when the JS-pass
lint surface lands (Phase C in the original ROADMAP).

## Unported codes

Currently 0 fixtures hit this bucket — every fixture's expected
codes are on `PORTED_CODES`. If a new upstream fixture lands with
a code we haven't ported, the runner will list both the fixture
and the uncovered code in its end-of-run scoreboard. Remove from
this bucket by either porting the rule (preferred) or adding a
`SKIP_UNPORTED` allowlist with a tracking issue (deferred).

## Maintenance

The skip-list size should monotonically decrease on average. A
new entry here should always carry a tracking pointer (issue,
ROADMAP item, or — for the few permanent waivers — an explicit
"upstream itself doesn't run this" line).
