<script lang="ts">
    import Producer from './Producer.svelte'

    function takesNumber(n: number): void {
        void n
    }
    function takesUnion(v: number | string): void {
        void v
    }
    function takesBoolean(b: boolean): void {
        void b
    }

    const rows = [
        { id: 1, label: 'a', extra: true },
        { id: 2, label: 'b', extra: false },
    ]
    const k: 'id' | 'label' = 'id'
</script>

<Producer {rows} {k} let:idAlias let:keyed let:rest>
    {takesNumber(idAlias)}
    {takesUnion(keyed)}
    {takesBoolean(rest.extra)}
    <!-- rest excludes both 'id' and typeof k. Accessing rest.id
         must fail because it was destructured out. -->
    {takesNumber(rest.id)}
</Producer>
