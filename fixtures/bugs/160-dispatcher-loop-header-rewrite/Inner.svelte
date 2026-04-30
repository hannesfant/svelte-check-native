<script lang="ts">
    // Round-14 follow-up #3: the typed-events rewrite walker
    // (`dispatcher_typing_rewrite::collect_rewrite_insertions`)
    // descended into for-init and every loop/switch BODY but
    // skipped the loop/switch HEADERS — for-test, for-update,
    // ForIn/ForOf.right, While/DoWhile.test, and Switch.discriminant
    // / per-case test. An untyped dispatcher hidden in an IIFE used
    // as a loop test (`while ((() => { const d =
    // createEventDispatcher(); … })())`) escaped the rewrite and
    // its `dispatch('name', detail)` calls ran through the lax
    // `<{}>` inference.
    //
    // The analyzer's `statement_collect_typed_dispatcher_slices`
    // already handled all these headers (round-12 #4 / round-13 #6);
    // post-fix the rewrite walker mirrors that coverage.
    import { createEventDispatcher } from 'svelte'
    interface $$Events {
        save: CustomEvent<{ id: number }>
    }
    let _e: $$Events | undefined
    void _e

    function probe() {
        while (
            (() => {
                const d = createEventDispatcher()
                // After the rewrite, `d`'s `save` detail is
                // `{id: number}`. Calling with a wrong shape MUST
                // fire TS2353.
                d('save', { wrong: 'shape' })
                return false
            })()
        ) {
            void 0
        }
    }
    void probe
</script>

<button>x</button>
