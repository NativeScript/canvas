import { Application, Button, EventData, GestureStateTypes, GridLayout, Label, Observable, Page, PanGestureEventData, StackLayout, TouchGestureEventData, Utils, View } from '@nativescript/core';
import type { Canvas } from '@nativescript/canvas';
import { launchArgs } from '../launch-args';
import { FX_NAMES, FxName, QUAD_FLOATS, Renderer, Tex, Vec4, webglRenderer, webgpuRenderer } from './canvas-html-transitions-renderer';

/*
 * Shared element transitions with HTML-in-Canvas. The header, cards, detail page and close button are
 * drawable children of the canvas; each new snapshot is uploaded once and every effect is drawn from
 * those textures, starting at the tapped card. Only views at rest get hit-testing geometry.
 */

interface Place {
	emoji: string;
	name: string;
	region: string;
	from: string;
	to: string;
	rating: string;
	temp: string;
	flight: string;
	blurb: string;
}

const PLACES: Place[] = [
	{ emoji: '🏔️', name: 'Dolomites', region: 'South Tyrol, Italy', from: '#5b8def', to: '#7f5af0', rating: '4.9', temp: '14°', flight: '9h', blurb: 'Jagged limestone towers above green alpine meadows. Hut-to-hut trails, via ferrata routes and the best apple strudel at 2,000 metres.' },
	{ emoji: '🏝️', name: 'Moorea', region: 'French Polynesia', from: '#00c6a7', to: '#1e90ff', rating: '4.8', temp: '28°', flight: '14h', blurb: 'A heart-shaped island of lagoons and volcanic peaks. Swim with rays at dawn, then watch the sun drop behind Tahiti.' },
	{ emoji: '🌋', name: 'Highlands', region: 'Iceland', from: '#ff7a45', to: '#c2185b', rating: '4.7', temp: '6°', flight: '6h', blurb: 'Black sand, steaming vents and rivers you ford by 4x4. Long summer light, and empty roads for days.' },
	{ emoji: '🏜️', name: 'Wadi Rum', region: 'Jordan', from: '#f7b267', to: '#f25c54', rating: '4.8', temp: '31°', flight: '5h', blurb: 'Red sandstone canyons and a sky full of stars. Sleep in a desert camp and climb the dunes before sunrise.' },
	{ emoji: '🌸', name: 'Kyoto', region: 'Japan', from: '#ff8fab', to: '#a06cd5', rating: '4.9', temp: '18°', flight: '12h', blurb: 'Temples, moss gardens and lantern-lit lanes. Come in spring for the cherry blossom along the Philosopher’s Path.' },
	{ emoji: '🌌', name: 'Atacama', region: 'Chile', from: '#3a0ca3', to: '#4cc9f0', rating: '4.8', temp: '22°', flight: '13h', blurb: 'The driest desert on Earth and the clearest night sky. Salt flats, flamingos and geysers at first light.' },
];

const EFFECTS = ['morph', 'ripple', 'warp', 'haze', 'flip', 'vortex', 'ink', 'shatter', 'burn', 'pixels', 'cube'] as const;
type Effect = (typeof EFFECTS)[number];

/** Spring per effect: the shader effects are slower and critically damped, the geometric ones bounce a little. */
const SPRINGS: Record<Effect, [number, number]> = {
	morph: [180, 24],
	ripple: [26, 10.2],
	warp: [16, 8],
	haze: [24, 9.8],
	flip: [110, 19],
	vortex: [16, 8],
	ink: [18, 8.5],
	shatter: [22, 9.4],
	burn: [14, 7.5],
	pixels: [26, 10.2],
	cube: [70, 15],
};

const PAD = 16;
const GAP = 12;
const CARD_RADIUS = 18;

type Rect = { x: number; y: number; w: number; h: number };

const clamp01 = (v: number) => Math.min(1, Math.max(0, v));
const lerp = (a: number, b: number, t: number) => a + (b - a) * t;
const lerpRect = (a: Rect, b: Rect, t: number): Rect => ({ x: lerp(a.x, b.x, t), y: lerp(a.y, b.y, t), w: lerp(a.w, b.w, t), h: lerp(a.h, b.h, t) });
const smooth = (a: number, b: number, t: number) => {
	const x = clamp01((t - a) / (b - a));
	return x * x * (3 - 2 * x);
};
const dip = (px: number) => Utils.layout.toDeviceIndependentPixels(px);
const rgb = (hex: string): [number, number, number] => {
	const n = parseInt(hex.slice(1), 16);
	return [((n >> 16) & 255) / 255, ((n >> 8) & 255) / 255, (n & 255) / 255];
};

/** Column-major 4x4 helpers for the flip. */
const m4 = {
	mul(a: Float32Array, b: Float32Array) {
		const o = new Float32Array(16);
		for (let c = 0; c < 4; c++) {
			for (let r = 0; r < 4; r++) {
				o[c * 4 + r] = a[r] * b[c * 4] + a[4 + r] * b[c * 4 + 1] + a[8 + r] * b[c * 4 + 2] + a[12 + r] * b[c * 4 + 3];
			}
		}
		return o;
	},
	chain: (...ms: Float32Array[]) => ms.reduce((a, b) => m4.mul(a, b)),
	identity: () => new Float32Array([1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1]),
	translate: (x: number, y: number, z = 0) => new Float32Array([1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, x, y, z, 1]),
	scale: (x: number, y: number) => new Float32Array([x, 0, 0, 0, 0, y, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1]),
	rotateX: (a: number) => new Float32Array([1, 0, 0, 0, 0, Math.cos(a), Math.sin(a), 0, 0, -Math.sin(a), Math.cos(a), 0, 0, 0, 0, 1]),
	rotateY: (a: number) => new Float32Array([Math.cos(a), 0, -Math.sin(a), 0, 0, 1, 0, 0, Math.sin(a), 0, Math.cos(a), 0, 0, 0, 0, 1]),
	rotateZ: (a: number) => new Float32Array([Math.cos(a), Math.sin(a), 0, 0, -Math.sin(a), Math.cos(a), 0, 0, 0, 0, 1, 0, 0, 0, 0, 1]),
};

/** A damped spring; keeps its velocity when the target changes, so transitions can be reversed mid-flight. */
class Spring {
	velocity = 0;
	constructor(
		public value: number,
		public target = value,
		public stiffness = 180,
		public damping = 24,
	) {}

	step(dt: number) {
		const steps = Math.max(1, Math.ceil(dt * 240));
		const h = dt / steps;
		for (let i = 0; i < steps; i++) {
			const a = -this.stiffness * (this.value - this.target) - this.damping * this.velocity;
			this.velocity += a * h;
			this.value += this.velocity * h;
		}
		if (this.resting) {
			this.value = this.target;
			this.velocity = 0;
		}
	}

	get resting() {
		return Math.abs(this.value - this.target) < 5e-4 && Math.abs(this.velocity) < 5e-3;
	}
}

/** `view`'s border box in `root`'s coordinates (DIP), from layout, so it ignores hit-testing transforms. */
function rectIn(view: View, root: View): Rect {
	let x = 0;
	let y = 0;
	let node: any = view;
	while (node && node !== root) {
		const b = node._getCurrentLayoutBounds();
		x += b.left;
		y += b.top;
		node = node.parent;
	}
	return { x: dip(x), y: dip(y), w: dip(view.getMeasuredWidth()), h: dip(view.getMeasuredHeight()) };
}

const size = (view: View) => ({ w: dip(view.getMeasuredWidth()), h: dip(view.getMeasuredHeight()) });

function label(text: string, fontSize: number, color: string, extra: Partial<Label> = {}): Label {
	const l = new Label();
	l.text = text;
	l.fontSize = fontSize;
	l.color = color as any;
	Object.assign(l, extra);
	return l;
}

export function navigatingTo(args: EventData) {
	const page = <Page>args.object;
	page.bindingContext = new TransitionsModel();
}

class TransitionsModel extends Observable {
	private raf = 0;
	private disposed = false;
	/** Time runs this many times slower; tap the status line to toggle. */
	private slow = Number(launchArgs.slow) || 1;

	constructor() {
		super();
		this.set('status', 'waiting for the first paint…');
		this.set('effect', 0);
	}

	pickEffect(args: EventData) {
		const index = EFFECTS.indexOf(String((args.object as Label).text).toLowerCase() as Effect);
		if (index > -1) {
			this.set('effect', index);
		}
	}

	toggleSlow() {
		this.slow = this.slow === 1 ? 6 : 1;
	}

	canvasLoaded(args: EventData) {
		this.disposed = false;
		const canvas = args.object as Canvas & any;
		const dark = Application.systemAppearance?.() === 'dark';
		const theme = dark ? { page: '#0b0b0f', card: '#1c1c22', text: '#f5f5f7', muted: '#9a9aa5', chip: '#2a2a32' } : { page: '#eef0f5', card: '#ffffff', text: '#121216', muted: '#6b6b76', chip: '#f1f2f6' };
		const pageColor: Vec4 = [...rgb(theme.page), 1];
		const cardColor: Vec4 = [...rgb(theme.card), 1];

		// Views. All are direct, drawable children of the canvas; where they sit in its layout does not matter.
		const header = new StackLayout();
		header.padding = '14 16 6 16';
		header.addChild(label('Explore', 34, theme.text, { fontWeight: 'bold' }));
		header.addChild(label('Tap a place. Drag the photo down to close it.', 14, theme.muted));

		const cards = PLACES.map((place, index) => {
			const root = new StackLayout();
			root.setInlineStyle(`background-color: ${theme.card}; border-radius: ${CARD_RADIUS};`);
			const hero = new GridLayout();
			hero.height = 112;
			hero.setInlineStyle(`background: linear-gradient(135deg, ${place.from}, ${place.to}); border-radius: ${CARD_RADIUS} ${CARD_RADIUS} 0 0;`);
			hero.addChild(label(place.emoji, 52, '#fff', { horizontalAlignment: 'center', verticalAlignment: 'middle' }));
			const title = label(place.name, 17, theme.text, { fontWeight: 'bold', margin: '12 14 0 14' });
			root.addChild(hero);
			root.addChild(title);
			root.addChild(label(place.region, 13, theme.muted, { margin: '2 14 14 14' }));
			root.on('tap', () => open(index));
			root.on('touch', (e: TouchGestureEventData) => {
				if (e.action === 'down') {
					pressed = index;
				} else if (e.action === 'up' || e.action === 'cancel') {
					pressed = -1;
				}
			});
			return { root, hero, title };
		});

		const detail = new StackLayout();
		detail.setInlineStyle(`background-color: ${theme.card};`);
		const detailHero = new GridLayout();
		detailHero.height = 300;
		const detailEmoji = label('', 120, '#fff', { horizontalAlignment: 'center', verticalAlignment: 'middle' });
		detailHero.addChild(detailEmoji);
		const detailTitle = label('', 34, theme.text, { fontWeight: 'bold', margin: '20 22 0 22' });
		const body = new StackLayout();
		body.margin = '2 22 0 22';
		const detailRegion = label('', 16, theme.muted);
		const chips = new GridLayout();
		chips.columns = '*, *, *';
		chips.rows = 'auto';
		chips.marginTop = 18;
		const chipValues = ['rating', 'temp', 'flight'].map((caption, i) => {
			const chip = new StackLayout();
			chip.col = i;
			chip.margin = i === 0 ? '0 6 0 0' : i === 2 ? '0 0 0 6' : '0 3';
			chip.setInlineStyle(`background-color: ${theme.chip}; border-radius: 14; padding: 10 12;`);
			const value = label('', 18, theme.text, { fontWeight: 'bold' });
			chip.addChild(value);
			chip.addChild(label(caption, 12, theme.muted));
			chips.addChild(chip);
			return value;
		});
		const blurb = label('', 16, theme.text, { textWrap: true, marginTop: 18, lineHeight: 4 } as any);
		const book = new Button();
		book.marginTop = 22;
		book.height = 50;
		book.fontSize = 16;
		book.fontWeight = 'bold';
		book.on('tap', () => {
			booked.has(current) ? booked.delete(current) : booked.add(current);
			bindDetail(current);
		});
		body.addChild(detailRegion);
		body.addChild(chips);
		body.addChild(blurb);
		body.addChild(book);
		detail.addChild(detailHero);
		detail.addChild(detailTitle);
		detail.addChild(body);
		detail.on('pan', (e: PanGestureEventData) => onPan(e));

		const close = new GridLayout();
		close.setInlineStyle('background-color: rgba(0,0,0,0.38); border-radius: 20;');
		close.addChild(label('✕', 18, '#fff', { horizontalAlignment: 'center', verticalAlignment: 'middle' }));
		close.on('tap', () => dismiss(0));

		const all: View[] = [header, ...cards.map((c) => c.root), detail, close];
		for (const view of all) {
			(view as any).drawable = true;
			canvas.addChild(view);
		}

		// WebGL on the Android emulator, which has no usable WebGPU adapter. Created once the canvas has
		// its real size: a WebGPU surface configured at 300x150 stays that size.
		let r: Renderer | null = null;
		let creating = false;
		const emulator = __ANDROID__ && /generic|emulator|sdk_gphone/i.test(String(android.os.Build.FINGERPRINT) + String(android.os.Build.HARDWARE));
		const createRenderer = async () => {
			const wanted = launchArgs.renderer ?? (emulator ? 'webgl' : 'webgpu');
			r = (wanted === 'webgpu' ? await webgpuRenderer(canvas) : null) ?? webglRenderer(canvas);
		};

		// One texture per drawable, refreshed when its snapshot changes.
		const textures = new Map<View, Tex>();
		const dirty = new Set<View>();
		const upload = (view: View) => {
			const def = canvas.getElementImageDefaultSize(view);
			const w = Math.max(1, Math.ceil(def.width - 1e-3));
			const h = Math.max(1, Math.ceil(def.height - 1e-3));
			const tex = r.upload(view, textures.get(view), w, h);
			tex.sx = def.width / w;
			tex.sy = def.height / h;
			textures.set(view, tex);
		};

		const booked = new Set<number>();
		let current = -1; // the place bound into the detail page
		let selected = -1; // the card the detail page comes out of
		let pending = -1; // waiting for the detail page's new snapshot before opening
		let pendingSince = 0;
		let pressed = -1;
		const press = cards.map(() => 0);
		const progress = new Spring(0); // 0: list, 1: detail
		const dragX = new Spring(0, 0, 260, 30);
		const dragY = new Spring(0, 0, 260, 30);
		let dragging = false;
		let dragSamples: { t: number; y: number }[] = [];
		// A drag-to-dismiss lets go of the page here; the close shrinks it from there.
		let releasedFrom: { rect: Rect; radius: number } | null = null;
		let effect: Effect = 'morph'; // the running transition's effect
		let openEffect: Effect = 'morph'; // restored after a drag that did not dismiss
		let interactive: 'list' | 'detail' | 'none' = 'none';
		let frozen = -1; // a fixed progress, for inspecting a transition frame
		let clock = 0;

		let W = 0;
		let H = 0;
		let g = 1;
		let proj = m4.identity();

		const bindDetail = (index: number) => {
			const place = PLACES[index];
			current = index;
			detailHero.setInlineStyle(`background: linear-gradient(135deg, ${place.from}, ${place.to});`);
			detailEmoji.text = place.emoji;
			detailTitle.text = place.name;
			detailRegion.text = place.region;
			chipValues[0].text = `★ ${place.rating}`;
			chipValues[1].text = place.temp;
			chipValues[2].text = place.flight;
			blurb.text = place.blurb;
			book.text = booked.has(index) ? 'Booked ✓' : 'Book this trip';
			book.setInlineStyle(`background-color: ${place.to}; color: #fff; border-radius: 14;`);
		};

		const useEffect = (next: Effect) => {
			effect = next;
			[progress.stiffness, progress.damping] = SPRINGS[next];
		};

		const open = (index: number) => {
			if (interactive !== 'list') {
				return;
			}
			selected = index;
			releasedFrom = null;
			dragX.value = dragY.value = 0;
			openEffect = EFFECTS[this.get('effect')] ?? 'morph';
			useEffect(openEffect);
			if (current === index) {
				start();
			} else {
				bindDetail(index);
				pending = index;
				pendingSince = Date.now();
			}
		};

		const start = () => {
			pending = -1;
			progress.target = 1;
		};

		/** `velocity` in progress units per second (negative closes faster). */
		const dismiss = (velocity: number) => {
			if (selected < 0 || progress.target === 0) {
				return;
			}
			progress.target = 0;
			progress.velocity = Math.min(progress.velocity, velocity);
		};

		const onPan = (e: PanGestureEventData) => {
			if (selected < 0 || progress.target !== 1 || !progress.resting) {
				return;
			}
			const now = Date.now();
			if (e.state === GestureStateTypes.began) {
				dragging = true;
				dragSamples = [];
				// You are holding the page, so it shrinks back into its card whatever the effect.
				useEffect('morph');
			} else if (e.state === GestureStateTypes.changed) {
				dragging = true;
				dragX.target = dragX.value = e.deltaX;
				dragY.target = dragY.value = e.deltaY > 0 ? e.deltaY : e.deltaY * 0.15;
				dragX.velocity = dragY.velocity = 0;
				dragSamples.push({ t: now, y: e.deltaY });
				dragSamples = dragSamples.filter((s) => now - s.t < 100);
			} else if (e.state === GestureStateTypes.ended || e.state === GestureStateTypes.cancelled) {
				dragging = false;
				const first = dragSamples[0];
				const last = dragSamples[dragSamples.length - 1];
				const vy = first && last && last.t > first.t ? ((last.y - first.y) / (last.t - first.t)) * 1000 : 0;
				if (dragY.value > H * 0.18 || vy > 900) {
					releasedFrom = { rect: dragRect(), radius: dragRadius() };
					dragX.value = dragX.target = dragY.value = dragY.target = 0;
					dismiss(-Math.min(4, Math.max(0, vy) / 700));
				} else {
					dragX.target = dragY.target = 0;
				}
			}
		};

		const cardW = () => (W - PAD * 2 - GAP) / 2;
		const headerH = () => size(header).h;
		const cardRect = (i: number): Rect => {
			const { h } = size(cards[i].root);
			const col = i % 2;
			const row = Math.floor(i / 2);
			return { x: PAD + col * (cardW() + GAP), y: headerH() + 6 + row * (h + GAP), w: cardW(), h };
		};
		const closeRect: Rect = { x: 16, y: 16, w: 40, h: 40 };
		const dragAmount = () => clamp01(dragY.value / (H * 0.7));
		const dragRadius = () => 34 * clamp01(dragAmount() * 6);
		const dragRect = (): Rect => {
			const s = 1 - 0.32 * dragAmount();
			const w = W * s;
			const h = H * s;
			return { x: (W - w) / 2 + dragX.value * 0.55, y: dragY.value * 0.6 + (H - h) * 0.12, w, h };
		};
		const isDragged = () => dragging || !dragY.resting || !dragX.resting;

		// Hit testing: the views at rest get their geometry, everything else none.
		const place = (view: View, r: Rect) => canvas.updateElementGeometry(view, { canvasTransform: { a: g, b: 0, c: 0, d: g, e: r.x * g, f: r.y * g } });
		const setInteractive = (mode: 'list' | 'detail' | 'none', force = false) => {
			if (mode === interactive && !force) {
				return;
			}
			interactive = mode;
			for (const view of all) {
				try {
					canvas.clearElementGeometry(view);
				} catch (e) {}
			}
			if (mode === 'list') {
				place(header, { x: 0, y: 0, ...size(header) });
				cards.forEach((card, i) => place(card.root, cardRect(i)));
			} else if (mode === 'detail') {
				place(detail, { x: 0, y: 0, w: W, h: H });
				place(close, closeRect);
			}
		};

		const sizeViews = () => {
			header.width = W;
			for (const card of cards) {
				card.root.width = cardW();
			}
			detail.width = W;
			detail.height = H;
			close.width = close.height = 40;
		};

		// Drawing. Rects are DIP; `quad` takes pixels.
		const px = (r: Rect): Rect => ({ x: r.x * g, y: r.y * g, w: r.w * g, h: r.h * g });
		type QuadOptions = { tex?: Tex; uv?: Vec4; color?: Vec4; alpha?: number; radius?: number; clip?: Rect | null; clipRadius?: number; shade?: number; shadow?: number; model?: Float32Array; size?: [number, number] };
		const identity = m4.identity();
		const qd = new Float32Array(QUAD_FLOATS);
		const quad = (dst: Rect, o: QuadOptions) => {
			qd.set(proj, 0);
			qd.set(o.model ?? identity, 16);
			const uv = o.uv ?? [0, 0, 1, 1];
			const c = o.color ?? [0, 0, 0, 0];
			const clip = o.clip ? px(o.clip) : { x: 0, y: 0, w: -1, h: -1 };
			const sz = o.size ?? [dst.w, dst.h];
			qd.set([dst.x, dst.y, dst.w, dst.h, uv[0], uv[1], uv[2], uv[3], c[0] * c[3], c[1] * c[3], c[2] * c[3], c[3], clip.x, clip.y, clip.w, clip.h], 32);
			qd.set([sz[0], sz[1], (o.radius ?? 0) * g, o.alpha ?? 1, o.shadow ? 2 : o.color ? 1 : 0, (o.shadow ?? 0) * g, o.shade ?? 1, (o.clipRadius ?? 0) * g], 48);
			r.quad(qd, o.tex ?? null);
		};
		/** Part `r` (DIP, in the view) of a view's snapshot at `d` (DIP, on the canvas). */
		const crop = (view: View, r: Rect, d: Rect, o: QuadOptions = {}) => {
			const t = textures.get(view);
			if (!t) {
				return;
			}
			const { w, h } = size(view);
			const x = Math.max(0, r.x);
			const y = Math.max(0, r.y);
			const cw = Math.min(r.w - (x - r.x), w - x);
			const ch = Math.min(r.h - (y - r.y), h - y);
			if (cw <= 0 || ch <= 0 || d.w <= 0 || d.h <= 0) {
				return;
			}
			const uv: Vec4 = [(x / w) * t.sx, (y / h) * t.sy, ((x + cw) / w) * t.sx, ((y + ch) / h) * t.sy];
			quad(px({ x: d.x + ((x - r.x) / r.w) * d.w, y: d.y + ((y - r.y) / r.h) * d.h, w: d.w * (cw / r.w), h: d.h * (ch / r.h) }), { ...o, tex: t, uv });
		};
		const whole = (view: View, d: Rect, o: QuadOptions = {}) => crop(view, { x: 0, y: 0, ...size(view) }, d, o);
		/** Like object-fit: cover, by cropping the source to the destination's aspect. */
		const cover = (view: View, r: Rect, d: Rect, o: QuadOptions = {}) => {
			const aspect = d.w / d.h;
			let sw = r.w;
			let sh = r.h;
			if (sw / sh > aspect) {
				sw = sh * aspect;
			} else {
				sh = sw / aspect;
			}
			crop(view, { x: r.x + (r.w - sw) / 2, y: r.y + (r.h - sh) / 2, w: sw, h: sh }, d, o);
		};
		const fill = (r: Rect, color: Vec4, o: QuadOptions = {}) => quad(px(r), { ...o, color });

		/** The list, into the scene texture. The geometric effects move the other cards out of the way. */
		const drawList = (q: number) => {
			r.begin('scene', pageColor);
			const choreograph = (effect === 'morph' || effect === 'flip') && selected >= 0 && q > 0;
			const depth = choreograph && effect === 'morph' ? 1 - 0.06 * q : 1;
			const scaleAround = (r: Rect): Rect => ({ x: W / 2 + (r.x - W / 2) * depth, y: H / 2 + (r.y - H / 2) * depth, w: r.w * depth, h: r.h * depth });
			whole(header, scaleAround({ x: 0, y: 0, ...size(header) }), { alpha: choreograph ? 1 - q : 1 });
			const origin = selected >= 0 ? cardRect(selected) : null;
			const diagonal = Math.hypot(W, H);
			for (let i = 0; i < cards.length; i++) {
				if (choreograph && i === selected) {
					continue;
				}
				const r = cardRect(i);
				let t = 0;
				let dx = 0;
				let dy = 0;
				if (choreograph && origin) {
					const vx = r.x + r.w / 2 - (origin.x + origin.w / 2);
					const vy = r.y + r.h / 2 - (origin.y + origin.h / 2);
					const distance = Math.hypot(vx, vy) || 1;
					// Neighbours move first, then the ones further away.
					t = smooth(0, 1, (q - (distance / diagonal) * 0.35) / 0.65);
					dx = (vx / distance) * 46 * t;
					dy = (vy / distance) * 46 * t;
				}
				const scale = (1 - 0.1 * t) * (1 - 0.04 * press[i]);
				const w = r.w * scale;
				const h = r.h * scale;
				whole(cards[i].root, scaleAround({ x: r.x + (r.w - w) / 2 + dx, y: r.y + (r.h - h) / 2 + dy, w, h }), { alpha: 1 - 0.75 * t, radius: CARD_RADIUS });
			}
			r.end();
		};
		const sceneQuad = (o: QuadOptions = {}) => quad({ x: 0, y: 0, w: r.scene.w, h: r.scene.h }, { ...o, tex: r.scene, uv: r.sceneUv });

		/** The card's rect inside the receding list (the morph scales the list back a little). */
		const startRect = (q: number): Rect => {
			const base = cardRect(selected);
			const depth = effect === 'morph' ? 1 - 0.06 * q : 1;
			return { x: W / 2 + (base.x - W / 2) * depth, y: H / 2 + (base.y - H / 2) * depth, w: base.w * depth, h: base.h * depth };
		};

		/** The detail page at rest or held by a drag, over the dimmed list. */
		const drawOpen = () => {
			const target = isDragged() ? dragRect() : { x: 0, y: 0, w: W, h: H };
			const radius = isDragged() ? dragRadius() : 0;
			if (isDragged()) {
				sceneQuad();
				fill({ x: 0, y: 0, w: W, h: H }, [0, 0, 0, 0.42 * (1 - dragAmount() * 0.5)]);
				fill({ x: target.x, y: target.y + 10, w: target.w, h: target.h }, [0, 0, 0, 0.35], { shadow: 30, radius });
			}
			whole(detail, target, { radius });
			drawClose(1, target);
		};

		const drawClose = (b: number, target: Rect) => {
			const shown = b * (1 - clamp01(dragAmount() * 5));
			if (shown <= 0) {
				return;
			}
			const so = target.w / W;
			const s = (0.6 + 0.4 * shown) * so;
			whole(close, { x: target.x + closeRect.x * so + 20 * (so - s), y: target.y + closeRect.y * so + 20 * (so - s), w: 40 * s, h: 40 * s }, { alpha: shown, radius: 20 });
		};

		/** Container transform: the page grows out of the card; the photo and title are shared elements. */
		const drawMorph = (p: number) => {
			const q = clamp01(p);
			sceneQuad();
			fill({ x: 0, y: 0, w: W, h: H }, [0, 0, 0, 0.42 * q]);
			const card = cards[selected];
			const base = cardRect(selected);
			const c = startRect(q);
			const target = releasedFrom?.rect ?? { x: 0, y: 0, w: W, h: H };
			const R = lerpRect(c, target, p);
			const radius = lerp(CARD_RADIUS * (c.w / base.w), releasedFrom?.radius ?? 0, q);
			const sc = c.w / base.w;
			const so = target.w / W;
			const lift = Math.max(4 * q * (1 - q), releasedFrom ? q : 0);
			fill({ x: R.x, y: R.y + 10 * q, w: R.w, h: R.h }, [0, 0, 0, 0.35 * lift], { shadow: 30, radius });
			fill(R, cardColor, { radius });
			const clip = { clip: R, clipRadius: radius };

			const hc = rectIn(card.hero, card.root);
			const tc = rectIn(card.title, card.root);
			const hd = rectIn(detailHero, detail);
			const td = rectIn(detailTitle, detail);
			const bodyTop = rectIn(body, detail).y;

			// The photo: one rect, the page's under the card's, which fades out.
			const hero = lerpRect({ x: c.x + hc.x * sc, y: c.y + hc.y * sc, w: hc.w * sc, h: hc.h * sc }, { x: target.x + hd.x * so, y: target.y + hd.y * so, w: hd.w * so, h: hd.h * so }, p);
			cover(detail, hd, hero, clip);
			cover(card.root, hc, hero, { ...clip, alpha: 1 - smooth(0, 0.6, p) });

			// The title, anchored top-left and scaled uniformly so the text grows rather than stretches.
			const title = lerpRect({ x: c.x + tc.x * sc, y: c.y + tc.y * sc, w: tc.w * sc, h: tc.h * sc }, { x: target.x + td.x * so, y: target.y + td.y * so, w: td.w * so, h: td.h * so }, p);
			const k1 = title.h / tc.h;
			const k2 = title.h / td.h;
			crop(card.root, tc, { x: title.x, y: title.y, w: tc.w * k1, h: title.h }, { ...clip, alpha: 1 - smooth(0, 0.5, p) });
			const restTop = tc.y + tc.h;
			const restH = size(card.root).h - restTop;
			crop(card.root, { x: 0, y: restTop, w: base.w, h: restH }, { x: title.x - tc.x * k1, y: title.y + title.h, w: base.w * k1, h: restH * k1 }, { ...clip, alpha: 1 - smooth(0, 0.25, p) });
			crop(detail, td, { x: title.x, y: title.y, w: td.w * k2, h: title.h }, { ...clip, alpha: smooth(0.15, 0.65, p) });

			// Everything below the title slides up into place once the photo has mostly landed.
			const e = smooth(0.4, 1, p);
			const s = R.w / W;
			const bodyH = H - bodyTop;
			crop(detail, { x: 0, y: bodyTop, w: W, h: bodyH }, { x: R.x, y: title.y + title.h + (bodyTop - (td.y + td.h)) * s + (1 - e) * 40, w: W * s, h: bodyH * s }, { ...clip, alpha: e });
			drawClose(smooth(0.7, 1, p), target);
		};

		/** The card lifts off the list, turns over and lands as the page; the list tilts back like a floor. */
		const drawFlip = (p: number) => {
			const q = clamp01(p);
			const Wp = W * g;
			const Hp = H * g;
			const floor = m4.chain(m4.translate(Wp / 2, Hp, Hp * 0.25 * q), m4.rotateX(-0.55 * q), m4.translate(-Wp / 2, -Hp, 0));
			sceneQuad({ model: floor, shade: 1 - 0.55 * q });

			const c = px(cardRect(selected));
			const w = lerp(c.w, Wp, q);
			const h = lerp(c.h, Hp, q);
			const cx = lerp(c.x + c.w / 2, Wp / 2, p);
			const cy = lerp(c.y + c.h / 2, Hp / 2, p);
			const angle = Math.PI * p;
			const arc = Math.sin(Math.PI * q);
			const model = m4.chain(m4.translate(cx, cy, -Hp * 0.3 * arc), m4.rotateY(angle), m4.rotateX(0.3 * arc));
			// Its shadow on the floor: narrower edge-on, further and softer the higher it lifts.
			const shadowW = w * (0.35 + 0.65 * Math.abs(Math.cos(angle)));
			quad({ x: cx - shadowW / 2, y: cy - h / 2 + Hp * 0.06 * arc, w: shadowW, h }, { color: [0, 0, 0, 0.45 * arc], shadow: 24 + 40 * arc, radius: CARD_RADIUS });
			const front = Math.cos(angle) >= 0;
			const view = front ? cards[selected].root : detail;
			const t = textures.get(view);
			if (t) {
				const shade = 0.5 + 0.5 * Math.abs(Math.cos(angle));
				const uv: Vec4 = front ? [0, 0, t.sx, t.sy] : [t.sx, 0, 0, t.sy];
				quad({ x: -0.5, y: -0.5, w: 1, h: 1 }, { model: m4.mul(model, m4.scale(w, h)), size: [w, h], tex: t, uv, shade, radius: front ? CARD_RADIUS : CARD_RADIUS * (1 - q) });
			}
			drawClose(smooth(0.85, 1, p), { x: 0, y: 0, w: W, h: H });
		};

		const hash = (x: number, y: number) => {
			const v = Math.sin(x * 127.1 + y * 311.7) * 43758.5453;
			return v - Math.floor(v);
		};
		const sceneUvAt = (x: number, y: number, w: number, h: number, Wp: number, Hp: number): Vec4 => {
			const [u0, v0, u1, v1] = r.sceneUv;
			return [lerp(u0, u1, x / Wp), lerp(v0, v1, y / Hp), lerp(u0, u1, (x + w) / Wp), lerp(v0, v1, (y + h) / Hp)];
		};
		const pageUv = (t: Tex): Vec4 => [0, 0, t.sx, t.sy];
		const unit: Rect = { x: -0.5, y: -0.5, w: 1, h: 1 };

		/** The list cracks into tiles that tumble toward you, nearest the card first; the page waits beneath. */
		const drawShatter = (p: number) => {
			const t = textures.get(detail);
			if (!t) {
				return;
			}
			const Wp = W * g;
			const Hp = H * g;
			const settle = smooth(0.2, 1, p);
			const k = lerp(0.86, 1, settle);
			quad({ x: (Wp * (1 - k)) / 2, y: (Hp * (1 - k)) / 2, w: Wp * k, h: Hp * k }, { tex: t, uv: pageUv(t), shade: lerp(0.3, 1, settle) });
			const c = px(cardRect(selected));
			const ox = c.x + c.w / 2;
			const oy = c.y + c.h / 2;
			const cols = 8;
			const rows = Math.round((cols * Hp) / Wp);
			const tw = Wp / cols;
			const th = Hp / rows;
			const far = Math.hypot(Math.max(ox, Wp - ox), Math.max(oy, Hp - oy));
			const tiles: { t: number; draw: () => void }[] = [];
			for (let j = 0; j < rows; j++) {
				for (let i = 0; i < cols; i++) {
					const x = i * tw;
					const y = j * th;
					const cx = x + tw / 2;
					const cy = y + th / 2;
					const h1 = hash(i, j);
					const h2 = hash(j + 7, i + 3);
					const delay = (0.45 * Math.hypot(cx - ox, cy - oy)) / far + 0.1 * h1;
					const life = clamp01((p - delay) / 0.45);
					if (life >= 1) {
						continue;
					}
					const e = life * life;
					const dx = cx - ox;
					const dy = cy - oy;
					const len = Math.hypot(dx, dy) || 1;
					// Still tiles overlap a little so no seams show; moving ones open a crack.
					const gap = life > 0 ? 1 - 0.06 * Math.min(1, life * 12) : 1 + 1.5 / Math.min(tw, th);
					const model = m4.chain(m4.translate(cx + (dx / len) * e * Hp * 0.5, cy + (dy / len) * e * Hp * 0.35 + e * e * Hp * 0.6, -e * Hp * 0.8), m4.rotateY((h1 - 0.5) * 7 * e), m4.rotateX((h2 - 0.5) * 7 * e), m4.rotateZ((h1 - h2) * 3 * e), m4.scale(tw * gap, th * gap));
					tiles.push({ t: life, draw: () => quad(unit, { model, size: [tw * gap, th * gap], tex: r.scene, uv: sceneUvAt(x, y, tw, th, Wp, Hp), alpha: 1 - smooth(0.55, 1, life), shade: 1 - 0.45 * Math.abs(Math.sin(e * 3 + h2 * 6)) * Math.min(1, life * 6), radius: life > 0 ? 3 : 0 }) });
				}
			}
			tiles.sort((a, b) => a.t - b.t);
			for (const tile of tiles) {
				tile.draw();
			}
			drawClose(smooth(0.85, 1, p), { x: 0, y: 0, w: W, h: H });
		};

		/** The list and the page are two faces of a cube; it turns to the page while the camera pulls back. */
		const drawCube = (p: number) => {
			const t = textures.get(detail);
			if (!t) {
				return;
			}
			const Wp = W * g;
			const Hp = H * g;
			const angle = (Math.PI / 2) * p;
			const base = m4.chain(m4.translate(Wp / 2, Hp / 2, Wp / 2 + Math.sin(Math.PI * clamp01(p)) * Hp * 0.55), m4.rotateY(angle));
			const list = () => quad(unit, { model: m4.chain(base, m4.translate(0, 0, -Wp / 2), m4.scale(Wp, Hp)), size: [Wp, Hp], tex: r.scene, uv: r.sceneUv, shade: 0.3 + 0.7 * Math.max(0, Math.cos(angle)) });
			const page = () => quad(unit, { model: m4.chain(base, m4.translate(Wp / 2, 0, 0), m4.rotateY(-Math.PI / 2), m4.scale(Wp, Hp)), size: [Wp, Hp], tex: t, uv: pageUv(t), shade: 0.3 + 0.7 * Math.max(0, Math.sin(angle)) });
			if (angle < Math.PI / 4) {
				page();
				list();
			} else {
				list();
				page();
			}
			drawClose(smooth(0.9, 1, p), { x: 0, y: 0, w: W, h: H });
		};

		/** The full-screen shader effects, from the list scene to the page. */
		const fxData = new Float32Array(QUAD_FLOATS);
		const drawShader = (p: number) => {
			const t = textures.get(detail);
			if (!t) {
				return;
			}
			const c = px(cardRect(selected));
			const colors = PLACES[current] ?? PLACES[0];
			const ox = c.x + c.w / 2;
			const oy = c.y + c.h * 0.35;
			// Puts the page's photo under the card at the start; the effects slide it home.
			const hero = px(rectIn(detailHero, detail));
			fxData.set([r.scene.w, r.scene.h, t.sx, t.sy, ox, oy, clamp01(p), clock, ...rgb(colors.from), 1, ...rgb(colors.to), 1, hero.x + hero.w / 2 - ox, hero.y + hero.h / 2 - oy]);
			r.effect(effect as FxName, fxData, t);
			drawClose(smooth(0.85, 1, p), { x: 0, y: 0, w: W, h: H });
		};

		let frames = 0;
		let paints = 0;
		let uploadMs = 0;
		let lastFps = 0;
		let worst = 0;

		// Upload while the paint event's snapshots are guaranteed; anything that fails is retried next frame.
		const flush = () => {
			const t0 = Date.now();
			for (const view of Array.from(dirty)) {
				try {
					upload(view);
					dirty.delete(view);
					if (view === detail && pending >= 0) {
						start();
					}
				} catch (e) {}
			}
			uploadMs += Date.now() - t0;
		};
		canvas.on('paint', (event: any) => {
			for (const view of event.changedElements) {
				dirty.add(view);
			}
			paints++;
			if (r) {
				flush();
			}
		});

		// `--auto=effect:2@200,open:3@800,freeze:0.4@1500,freeze@3000,close@5000,drag:260@8000,hit:200:300@9000`
		const runScript = (script: string) => {
			const names = (view: any) => (view === header ? 'header' : view === detail ? 'detail' : view === close ? 'close' : `card${cards.findIndex((c) => c.root === view)}`);
			for (const step of script.split(',')) {
				const [action, at] = step.split('@');
				const [name, a, b] = action.split(':');
				setTimeout(() => {
					if (name === 'effect') {
						this.set('effect', +a);
					} else if (name === 'open') {
						open(+a);
					} else if (name === 'close') {
						dismiss(0);
					} else if (name === 'freeze') {
						frozen = a === undefined ? -1 : +a;
					} else if (name === 'drag') {
						const pan = (state: number, deltaY: number) => onPan({ state, deltaX: 0, deltaY } as any);
						pan(GestureStateTypes.began, 0);
						for (let i = 1; i <= 20; i++) {
							setTimeout(() => pan(i === 20 ? GestureStateTypes.ended : GestureStateTypes.changed, (+a * i) / 20), i * 16);
						}
					} else if (name === 'hit' && __APPLE__) {
						let hit = canvas._drawables?.host?.hitTestWithEvent(CGPointMake(+a, +b), null);
						const leaf = hit ? String(hit.class()) : 'nothing';
						while (hit && !all.some((v) => v.nativeViewProtected === hit)) {
							hit = hit.superview;
						}
						console.log(`HIC|hit ${a},${b} -> ${hit ? names(all.find((v) => v.nativeViewProtected === hit)) : 'none'} (${leaf})`);
					}
					console.log(`HIC|${action} interactive=${interactive} effect=${effect} p=${progress.value.toFixed(3)}`);
				}, +at);
			}
		};
		if (launchArgs.auto) {
			runScript(launchArgs.auto);
		}

		let last = 0;
		let failed = false;
		const draw = (time: number) => {
			if (this.disposed) {
				return;
			}
			this.raf = requestAnimationFrame(draw);
			// Core routes rAF exceptions to Trace, where they are easy to miss.
			try {
				frame(time);
			} catch (e) {
				if (!failed) {
					failed = true;
					console.error('canvas-html-transitions frame', e, e?.stack);
				}
			}
		};
		const frame = (time: number) => {
			// Core stamps a first frame requested outside a frame from another clock than the display link.
			const delta = time - last;
			const sane = last > 0 && delta > 0 && delta < 1000;
			if (!sane) {
				lastFps = time;
				frames = 0;
			}
			const dt = (sane ? Math.min(1 / 20, delta / 1000) : 1 / 60) / this.slow;
			worst = Math.max(worst, sane ? delta : 0);
			last = time;
			clock += dt;

			// Size the grid to the canvas; the views follow its DIP size.
			const pxW = Math.round(canvas.getMeasuredWidth());
			const pxH = Math.round(canvas.getMeasuredHeight());
			if (pxW <= 0 || pxH <= 0) {
				return;
			}
			if (canvas.width !== pxW || canvas.height !== pxH) {
				canvas.width = pxW;
				canvas.height = pxH;
				r?.resize(pxW, pxH);
			}
			if (!r) {
				if (!creating) {
					creating = true;
					createRenderer().catch((e) => console.error('canvas-html-transitions renderer', e));
				}
				return;
			}
			const cssW = dip(pxW);
			const cssH = dip(pxH);
			const resized = cssW !== W || cssH !== H;
			if (resized) {
				W = cssW;
				H = cssH;
				sizeViews();
			}
			g = pxW / W;
			// Pixels, y down, to clip space; w grows with depth for the flip's perspective, z stays inside 0..w.
			proj = new Float32Array([2 / pxW, 0, 0, 0, 0, -2 / pxH, 0, 0, 0, 0, 1e-5, 1 / (pxH * 2.2), -1, 1, 0.5, 1]);

			if (dirty.size) {
				flush();
			}

			// A new detail snapshot normally lands in a frame or two; don't hang if it never does.
			if (pending >= 0 && Date.now() - pendingSince > 500) {
				start();
			}

			progress.step(dt);
			if (frozen >= 0) {
				progress.value = frozen;
				progress.velocity = 0;
			}
			dragX.step(dt);
			dragY.step(dt);
			for (let i = 0; i < press.length; i++) {
				press[i] = lerp(press[i], pressed === i ? 1 : 0, 1 - Math.exp(-dt * 25));
			}
			if (!isDragged() && effect !== openEffect && progress.target === 1 && progress.resting) {
				useEffect(openEffect);
			}
			if (progress.target === 0 && progress.resting && pending < 0) {
				selected = -1;
				releasedFrom = null;
			}

			const p = progress.value;
			const settledList = selected < 0 && pending < 0;
			const settledDetail = selected >= 0 && progress.target === 1 && progress.resting && frozen < 0;
			setInteractive(settledList ? 'list' : settledDetail ? 'detail' : 'none', resized);

			r.frame(pxW, pxH);
			const transitioning = textures.has(header) && selected >= 0 && pending < 0 && textures.has(detail);
			if (transitioning && settledDetail) {
				if (isDragged()) {
					drawList(1);
				}
				r.begin('screen', pageColor);
				drawOpen();
			} else if (textures.has(header)) {
				drawList(transitioning ? clamp01(p) : 0);
				r.begin('screen', transitioning && (effect === 'flip' || effect === 'cube') ? [0.02, 0.02, 0.04, 1] : pageColor);
				if (!transitioning) {
					sceneQuad();
				} else if (effect === 'morph') {
					drawMorph(p);
				} else if (effect === 'flip') {
					drawFlip(p);
				} else if (effect === 'shatter') {
					drawShatter(p);
				} else if (effect === 'cube') {
					drawCube(p);
				} else if ((FX_NAMES as readonly string[]).includes(effect)) {
					drawShader(p);
				}
			} else {
				r.begin('screen', pageColor);
			}
			r.end();
			r.submit();

			frames++;
			if (time - lastFps >= 1000) {
				const fps = Math.round((frames * 1000) / (time - lastFps || 1));
				this.set('status', `${r.name} · ${fps} fps · worst frame ${worst.toFixed(1)} ms · ${paints} snapshots (${uploadMs} ms upload) · ${interactive}${this.slow !== 1 ? ` · ${this.slow}x slow motion` : ''}`);
				frames = 0;
				paints = 0;
				uploadMs = 0;
				worst = 0;
				lastFps = time;
			}
		};
		this.raf = requestAnimationFrame(draw);
	}

	canvasUnloaded() {
		this.disposed = true;
		cancelAnimationFrame(this.raf);
	}
}
