<script lang="ts">
    import Child from './Child.svelte'

    // Wrong handler: declared `(e: number)`, can't accept
    // `CustomEvent<{id: number}>`. Post-fix this fires TS2345 because
    // AST-based detection saw `type $$Events=` (no whitespace before
    // `=`) and routed through the strict path.
    function wrong(e: number): void {
        void e
    }
</script>

<Child on:click={wrong} />
