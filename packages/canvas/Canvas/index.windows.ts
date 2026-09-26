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

/** Web `deltaY` pixels per wheel notch (`WHEEL_DELTA`, 120), as Chromium reports on Windows. */
const WHEEL_PIXELS_PER_NOTCH = 100;

export class Canvas extends NapiCanvas {
	private _panel: any;
	private _ignoreTouchEvents = false;
	/** Pointers down on the panel (captured, so moves outside it still arrive). */
	private _down = new Set<number>();
	/** Delegates are held so they live as long as the subscriptions. */
	private _pointerDelegates: Map<PointerEvent, any> | null = null;
	private _scaleDelegate: any;
	private _sizeDelegate: any;

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
		// XAML lays the panel out (NativeScript's measure/layout pass does not run for it).
		this._sizeDelegate = NSWinRT.asDelegate('Microsoft.UI.Xaml.SizeChangedEventHandler', () => ref.deref()?._syncViewSize());
		panel.SizeChanged = this._sizeDelegate;

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
		this._scaleDelegate = undefined;
		this._sizeDelegate = undefined;
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
