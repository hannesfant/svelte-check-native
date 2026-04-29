<script lang="ts">
    // Round-11 follow-up #1: a typed dispatcher declared inside an
    // IIFE-wrapped initializer. Round-10 #2 documented the recursion
    // for IIFE shapes (`(() => { ... })()`) but
    // `statements_inside_function_expr` only handled bare
    // ArrowFunctionExpression / FunctionExpression — IIFE's
    // CallExpression callee was missed. Round-11 #1 extends the
    // helper to peek through CallExpression and ParenthesizedExpression
    // wrappers.
    import { createEventDispatcher } from 'svelte'
    let { id }: { id: string } = $props()
    void id

    const setup = (() => {
        const _dispatch = createEventDispatcher<{ wrapped: number }>()
        return _dispatch
    })()
    void setup
</script>

<p>id={id}</p>
