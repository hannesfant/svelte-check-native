<svelte:options runes />
<script lang="ts">
    // Reviewer follow-up #3: a component with multiple typed
    // dispatchers AND an untyped dispatcher must merge ALL their
    // event signatures into the synthesised `$$Events`. Pre-fix the
    // typed path returned only the FIRST `<T>` (so `b` events were
    // silently dropped), and `typed.or_else(...)` suppressed the
    // untyped path entirely whenever any typed dispatcher existed
    // (so `c`'s dispatched names also disappeared).
    //
    // Post-fix the merge intersects all typed args with the
    // untyped detail map: `({a: {x: number}}) & ({b: {y: string}}) &
    // { bye: any }`. After the wrap-once at synthesis the FINAL
    // `$$Events` carries `a`, `b`, AND `bye` keys.
    import { createEventDispatcher } from 'svelte'

    let { label = '' }: { label?: string } = $props()

    const a = createEventDispatcher<{ a: { x: number } }>()
    const b = createEventDispatcher<{ b: { y: string } }>()
    const c = createEventDispatcher()

    void a
    void b
    void c

    function fire() {
        a('a', { x: 1 })
        b('b', { y: 'hi' })
        c('bye')
    }
    void fire
</script>

<button>{label}</button>
