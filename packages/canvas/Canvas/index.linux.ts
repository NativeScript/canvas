/**
 * The canvas view on NativeScript Linux: a GTK 4 widget made by the native host
 * (`CanvasModule.NSCCanvas`), which shows its context's frames as GL textures. Everything but the
 * widget and its events is shared with the other Node-API hosts (./napi-host).
 */
import { isUserInteractionEnabledProperty, Utils } from '@nativescript/core';
import { ignoreTouchEventsProperty } from './common';
import { NapiCanvas } from './napi-host';

export * from './common';
export { createSVGMatrix } from './napi-host';

declare const interop: any;
declare const gtk_gesture_drag_new: any, gtk_event_controller_scroll_new: any, gtk_event_controller_key_new: any;
declare const gtk_widget_add_controller: any, gtk_widget_remove_controller: any, gtk_widget_set_focusable: any, gtk_widget_grab_focus: any;
declare const gtk_widget_get_scale_factor: any, gtk_widget_get_root: any, gtk_widget_translate_coordinates: any, gtk_widget_get_width: any, gtk_widget_get_height: any;
declare const gtk_event_controller_set_propagation_phase: any, gtk_gesture_single_set_button: any, gtk_window_get_default_size: any;
declare const g_signal_connect_data: any, g_signal_handler_disconnect: any, gdk_keyval_name: any, gdk_keyval_to_unicode: any;
declare const gtk_event_controller_motion_new: any, gtk_event_controller_get_current_event_device: any, gtk_gesture_single_get_current_button: any, gdk_device_get_source: any;

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
};

/** Web `key` / `code` for a GDK keyval. */
function keyInfo(keyval: number): { key: string; code: string } {
	const raw = gdk_keyval_name(keyval);
	const name: string = typeof raw === 'string' ? raw : raw ? interop.stringFromCString(raw) : '';
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

export class Canvas extends NapiCanvas {
	private _widget: any;
	private _ignoreTouchEvents = false;
	private _controllers: any[] = [];
	private _disconnects: (() => void)[] = [];

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
			// A press on a hosted child (content="drawable") leaves focus to it.
			if (!canvas._canvas.picksChild(x, y)) {
				gtk_widget_grab_focus(widget);
			}
			canvas._handleEvents({ event: 'down', ptrId: 0, x, y, isPrimary: true, pointerType, button, buttons });
		});
		on(drag, 'drag-update', ['double', 'double'], (dx: number, dy: number) => {
			if (!down) {
				return;
			}
			last = [start[0] + dx, start[1] + dy];
			ref.deref()?._handleEvents({ event: 'move', pointers: [{ ptrId: 0, x: last[0], y: last[1], isPrimary: true, pointerType, button: -1, buttons }] });
		});
		on(drag, 'drag-end', ['double', 'double'], (dx: number, dy: number) => {
			if (!down) {
				return;
			}
			down = false;
			const [x, y] = (last = [start[0] + dx, start[1] + dy]);
			const canvas = ref.deref();
			canvas?._handleEvents({ event: 'up', ptrId: 0, x, y, isPrimary: true, pointerType, button, buttons: 0 });
			// The drag's grab held back the motion controller's leave.
			if (pointerType !== 'touch' && !(x >= 0 && y >= 0 && x < gtk_widget_get_width(widget) && y < gtk_widget_get_height(widget))) {
				canvas?._handleEvents({ event: 'leave', ptrId: 0, x, y, isPrimary: true, pointerType });
			}
		});
		on(drag, 'cancel', ['pointer'], () => {
			if (down) {
				down = false;
				ref.deref()?._handleEvents({ event: 'cancel', ptrId: 0, x: last[0], y: last[1], isPrimary: true, pointerType });
			}
		});

		// Hover: a mouse or pen with nothing pressed moves with buttons 0, as on the web. While pressed,
		// the drag reports the moves.
		const motion = add(gtk_event_controller_motion_new());
		const hover = (event: 'enter' | 'move', x: number, y: number) => {
			const canvas = ref.deref();
			if (!canvas || canvas._ignoreTouchEvents || down) {
				return;
			}
			const type = pointerTypeOf(motion);
			if (type === 'touch') {
				return;
			}
			pointerType = type;
			last = [x, y];
			const pointer = { ptrId: 0, x, y, isPrimary: true, pointerType };
			canvas._handleEvents(event === 'enter' ? { event, ...pointer } : { event, pointers: [{ ...pointer, button: -1, buttons: 0 }] });
		};
		on(motion, 'enter', ['double', 'double'], (x: number, y: number) => hover('enter', x, y));
		on(motion, 'motion', ['double', 'double'], (x: number, y: number) => hover('move', x, y));
		on(motion, 'leave', [], () => {
			if (!down && pointerType !== 'touch') {
				ref.deref()?._handleEvents({ event: 'leave', ptrId: 0, x: last[0], y: last[1], isPrimary: true, pointerType });
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

		const keys = add(gtk_event_controller_key_new());
		const key = (phase: 'down' | 'up', keyval: number) => {
			const canvas = ref.deref();
			if (!canvas || canvas._ignoreTouchEvents) {
				return false;
			}
			canvas._handleEvents({ event: 'key', phase, ...keyInfo(keyval), repeat: false });
			return true;
		};
		on(keys, 'key-pressed', ['uint', 'uint', 'int'], (keyval: number) => key('down', keyval), true);
		on(keys, 'key-released', ['uint', 'uint', 'int'], (keyval: number) => key('up', keyval));
	}

	disposeNativeView(): void {
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
