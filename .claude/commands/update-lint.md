Check whether upstream `sveltejs/svelte` (the compiler repo) has
changed in ways that affect our native compile-warning port
(`crates/svn-lint/`).

Companion to `/update-check` (which tracks `sveltejs/language-tools`).
This one tracks the warning dispatcher, message templates, fire-
sites, and test fixtures that our Rust port mirrors.

## Steps (do not skip or reorder)

1. Read the pinned SHA from `.upstream-pin` — the `sha = "..."` line
   under `[svelte-compiler]`. Call this `<PINNED>`.

2. Refresh the upstream submodule. The upstream repo lives at
   `.svelte-upstream/svelte/` (blobless, sparse-checkout of
   `packages/svelte/{messages,src/compiler,tests/validator}`).
   ```
   git -C .svelte-upstream/svelte fetch origin
   ```
   Resolve `origin/HEAD`'s SHA — call this `<NEW>`. If `<NEW>` ==
   `<PINNED>`, report "no upstream compiler changes since last
   review" and stop.

3. List compile-warning-relevant commits landed since the pin:
   ```
   git -C .svelte-upstream/svelte log --oneline <PINNED>..origin/HEAD -- \
     packages/svelte/messages/compile-warnings/ \
     packages/svelte/src/compiler/warnings.js \
     packages/svelte/src/compiler/phases/2-analyze/ \
     packages/svelte/src/compiler/phases/1-parse/ \
     packages/svelte/tests/validator/
   ```

4. Diff each of the four surfaces our port depends on. Run these in
   parallel and capture summaries.

   **Surface A — message templates (primary source for codegen):**
   ```
   git -C .svelte-upstream/svelte diff --stat <PINNED>..origin/HEAD \
     -- packages/svelte/messages/compile-warnings/
   git -C .svelte-upstream/svelte diff <PINNED>..origin/HEAD \
     -- packages/svelte/messages/compile-warnings/
   ```
   If there's ANY change, regenerate the Rust catalog:
   ```
   cargo run -p xtask --bin regen-lint-catalog
   ```
   Review the resulting `crates/svn-lint/src/codes.rs` + `messages.rs`
   diff.

   **Surface B — warnings.js (code list sanity check):**
   ```
   git -C .svelte-upstream/svelte diff <PINNED>..origin/HEAD \
     -- packages/svelte/src/compiler/warnings.js
   ```
   Any `+ export function CODE` line means a new warning was added
   upstream and must be ported before bumping the pin. Any `-` means
   a warning was removed; delete the rule + fixture.

   **Surface C — analyze visitors (fire-site drift):**
   ```
   git -C .svelte-upstream/svelte diff --stat <PINNED>..origin/HEAD \
     -- packages/svelte/src/compiler/phases/2-analyze/visitors/
   git -C .svelte-upstream/svelte diff <PINNED>..origin/HEAD \
     -- packages/svelte/src/compiler/phases/2-analyze/visitors/shared/a11y/
   ```
   For each touched visitor, skim the diff. Classify each commit
   into one of:

   - **New fire-site** (`+ w.<code>(...)`) for an existing code →
     port the new condition into `crates/svn-lint/src/rules/`.
   - **False-positive silencing** (gate added around existing
     `w.<code>`) → mirror the gate.
   - **New code referenced** → requires Surface A/B work first.
   - **Pure refactor** (zero behavior change) → no action.

   The a11y subsystem is special — any change there blocks on
   re-running `cargo test -p svn-lint --test upstream_validator`
   with the a11y codes ported. Flag for manual review.

   **Surface D — upstream validator fixtures (primary test corpus):**
   ```
   git -C .svelte-upstream/svelte diff --name-status <PINNED>..origin/HEAD \
     -- packages/svelte/tests/validator/samples/
   ```
   New fixture directories start running immediately after the pin
   bump. If a new fixture has a code we haven't ported, the runner
   will SKIP it (won't fail); if all codes are ported and we
   disagree with the expected output, the runner FAILS.

   Modified `warnings.json` files (e.g. after an upstream message
   reword) need the catalog regeneration from Surface A — check
   that was done.

   Deleted fixtures: nothing to do; the runner stops running them
   automatically.

5. Run the regression gate:
   ```
   cargo test -p svn-lint --test upstream_validator
   ```
   Confirm it still passes. The runner enforces fixtures whose
   codes are all in `PORTED_CODES` (see
   `crates/svn-lint/tests/upstream_validator.rs`). If anything
   fails, STOP — either the port diverged, or upstream changed a
   message/behavior we need to follow.

6. Run the broader test suite:
   ```
   cargo test -p svn-lint
   cargo test -p svelte-check-native
   ```

7. If all gates green, bump the pin:
   ```
   # Edit .upstream-pin, replace [svelte-compiler].sha with <NEW>.
   # Update the `note = "..."` line with the new version tag if
   # origin/main is close to a tagged release.
   ```

8. Commit:
   ```
   git add .upstream-pin .svelte-upstream/svelte \
          crates/svn-lint/src/codes.rs \
          crates/svn-lint/src/messages.rs \
          crates/svn-lint/tests/upstream_validator.rs  # if PORTED_CODES grew
   git commit -m "lint: bump svelte-compiler pin to <NEW_SHORT_SHA>"
   ```

## Important notes

- **Never bump the pin without running the upstream_validator test.**
  That's the whole point of having the corpus in-place — don't let
  the pin drift past a fixture we'd fail on.
- **Don't port new rules defensively.** If a new warning appears
  upstream but has no real-world fixture, add it to the TODO list
  and decide priority before spending time on it. The long-tail of
  rarely-fired warnings isn't worth the drift maintenance.
- **Message text fidelity matters.** Tier A's parity check is exact
  string match (sans docs URL). If you skip regen-lint-catalog
  after an upstream message reword, the runner will go red.
- **The clone is a submodule.** `.svelte-upstream/svelte` is a
  shallow + sparse-checkout submodule (gitlinked at the build/test
  pin). `.upstream-pin` tracks the LAST-REVIEWED sha — distinct
  from the gitlink (which is what we currently build/test against).
  A fresh checkout needs:
  ```
  git submodule update --init --recursive .svelte-upstream/svelte
  ```
  Sparse-checkout paths are configured in
  `.svelte-upstream/svelte/.git/info/sparse-checkout`:
  `packages/svelte/messages/`, `packages/svelte/src/compiler/`,
  `packages/svelte/tests/validator/`.
