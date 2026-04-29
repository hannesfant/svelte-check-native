<script lang="ts">
    import Wrapper from './Wrapper.svelte'

    // Wrapper's `click` is the union
    //   CustomEvent<{ source: 'button' }> | CustomEvent<{ source: 'radio' }>
    // Detail discriminates on `source`. A handler accepting the
    // union types fine.
    function correctHandler(e: CustomEvent<{ source: 'button' | 'radio' }>): void {
        void e.detail.source
    }

    // Wrong: handler narrows to only Button's source. Receives the
    // union — `'button' | 'radio'` isn't assignable to `'button'`.
    // Pre-fix Radio's contribution was dropped, the wrapper's
    // `click` typed as just Button's CustomEvent, and this passed
    // silently.
    function tooNarrowHandler(e: CustomEvent<{ source: 'button' }>): void {
        void e.detail
    }
</script>

<Wrapper on:click={correctHandler} />
<Wrapper on:click={tooNarrowHandler} />
