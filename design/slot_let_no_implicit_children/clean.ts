// Validates that emitting `<Comp let:b>body</Comp>` WITHOUT an
// implicit `children: () => …` prop type-checks cleanly against a
// component declaring `Record<string, never>` props. The body
// content goes through the slot scope, not the children prop.

declare class Comp {
    constructor(opts: { target?: any; props: Partial<Record<string, never>> & { children?: any } });
}

// Without implicit children synthesis: empty props is OK.
new Comp({ target: undefined as any, props: {} });
