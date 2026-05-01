// Companion: with the (current bug) inner shadow emission, the
// `let valid2: any` shadows the outer `valid2: boolean` destructure
// and the comparison silently resolves as `any === true`. The TS2367
// expected diagnostic vanishes — that's the bug.

declare const __svn_inst_4f: { $$slot_def: { foo: { valid2: boolean; validPropWrongType2: string } } };

{
    const { valid2, validPropWrongType2 } = __svn_inst_4f.$$slot_def["foo"];
    {
        {
            let valid2: any;
            void valid2;
            let validPropWrongType2: any;
            void validPropWrongType2;
            (valid2 === true);                    // OK: any
            (validPropWrongType2 === true);       // OK because of `any` shadow — should fire TS2367 but doesn't
        }
    }
}

// To prove the broken case actually does fire NO diagnostic, also probe
// that the outer destructure types still bind correctly to direct use.
{
    const { valid2: outer_valid2 } = __svn_inst_4f.$$slot_def["foo"];
    (outer_valid2 === "string");                  // expect TS2367: boolean vs string
}
