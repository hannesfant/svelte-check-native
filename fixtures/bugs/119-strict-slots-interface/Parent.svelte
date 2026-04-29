<script lang="ts">
    import Child from './Child.svelte'

    function takesNumber(n: number): void {
        void n
    }
</script>

<Child let:label>
    <!-- The declared $$Slots says `label: string`. Calling
         takesNumber(label) must fail TS2345 since the resolved
         type is string, not number. -->
    <span>{takesNumber(label)}</span>
</Child>
