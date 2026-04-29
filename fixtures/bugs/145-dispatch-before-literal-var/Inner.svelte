<script lang="ts">
    // Round-11 follow-up #2: a `dispatch(EV)` call BEFORE the
    // `const EV = 'save'` declaration. Pre-fix native ran a separate
    // pre-pass that registered EV='save' globally before the
    // dispatched-name scan, so the forward-referenced call was
    // wrongly resolved and 'save' contributed to the events surface.
    //
    // Upstream's single source-order TS walker visits the call
    // BEFORE `checkIfIsStringLiteralDeclaration` records EV
    // (`processInstanceScriptContent.ts:271`), so the lookup misses
    // and 'save' is NOT added.
    //
    // Native now uses a single-pass walker that grows literal_vars
    // as it encounters `const NAME = 'literal'` bindings — forward
    // references resolve to nothing, matching upstream.
    import { createEventDispatcher } from 'svelte'
    const dispatch = createEventDispatcher()

    function fire() {
        // JS-legal: function body executes later when EV is in
        // scope. At the AST walker level the call is visited
        // BEFORE EV's declaration in source order, so the literal-
        // var pass shouldn't see EV when resolving this call.
        dispatch(EV)
    }
    void fire

    const EV = 'save'
    void EV
</script>

<button>x</button>
