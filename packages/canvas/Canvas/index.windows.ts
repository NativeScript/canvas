/**
 * The canvas view on NativeScript Windows: a WinUI 3 `SwapChainPanel` whose composition swapchain
 * the native host (`CanvasModule.NSCCanvas`) presents into. Everything but the panel and its
 * events is shared with the other Node-API hosts (./napi-host).
 */
import { isUserInteractionEnabledProperty } from '@nativescript/core';
import { dispatchWindowKey, ignoreTouchEventsProperty } from './common';
import { NapiCanvas, cursorKeyword, cursorProperty } from './napi-host';

export * from './common';
export { createSVGMatrix } from './napi-host';

declare const Microsoft: any, Windows: any, NSWinRT: any;

type PointerEvent = 'Pressed' | 'Moved' | 'Released' | 'Canceled' | 'CaptureLost' | 'WheelChanged' | 'Entered' | 'Exited';

const POINTER_EVENTS: PointerEvent[] = ['Pressed', 'Moved', 'Released', 'Canceled', 'CaptureLost', 'WheelChanged', 'Entered', 'Exited'];

/** Web `pointerType` for a `Microsoft.UI.Input.PointerDeviceType`: Touch 0, Pen 1, Mouse 2, Touchpad 3 (a mouse on the web). */
function pointerTypeOf(deviceType: number): 'mouse' | 'pen' | 'touch' {
	return deviceType === 0 ? 'touch' : deviceType === 1 ? 'pen' : 'mouse';
}

/** Web `buttons` bitmask (1 left or contact, 2 right or barrel, 4 middle, 8 back, 16 forward) of a pointer point. */
function buttonsOf(point: any): number {
	const p = point.Properties;
	if (!p) {
		return 0;
	}
	return (p.IsLeftButtonPressed ? 1 : 0) | (p.IsRightButtonPressed || p.IsBarrelButtonPressed ? 2 : 0) | (p.IsMiddleButtonPressed ? 4 : 0) | (p.IsXButton1Pressed ? 8 : 0) | (p.IsXButton2Pressed ? 16 : 0);
}

/**
 * Web `button` that changed (0 left, 1 middle, 2 right, 3 back, 4 forward), or -1, from the point's
 * `PointerUpdateKind` (1 to 10: Left, Right, Middle, XButton1, XButton2, each Pressed then Released).
 */
function changedButtonOf(point: any): number {
	const kind: number = point.Properties?.PointerUpdateKind ?? 0;
	return kind >= 1 && kind <= 10 ? [0, 2, 1, 3, 4][(kind - 1) >> 1] : -1;
}

/** Web `key` / `code` for a `Windows.System.VirtualKey`. */
function keyInfo(virtualKey: number): { key: string; code: string } {
	if (virtualKey >= 0x41 && virtualKey <= 0x5a) {
		const letter = String.fromCharCode(virtualKey);
		return { key: letter.toLowerCase(), code: `Key${letter}` };
	}
	if (virtualKey >= 0x30 && virtualKey <= 0x39) {
		const digit = String.fromCharCode(virtualKey);
		return { key: digit, code: `Digit${digit}` };
	}
	if (virtualKey >= 0x70 && virtualKey <= 0x87) {
		const name = `F${virtualKey - 0x6f}`;
		return { key: name, code: name };
	}
	const named: Record<number, [string, string]> = {
		0x08: ['Backspace', 'Backspace'],
		0x09: ['Tab', 'Tab'],
		0x0d: ['Enter', 'Enter'],
		0x10: ['Shift', 'ShiftLeft'],
		0x11: ['Control', 'ControlLeft'],
		0x12: ['Alt', 'AltLeft'],
		0x1b: ['Escape', 'Escape'],
		0x20: [' ', 'Space'],
		0x21: ['PageUp', 'PageUp'],
		0x22: ['PageDown', 'PageDown'],
		0x23: ['End', 'End'],
		0x24: ['Home', 'Home'],
		0x25: ['ArrowLeft', 'ArrowLeft'],
		0x26: ['ArrowUp', 'ArrowUp'],
		0x27: ['ArrowRight', 'ArrowRight'],
		0x28: ['ArrowDown', 'ArrowDown'],
		0x2e: ['Delete', 'Delete'],
	};
	const [key, code] = named[virtualKey] ?? ['Unidentified', 'Unidentified'];
	return { key, code };
}

/** Web `deltaY` pixels per wheel notch (`WHEEL_DELTA`, 120), as Chromium reports on Windows. */
const WHEEL_PIXELS_PER_NOTCH = 100;

/** Shift, Control, Alt (Menu) and the Windows keys, held as of the current key event. */
function modifierKeys() {
	const held = (virtualKey: number) => {
		try {
			// CoreVirtualKeyStates: 1 Down.
			return (Number(Microsoft.UI.Input.InputKeyboardSource.GetKeyStateForCurrentThread(virtualKey)) & 1) === 1;
		} catch (e) {
			return false;
		}
	};
	return { shiftKey: held(0x10), ctrlKey: held(0x11), altKey: held(0x12), metaKey: held(0x5b) || held(0x5c) };
}

/** `Microsoft.UI.Input.InputSystemCursorShape` for a CSS cursor keyword. */
const CURSOR_SHAPES: Record<string, string> = {
	pointer: 'Hand',
	grab: 'Hand',
	grabbing: 'Hand',
	crosshair: 'Cross',
	text: 'IBeam',
	'vertical-text': 'IBeam',
	move: 'SizeAll',
	'all-scroll': 'SizeAll',
	wait: 'Wait',
	progress: 'AppStarting',
	help: 'Help',
	'not-allowed': 'UniversalNo',
	'no-drop': 'UniversalNo',
	'ew-resize': 'SizeWestEast',
	'col-resize': 'SizeWestEast',
	'e-resize': 'SizeWestEast',
	'w-resize': 'SizeWestEast',
	'ns-resize': 'SizeNorthSouth',
	'row-resize': 'SizeNorthSouth',
	'n-resize': 'SizeNorthSouth',
	's-resize': 'SizeNorthSouth',
	'nwse-resize': 'SizeNorthwestSoutheast',
	'nw-resize': 'SizeNorthwestSoutheast',
	'se-resize': 'SizeNorthwestSoutheast',
	'nesw-resize': 'SizeNortheastSouthwest',
	'ne-resize': 'SizeNortheastSouthwest',
	'sw-resize': 'SizeNortheastSouthwest',
};

let user32: any;

/** The user32 calls behind pointer lock and a hidden cursor, through the runtime's Win32 FFI. */
function win32() {
	if (user32 === undefined) {
		try {
			user32 = NSWinRT.win32.define('user32.dll', {
				ClipCursor: { args: ['pointer'], ret: 'bool' },
				ClientToScreen: { args: ['pointer', 'pointer'], ret: 'bool' },
				GetCursorPos: { args: ['pointer'], ret: 'bool' },
				SetCursorPos: { args: ['i32', 'i32'], ret: 'bool' },
				ShowCursor: { args: ['bool'], ret: 'i32' },
				GetForegroundWindow: { args: [], ret: 'pointer' },
			});
		} catch (e) {
			user32 = null;
		}
	}
	return user32;
}

/** Canvases hiding the system cursor. ShowCursor keeps a count: hide it once for all of them. */
const hidingCursor = new Set<Canvas>();

function hideCursor(canvas: Canvas, hide: boolean) {
	const hidden = hidingCursor.size > 0;
	if (hide) {
		hidingCursor.add(canvas);
	} else {
		hidingCursor.delete(canvas);
	}
	if (hidden !== hidingCursor.size > 0) {
		try {
			win32()?.ShowCursor(hidden);
		} catch (e) {}
	}
}

/** The canvases in one window, which share a key hook on its XAML content. */
interface WindowHook {
	key: string;
	content: any;
	canvases: Set<Canvas>;
	delegates: any[];
}

const windowHooks = new Map<string, WindowHook>();

export class Canvas extends NapiCanvas {
	/** Default for `getContext('2d', { threaded })`: rasterize on a shared render thread. */
	static threaded2D = true;
	/** Default for `getContext('webgl' | 'webgl2', { threaded })`: run the context on the shared WebGL thread. */
	static threadedWebGL = true;

	private _panel: any;
	private _ignoreTouchEvents = false;
	/** Pointers down on the panel (captured, so moves outside it still arrive). */
	private _down = new Set<number>();
	/** Mouse and pen pointers down: `cursor: none` keeps the cursor hidden while one is held. */
	private _mouseDown = new Set<number>();
	/** Delegates are held so they live as long as the subscriptions. */
	private _pointerDelegates: Map<PointerEvent, any> | null = null;
	private _scaleDelegate: any;
	private _sizeDelegate: any;
	private _panelKey = '';
	private _windowHook: WindowHook | null = null;
	/** CSS cursor keyword, null for the default. */
	private _cursor: string | null = null;
	private _mouseOver = false;
	private _lastMouse: { x: number; y: number } | null = null;
	/** While locked: the window, the clip rect in screen pixels and where moves are reported. */
	private _lockWindow: any = null;
	private _lockRect: Int32Array | null = null;
	private _lockAt = { x: 0, y: 0 };
	private _lockTimer: any = null;
	/** Transparent canvases: a XAML SurfaceImageSource in an Image instead of the panel's swapchain. */
	private _image: any;
	private _imageSize = '';
	private _imageFlipped = false;

	constructor(nativeInstance?: any) {
		super();
		if (nativeInstance) {
			// An existing host (e.g. from a worker).
			this._attachHost(nativeInstance);
			return;
		}
		// (A SwapChainPanel rejects Background: its content is the swapchain.)
		const panel = new Microsoft.UI.Xaml.Controls.SwapChainPanel();
		this._panel = panel;
		this._panelKey = String(NSWinRT.interop.pointerKey(panel) ?? '');
		this._attachHost(new global.CanvasModule.NSCCanvas(NSWinRT.interop.pointerKey(panel)));
	}

	// @ts-ignore
	get windows() {
		return this._panel;
	}

	createNativeView() {
		return this._panel;
	}

	initNativeView(): void {
		super.initNativeView();
		const panel = this._panel;
		if (!panel) {
			return;
		}
		const ref = new WeakRef(this);
		this._scaleDelegate = NSWinRT.asDelegate('Windows.Foundation.TypedEventHandler`2<Microsoft.UI.Xaml.Controls.SwapChainPanel,Object>', () => ref.deref()?._syncCompositionScale());
		panel.CompositionScaleChanged = this._scaleDelegate;
		this._syncCompositionScale();
		// Keyboard: the panel takes focus when pressed; keys come through its window's hook.
		panel.IsTabStop = true;

		// XAML lays the panel out (NativeScript's measure/layout pass does not run for it). An event
		// takes one delegate: for % sizes core watches SizeChanged itself and calls _onSizeChanged.
		if (!(this as any)._sizeWatchWired) {
			this._sizeDelegate = NSWinRT.asDelegate('Microsoft.UI.Xaml.SizeChangedEventHandler', () => ref.deref()?._syncViewSize());
			panel.SizeChanged = this._sizeDelegate;
		}

		this._pointerDelegates = new Map();
		for (const name of POINTER_EVENTS) {
			const delegate = NSWinRT.asDelegate('Microsoft.UI.Xaml.Input.PointerEventHandler', (_sender: any, args: any) => ref.deref()?._onPointer(name, args));
			this._pointerDelegates.set(name, delegate);
			// Assigning subscribes. (AddHandler would also see handled events, but it takes the
			// handler as IInspectable, which runtime delegates are not.)
			panel[`Pointer${name}`] = delegate;
		}
	}

	disposeNativeView(): void {
		const panel = this._panel;
		if (panel) {
			try {
				panel.CompositionScaleChanged = null;
				panel.SizeChanged = null;
				this._pointerDelegates?.forEach((_, name) => (panel[`Pointer${name}`] = null));
			} catch (e) {}
		}
		this._detachWindow();
		this._scaleDelegate = undefined;
		this._sizeDelegate = undefined;
		this._image = undefined;
		this._imageSize = '';
		this._pointerDelegates = null;
		this._down.clear();
		this._mouseDown.clear();
		this._panel = undefined;
		super.disposeNativeView();
	}

	[ignoreTouchEventsProperty.setNative](value: boolean) {
		this._ignoreTouchEvents = value;
	}

	[isUserInteractionEnabledProperty.setNative](value: boolean) {
		this._ignoreTouchEvents = !value;
		if (this._panel) {
			this._panel.IsHitTestVisible = value;
		}
	}

	[cursorProperty.setNative](value: string) {
		this._cursor = cursorKeyword(value);
		const panel = this._panel;
		if (panel) {
			const shape = this._cursor ? CURSOR_SHAPES[this._cursor] : undefined;
			try {
				// 'none' keeps the default here; ShowCursor hides it over the panel.
				panel.ProtectedCursor = shape ? Microsoft.UI.Input.InputSystemCursor.Create(Microsoft.UI.Input.InputSystemCursorShape[shape]) : null;
			} catch (e) {}
		}
		this._syncCursor();
	}

	onLoaded(): void {
		super.onLoaded();
		this._attachWindow();
	}

	onUnloaded(): void {
		this._detachWindow();
		super.onUnloaded();
	}

	/** Keys for the whole window: PreviewKeyDown/Up on its XAML content, where they tunnel first whatever has focus. */
	private _attachWindow() {
		const panel = this._panel;
		if (this._windowHook || !panel || !this.isLoaded) {
			return;
		}
		let content: any = null;
		try {
			content = panel.XamlRoot?.Content ?? null;
		} catch (e) {}
		const key = content ? String(NSWinRT.interop.pointerKey(content) ?? '') : '';
		if (!key) {
			return;
		}
		let hook = windowHooks.get(key);
		if (!hook) {
			const created: WindowHook = { key, content, canvases: new Set(), delegates: [] };
			for (const [name, phase] of [
				['PreviewKeyDown', 'down'],
				['PreviewKeyUp', 'up'],
			] as const) {
				const delegate = NSWinRT.asDelegate('Microsoft.UI.Xaml.Input.KeyEventHandler', (_sender: any, args: any) => Canvas._onWindowKey(created, phase, args));
				created.delegates.push(delegate);
				content[name] = delegate;
			}
			hook = created;
			windowHooks.set(key, hook);
		}
		hook.canvases.add(this);
		this._windowHook = hook;
	}

	private _detachWindow() {
		this._pointerLockLost();
		this._mouseOver = false;
		hideCursor(this, false);
		const hook = this._windowHook;
		if (!hook) {
			return;
		}
		this._windowHook = null;
		hook.canvases.delete(this);
		if (hook.canvases.size === 0) {
			windowHooks.delete(hook.key);
			try {
				hook.content.PreviewKeyDown = null;
				hook.content.PreviewKeyUp = null;
			} catch (e) {}
		}
	}

	/** The canvas whose panel has focus or holds `element` (the key's source), if any. */
	private static _canvasOf(hook: WindowHook, element: any): Canvas | null {
		const panels = new Map<string, Canvas>();
		for (const canvas of hook.canvases) {
			// FocusState: 0 Unfocused.
			if (Number(canvas._panel?.FocusState) > 0) {
				return canvas;
			}
			panels.set(canvas._panelKey, canvas);
		}
		let node = element;
		for (let depth = 0; node && depth < 64; depth++) {
			const canvas = panels.get(String(NSWinRT.interop.pointerKey(node) ?? ''));
			if (canvas) {
				return canvas;
			}
			try {
				node = Microsoft.UI.Xaml.Media.VisualTreeHelper.GetParent(node);
			} catch (e) {
				return null;
			}
		}
		return null;
	}

	private static _onWindowKey(hook: WindowHook, phase: 'down' | 'up', args: any) {
		const { key, code } = keyInfo(args.Key);
		if (phase === 'down' && key === 'Escape') {
			for (const canvas of hook.canvases) {
				if (canvas._pointerLocked) {
					// Esc releases the lock and, as in browsers, is not delivered.
					canvas._pointerLockLost();
					args.Handled = true;
					return;
				}
			}
		}
		const canvas = Canvas._canvasOf(hook, args.OriginalSource);
		const data = { phase, key, code, repeat: phase === 'down' && !!args.KeyStatus?.WasKeyDown, ...modifierKeys() };
		if (dispatchWindowKey(data, canvas && !canvas._ignoreTouchEvents ? canvas : null)) {
			args.Handled = true;
		}
	}

	private _syncCursor() {
		hideCursor(this, !!this._lockRect || (this._cursor === 'none' && (this._mouseOver || this._mouseDown.size > 0)));
	}

	protected _acquirePointerLock(): boolean {
		const panel = this._panel;
		const api = win32();
		if (!panel?.XamlRoot || !api || !this.isLoaded) {
			return false;
		}
		const foreground = api.GetForegroundWindow();
		let appWindow = NaN;
		try {
			// A top-level window's WindowId is its HWND.
			appWindow = Number(panel.XamlRoot.ContentIslandEnvironment?.AppWindowId?.Value);
		} catch (e) {}
		// Only the foreground window can lock the pointer.
		if (!foreground || (appWindow > 0 && appWindow !== Number(foreground))) {
			return false;
		}
		this._lockWindow = foreground;
		if (!this._clipCursor()) {
			this._lockWindow = null;
			return false;
		}
		const width = panel.ActualWidth || 0;
		const height = panel.ActualHeight || 0;
		this._lockAt = this._mouseOver && this._lastMouse ? { ...this._lastMouse } : { x: width / 2, y: height / 2 };
		const rect = this._lockRect;
		api.SetCursorPos((rect[0] + rect[2]) >> 1, (rect[1] + rect[3]) >> 1);
		this._syncCursor();
		try {
			panel.Focus(Microsoft.UI.Xaml.FocusState.Programmatic);
		} catch (e) {}
		const ref = new WeakRef(this);
		this._lockTimer = setInterval(() => ref.deref()?._checkPointerLock(), 100);
		return true;
	}

	protected _releasePointerLock() {
		clearInterval(this._lockTimer);
		this._lockTimer = null;
		if (this._lockRect) {
			try {
				win32()?.ClipCursor(null);
			} catch (e) {}
		}
		this._lockRect = null;
		this._lockWindow = null;
		this._syncCursor();
	}

	/** Holds the cursor in the panel's rect on screen; false if the panel has none. */
	private _clipCursor(): boolean {
		const api = win32();
		const panel = this._panel;
		const rect = this._rectOf(panel);
		if (!api || !rect || !(rect.width > 0 && rect.height > 0)) {
			return false;
		}
		const scale = panel.XamlRoot?.RasterizationScale || 1;
		const origin = new Int32Array(2);
		if (!api.ClientToScreen(this._lockWindow, origin)) {
			return false;
		}
		const [x, y] = origin;
		const clip = Int32Array.of(x + Math.round(rect.x * scale), y + Math.round(rect.y * scale), x + Math.round((rect.x + rect.width) * scale), y + Math.round((rect.y + rect.height) * scale));
		// Set again each time: the system drops the clip on some window changes.
		api.ClipCursor(clip);
		this._lockRect = clip;
		return true;
	}

	/** Polled while locked: the lock ends when the window is deactivated, and the clip follows the panel as the window moves or resizes. */
	private _checkPointerLock() {
		const api = win32();
		if (!this._lockRect || !api) {
			return;
		}
		if (Number(api.GetForegroundWindow()) !== Number(this._lockWindow) || !this._clipCursor()) {
			this._pointerLockLost();
		}
	}

	/** A locked mouse moved: report the travel from the center, and put it back there. */
	private _lockedMove(ptrId: number, isPrimary: boolean, point: any) {
		const api = win32();
		const rect = this._lockRect;
		if (!api || !rect) {
			return;
		}
		const cursor = new Int32Array(2);
		api.GetCursorPos(cursor);
		const centerX = (rect[0] + rect[2]) >> 1;
		const centerY = (rect[1] + rect[3]) >> 1;
		const dx = cursor[0] - centerX;
		const dy = cursor[1] - centerY;
		if (dx || dy) {
			api.SetCursorPos(centerX, centerY);
		}
		const button = changedButtonOf(point);
		if (!dx && !dy && button < 0) {
			// The move back to the center.
			return;
		}
		const scale = this._panel?.XamlRoot?.RasterizationScale || 1;
		const { x, y } = this._lockAt;
		this._handleEvents({ event: 'move', pointers: [{ ptrId, x, y, isPrimary, pointerType: 'mouse', button, buttons: buttonsOf(point), movementX: dx / scale, movementY: dy / scale }] });
	}

	/**
	 * A SwapChainPanel is external content in WinUI 3: nothing behind it shows through, whatever
	 * the swapchain's alpha. A transparent canvas presents into a SurfaceImageSource instead (XAML
	 * composites it like any image) shown by an Image inside the panel, which then has no
	 * swapchain and stays see-through. The Image stays hit-testable: pointer events bubble to the
	 * panel's handlers.
	 */
	protected _prepareSurface(transparent: boolean, flipped: boolean) {
		const panel = this._panel;
		if (!transparent || !panel || this._image) {
			return;
		}
		const image = new Microsoft.UI.Xaml.Controls.Image();
		image.Stretch = Microsoft.UI.Xaml.Media.Stretch.Fill;
		image.HorizontalAlignment = Microsoft.UI.Xaml.HorizontalAlignment.Left;
		image.VerticalAlignment = Microsoft.UI.Xaml.VerticalAlignment.Top;
		panel.Children.Append(image);
		this._image = image;
		this._imageFlipped = flipped;
		this._layoutSurface();
	}

	/** A SurfaceImageSource of the drawing buffer's size, placed like the swapchain would be. */
	protected _layoutSurface() {
		const image = this._image;
		const host = this._canvas;
		if (!image || !host) {
			return;
		}
		const width = host.surfaceWidth;
		const height = host.surfaceHeight;
		const size = `${width}x${height}`;
		if (size !== this._imageSize) {
			const source = new Microsoft.UI.Xaml.Media.Imaging.SurfaceImageSource(width, height, false);
			if (host.attachSurfaceImageSource(NSWinRT.interop.pointerKey(source))) {
				image.Source = source;
				this._imageSize = size;
			}
		}
		// The panel clips a swapchain to its bounds, but not its children.
		const panel = this._panel;
		if (panel) {
			const clip = new Microsoft.UI.Xaml.Media.RectangleGeometry();
			clip.Rect = { X: 0, Y: 0, Width: panel.ActualWidth || 0, Height: panel.ActualHeight || 0 };
			panel.Clip = clip;
		}
		const [scaleX, scaleY, offsetX, offsetY] = host.surfaceTransform;
		const shownWidth = width * scaleX;
		const shownHeight = height * scaleY;
		image.Width = shownWidth;
		image.Height = shownHeight;
		// A margin, not a translate: an image larger than the panel is clipped to its layout slot
		// before its render transform moves it.
		image.Margin = { Left: offsetX, Top: offsetY, Right: 0, Bottom: 0 };
		const transform = new Microsoft.UI.Xaml.Media.CompositeTransform();
		if (this._imageFlipped) {
			transform.ScaleY = -1;
			transform.CenterY = shownHeight / 2;
		}
		image.RenderTransform = transform;
	}

	/** Core's size watch (it replaces the panel's SizeChanged delegate for % sizes). */
	_onSizeChanged() {
		(Object.getPrototypeOf(Canvas.prototype) as any)._onSizeChanged?.call(this);
		this._syncViewSize();
	}

	_syncViewSize() {
		const panel = this._panel;
		if (panel) {
			// XamlRoot may not be set until the panel is laid out.
			this._attachWindow();
			// Only native state changes here: mutating the XAML tree inside SizeChanged is not safe.
			// The scale too: it is only final once the panel is in the tree.
			this._syncCompositionScale();
			this._onViewSize(panel.ActualWidth || 0, panel.ActualHeight || 0);
		}
	}

	_syncCompositionScale() {
		const panel = this._panel;
		if (panel) {
			this._onViewScale(panel.CompositionScaleX || 1, panel.CompositionScaleY || 1);
		}
	}

	_rectOf(nativeView: any) {
		try {
			if (!nativeView?.XamlRoot) {
				return null;
			}
			const origin = nativeView.TransformToVisual(null).TransformPoint(new Windows.Foundation.Point(0, 0));
			return { x: origin.X, y: origin.Y, width: nativeView.ActualWidth, height: nativeView.ActualHeight };
		} catch (e) {
			return null;
		}
	}

	private _contains(x: number, y: number) {
		const panel = this._panel;
		return !!panel && x >= 0 && y >= 0 && x < (panel.ActualWidth || 0) && y < (panel.ActualHeight || 0);
	}

	private _onPointer(name: PointerEvent, args: any) {
		const panel = this._panel;
		if (!panel || this._ignoreTouchEvents) {
			return;
		}
		const pointer = args.Pointer;
		const ptrId: number = pointer.PointerId;
		const point = args.GetCurrentPoint(panel);
		let { X: x, Y: y } = point.Position;
		const isPrimary = !!point.Properties?.IsPrimary;
		const pointerType = pointerTypeOf(pointer.PointerDeviceType ?? point.PointerDeviceType);
		if (pointerType === 'mouse') {
			if (name === 'Entered' || name === 'Exited') {
				this._mouseOver = name === 'Entered';
				this._syncCursor();
			}
			if (this._lockRect) {
				// Locked: moves report travel, presses happen where the pointer was locked.
				if (name === 'Moved') {
					this._lockedMove(ptrId, isPrimary, point);
				}
				if (name === 'Moved' || name === 'Entered' || name === 'Exited') {
					return;
				}
				({ x, y } = this._lockAt);
			} else {
				this._lastMouse = { x, y };
			}
		}
		switch (name) {
			case 'Pressed':
				this._down.add(ptrId);
				if (pointerType !== 'touch') this._mouseDown.add(ptrId);
				// A press on a hosted child (content="drawable") leaves capture and focus to it.
				if (!(this._drawables as any)?.owns?.(args.OriginalSource)) {
					panel.CapturePointer(pointer);
					try {
						panel.Focus(Microsoft.UI.Xaml.FocusState.Pointer);
					} catch (e) {}
				}
				this._handleEvents({ event: 'down', ptrId, x, y, isPrimary, pointerType, button: Math.max(changedButtonOf(point), 0), buttons: buttonsOf(point) || 1 });
				this._syncCursor();
				break;
			case 'Moved':
				if (this._down.has(ptrId)) {
					// Further mouse buttons pressed or released while one is down arrive as moves.
					this._handleEvents({ event: 'move', pointers: [{ ptrId, x, y, isPrimary, pointerType, button: changedButtonOf(point), buttons: buttonsOf(point) || 1 }] });
				} else if (pointerType !== 'touch' && buttonsOf(point) === 0) {
					// A mouse or pen hovering moves with no buttons, as on the web. A touch is only
					// reported between down and up, as on iOS and Android.
					this._handleEvents({ event: 'move', pointers: [{ ptrId, x, y, isPrimary, pointerType, button: -1, buttons: 0 }] });
				}
				break;
			case 'Released':
				this._mouseDown.delete(ptrId);
				if (this._down.delete(ptrId)) {
					panel.ReleasePointerCapture(pointer);
					this._handleEvents({ event: 'up', ptrId, x, y, isPrimary, pointerType, button: Math.max(changedButtonOf(point), 0), buttons: buttonsOf(point) });
					// Exited is ignored while down: leave here if it was released outside.
					if (pointerType !== 'touch' && !this._contains(x, y)) {
						this._mouseOver = false;
						this._handleEvents({ event: 'leave', ptrId, x, y, isPrimary, pointerType });
					}
					this._syncCursor();
				}
				break;
			case 'Canceled':
			case 'CaptureLost':
				this._mouseDown.delete(ptrId);
				if (this._down.delete(ptrId)) {
					this._handleEvents({ event: 'cancel', ptrId, x, y, isPrimary, pointerType });
					this._syncCursor();
				}
				break;
			// Mouse and pen only: a touch leaves on up, and a pressed pointer on release.
			case 'Entered':
				if (pointerType !== 'touch' && !this._down.has(ptrId)) {
					this._handleEvents({ event: 'enter', ptrId, x, y, isPrimary, pointerType });
				}
				break;
			case 'Exited':
				if (pointerType !== 'touch' && !this._down.has(ptrId)) {
					this._handleEvents({ event: 'leave', ptrId, x, y, isPrimary, pointerType });
				}
				break;
			case 'WheelChanged': {
				const notches = -(point.Properties?.MouseWheelDelta ?? 0) / 120;
				const horizontal = !!point.Properties?.IsHorizontalMouseWheel;
				this._handleEvents({
					event: 'scale',
					deltaX: horizontal ? -notches * WHEEL_PIXELS_PER_NOTCH : 0,
					deltaY: horizontal ? 0 : notches * WHEEL_PIXELS_PER_NOTCH,
					deltaMode: 0,
					// No pointers: a wheel is not a move.
					pointers: [],
					isInProgress: false,
				});
				break;
			}
		}
	}
}
