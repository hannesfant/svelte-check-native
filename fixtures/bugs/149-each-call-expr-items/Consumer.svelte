<script lang="ts">
    import Producer from './Producer.svelte'

    function takesNumber(n: number): void {
        void n
    }
    function takesString(s: string): void {
        void s
    }
</script>

<Producer let:id let:label>
    <svelte:fragment slot="default">
        {takesNumber(id)}
        {takesString(label)}
        <!-- Wrong shape — must fire TS2345. -->
        {takesString(id)}
    </svelte:fragment>
</Producer>
