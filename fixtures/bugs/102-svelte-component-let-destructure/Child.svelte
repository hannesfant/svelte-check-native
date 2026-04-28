<script lang="ts">
    // Child has a default slot exposing an `item` prop. Used only
    // for the type channel here — the producer-side type for the
    // slot prop is the full SlotHandler port territory (see
    // design/slot_handler/PLAN.md), not part of this slice.
    export let label: string = ''
</script>

<slot item={{ id: 1 }}>{label}</slot>
