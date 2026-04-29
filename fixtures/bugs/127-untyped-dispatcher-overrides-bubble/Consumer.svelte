<script lang="ts">
    import Inner from './Inner.svelte'

    // `click` ends up as `CustomEvent<any>` (untyped dispatched name
    // wins per upstream's emit order: typed → bubble → untyped). A
    // CustomEvent handler passes.
    function handleCustom(e: CustomEvent): void {
        void e.detail
    }

    // A handler typed for MouseEvent (the DOM bubble's projection)
    // is now WRONG — the untyped dispatched name overrode the bubble.
    // Must fire TS2345.
    function handleMouse(e: MouseEvent): void {
        void e.clientX
    }
</script>

<Inner on:click={handleCustom} />
<Inner on:click={handleMouse} />
