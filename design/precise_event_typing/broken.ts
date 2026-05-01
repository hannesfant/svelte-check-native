// Companion: deliberately-broken usages that should fire the exact
// LS `element-events` fixture diagnostics (2353 + 2339).

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

// expect 2353: 'on:wat' not in HTMLAttributes
const x: typeof div = { "on:wat": () => '' };
// expect 2339: e.asd does not exist on MouseEvent & { currentTarget: ... }
const y: typeof div = { "on:click": (e) => e.asd };
void x; void y;
