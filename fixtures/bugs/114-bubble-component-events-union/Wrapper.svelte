<svelte:options runes />
<script lang="ts">
    // Reviewer follow-up #2 (round 4): when a wrapper bubbles the
    // SAME event name from multiple child components, the wrapper's
    // own `$$Events` surface should carry the UNION of every
    // source's declared event type. Pre-fix the dedup-by-name dropped
    // every bubble after the first — Radio's contribution
    // disappeared. Post-fix the wrapper's `click` event is
    // `__SvnComponentEvents<typeof Button>['click'] |
    //  __SvnComponentEvents<typeof Radio>['click']`.
    //
    // Mirrors upstream svelte2tsx
    // `__sveltets_2_unionType(__sveltets_2_bubbleEventDef(...),
    //                         __sveltets_2_bubbleEventDef(...))`
    // (`event-handler.ts:55-60`).
    import Button from './Button.svelte'
    import Radio from './Radio.svelte'
    let { label = '' }: { label?: string } = $props()
    void label
</script>

<Button on:click />
<Radio on:click />
