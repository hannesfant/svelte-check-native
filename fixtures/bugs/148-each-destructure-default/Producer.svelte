<script lang="ts">
    // Round-12 follow-up #2: destructure defaults
    // (`{ a = 'fallback' }`) — pre-fix native projected `a` as
    // `parent['a']`, leaving the type as `string | undefined`
    // when the source slot is optional. Upstream's IIFE
    // `(({ a = 'x' }) => a)(source)` narrows to `string` (the
    // default kicks in when source.a is undefined).
    //
    // Native now wraps the projected leaf in `Exclude<…, undefined>`
    // when the binding sits under an AssignmentPattern. Doesn't
    // contribute the default's type, but at least removes
    // undefined from the narrowed slice.
    type Row = { id: number; label?: string }
    let { rows }: { rows: Row[] } = $props()
</script>

{#each rows as { id, label = 'fallback' }}
    <slot {id} {label} />
{/each}
