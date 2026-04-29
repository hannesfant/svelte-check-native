<script lang="ts">
    // Round-12 follow-up #6: extends round-11 #5's computed-key
    // handling.
    //   - String-literal computed key `['id']` → resolves like
    //     a static key (since the literal is known at synth time).
    //   - Bare-ident computed sibling in ObjectRest siblings list:
    //     `{ [k]: v, ...rest }` — `rest` should `Omit<…, typeof k>`.
    //
    // Pre-fix native dropped string-literal computed keys (no
    // segment pushed) and excluded bare-ident computed siblings
    // from the rest's exclusion list, so `rest` projection wrongly
    // included the corresponding slot.
    type Row = { id: number; label: string; extra: boolean }
    let { rows, k }: { rows: Row[]; k: 'id' | 'label' } = $props()
</script>

{#each rows as { ['id']: idAlias, [k]: keyed, ...rest }}
    <slot {idAlias} {keyed} {rest} />
{/each}
