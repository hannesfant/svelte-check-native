// Companion broken fixture: prove each expected diagnostic fires.

declare function __svn_create_create_slot<
    Slots = Record<string, Record<string, any>>,
>(): <SlotName extends keyof Slots>(
    slotName: SlotName,
    attrs: Slots[SlotName],
) => Record<string, any>;

interface $$Slots {
    default: { valid1: boolean; validPropWrongType1: string };
    foo: { valid2: boolean; validPropWrongType2: string };
}

const __svn_create_slot = __svn_create_create_slot<$$Slots>();

(async () => {
    {
        // expect TS2322 — boolean -> string
        __svn_create_slot('default', { valid1: true, validPropWrongType1: true });
    }
    {
        // expect TS2353 — invalidProp1 not on default slot
        __svn_create_slot('default', { valid1: true, invalidProp1: true });
    }
    {
        // expect TS2322 — boolean -> string
        __svn_create_slot('foo', { valid2: true, validPropWrongType2: true });
    }
    {
        // expect TS2353 — invalidProp2 not on foo slot
        __svn_create_slot('foo', { valid2: true, invalidProp2: true });
    }
    {
        // expect TS2345 — "invalid" not in keyof $$Slots
        __svn_create_slot('invalid', { prop: true });
    }
})();
export {};
