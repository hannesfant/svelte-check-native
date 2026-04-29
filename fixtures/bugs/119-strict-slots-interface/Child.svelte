<script lang="ts">
    // Stage 5: when the instance script declares `interface $$Slots`
    // (or `type $$Slots`), the render-fn return uses that as the
    // slots surface — synthesised slot-defs are ignored. Mirrors
    // upstream's `uses$$SlotsInterface` behavior.
    //
    // Without the override, the synthesised slot-def from
    // `<slot label={123}>` would type `label` as `number`, but
    // the user's $$Slots declaration says `label: string`. The
    // declaration is authoritative — consumers should see the
    // declared shape.
    interface $$Slots {
        default: { label: string }
    }
    let _s: $$Slots | undefined
    void _s
    export let value: string = ''
    void value
</script>

<slot label="hi" />
