// Companion: shows the BROKEN shape — emitting an implicit
// `children: () => any` against `Partial<Record<string, never>> &
// { children?: any }` fires TS2322 because Partial<Record<string,
// never>> reduces to `{ [k: string]: undefined }` and forces
// children to be `undefined`, not a function.

declare class Comp {
    constructor(opts: { target?: any; props: Partial<Record<string, never>> & { children?: any } });
}

// expect 2322: { children: () => any } not assignable to
// Partial<Record<string, never>> & { children?: any }
new Comp({ target: undefined as any, props: { children: () => undefined as any } });
