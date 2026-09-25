import { booleanConverter, CssProperty, CSSType, File, Http, knownFolders, path, Property, Style, Utils, View } from '@nativescript/core';
import { ANIMATION_CHANGED, ANIMATION_RUNNING, SvgDocumentWrapper, SvgNodeWrapper } from './NativeNode';

declare const requestAnimationFrame: ((cb: () => void) => void) | undefined;

// Prefer the frame clock so an animation renders at most once per frame; the macrotask
// queue is only a fallback for when the canvas polyfill is not installed.
function scheduleFrame(callback: () => void) {
	if (typeof requestAnimationFrame === 'function') {
		requestAnimationFrame(callback);
	} else {
		Utils.queueMacrotask(callback);
	}
}
export * from './Elements';
export * from './NativeNode';
export const strokeProperty = new CssProperty<Style, any>({
	name: 'stroke',
	cssName: 'stroke',
	defaultValue: undefined,
});

export const strokeWidthProperty = new CssProperty<Style, number>({
	name: 'strokeWidth',
	cssName: 'stroke-width',
	defaultValue: 1,
});

export const fillProperty = new CssProperty<Style, any>({
	name: 'fill',
	cssName: 'fill',
	defaultValue: undefined,
});

export const fillRuleProperty = new CssProperty<Style, any>({
	name: 'fillRule',
	cssName: 'fill-rule',
	defaultValue: undefined,
});

export const fillOpacityProperty = new CssProperty<Style, any>({
	name: 'fillOpacity',
	cssName: 'fill-opacity',
	defaultValue: undefined,
});

export const stopColorProperty = new CssProperty<Style, any>({
	name: 'stopColor',
	cssName: 'stop-color',
	defaultValue: undefined,
});

export const strokeLinecapProperty = new CssProperty<Style, any>({
	name: 'strokeLinecap',
	cssName: 'stroke-linecap',
});

export const strokeLinejoinProperty = new CssProperty<Style, any>({
	name: 'strokeLinejoin',
	cssName: 'stroke-linejoin',
});

export const strokeMiterlimitProperty = new CssProperty<Style, any>({
	name: 'strokeMiterlimit',
	cssName: 'stroke-miterlimit',
});
export const srcProperty = new Property<SVGBase, string>({
	name: 'src',
});

export const syncProperty = new Property<SVGBase, boolean>({
	name: 'sync',
	defaultValue: false,
	valueConverter: booleanConverter,
});

/**
 * GPU rasterization. On by default: filters and masks are ~94% of a CPU frame here and are
 * fragment-shader work on the GPU. Falls back to the CPU bitmap automatically when no context
 * can be made, so turning this off is only for ruling the GPU out.
 */
export const gpuProperty = new Property<SVGBase, boolean>({
	name: 'gpu',
	defaultValue: true,
	valueConverter: booleanConverter,
});

/**
 * Rasterize off the UI thread. The frame is recorded into a display list here and rasterized on
 * one render thread shared by every threaded view, so an expensive svg does not hold up the UI.
 */
export const threadedProperty = new Property<SVGBase, boolean>({
	name: 'threaded',
	defaultValue: true,
	valueConverter: booleanConverter,
});

/** Views with the same `src` share one document and clock, like `<img>` on the web. */
export const shareSrcProperty = new Property<SVGBase, boolean>({
	name: 'shareSrc',
	defaultValue: true,
	valueConverter: booleanConverter,
	valueChanged(target, oldValue, newValue) {
		// Without a native view, `src` is applied later anyway.
		if (target.nativeViewProtected && target.src) {
			target.__loadSrc(target.src);
		}
	},
});

/** `auto` | `gl` | `vulkan` | `metal`. `auto` is right unless a device's driver is the problem. */
export type SvgBackend = 'auto' | 'gl' | 'vulkan' | 'metal';

export const backendProperty = new Property<SVGBase, SvgBackend>({
	name: 'backend',
	defaultValue: 'auto',
});

/**
 * `texture` composites like a normal view (transformable, overlappable) at the cost of a copy;
 * `surface` is scanned out directly but cannot be transformed or overlapped.
 */
export type SvgSurfaceType = 'texture' | 'surface';

export const surfaceTypeProperty = new Property<SVGBase, SvgSurfaceType>({
	name: 'surfaceType',
	defaultValue: 'texture',
});

/** Resolves inline markup, a path or a URL to markup. Inline markup answers synchronously. */
export function readSrc(value: string, done: (source: string) => void, failed: (error: unknown) => void) {
	if (value.indexOf('<svg') > -1) {
		done(value);
		return;
	}
	if (value.startsWith('http')) {
		Http.getString(value).then(done).catch(failed);
		return;
	}
	const file = value.startsWith('~') ? path.join(knownFolders.currentApp().path, value.replace('~', '')) : value;
	File.fromPath(file).readText().then(done).catch(failed);
}

const sharedSources = new Map<string, SharedSource>();

/** One document and clock for every view showing the same `src` (Blink's `SVGImage`). */
class SharedSource {
	document: SvgDocumentWrapper | null = null;
	readonly holders = new Set<SVGBase>();
	private waiting: Array<(document: SvgDocumentWrapper) => void> = [];
	private ticking = false;
	private start = 0;

	private constructor(readonly key: string) {}

	/** `existing` lets a view that was set up again reuse its document instead of reparsing. */
	static acquire(key: string, view: SVGBase, existing?: SvgDocumentWrapper | null): SharedSource {
		let source = sharedSources.get(key);
		if (!source) {
			source = new SharedSource(key);
			sharedSources.set(key, source);
			if (existing) {
				source.ready(existing);
			} else {
				readSrc(
					key,
					(markup) => source.ready(new SvgDocumentWrapper(markup)),
					(error) => {
						console.error('Svg: could not load src', error);
						if (sharedSources.get(key) === source) {
							sharedSources.delete(key);
						}
					},
				);
			}
		}
		source.holders.add(view);
		return source;
	}

	release(view: SVGBase) {
		this.holders.delete(view);
		if (this.holders.size === 0 && sharedSources.get(this.key) === this) {
			sharedSources.delete(this.key);
			this.ticking = false;
			this.document?.setFrameSharing(false);
		}
	}

	whenReady(callback: (document: SvgDocumentWrapper) => void) {
		if (this.document) {
			callback(this.document);
		} else {
			this.waiting.push(callback);
		}
	}

	private ready(document: SvgDocumentWrapper) {
		this.document = document;
		document.owner = this;
		document.setFrameSharing(true);
		const waiting = this.waiting;
		this.waiting = [];
		for (const callback of waiting) {
			callback(document);
		}
	}

	// Node writes bypass the document, so its shared recordings are dropped here.

	__invalidate(node?: unknown, attribute?: string) {
		this.document?.invalidateFrames();
		for (const view of this.holders) {
			view.__invalidate(node, attribute);
		}
	}

	startClock() {
		if (this.ticking || !this.document?.hasAnimations) {
			return;
		}
		this.ticking = true;
		this.start = Date.now();
		const tick = () => {
			if (!this.ticking || !this.document) {
				return;
			}
			let onScreen = false;
			for (const view of this.holders) {
				if (view._attachedToDom) {
					onScreen = true;
					break;
				}
			}
			if (!onScreen) {
				// Nobody can see it: hold still until a view is loaded again.
				this.ticking = false;
				return;
			}
			const state = this.document.setCurrentTime((Date.now() - this.start) / 1000);
			if ((state & ANIMATION_CHANGED) !== 0) {
				// The first view to draw records the frame; the rest replay it.
				for (const view of this.holders) {
					if (view._attachedToDom) {
						view.__redraw();
					}
				}
			}
			if ((state & ANIMATION_RUNNING) !== 0) {
				scheduleFrame(tick);
				return;
			}
			this.ticking = false;
			for (const view of this.holders) {
				view.notify({ eventName: SVGBase.animationEndEvent, object: view });
			}
		};
		scheduleFrame(tick);
	}
}

@CSSType('Svg')
export class SVGBase extends View {
	public static readyEvent = 'ready';

	/**
	 * The GPU context died. Recovery is automatic and mostly invisible (the view keeps
	 * drawing either way), so this fires only when it could not be rebuilt and rendering has
	 * dropped to the CPU raster, which is also when `gpu` goes false.
	 */
	public static contextLostEvent = 'contextLost';

	/** A context was rebuilt after being lost, and the GPU path is running again. */
	public static contextRestoredEvent = 'contextRestored';

	/** Every SMIL animation in the document has finished. Never fires for one that repeats. */
	public static animationEndEvent = 'animationEnd';
	__document: SvgDocumentWrapper;
	__domElement: SvgNodeWrapper;
	__children = [];
	__redrawScheduled = false;
	__animationScheduled = false;
	/** Timestamp the animation clock is measured from, in milliseconds. */
	__animationStart = 0;
	__shared: SharedSource | null = null;
	__srcKey: string | null = null;
	__rootWidth = 0;
	__rootHeight = 0;
	/** Shrinks a document whose intrinsic size is bigger than the view it has to fit into. */
	__fitScale = 1;
	__rootSizeValid = false;
	_attachedToDom = false;
	src: string;
	sync: boolean;
	gpu: boolean;
	threaded: boolean;
	shareSrc: boolean;
	backend: SvgBackend;
	surfaceType: SvgSurfaceType;

	constructor() {
		super();
		this.__document = new SvgDocumentWrapper();
		this.__document.owner = this;
		this.__domElement = this.__document.rootElement;
	}

	/**
	 * Requests a redraw, coalescing every mutation made in the same frame into one.
	 * Building a subtree is many mutations but should still cost a single render.
	 */
	__invalidate(node?: unknown, attribute?: string) {
		// The root's width/height are read on every redraw, so they are cached; only a write
		// to them can invalidate that.
		if (node === this.__domElement && (attribute === 'width' || attribute === 'height')) {
			this.__rootSizeValid = false;
		}
		if (this.__redrawScheduled) {
			return;
		}
		this.__redrawScheduled = true;
		scheduleFrame(() => {
			this.__redrawScheduled = false;
			this.__redraw();
		});
	}

	/**
	 * The size to render the document at, in CSS pixels, cached until a write to the root's
	 * `width`/`height` invalidates it.
	 *
	 * A percentage resolves against the view, which is the whole point of one: `rocket.svg` is
	 * authored `width="100%" height="100%" viewBox="0 0 300 300"`, and reading that as a plain
	 * number gives a 100x100 document (a third of its intended size) rather than one that
	 * fills its view.
	 */
	__resolveRootSize() {
		if (this.__rootSizeValid) {
			return;
		}
		const root = this.__domElement;
		const actual = this.getActualSize?.() ?? { width: 0, height: 0 };

		const resolve = (name: 'width' | 'height', fallback: number, available: number) => {
			let attribute = root.getAttribute(name);
			if (!attribute) {
				// The SVG defaults, written back so the document says what it is being drawn at.
				attribute = String(fallback);
				root.setAttribute(name, attribute);
			}
			const value = parseFloat(attribute);
			if (!isFinite(value)) {
				return fallback;
			}
			if (attribute.trim().endsWith('%')) {
				return available > 0 ? (value / 100) * available : fallback;
			}
			return value;
		};

		this.__rootWidth = resolve('width', 300, actual.width);
		this.__rootHeight = resolve('height', 150, actual.height);

		// A source that states its own size (`width="700" height="400"` rather than `100%`)
		// would otherwise be drawn at that size into a surface only as big as the view, which
		// shows its top-left corner and nothing else. Scale it down to fit, as an <img> would.
		// Only ever down: a small document still draws at its natural size.
		const fits = (available: number, needed: number) => (available > 0 && needed > 0 ? available / needed : 1);
		this.__fitScale = Math.min(1, fits(actual.width, this.__rootWidth), fits(actual.height, this.__rootHeight)) || 1;
		this.__rootSizeValid = true;
	}

	/** Renders now, skipping the frame wait. */
	__redraw() {}

	/**
	 * Resolves a `src` (inline markup, an app-relative or absolute path, or a URL) into the
	 * live document.
	 *
	 * `src` used to go to a one-shot native rasterizer that drew into a bitmap the view then
	 * blitted. That bitmap stopped being drawn once the GPU path existed: the surface host sits
	 * in front, and both platforms skip their bitmap draw while a GPU context is alive, so a
	 * `src` rendered nothing at all under the default `gpu = true`. Going through the live
	 * document renders on whichever path is actually active, and is also the only way SMIL can
	 * reach the screen.
	 */
	__loadSrc(value: string) {
		this.__releaseShared();
		if (typeof value !== 'string' || value.length === 0) {
			this.__srcKey = null;
			return;
		}
		this.__srcKey = value;
		if (this.shareSrc !== false) {
			this.__joinShared(value);
			return;
		}
		readSrc(
			value,
			(source) => {
				// A later `src` may have landed while this one was still reading.
				if (this.__srcKey === value && !this.__shared) {
					this.__loadSource(source);
				}
			},
			(error) => console.error('Svg: could not load src', error),
		);
	}

	private __joinShared(key: string, existing?: SvgDocumentWrapper) {
		const source = SharedSource.acquire(key, this, existing);
		this.__shared = source;
		source.whenReady((document) => {
			if (this.__shared === source) {
				this.__adoptDocument(document);
			}
		});
	}

	__releaseShared() {
		this.__shared?.release(this);
		this.__shared = null;
	}

	/**
	 * Replaces the live document with one parsed from `source`, and starts its SMIL animations
	 * if it has any.
	 */
	__loadSource(source: string) {
		const document = new SvgDocumentWrapper(source);
		document.owner = this;
		this.__adoptDocument(document);
	}

	private __adoptDocument(document: SvgDocumentWrapper) {
		this.__document = document;
		this.__domElement = document.rootElement;
		this.__rootSizeValid = false;
		this.__startAnimations();
		this.__invalidate();
	}

	/**
	 * Drives the document's SMIL clock, one `setCurrentTime` plus one redraw per frame.
	 *
	 * The loop stops itself: `setCurrentTime` reports whether anything is still moving, and a
	 * document whose animations have all ended or frozen is static again. Keeping a frame
	 * callback alive to redraw an unchanging picture is how an idle SVG costs a device its
	 * battery.
	 */
	__startAnimations() {
		if (!this.__shared && this.__srcKey && this.shareSrc !== false && this.__document) {
			// Set up again after disposal: rejoin with the document it already has.
			this.__joinShared(this.__srcKey, this.__document);
		}
		if (this.__shared) {
			this.__shared.startClock();
			return;
		}
		if (this.__animationScheduled || !this.__document?.hasAnimations) {
			return;
		}
		this.__animationStart = Date.now();
		this.__animationScheduled = true;
		const tick = () => {
			if (!this.__animationScheduled) {
				return;
			}
			if (!this._attachedToDom || this.__shared) {
				// Off screen, or now shared and driven by its source's clock.
				this.__animationScheduled = false;
				return;
			}
			const state = this.__document.setCurrentTime((Date.now() - this.__animationStart) / 1000);
			const running = (state & ANIMATION_RUNNING) !== 0;
			// Only the frames that differ are worth rasterizing; the surface still holds the
			// last one. Redrawing an unchanged document is the single most expensive thing this
			// loop can do on a heavy source.
			if ((state & ANIMATION_CHANGED) !== 0) {
				this.__redraw();
			}
			if (running) {
				scheduleFrame(tick);
				return;
			}
			this.__animationScheduled = false;
			this.notify({ eventName: SVGBase.animationEndEvent, object: this });
		};
		scheduleFrame(tick);
	}

	__stopAnimations() {
		this.__animationScheduled = false;
	}

	/**
	 * Adds CSS `@keyframes` from a stylesheet outside the document; only `#id` selectors that
	 * exist apply. Restarts the animation clock if nothing was running.
	 */
	addStylesheet(css: string) {
		if (this.__document.addStylesheet(css)) {
			this.__startAnimations();
		}
	}

	disposeNativeView() {
		// The last view out frees the source; `__startAnimations` rejoins if set up again.
		this.__releaseShared();
		super.disposeNativeView();
	}

	__notifyContextLost() {
		this.notify({ eventName: SVGBase.contextLostEvent, object: this });
	}

	__notifyContextRestored() {
		this.notify({ eventName: SVGBase.contextRestoredEvent, object: this });
	}
}

syncProperty.register(SVGBase);
srcProperty.register(SVGBase);
gpuProperty.register(SVGBase);
threadedProperty.register(SVGBase);
shareSrcProperty.register(SVGBase);
backendProperty.register(SVGBase);
surfaceTypeProperty.register(SVGBase);
stopColorProperty.register(Style);
strokeWidthProperty.register(Style);
strokeProperty.register(Style);
fillProperty.register(Style);
fillOpacityProperty.register(Style);
fillRuleProperty.register(Style);
strokeLinecapProperty.register(Style);
strokeLinejoinProperty.register(Style);
strokeMiterlimitProperty.register(Style);
