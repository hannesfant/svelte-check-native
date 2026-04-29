<script lang="ts">
    import Inner from './Inner.svelte'

    function takesNumber(e: CustomEvent<number>): void {
        void e.detail
    }
    function takesString(e: CustomEvent<string>): void {
        void e.detail
    }
</script>

<Inner id="x" on:wrapped={takesNumber} />
<!-- Wrong shape: detail is number, not string. Must fire TS2345. -->
<Inner id="x" on:wrapped={takesString} />
