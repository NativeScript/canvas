/**
 * The canvas view on NativeScript Windows: a WinUI 3 `SwapChainPanel` whose composition swapchain
 * the native host (`CanvasModule.NSCCanvas`) presents into. Everything but the panel and its
 * events is shared with the other Node-API hosts (./napi-host).
 */
import { isUserInteractionEnabledProperty } from '@nativescript/core';
import { ignoreTouchEventsProperty } from './common';
import { NapiCanvas } from './napi-host';

export * from './common';
export { createSVGMatrix } from './napi-host';

declare const Microsoft: any, Windows: any, NSWinRT: any;

type PointerEvent = 'Pressed' | 'Moved' | 'Released' | 'Canceled' | 'CaptureLost' | 'WheelChanged';

const POINTER_EVENTS: PointerEvent[] = ['Pressed', 'Moved', 'Released', 'Canceled', 'CaptureLost', 'WheelChanged'];

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

export class Canvas extends NapiCanvas {
	/** Default for `getContext('2d', { threaded })`: rasterize on a shared render thread. */
	static threaded2D = true;

	private _panel: any;
	private _ignoreTouchEvents = false;
	/** Pointers down on the panel (captured, so moves outside it still arrive). */
	private _down = new Set<number>();
	/** Delegates are held so they live as long as the subscriptions. */
	private _pointerDelegates: Map<PointerEvent, any> | null = null;
	private _scaleDelegate: any;
	private _sizeDelegate: any;
	private _keyDelegates: any[] = [];
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
		// Keyboard: the panel takes focus when pressed and forwards keys like the other hosts do.
		panel.IsTabStop = true;
		for (const [name, phase] of [
			['KeyDown', 'down'],
			['KeyUp', 'up'],
		] as const) {
			const delegate = NSWinRT.asDelegate('Microsoft.UI.Xaml.Input.KeyEventHandler', (_sender: any, args: any) => ref.deref()?._onKey(phase, args));
			this._keyDelegates.push(delegate);
			panel[name] = delegate;
		}

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
				panel.KeyDown = null;
				panel.KeyUp = null;
				this._pointerDelegates?.forEach((_, name) => (panel[`Pointer${name}`] = null));
			} catch (e) {}
		}
		this._scaleDelegate = undefined;
		this._sizeDelegate = undefined;
		this._keyDelegates = [];
		this._image = undefined;
		this._imageSize = '';
		this._pointerDelegates = null;
		this._down.clear();
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
		const [scaleX, scaleY, offsetX, offsetY] = host.surfaceTransform;
		const shownWidth = width * scaleX;
		const shownHeight = height * scaleY;
		image.Width = shownWidth;
		image.Height = shownHeight;
		const transform = new Microsoft.UI.Xaml.Media.CompositeTransform();
		transform.TranslateX = offsetX;
		transform.TranslateY = offsetY;
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

	private _onKey(phase: 'down' | 'up', args: any) {
		if (this._ignoreTouchEvents) {
			return;
		}
		const { key, code } = keyInfo(args.Key);
		this._handleEvents({ event: 'key', phase, key, code, repeat: phase === 'down' && !!args.KeyStatus?.WasKeyDown });
	}

	private _onPointer(name: PointerEvent, args: any) {
		const panel = this._panel;
		if (!panel || this._ignoreTouchEvents) {
			return;
		}
		const pointer = args.Pointer;
		const ptrId: number = pointer.PointerId;
		const point = args.GetCurrentPoint(panel);
		const { X: x, Y: y } = point.Position;
		const isPrimary = !!point.Properties?.IsPrimary;
		switch (name) {
			case 'Pressed':
				this._down.add(ptrId);
				panel.CapturePointer(pointer);
				try {
					panel.Focus(Microsoft.UI.Xaml.FocusState.Pointer);
				} catch (e) {}
				this._handleEvents({ event: 'down', ptrId, x, y, isPrimary });
				break;
			case 'Moved':
				this._handleEvents({ event: 'move', pointers: [{ ptrId, x, y, isPrimary }] });
				break;
			case 'Released':
				if (this._down.delete(ptrId)) {
					panel.ReleasePointerCapture(pointer);
					this._handleEvents({ event: 'up', ptrId, x, y, isPrimary });
				}
				break;
			case 'Canceled':
			case 'CaptureLost':
				if (this._down.delete(ptrId)) {
					this._handleEvents({ event: 'cancel', ptrId, x, y, isPrimary });
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
