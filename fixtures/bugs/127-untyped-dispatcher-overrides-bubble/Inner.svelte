<script lang="ts">
    // Round-7 follow-up #5: when the SAME event name is BOTH a DOM
    // bubble (`<button on:click>`) AND an untyped dispatched name
    // (`dispatch('click', …)`), upstream's `toDefString` emits them
    // in this order:
    //   1. typed dispatcher spreads (none here)
    //   2. bubble entries — `'click': mapElementEvent('click')`
    //   3. untyped/duplicate-typed-collapse — `'click': customEvent`
    //
    // JS object-literal semantics: the LAST occurrence of each key
    // wins. So `click` ends up as `__sveltets_2_customEvent`
    // (declared `CustomEvent<any>`) — NOT the DOM event shape.
    //
    // Pre-round-7 native's `Omit<Dispatcher, keyof Bubble> & Bubble`
    // had the bubble override the dispatcher entirely, even for
    // untyped names. Post-fix native layers as
    // `Omit<Omit<Typed, keyof Bubble> & Bubble, keyof Untyped> &
    // Untyped` — untyped wins last.
    import { createEventDispatcher } from 'svelte'
    const dispatch = createEventDispatcher()
</script>

<button on:click={() => dispatch('click', { id: 1 })}>tap</button>
