<script lang="ts">
    import Wrapper from './Wrapper.svelte'

    // Wrapper's `click` is the DOM bubble's MouseEvent (last source
    // wins per upstream Map.set semantics). Pre-fix this was the
    // intersection of MouseEvent and Inner's CustomEvent — neither
    // shape was usable, and most consumer handlers failed.
    function handleMouseClick(e: MouseEvent): void {
        void e.clientX
    }

    // A handler typed for Inner's CustomEvent shape is now WRONG —
    // the DOM bubble's MouseEvent overrode the component bubble.
    // Must fire TS2345.
    function handleCustomClick(e: CustomEvent<{ id: number }>): void {
        void e.detail.id
    }
</script>

<Wrapper on:click={handleMouseClick} />
<Wrapper on:click={handleCustomClick} />
