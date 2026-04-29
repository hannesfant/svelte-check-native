<script lang="ts">
    // Round-13 follow-up #4: destructure default's expression-type
    // gets unioned into the projected leaf. Pre-fix native only
    // narrowed `Exclude<…, undefined>` — for `{:then v = 0}` where
    // the awaited value can be undefined, the narrow gave
    // `Exclude<undefined, undefined>` = `never`, leaving the
    // binding `v` typed as `never` and rejecting every consumer
    // use.
    //
    // Upstream's IIFE `(({a = 0}) => a)(source)` types the local as
    // `<source-type-minus-undefined> | <fallback-type>`. Native now
    // mirrors via `default_typeof_expr` which extracts the literal/
    // identifier/typeof-safe-call type from the default's source
    // and unions it into the projected leaf.
    let { promise }: { promise: Promise<{ value?: number }> } = $props()
</script>

{#await promise then { value = 42 }}
    <slot {value} />
{/await}
