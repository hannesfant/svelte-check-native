<script lang="ts">
    // Round-9 follow-up #3: a bare expression-statement
    // `createEventDispatcher<{...}>()` (no `const x = ...`
    // assignment) should NOT contribute to the events surface.
    // Upstream's `processInstanceScriptContent.ts:271` only counts
    // dispatchers that are bound to a callable identifier — bare
    // calls have no later `dispatch('foo', …)` reachable, so they
    // don't reach `setEventDispatcher` and don't add events.
    //
    // Pre-fix native scanned both VarDecl and ExpressionStatement,
    // so this Producer would have synthesised the bogus events
    // surface and blocked the runes fn-component shape.
    import { createEventDispatcher } from 'svelte'
    let { id }: { id: string } = $props()
    void id

    // Bare expression statement — no binding. Ignored by both
    // upstream and post-fix native.
    createEventDispatcher<{ phantom: string }>()
</script>

<p>id={id}</p>
