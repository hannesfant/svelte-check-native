// Tsgo validation fixture for B2 #3 (R-Conv #20).
// Validates the `__svn_create_create_slot<Slots>()` shape: when
// $$Slots is declared, slot-name + slot-prop literals type-check
// against it.

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
        __svn_create_slot('default', { valid1: true, validPropWrongType1: 'ok' });
    }
    {
        __svn_create_slot('foo', { valid2: true, validPropWrongType2: 'also ok' });
    }
})();
export {};
