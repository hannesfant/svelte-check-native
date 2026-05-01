// Validates the closed `HTMLAttributes` shape with explicit per-event
// entries (no `[name: `on${string}`]: any` wildcard). Mirrors the
// post-fix structure of the shim's HTMLAttributes for the LS
// `element-events` fixture.

type EventHandler<E extends Event = Event, T extends EventTarget = Element> =
    (event: E & { currentTarget: EventTarget & T }) => any;
type MouseEventHandler<T extends EventTarget = Element> = EventHandler<MouseEvent, T>;
type KeyboardEventHandler<T extends EventTarget = Element> = EventHandler<KeyboardEvent, T>;

interface HTMLAttributes<T extends EventTarget = HTMLElement> {
    'on:click'?: MouseEventHandler<T> | undefined | null;
    onclick?: MouseEventHandler<T> | undefined | null;
    'on:keydown'?: KeyboardEventHandler<T> | undefined | null;
    onkeydown?: KeyboardEventHandler<T> | undefined | null;
    [name: `data-${string}`]: any;
}

interface SvelteHTMLElements {
    div: HTMLAttributes<HTMLDivElement>;
}

type HTMLProps<K extends keyof SvelteHTMLElements, Override = {}> =
    Omit<SvelteHTMLElements[K], keyof Override> & Override;

declare const div: HTMLProps<"div">;

// OK — known event with correct handler shape, currentTarget narrows to HTMLDivElement
const a: typeof div = { "on:click": (e) => { e.currentTarget.tagName; } };
const b: typeof div = { onclick: (e) => { e.currentTarget.tagName; } };
const c: typeof div = { "on:keydown": (e) => { e.key; } };
void a; void b; void c;
