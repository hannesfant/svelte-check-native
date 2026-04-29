<script lang="ts">
    // Round-12 follow-up #1: `{#each <call-expression> as ...}` — pre-
    // fix native emitted raw `typeof getRows()` which is invalid TS
    // (typeof requires an identifier or qualified name). The slot
    // type then failed to parse and either crashed tsgo's whole-
    // file check or fell through silently.
    //
    // Native now detects the call shape and emits
    // `ReturnType<typeof getRows>` instead. For non-call non-
    // typeof-safe expressions (chained calls, indexing, ternaries),
    // it falls back to `any` so the slot type stays parseable.
    function getRows(): { id: number; label: string }[] {
        return [
            { id: 1, label: 'a' },
            { id: 2, label: 'b' },
        ]
    }
    let { _filler = '' }: { _filler?: string } = $props()
    void _filler
</script>

{#each getRows() as { id, label }}
    <slot {id} {label} />
{/each}
