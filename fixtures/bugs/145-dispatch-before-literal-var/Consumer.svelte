<script lang="ts">
    import Inner from './Inner.svelte'

    // No-error baseline. Inner forward-references EV inside `fire()`,
    // and `const EV = 'save'` is declared after. Pre-fix native ran
    // a separate pre-pass that registered EV='save' globally before
    // the dispatched-name scan, so the forward-referenced call
    // wrongly contributed 'save' to the events surface. Post-fix
    // the single source-order walk visits the call BEFORE EV's
    // declaration so 'save' is NOT added — matching upstream's
    // `processInstanceScriptContent.ts:271` AST visit-then-recurse
    // ordering. The behaviour is observable in the emit
    // (`--emit-ts`); the consumer-side typecheck doesn't fire under
    // non-strict mode because the lax `[evt:string]: CustomEvent<any>`
    // index signature accepts any name either way.
    void Inner
</script>
