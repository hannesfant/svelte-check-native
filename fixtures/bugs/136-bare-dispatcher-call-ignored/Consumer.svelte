<script lang="ts">
    import Producer from './Producer.svelte'

    // The bare-call dispatcher contributed nothing to the events
    // surface, so Producer keeps the fn-component shape. Threlte-
    // style instancing passes.
    function asArray(): (typeof Producer)[] {
        return [Producer]
    }
    void asArray
</script>
