<svelte:options runes />
<script lang="ts">
    // Reviewer follow-up #4: pre-fix `collect_ctor_locals` ALWAYS
    // inserted the bare name `createEventDispatcher` into the
    // dispatcher-locals set, regardless of whether any Svelte import
    // bound it. A local function (or a non-Svelte import) named
    // `createEventDispatcher` would then be treated as a real Svelte
    // dispatcher: `has_event_dispatcher_call` returns true, the
    // default-export shape switches off the fn-component path, the
    // `__svn_events` marker intersects, and a phantom event surface
    // gets attached to the value type.
    //
    // Upstream gates on the exact import source:
    // `language-tools/.../ComponentEvents.ts:387` requires
    // `node.moduleSpecifier.text === 'svelte'`. Mirror that: the
    // local fallback is dropped, only Svelte-imported names count.
    //
    // The local function below is INTENTIONALLY named
    // `createEventDispatcher` to exercise the false-positive path.
    // It returns a callable but is not Svelte's dispatcher — its
    // calls don't dispatch events at the framework level.
    function createEventDispatcher<T = unknown>(): (name: string, detail?: T) => void {
        return (_name: string, _detail?: T) => {}
    }

    const fakeDispatch = createEventDispatcher<{ id: number }>()
    void fakeDispatch
    fakeDispatch('foo', { id: 1 })

    let { id }: { id: string } = $props()
</script>

<p>id={id}</p>
