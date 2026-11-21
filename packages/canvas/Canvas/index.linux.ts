/**
 * The canvas view on NativeScript Linux: a GTK 4 widget made by the native host
 * (`CanvasModule.NSCCanvas`), which shows its context's frames as GL textures. Everything but the
 * widget and its events is shared with the other Node-API hosts (./napi-host).
 */
import { isUserInteractionEnabledProperty, Utils } from '@nativescript/core';
import { dispatchWindowKey, ignoreTouchEventsProperty } from './common';
import { NapiCanvas, cursorKeyword, cursorProperty } from './napi-host';

export * from './common';
export { createSVGMatrix } from './napi-host';

declare const interop: any;
declare const gtk_gesture_drag_new: any, gtk_event_controller_scroll_new: any, gtk_event_controller_key_new: any;
declare const gtk_widget_add_controller: any, gtk_widget_remove_controller: any, gtk_widget_set_focusable: any, gtk_widget_grab_focus: any;
declare const gtk_widget_get_scale_factor: any, gtk_widget_get_root: any, gtk_widget_translate_coordinates: any, gtk_widget_get_width: any, gtk_widget_get_height: any;
declare const gtk_event_controller_set_propagation_phase: any, gtk_gesture_single_set_button: any, gtk_window_get_default_size: any;
declare const g_signal_connect_data: any, g_signal_handler_disconnect: any, gdk_keyval_name: any, gdk_keyval_to_unicode: any;
declare const gtk_event_controller_motion_new: any, gtk_event_controller_get_current_event_device: any, gtk_gesture_single_get_current_button: any, gdk_device_get_source: any;
declare const gtk_root_get_focus: any, gtk_widget_is_ancestor: any, gtk_window_is_active: any, gtk_widget_set_cursor: any, gtk_widget_set_cursor_from_name: any, gtk_widget_get_cursor: any;
declare const gtk_widget_get_display: any, gtk_widget_get_native: any, gtk_native_get_surface: any, gtk_native_get_surface_transform: any, g_type_name_from_instance: any;
declare const gdk_surface_get_device_position: any, gdk_surface_get_scale_factor: any, gdk_display_get_default_seat: any, gdk_seat_get_pointer: any;
declare const gdk_x11_display_get_xdisplay: any, gdk_x11_surface_get_xid: any, XGrabPointer: any, XUngrabPointer: any, XWarpPointer: any, XFlush: any;

type Arg = 'double' | 'uint' | 'int' | 'pointer';

const handlers = new Map<number, (...args: any[]) => any>();
const trampolines = new Map<string, any>();
let nextHandler = 1;

/** One C trampoline per signal signature; `user_data` carries the handler id. */
function trampoline(args: Arg[], returnsBool: boolean) {
	const key = `${args.join(',')}:${returnsBool}`;
	let callback = trampolines.get(key);
	if (!callback) {
		const types = { double: interop.types.double, uint: interop.types.uint32, int: interop.types.int32, pointer: interop.types.pointer };
		callback = interop.callback(
			(...params: any[]) => {
				const handler = handlers.get(params[params.length - 1]?.toNumber?.() ?? 0);
				const result = handler ? handler(...params.slice(1, -1)) : undefined;
				return returnsBool ? (result ? 1 : 0) : undefined;
			},
			{ returns: returnsBool ? interop.types.int32 : interop.types.void, args: [interop.types.pointer, ...args.map((a) => types[a]), interop.types.pointer] },
		);
		trampolines.set(key, callback);
	}
	return callback;
}

/** Connects `handler(...args)` to `name` on `instance`; returns a disconnect function. */
function connect(instance: any, name: string, args: Arg[], handler: (...args: any[]) => any, returnsBool = false) {
	const id = nextHandler++;
	handlers.set(id, handler);
	const handlerId = g_signal_connect_data(instance, name, trampoline(args, returnsBool), new interop.Pointer(id), null, 0);
	return () => {
		if (handlers.delete(id)) {
			g_signal_handler_disconnect(instance, handlerId);
		}
	};
}

const GTK_EVENT_CONTROLLER_SCROLL_BOTH_AXES = 3;
const GTK_PHASE_CAPTURE = 1;
const GDK_SOURCE_PEN = 1;
const GDK_SOURCE_TOUCHSCREEN = 3;
const GDK_SHIFT_MASK = 1 << 0;
const GDK_CONTROL_MASK = 1 << 2;
const GDK_ALT_MASK = 1 << 3;
const GDK_SUPER_MASK = 1 << 26;
// X11: ButtonPress, ButtonRelease and PointerMotion masks; GrabModeAsync.
const X_GRAB_EVENTS = (1 << 2) | (1 << 3) | (1 << 6);
const X_GRAB_MODE_ASYNC = 1;

function cString(raw: any): string {
	return typeof raw === 'string' ? raw : raw ? interop.stringFromCString(raw) : '';
}

function addressOf(pointer: any): number {
	return pointer?.toNumber?.() ?? 0;
}

type PointerType = 'mouse' | 'pen' | 'touch';

/** Web `pointerType` of the device behind the event `controller` is handling. */
function pointerTypeOf(controller: any): PointerType {
	const device = gtk_event_controller_get_current_event_device(controller);
	switch (device ? gdk_device_get_source(device) : -1) {
		case GDK_SOURCE_PEN:
			return 'pen';
		case GDK_SOURCE_TOUCHSCREEN:
			return 'touch';
		default:
			return 'mouse';
	}
}

/** Web `button` and `buttons` bit for GDK buttons 1 left, 2 middle, 3 right, 8 back, 9 forward. */
const WEB_BUTTONS: Record<number, [number, number]> = { 1: [0, 1], 2: [1, 4], 3: [2, 2], 8: [3, 8], 9: [4, 16] };

/** Web `deltaY` pixels per wheel notch, as Chromium reports. */
const WHEEL_PIXELS_PER_NOTCH = 100;

const NAMED_KEYS: Record<string, [string, string]> = {
	Return: ['Enter', 'Enter'],
	KP_Enter: ['Enter', 'NumpadEnter'],
	BackSpace: ['Backspace', 'Backspace'],
	Tab: ['Tab', 'Tab'],
	Escape: ['Escape', 'Escape'],
	space: [' ', 'Space'],
	Delete: ['Delete', 'Delete'],
	Home: ['Home', 'Home'],
	End: ['End', 'End'],
	Page_Up: ['PageUp', 'PageUp'],
	Page_Down: ['PageDown', 'PageDown'],
	Left: ['ArrowLeft', 'ArrowLeft'],
	Up: ['ArrowUp', 'ArrowUp'],
	Right: ['ArrowRight', 'ArrowRight'],
	Down: ['ArrowDown', 'ArrowDown'],
	Shift_L: ['Shift', 'ShiftLeft'],
	Shift_R: ['Shift', 'ShiftRight'],
	Control_L: ['Control', 'ControlLeft'],
	Control_R: ['Control', 'ControlRight'],
	Alt_L: ['Alt', 'AltLeft'],
	Alt_R: ['Alt', 'AltRight'],
	Super_L: ['Meta', 'MetaLeft'],
	Super_R: ['Meta', 'MetaRight'],
	Meta_L: ['Meta', 'MetaLeft'],
	Meta_R: ['Meta', 'MetaRight'],
};

/** Web `key` / `code` for a GDK keyval. */
function keyInfo(keyval: number): { key: string; code: string } {
	const name = cString(gdk_keyval_name(keyval));
	const named = NAMED_KEYS[name];
	if (named) {
		return { key: named[0], code: named[1] };
	}
	if (/^F([1-9]|1[0-9]|2[0-4])$/.test(name)) {
		return { key: name, code: name };
	}
	const unicode = gdk_keyval_to_unicode(keyval);
	const key = unicode ? String.fromCodePoint(unicode) : 'Unidentified';
	if (/^[a-z]$/i.test(key)) {
		return { key, code: `Key${key.toUpperCase()}` };
	}
	if (/^[0-9]$/.test(key)) {
		return { key, code: `Digit${key}` };
	}
	return { key, code: 'Unidentified' };
}

/** The canvases in one window, which share a key controller on it. */
interface WindowHook {
	root: any;
	canvases: Set<Canvas>;
	controller: any;
	disconnects: (() => void)[];
	/** Keyvals held, for `repeat`. */
	held: Set<number>;
}

const windowHooks = new Map<number, WindowHook>();

/** X11: the pointer is grabbed (confined to the window) and warped back to the canvas center. */
interface X11Lock {
	xdisplay: any;
	xid: number;
	surface: any;
	device: any;
	scale: number;
	/** Widget tree origin in the surface (client-side decorations). */
	offset: [number, number];
	motion: any;
	disconnect: () => void;
	/** The canvas center in window coordinates. */
	center: [number, number];
	/** Where the pointer was in the window, to put it back on release, as Chrome does. */
	restore: [number, number] | null;
}

/** Wayland: the compositor holds the pointer in place and the widget's `pointer-lock-*` signals report its travel. */
interface WaylandLock {
	/** Travel without pointer acceleration (`unadjustedMovement`). */
	unadjusted: boolean;
	/** The compositor locked the pointer, which may wait until it enters the window. */
	held: boolean;
	/** Sub-pixel travel not yet reported: movementX/Y are whole pixels, as in browsers. */
	remainder: [number, number];
}

interface PointerLock {
	root: any;
	/** The window had no cursor of its own and was given 'none'. */
	rootCursor: boolean;
	x11: X11Lock | null;
	wayland: WaylandLock | null;
	/** Where moves are reported, in the canvas: clientX/Y stay where the lock began. */
	at: [number, number];
}

const X_GRAB_SUCCESS = 0;

/** Grabs the pointer for `widget`'s window under X11; null if the display is not X11 or the grab failed. */
function x11Lock(widget: any, center: [number, number], onMotion: (x: number, y: number) => void): X11Lock | null {
	let grabbed: any = null;
	let attached: { root: any; motion: any; disconnect?: () => void } | null = null;
	try {
		const display = gtk_widget_get_display(widget);
		if (cString(g_type_name_from_instance(display)) !== 'GdkX11Display') {
			return null;
		}
		const native = gtk_widget_get_native(widget);
		const surface = gtk_native_get_surface(native);
		const x = new interop.Reference(interop.types.double, 0);
		const y = new interop.Reference(interop.types.double, 0);
		gtk_native_get_surface_transform(native, x, y);
		const xdisplay = gdk_x11_display_get_xdisplay(display);
		const xid = gdk_x11_surface_get_xid(surface);
		// Owner events, so GTK still gets the pointer's events; cursor None: the window's own.
		if (XGrabPointer(xdisplay, xid, 1, X_GRAB_EVENTS, X_GRAB_MODE_ASYNC, X_GRAB_MODE_ASYNC, xid, 0, 0) !== X_GRAB_SUCCESS) {
			return null;
		}
		grabbed = xdisplay;
		const device = gdk_seat_get_pointer(gdk_display_get_default_seat(display));
		const offset: [number, number] = [x.value, y.value];
		const px = new interop.Reference(interop.types.double, 0);
		const py = new interop.Reference(interop.types.double, 0);
		const restore: [number, number] | null = gdk_surface_get_device_position(surface, device, px, py, null) ? [px.value - offset[0], py.value - offset[1]] : null;
		// Moves anywhere in the window, so the pointer is not lost off the canvas.
		const root = gtk_widget_get_root(widget);
		const motion = gtk_event_controller_motion_new();
		gtk_event_controller_set_propagation_phase(motion, GTK_PHASE_CAPTURE);
		gtk_widget_add_controller(root, motion);
		attached = { root, motion };
		const disconnect = connect(motion, 'motion', ['double', 'double'], onMotion);
		attached.disconnect = disconnect;
		const lock: X11Lock = { xdisplay, xid, surface, device, scale: gdk_surface_get_scale_factor(surface) || 1, offset, motion, disconnect, center, restore };
		lock.center = warp(lock, center);
		return lock;
	} catch (e) {
		// A failure after the grab must not leave the pointer grabbed.
		try {
			attached?.disconnect?.();
			if (attached) gtk_widget_remove_controller(attached.root, attached.motion);
			if (grabbed) XUngrabPointer(grabbed, 0);
		} catch (_) {}
		return null;
	}
}

/** Warps the pointer to `to` (window coordinates), snapped to a device pixel; returns where it went. */
function warp(lock: X11Lock, to: [number, number]): [number, number] {
	const x = Math.round((to[0] + lock.offset[0]) * lock.scale);
	const y = Math.round((to[1] + lock.offset[1]) * lock.scale);
	XWarpPointer(lock.xdisplay, 0, lock.xid, 0, 0, 0, 0, x, y);
	XFlush(lock.xdisplay);
	return [x / lock.scale - lock.offset[0], y / lock.scale - lock.offset[1]];
}

export class Canvas extends NapiCanvas {
	private _widget: any;
	private _ignoreTouchEvents = false;
	private _controllers: any[] = [];
	private _disconnects: (() => void)[] = [];
	private _windowHook: WindowHook | null = null;
	/** CSS cursor keyword, null for the default. */
	private _cursor: string | null = null;
	/** Where a mouse or pen is over the canvas, or null. */
	private _pointerAt: [number, number] | null = null;
	private _buttons = 0;
	private _lock: PointerLock | null = null;

	constructor(nativeInstance?: any) {
		super();
		this._attachHost(nativeInstance ?? new global.CanvasModule.NSCCanvas());
		this._widget = new interop.Pointer(this._canvas.widget);
	}

	// @ts-ignore
	get linux() {
		return this._widget;
	}

	createNativeView() {
		return this._widget;
	}

	initNativeView(): void {
		super.initNativeView();
		const widget = this._widget;
		if (!widget) {
			return;
		}
		const on = (instance: any, name: string, args: Arg[], handler: (...args: any[]) => any, returnsBool = false) => this._disconnects.push(connect(instance, name, args, handler, returnsBool));
		const add = (controller: any) => {
			gtk_widget_add_controller(widget, controller);
			this._controllers.push(controller);
			return controller;
		};
		const ref = new WeakRef(this);

		gtk_widget_set_focusable(widget, 1);
		on(widget, 'notify::scale-factor', ['pointer'], () => ref.deref()?._syncScale());
		on(widget, 'pointer-lock-state', ['int'], (held: number) => ref.deref()?._waylandLockState(held));
		on(widget, 'pointer-lock-motion', ['double', 'double', 'double', 'double'], (dx: number, dy: number, rawX: number, rawY: number) => ref.deref()?._waylandMotion(dx, dy, rawX, rawY));
		this._syncScale();

		// One pointer (mouse, pen or first touch) pressed. Touch moves are reported between down and up
		// only, as on iOS and Android; a mouse or pen also hovers (below).
		const drag = add(gtk_gesture_drag_new());
		gtk_gesture_single_set_button(drag, 0);
		gtk_event_controller_set_propagation_phase(drag, GTK_PHASE_CAPTURE);
		let start = [0, 0];
		let last = [0, 0];
		let down = false;
		let pointerType: PointerType = 'mouse';
		let button = 0;
		let buttons = 1;
		on(drag, 'drag-begin', ['double', 'double'], (x: number, y: number) => {
			const canvas = ref.deref();
			if (!canvas || canvas._ignoreTouchEvents) {
				return;
			}
			start = last = [x, y];
			down = true;
			pointerType = pointerTypeOf(drag);
			[button, buttons] = (pointerType !== 'touch' && WEB_BUTTONS[gtk_gesture_single_get_current_button(drag)]) || [0, 1];
			canvas._buttons = buttons;
			// A press on a hosted child (content="drawable") leaves focus to it.
			if (!canvas._canvas.picksChild(x, y)) {
				gtk_widget_grab_focus(widget);
			}
			// Locked, a mouse presses where it was locked.
			const [px, py] = canvas._lock && pointerType === 'mouse' ? canvas._lock.at : [x, y];
			canvas._handleEvents({ event: 'down', ptrId: 0, x: px, y: py, isPrimary: true, pointerType, button, buttons });
		});
		on(drag, 'drag-update', ['double', 'double'], (dx: number, dy: number) => {
			if (!down) {
				return;
			}
			last = [start[0] + dx, start[1] + dy];
			const canvas = ref.deref();
			// Locked moves come from the window's motion controller.
			if (canvas && !(canvas._lock && pointerType === 'mouse')) {
				canvas._handleEvents({ event: 'move', pointers: [{ ptrId: 0, x: last[0], y: last[1], isPrimary: true, pointerType, button: -1, buttons }] });
			}
		});
		on(drag, 'drag-end', ['double', 'double'], (dx: number, dy: number) => {
			if (!down) {
				return;
			}
			down = false;
			const [x, y] = (last = [start[0] + dx, start[1] + dy]);
			const canvas = ref.deref();
			if (!canvas) {
				return;
			}
			canvas._buttons = 0;
			if (canvas._lock && pointerType === 'mouse') {
				const [px, py] = canvas._lock.at;
				canvas._handleEvents({ event: 'up', ptrId: 0, x: px, y: py, isPrimary: true, pointerType, button, buttons: 0 });
				return;
			}
			canvas._handleEvents({ event: 'up', ptrId: 0, x, y, isPrimary: true, pointerType, button, buttons: 0 });
			// The drag's grab held back the motion controller's leave.
			if (pointerType !== 'touch' && !(x >= 0 && y >= 0 && x < gtk_widget_get_width(widget) && y < gtk_widget_get_height(widget))) {
				canvas._pointerAt = null;
				canvas._handleEvents({ event: 'leave', ptrId: 0, x, y, isPrimary: true, pointerType });
			}
		});
		on(drag, 'cancel', ['pointer'], () => {
			if (down) {
				down = false;
				const canvas = ref.deref();
				if (canvas) {
					canvas._buttons = 0;
					canvas._handleEvents({ event: 'cancel', ptrId: 0, x: last[0], y: last[1], isPrimary: true, pointerType });
				}
			}
		});

		// Hover: a mouse or pen with nothing pressed moves with buttons 0, as on the web. While pressed,
		// the drag reports the moves.
		const motion = add(gtk_event_controller_motion_new());
		const hover = (event: 'enter' | 'move', x: number, y: number) => {
			const canvas = ref.deref();
			if (!canvas || canvas._ignoreTouchEvents || down || canvas._lock) {
				return;
			}
			const type = pointerTypeOf(motion);
			if (type === 'touch') {
				return;
			}
			pointerType = type;
			last = [x, y];
			canvas._pointerAt = [x, y];
			const pointer = { ptrId: 0, x, y, isPrimary: true, pointerType };
			canvas._handleEvents(event === 'enter' ? { event, ...pointer } : { event, pointers: [{ ...pointer, button: -1, buttons: 0 }] });
		};
		on(motion, 'enter', ['double', 'double'], (x: number, y: number) => hover('enter', x, y));
		on(motion, 'motion', ['double', 'double'], (x: number, y: number) => hover('move', x, y));
		on(motion, 'leave', [], () => {
			const canvas = ref.deref();
			if (canvas && !down && !canvas._lock && pointerType !== 'touch') {
				canvas._pointerAt = null;
				canvas._handleEvents({ event: 'leave', ptrId: 0, x: last[0], y: last[1], isPrimary: true, pointerType });
			}
		});

		const scroll = add(gtk_event_controller_scroll_new(GTK_EVENT_CONTROLLER_SCROLL_BOTH_AXES));
		on(
			scroll,
			'scroll',
			['double', 'double'],
			(dx: number, dy: number) => {
				const canvas = ref.deref();
				if (!canvas || canvas._ignoreTouchEvents) {
					return false;
				}
				// No pointers: a wheel is not a move.
				canvas._handleEvents({ event: 'scale', deltaX: dx * WHEEL_PIXELS_PER_NOTCH, deltaY: dy * WHEEL_PIXELS_PER_NOTCH, deltaMode: 0, pointers: [], isInProgress: false });
				return true;
			},
			true,
		);
	}

	disposeNativeView(): void {
		this._detachWindow();
		for (const disconnect of this._disconnects.splice(0)) {
			disconnect();
		}
		const widget = this._widget;
		for (const controller of this._controllers.splice(0)) {
			try {
				gtk_widget_remove_controller(widget, controller);
			} catch (e) {}
		}
		super.disposeNativeView();
	}

	[ignoreTouchEventsProperty.setNative](value: boolean) {
		this._ignoreTouchEvents = value;
	}

	[isUserInteractionEnabledProperty.setNative](value: boolean) {
		this._ignoreTouchEvents = !value;
	}

	[cursorProperty.setNative](value: string) {
		this._cursor = cursorKeyword(value);
		if (!this._lock) {
			this._applyCursor();
		}
	}

	private _applyCursor() {
		if (this._widget) {
			// GTK takes CSS cursor names, 'none' included.
			gtk_widget_set_cursor_from_name(this._widget, this._cursor);
		}
	}

	onLoaded(): void {
		super.onLoaded();
		this._attachWindow();
	}

	onUnloaded(): void {
		this._detachWindow();
		super.onUnloaded();
	}

	/** Keys for the whole window: one capture-phase key controller on it, which sees them first whatever has focus. */
	private _attachWindow() {
		const widget = this._widget;
		if (this._windowHook || !widget || !this.isLoaded) {
			return;
		}
		const root = gtk_widget_get_root(widget);
		const id = addressOf(root);
		if (!id) {
			return;
		}
		let hook = windowHooks.get(id);
		if (!hook) {
			const controller = gtk_event_controller_key_new();
			gtk_event_controller_set_propagation_phase(controller, GTK_PHASE_CAPTURE);
			gtk_widget_add_controller(root, controller);
			const created: WindowHook = { root, canvases: new Set(), controller, disconnects: [], held: new Set() };
			created.disconnects.push(
				connect(controller, 'key-pressed', ['uint', 'uint', 'int'], (keyval: number, _keycode: number, state: number) => Canvas._onWindowKey(created, 'down', keyval, state), true),
				connect(controller, 'key-released', ['uint', 'uint', 'int'], (keyval: number, _keycode: number, state: number) => {
					Canvas._onWindowKey(created, 'up', keyval, state);
				}),
				connect(root, 'notify::is-active', ['pointer'], () => {
					if (!gtk_window_is_active(root)) {
						created.held.clear();
						for (const canvas of created.canvases) {
							canvas._pointerLockLost();
						}
					}
				}),
			);
			hook = created;
			windowHooks.set(id, hook);
		}
		hook.canvases.add(this);
		this._windowHook = hook;
	}

	private _detachWindow() {
		this._pointerLockLost();
		const hook = this._windowHook;
		if (!hook) {
			return;
		}
		this._windowHook = null;
		hook.canvases.delete(this);
		if (hook.canvases.size === 0) {
			windowHooks.delete(addressOf(hook.root));
			for (const disconnect of hook.disconnects) {
				disconnect();
			}
			try {
				gtk_widget_remove_controller(hook.root, hook.controller);
			} catch (e) {}
		}
	}

	private static _onWindowKey(hook: WindowHook, phase: 'down' | 'up', keyval: number, state: number): boolean {
		const { key, code } = keyInfo(keyval);
		const down = phase === 'down';
		const repeat = down && hook.held.has(keyval);
		if (down) {
			hook.held.add(keyval);
		} else {
			hook.held.delete(keyval);
		}
		if (down && key === 'Escape') {
			for (const canvas of hook.canvases) {
				if (canvas._pointerLocked) {
					// Esc releases the lock and, as in browsers, is not delivered.
					canvas._pointerLockLost();
					return true;
				}
			}
		}
		const focus = gtk_root_get_focus(hook.root);
		let canvas: Canvas | null = null;
		let focused = false;
		if (addressOf(focus)) {
			for (const candidate of hook.canvases) {
				focused = addressOf(focus) === addressOf(candidate._widget);
				if (focused || gtk_widget_is_ancestor(focus, candidate._widget)) {
					canvas = candidate;
					break;
				}
			}
		}
		const target = canvas && !canvas._ignoreTouchEvents ? canvas : null;
		// The state is from before this event: a modifier's own press or release flips it.
		const data = {
			phase,
			key,
			code,
			repeat,
			shiftKey: key === 'Shift' ? down : !!(state & GDK_SHIFT_MASK),
			ctrlKey: key === 'Control' ? down : !!(state & GDK_CONTROL_MASK),
			altKey: key === 'Alt' ? down : !!(state & GDK_ALT_MASK),
			// (not GDK_META_MASK: many X11 keymaps put Meta on Mod1 with Alt)
			metaKey: key === 'Meta' ? down : !!(state & GDK_SUPER_MASK),
		};
		const prevented = dispatchWindowKey(data, target);
		// A focused canvas keeps its keys from GTK's key navigation.
		return prevented || (focused && !!target);
	}

	protected _acquirePointerLock(options?: { unadjustedMovement?: boolean }): boolean {
		const widget = this._widget;
		const root = widget && this.isLoaded ? gtk_widget_get_root(widget) : null;
		const rect = addressOf(root) && gtk_window_is_active(root) ? this._rectOf(widget) : null;
		if (!rect || rect.width <= 0 || rect.height <= 0) {
			return false;
		}
		const at = this._pointerAt ?? [rect.width / 2, rect.height / 2];
		let wayland: WaylandLock | null = null;
		let x11: X11Lock | null = null;
		// 1 locking (Wayland), 0 not Wayland, -1 the compositor can't lock.
		const native = typeof this._canvas.lockPointer === 'function' ? this._canvas.lockPointer() : 0;
		if (native === 1) {
			wayland = { unadjusted: !!options?.unadjustedMovement, held: false, remainder: [0, 0] };
		} else if (native === 0) {
			const ref = new WeakRef(this);
			x11 = x11Lock(widget, [rect.x + rect.width / 2, rect.y + rect.height / 2], (x: number, y: number) => ref.deref()?._x11Motion(x, y));
		}
		if (!wayland && !x11) {
			return false;
		}
		gtk_widget_set_cursor_from_name(widget, 'none');
		const rootCursor = !addressOf(gtk_widget_get_cursor(root));
		if (rootCursor) {
			gtk_widget_set_cursor_from_name(root, 'none');
		}
		this._lock = { root, rootCursor, x11, wayland, at: [at[0], at[1]] };
		return true;
	}

	protected _releasePointerLock() {
		const lock = this._lock;
		if (!lock) {
			return;
		}
		this._lock = null;
		try {
			if (lock.wayland) {
				this._canvas.unlockPointer();
			}
			const x11 = lock.x11;
			if (x11) {
				x11.disconnect();
				gtk_widget_remove_controller(lock.root, x11.motion);
				// Back where it was locked, unless the window lost focus to another.
				if (x11.restore && gtk_window_is_active(lock.root)) {
					warp(x11, x11.restore);
				}
				XUngrabPointer(x11.xdisplay, 0);
				XFlush(x11.xdisplay);
			}
			if (lock.rootCursor) {
				gtk_widget_set_cursor(lock.root, null);
			}
		} catch (e) {}
		this._applyCursor();
	}

	private _lockedMove(dx: number, dy: number) {
		const [ax, ay] = this._lock.at;
		this._handleEvents({ event: 'move', pointers: [{ ptrId: 0, x: ax, y: ay, isPrimary: true, pointerType: 'mouse', button: -1, buttons: this._buttons, movementX: dx, movementY: dy }] });
	}

	/** X11: a locked mouse moved. Its travel is read from the device relative to the canvas center, and it is warped back there. */
	private _x11Motion(x: number, y: number) {
		const x11 = this._lock?.x11;
		if (!x11 || this._ignoreTouchEvents) {
			return;
		}
		const px = new interop.Reference(interop.types.double, 0);
		const py = new interop.Reference(interop.types.double, 0);
		if (gdk_surface_get_device_position(x11.surface, x11.device, px, py, null)) {
			x = px.value - x11.offset[0];
			y = py.value - x11.offset[1];
		}
		const dx = x - x11.center[0];
		const dy = y - x11.center[1];
		if (!dx && !dy) {
			// The warp back to the center.
			return;
		}
		warp(x11, x11.center);
		this._lockedMove(dx, dy);
	}

	/** Wayland: the compositor locked (1) or unlocked (0) the pointer. The lock is one-shot: once unlocked (the window lost focus), it is over. */
	private _waylandLockState(held: number) {
		const wayland = this._lock?.wayland;
		if (!wayland) {
			return;
		}
		if (held) {
			wayland.held = true;
		} else if (wayland.held) {
			this._pointerLockLost();
		}
	}

	/** Wayland: the locked pointer's travel (DIPs), accelerated and not. */
	private _waylandMotion(dx: number, dy: number, rawX: number, rawY: number) {
		const wayland = this._lock?.wayland;
		if (!wayland || this._ignoreTouchEvents) {
			return;
		}
		const remainder = wayland.remainder;
		const x = (wayland.unadjusted ? rawX : dx) + remainder[0];
		const y = (wayland.unadjusted ? rawY : dy) + remainder[1];
		const mx = Math.trunc(x);
		const my = Math.trunc(y);
		remainder[0] = x - mx;
		remainder[1] = y - my;
		if (mx || my) {
			this._lockedMove(mx, my);
		}
	}

	// As in browsers, reading a size lays out first: `loaded` fires before GTK's first layout
	// pass, and libraries size their buffers from these.
	get clientWidth() {
		this._layoutNow();
		return super.clientWidth;
	}

	get clientHeight() {
		this._layoutNow();
		return super.clientHeight;
	}

	private _layoutNow() {
		if (this.getMeasuredWidth() > 0 || !this.isLoaded) {
			return;
		}
		// The topmost view below the root: a page still being navigated to is not laid out with it.
		let top: any = this;
		while (top.parent?.parent) {
			top = top.parent;
		}
		const widget = top.parent?.nativeViewProtected ?? top.nativeViewProtected;
		let width = widget ? gtk_widget_get_width(widget) : 0;
		let height = widget ? gtk_widget_get_height(widget) : 0;
		const window = widget && !width ? gtk_widget_get_root(widget) : null;
		if (window) {
			// Not shown yet: the size it is about to get.
			const w = new interop.Reference(interop.types.int32, 0);
			const h = new interop.Reference(interop.types.int32, 0);
			gtk_window_get_default_size(window, w, h);
			[width, height] = [w.value, h.value];
		}
		if (width > 0 && height > 0) {
			top.measure(Utils.layout.makeMeasureSpec(width, Utils.layout.EXACTLY), Utils.layout.makeMeasureSpec(height, Utils.layout.EXACTLY));
			top.layout(0, 0, width, height);
		}
	}

	onLayout(left: number, top: number, right: number, bottom: number) {
		super.onLayout(left, top, right, bottom);
		this._attachWindow();
		// Layout units on Linux are DIPs.
		this._onViewSize(right - left, bottom - top);
	}

	_syncScale() {
		const widget = this._widget;
		if (widget) {
			const scale = gtk_widget_get_scale_factor(widget) || 1;
			this._onViewScale(scale, scale);
		}
	}

	_rectOf(nativeView: any) {
		try {
			const root = nativeView && gtk_widget_get_root(nativeView);
			if (!root) {
				return null;
			}
			const x = new interop.Reference(interop.types.double, 0);
			const y = new interop.Reference(interop.types.double, 0);
			if (!gtk_widget_translate_coordinates(nativeView, root, 0, 0, x, y)) {
				return null;
			}
			return { x: x.value, y: y.value, width: gtk_widget_get_width(nativeView), height: gtk_widget_get_height(nativeView) };
		} catch (e) {
			return null;
		}
	}
}
