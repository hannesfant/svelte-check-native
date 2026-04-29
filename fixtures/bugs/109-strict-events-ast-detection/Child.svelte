<script lang="ts">
    // Reviewer follow-up #3a: pre-fix the `$$Events` detection scanned
    // the raw source for the substrings `"interface $$Events"` /
    // `"type $$Events "` (with trailing space), which:
    //   - false-fired on comments / string literals containing those
    //     phrases
    //   - silently MISSED `type $$Events=…` (no whitespace before `=`)
    //   - silently MISSED `type $$Events<T>=…` (generic)
    //
    // This component declares `type $$Events=` with no whitespace
    // before the `=`. AST-based detection catches the declaration
    // and routes through the strict path; consumers' wrong-shape
    // handlers fail. Pre-fix the substring scan missed this and the
    // surface fell back to lax `{ [evt: string]: CustomEvent<any> }`.
    type $$Events={
        click: CustomEvent<{ id: number }>
    }
    let _e: $$Events | undefined
    void _e
</script>

<button>strict-via-AST</button>
