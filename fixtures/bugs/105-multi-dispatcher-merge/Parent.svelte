<svelte:options runes />
<script lang="ts">
    import Child from './Child.svelte'

    function handleA(e: CustomEvent<{ x: number }>): void {
        void e.detail.x
    }
    function handleB(e: CustomEvent<{ y: string }>): void {
        void e.detail.y
    }
    function handleBye(e: CustomEvent<any>): void {
        void e.detail
    }
    function wrongHandlerForA(e: number): void {
        void e
    }
</script>

<!-- Each handler matches its dispatcher's typed detail. -->
<Child on:a={handleA} on:b={handleB} on:bye={handleBye} />

<!-- Wrong-shape handler for `a`: `(e: number) => void` doesn't accept
     `CustomEvent<{x: number}>`. Must fire TS2345 at the directive
     value position. Pre-fix this MAY have passed silently because
     the merge wasn't in place — only `a` was synthesised, but
     dropping `b` / `bye` would have made consumers' on:b /
     on:bye fail too. -->
<Child on:a={wrongHandlerForA} />
