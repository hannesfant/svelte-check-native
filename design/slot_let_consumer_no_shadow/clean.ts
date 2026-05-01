// Verifies that without inner shadow emission, the parent's slot-let
// destructure resolves with strict types.
//
// Mirrors the post-fix overlay shape for $$slots-usage's slot="foo"
// child block.

declare const __svn_inst_4f: { $$slot_def: { foo: { valid2: boolean; validPropWrongType2: string } } };
declare function __svn_any(): any;

{
    const { valid2, validPropWrongType2 } = __svn_inst_4f.$$slot_def["foo"];
    {
        // Imagined `svelteHTML.createElement("div", {});` here.
        {
            // No shadow declarations — the consumer expressions resolve
            // to the outer destructured types.
            (valid2 === true);                    // OK: boolean === true
            (validPropWrongType2 === true);       // expect TS2367: string vs boolean no overlap
        }
    }
}
