import { Helpers } from './helpers';

declare const SVGModule: any;

export class SvgNodeWrapper {
	__native: any;
	ownerDocument: SvgDocumentWrapper | null;
	__parent: SvgNodeWrapper | null = null;
	private __childList: SvgNodeWrapper[] = [];
	private __id: string | null = null;

	constructor(native: any, ownerDocument: SvgDocumentWrapper | null = null) {
		this.__native = native;
		this.ownerDocument = ownerDocument;
	}

	get nodeName(): string {
		return this.__native.tagName();
	}

	get childNodes() {
		const list = this.__childList;
		return {
			get length() {
				return list.length;
			},
			item(i: number) {
				return list[i] ?? null;
			},
		};
	}

	get id(): string | null {
		return this.__id;
	}

	set id(value: unknown) {
		this.setAttribute('id', value);
	}

	setAttribute(name: string, value: unknown) {
		const stringValue = String(value);
		if (name === 'id') {
			// Skia keeps ids in a map it only fills while parsing, not on the node.
			const doc = this.ownerDocument;
			if (doc && this.__id && this.__id !== stringValue) {
				doc.__native.unregisterId(this.__id);
			}
			this.__id = stringValue;
			doc?.__native.registerId(stringValue, this.__native);
			this.__invalidate(name);
			return;
		}
		this.__native.setAttribute(name, stringValue);
		this.__invalidate(name);
	}

	/** Records an id this node already carries natively, without re-registering it. */
	__adoptId(id: string) {
		this.__id = id;
	}

	/** Whether this node is the promoted layer, or inside it. */
	__isInPromotedLayer(): boolean {
		const layerId = this.ownerDocument?.layerId;
		if (!layerId) {
			return false;
		}
		let node: SvgNodeWrapper | null = this;
		while (node) {
			if (node.__id === layerId) {
				return true;
			}
			node = node.__parent;
		}
		return false;
	}

	// Nodes live outside the view tree, so changes reach the view through the document. The
	// node and attribute are passed along so the view can drop anything it caches about them.
	private __invalidate(attribute?: string) {
		// Mutating the promoted subtree is exactly what the cached backdrop is meant to
		// survive; anything else means the static content changed.
		if (!this.__isInPromotedLayer()) {
			this.ownerDocument?.invalidateBackdrop();
		}
		if (this.ownerDocument) {
			this.ownerDocument.revision++;
		}
		this.ownerDocument?.owner?.__invalidate?.(this, attribute);
	}

	getAttribute(name: string): string | null {
		if (name === 'id') {
			return this.__id;
		}
		return this.__native.getAttribute(name) ?? null;
	}

	removeAttribute(name: string) {}

	// Built from tracked children: Skia cannot list a text element's children.
	get textContent(): string {
		const own = this.__native.text();
		if (own != null) {
			return own;
		}
		let text = '';
		for (const child of this.__childList) {
			text += child.textContent;
		}
		return text;
	}

	set textContent(value: unknown) {
		const text = value == null ? '' : String(value);
		if (this.__native.setText(text)) {
			this.__invalidate();
			return;
		}
		for (let i = this.__childList.length - 1; i >= 0; i--) {
			this.removeChild(this.__childList[i]);
		}
		if (text.length > 0) {
			const node = createSvgTextNode(text);
			if (node) {
				this.appendChild(node);
			}
		}
	}

	createTextNode(text: string): SvgNodeWrapper | null {
		return createSvgTextNode(text);
	}

	appendChild(child: SvgNodeWrapper) {
		if (this.__native.appendChild(child.__native)) {
			this.__childList.push(child);
			child.__parent = this;
			if (!child.ownerDocument) {
				child.__adopt(this.ownerDocument);
			}
			this.__invalidate();
		}
		return child;
	}

	// Ids are usually set before there's a document to register them with.
	private __adopt(document: SvgDocumentWrapper | null) {
		this.ownerDocument = document;
		if (document && this.__id) {
			document.__native.registerId(this.__id, this.__native);
		}
		for (const child of this.__childList) {
			if (!child.ownerDocument) {
				child.__adopt(document);
			}
		}
	}

	removeChild(child: SvgNodeWrapper) {
		const index = this.__childList.indexOf(child);
		if (index === -1) {
			return null;
		}
		const removedNative = this.__native.removeChild(index);
		if (removedNative) {
			this.__childList.splice(index, 1);
			child.__parent = null;
			child.__native = removedNative;
			this.__invalidate();
		}
		return child;
	}
}

// Standalone creation — no owning document yet.
export function createSvgElement(tag: string): SvgNodeWrapper | null {
	Helpers.initialize();
	const native = SVGModule.createElement(tag);
	return native ? new SvgNodeWrapper(native) : null;
}

export function createSvgTextNode(text: string): SvgNodeWrapper | null {
	Helpers.initialize();
	const native = SVGModule.createTextNode(text);
	return native ? new SvgNodeWrapper(native) : null;
}

export const ANIMATION_RUNNING = 1;
export const ANIMATION_CHANGED = 2;

export class SvgDocumentWrapper {
	__native: any;
	rootElement: SvgNodeWrapper;
	// The Svg view to invalidate when any node in this document changes. Nodes are not in
	// the view tree, so this is their only route back to something that can redraw.
	owner: { __invalidate?(node?: unknown, attribute?: string): void } | null = null;
	layerId: string | null = null;
	/** Bumped on any visual change. */
	revision = 0;
	private __nativePointer = 0;
	private __containerWidth = 0;
	private __containerHeight = 0;

	constructor(src?: string) {
		Helpers.initialize();
		this.__native = typeof src === 'string' ? SVGModule.createSVGDocument(src) : SVGModule.createSVGDocument();
		this.rootElement = new SvgNodeWrapper(this.__native.root(), this);
	}

	createElement(tag: string): SvgNodeWrapper | null {
		const native = this.__native.createElement(tag);
		return native ? new SvgNodeWrapper(native, this) : null;
	}

	createTextNode(text: string): SvgNodeWrapper | null {
		const native = this.__native.createTextNode(text);
		return native ? new SvgNodeWrapper(native, this) : null;
	}

	getElementById(id: string): SvgNodeWrapper | null {
		const native = this.__native.getElementById(id);
		if (!native) {
			return null;
		}
		const node = new SvgNodeWrapper(native, this);
		node.__adoptId(id);
		return node;
	}

	/**
	 * Promotes the node with this id out of the static content: redraws then cost one
	 * `renderNode` over a cached raster instead of walking the whole tree. Pass null to clear.
	 *
	 * The promoted node composites last, so anything that should paint over it will appear
	 * beneath it. Promote the topmost animating subtree, as a compositor would.
	 */
	setLayer(id: string | null) {
		this.layerId = id;
		this.__native.setLayer(id ?? undefined);
	}

	invalidateBackdrop() {
		this.__native.invalidateBackdrop();
	}

	// Deduped here, not on the view: a shared document is laid out by whichever view drew last.
	setContainerSize(width: number, height: number) {
		if (this.__containerWidth === width && this.__containerHeight === height) {
			return;
		}
		this.__containerWidth = width;
		this.__containerHeight = height;
		this.__native.setContainerSize(width, height);
	}

	/** Direct node mutations must be followed by `invalidateFrames`. */
	setFrameSharing(enabled: boolean) {
		this.__native.setFrameSharing(enabled);
	}

	invalidateFrames() {
		this.__native.invalidateFrames();
	}

	/**
	 * Whether this document carries SMIL animation (`<animate>`, `<animateTransform>`,
	 * `<animateMotion>`, `<set>`). Only true for a document parsed from a source that contained
	 * them. They cannot be created through `createElement`, because they are not elements as
	 * far as the renderer is concerned, they are a schedule of attribute writes.
	 */
	get hasAnimations(): boolean {
		return this.__native.hasAnimations();
	}

	/** Seconds until every animation has finished, or Infinity when one repeats forever. */
	get animationDuration(): number {
		const duration = this.__native.animationDuration();
		return duration < 0 ? Infinity : duration;
	}

	get currentTime(): number {
		return this.__native.currentTime();
	}

	/**
	 * Moves the animation clock and writes every animated attribute. Returns whether anything is
	 * still animating; false means the document has gone static and the caller should stop
	 * asking for frames.
	 */
	/**
	 * Moves the animation clock, returning what that did: `running` is the cue to schedule
	 * another frame, `changed` whether anything actually moved. A frame where nothing moved
	 * does not need redrawing: discrete animations hold a value for most of their duration,
	 * so on a heavy document this is the difference between rasterizing every frame and
	 * rasterizing only the ones that differ.
	 *
	 * Returned as a bitmask rather than an object because this runs every frame and the loop
	 * is deliberately allocation-free.
	 */
	setCurrentTime(seconds: number): number {
		const state = this.__native.setCurrentTime(seconds);
		if ((state & ANIMATION_CHANGED) !== 0) {
			this.revision++;
		}
		return state;
	}

	/**
	 * The native SvgDocument, for platform renderers that draw it themselves. Read once, since
	 * it is fixed for the document's lifetime, and this sits on the per-frame redraw path.
	 */
	get nativePointer(): number {
		if (this.__nativePointer === 0) {
			this.__nativePointer = this.__native.nativePointer();
		}
		return this.__nativePointer;
	}

	renderToBuffer(buffer: Uint8Array, width: number, height: number, scale: number = 1) {
		this.__native.renderToBuffer(buffer, width, height, scale);
	}
}
