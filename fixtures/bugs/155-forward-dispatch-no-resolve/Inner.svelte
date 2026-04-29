<script lang="ts">
    // Round-13 follow-up #1: a `dispatch('ready', …)` call BEFORE
    // `const dispatch = createEventDispatcher()` should NOT
    // contribute 'ready' to the events surface. Pre-fix native
    // pre-collected dispatcher locals via
    // `find_untyped_dispatcher_local_names` and then scanned calls
    // — forward references resolved against the fully-populated
    // set.
    //
    // Upstream's TS walker visits the call BEFORE
    // `setEventDispatcher` registers the binding
    // (`processInstanceScriptContent.ts:271` — visit-then-recurse
    // ordering on VariableStatement). Native now mirrors with
    // single-pass tracking: dispatcher_locals grows incrementally,
    // and forward calls miss.
    import { createEventDispatcher } from 'svelte'

    function fire() {
        // JS-legal because the function body executes after `dispatch`
        // is initialised. The TS WALKER visits this statement before
        // the const decl, so 'ready' should NOT register.
        dispatch('ready')
    }
    void fire

    const dispatch = createEventDispatcher()
    void dispatch
</script>

<button>x</button>
