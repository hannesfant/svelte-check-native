<svelte:options runes />
<script lang="ts">
    // Reviewer follow-up #1 (round 5): when a wrapper has BOTH a
    // `<Child on:click />` (component bubble) AND a `<button on:click>`
    // (DOM bubble) for the SAME event name, pre-fix the two were
    // built into separate type fragments and INTERSECTED:
    //   `(__SvnComponentEvents<typeof Child>['click']) & MouseEvent`
    // — neither shape is a usual handler argument and TS rejected most
    // user handlers.
    //
    // Upstream svelte2tsx runs all bubbles through one
    // `EventHandler.bubbledEvents` map. DOM bubbles call
    // `Map.set(name, ...)` which OVERWRITES; component bubbles
    // accumulate into a list per name that's later wrapped in
    // `__sveltets_2_unionType(...)`. So same-name DOM-then-component
    // ends up as the component's projection; component-then-DOM ends
    // up as the DOM projection.
    //
    // Post-fix: merge DOM and component bubbles into ONE flat map in
    // source-position order. DOM source REPLACES the entry; component
    // source after a Component entry APPENDS (union); component
    // after DOM REPLACES with the component projection.
    //
    // This fixture has the order Inner (component) → button (DOM).
    // The button's DOM bubble is the LAST source for `click`, so the
    // wrapper's `$$Events.click` should be `MouseEvent` (the DOM
    // event), not the intersection / not Inner's CustomEvent.
    import Inner from './Inner.svelte'
    let { label = '' }: { label?: string } = $props()
    void label
</script>

<Inner on:click />
<button on:click>{label}</button>
