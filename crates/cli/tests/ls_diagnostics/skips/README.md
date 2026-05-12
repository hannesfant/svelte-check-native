# LS-diagnostic fixture skips

One JSON file per upstream LS fixture we deliberately skip. The
basename (sans `.json`) is the fixture-directory name as it appears
under
`language-tools/packages/language-server/test/plugins/typescript/features/diagnostics/fixtures/`.

The runner (`../run.cjs`) loads this directory at startup and treats
the resulting map exactly like the previous hard-coded `SKIP_LIST`
constant. Adding a new skip = drop a JSON here, no code change.
Closing a skip = `git rm` the file (the stale-skip detector flags
fixtures that pass strict but are still listed).

## Schema

```json
{
  "bucket": "missing-code",
  "reason": "6385/6387 deprecation hints not surfaced"
}
```

Both fields are required. The printed scoreboard joins them as
`"<bucket>: <reason>"`.

## Buckets

| bucket               | meaning                                                                         |
| -------------------- | ------------------------------------------------------------------------------- |
| `missing-code`       | TS code our typecheck doesn't surface in this scenario (deprecation/unused hints filtered, parser-error path differs, etc.). |
| `shim-resolution`    | TS2307 "cannot find module 'svelte'" cascade — the LS fixtures dir has no `svelte` package installed; upstream LS resolves it in-process. |
| `overlay-counts`     | Same code categories as upstream but our overlay produces a different count per source position. |
| `tsgo-divergence`    | Tsgo emits a different code than upstream's LS for the same trigger.            |
| `namespace-handling` | Fixture relies on a JSX-namespace switch (`svelteOptions.namespace=...`) that our overlay doesn't model. |

(See the bucket comments in `../run.cjs` for the full prose definitions.)

## Upstream-excluded vs skipped

`node16` and `project-reference` are in `UPSTREAM_EXCLUDED` (the
upstream root tsconfig excludes them) — those don't get JSON files
here. The runner falls back to the generic
"upstream-root tsconfig excludes it" reason for those.
