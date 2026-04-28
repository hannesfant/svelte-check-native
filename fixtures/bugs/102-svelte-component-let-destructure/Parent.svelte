<script lang="ts">
    import Child from './Child.svelte'
    const Dynamic = Child
</script>

<!-- Reviewer follow-up #5: `<svelte:component this={X} let:item>` and
     `<svelte:self let:item>` should consume the synthetic component's
     default slot — same shape as a regular `<Comp let:item>`. Pre-fix
     the special-element path hardcoded "no let:" so any `let:foo`
     directive was silently dropped: `foo` was undeclared inside the
     children body and TS2304 fired on every reference.

     Post-fix the destructure lands as `const { $$_$$, item, … } =
     inst.$$slot_def.default; $$_$$;` so `item` is in scope.
     `inst.$$slot_def` resolves to `any` for the synthetic component
     root (full SlotHandler port for typed slot defs is deferred), so
     `item: any` doesn't constrain consumer access — but the
     declaration must exist for the children walk to type-check.
     -->
<svelte:component this={Dynamic} let:item>
    {JSON.stringify(item)}
</svelte:component>
