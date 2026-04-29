<script lang="ts">
    import Producer from './Producer.svelte'

    function takesNumber(n: number): void {
        void n
    }
    function takesString(s: string): void {
        void s
    }

    const rows = [
        { id: 1, label: 'one' },
        { id: 2, label: 'two' },
    ]
</script>

<Producer {rows} let:id let:label>
    {takesNumber(id)}
    {takesString(label)}
    <!-- swap arg types — both must fire TS2345 -->
    {takesString(id)}
    {takesNumber(label)}
</Producer>
