<script lang="ts">
    import Producer from './Producer.svelte'

    function takesNumber(n: number): void {
        void n
    }
    function takesString(s: string): void {
        void s
    }

    const rows = [
        { id: 1, label: 'a' },
        { id: 2 },
    ]
</script>

<Producer {rows} let:id let:label>
    {takesNumber(id)}
    <!-- label is `Exclude<string | undefined, undefined>` = `string`
         (the default narrowing). Pre-fix label was `string | undefined`
         and takesString(label) would have failed. -->
    {takesString(label)}
</Producer>
