<svelte:options runes />
<script lang="ts">
    // Reviewer follow-up #6: bare `on:NAME` on `<svelte:body>` /
    // `<svelte:window>` (no value) forwards the native DOM event to a
    // parent listener. Pre-fix the bubbled-DOM-event collector
    // explicitly skipped both — it only handled regular `Element` kind
    // — so the consumer's `<Child on:resize={cb}>` typed as
    // `(...args: any[]) => any` and any handler shape passed.
    //
    // Upstream svelte2tsx `event-handler.ts:63-72` routes:
    //   - Element  → `__sveltets_2_mapElementEvent` (HTMLElementEventMap)
    //   - Body     → `__sveltets_2_mapBodyEvent` (HTMLBodyElementEventMap)
    //   - Window   → `__sveltets_2_mapWindowEvent` (WindowEventMap)
    //
    // Post-fix the collector dispatches on the SvelteElementKind so
    // the synthesised `$$Events` projects each name into its proper
    // event-map source.
    let { label = '' }: { label?: string } = $props()
</script>

<svelte:body on:click />
<svelte:window on:resize />

<p>{label}</p>
