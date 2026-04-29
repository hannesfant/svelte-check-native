<script lang="ts">
    import Producer from './Producer.svelte'

    function takesNumber(n: number): void {
        void n
    }
    function takesString(s: string): void {
        void s
    }

    const promise: Promise<{ value?: number }> = Promise.resolve({})
</script>

<Producer {promise} let:value>
    {takesNumber(value)}
    <!-- value is `number | 42` (= number). Wrong shape — must fire TS2345. -->
    {takesString(value)}
</Producer>
