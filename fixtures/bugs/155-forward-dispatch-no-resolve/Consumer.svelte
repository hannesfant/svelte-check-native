<script lang="ts">
    import Inner from './Inner.svelte'

    // No-error baseline. Inner has `function fire() { dispatch('ready') }`
    // BEFORE `const dispatch = createEventDispatcher()`. Pre-fix
    // native pre-collected dispatcher locals globally and scanned
    // calls afterward — `'ready'` would have been added to the
    // events surface from the forward call. Post-fix the
    // single-pass walker visits the call before the dispatcher
    // decl is registered, so 'ready' is NOT added. Verify via
    // `--emit-ts`: events surface is just the lax
    // `[evt: string]: CustomEvent<any>` (no specific 'ready' key).
    void Inner
</script>
