<script lang="ts">
    // Round-7 follow-up #3: pre-fix native dropped destructured each-
    // block bindings out of the slot-attr resolver — `{#each rows as
    // { id }}` produced multiple bindings whose `b.name == "id"` but
    // no source expression match for "id", so the resolver returned
    // None and slot attrs referencing `id` were dropped from the
    // emitted slot literal entirely.
    //
    // Upstream's `slot.ts:resolveDestructuringAssignment` projects
    // each leaf as the IIFE
    //   `((${destructuring}) => ${id.name})(unwrapArr(items))`
    // which at type level reduces to the destructured property of
    // the element type. We mirror that with TS bracket-notation:
    //   `((typeof rows) extends Iterable<infer T> ? T : never)['id']`
    let { rows }: { rows: { id: number; label: string }[] } = $props()
</script>

{#each rows as { id, label }}
    <slot {id} {label} />
{/each}
