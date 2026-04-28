<script lang="ts">
    // Reviewer follow-up #8: `<svelte:boundary>`'s `failed` snippet
    // declares `failed?: Snippet<[error: unknown, reset: () => void]>`
    // in `svelte/elements.d.ts`. Pre-fix the snippet body emitted via
    // the regular path produced `void ((e: any) => {…})` — disconnected
    // from the boundary's prop signature and `e` typed as `any`.
    //
    // Post-fix the snippet emits as `failed: (e) => {…}` INSIDE the
    // createElement attrs object, so contextual typing flows the
    // declared `Snippet<[error: unknown, reset: () => void]>` shape
    // through. `e` types as `unknown`, and the wrong-shape access below
    // fires TS18046 / TS2571 (the exact code depends on which `unknown`
    // narrowing rule TS picks).

    function takesString(s: string): void {
        void s
    }
    void takesString
</script>

<svelte:boundary onerror={(e) => void e}>
    <p>boundary content</p>
    {#snippet failed(err)}
        <!-- err: unknown — passing it to takesString must fire TS2345
             (`Argument of type 'unknown' is not assignable to parameter
             of type 'string'`). Pre-fix err was `any` so this passed. -->
        <p>{takesString(err)}</p>
    {/snippet}
</svelte:boundary>
