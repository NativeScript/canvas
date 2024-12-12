import '@nativescript/foundation';

import type { NSCCanvas } from './canvas';

class Pointer {
	pointerId: number;
	location: CGPoint;
	touch: NSTouch;
	released: boolean;

	constructor(pointerId: number, location: CGPoint, touch: NSTouch, released: boolean) {
		this.pointerId = pointerId;
		this.location = location;
		this.touch = touch;
		this.released = released;
	}
}

class NSCClickGestureRecognizerImpl extends NSGestureRecognizer {
	_owner?: WeakRef<NSCTouchHandler>;
	_view?: WeakRef<NSCCanvas>;
	static {
		NativeClass(this);
	}

	static initWithOwner(owner: NSCTouchHandler) {
		const ret = NSCClickGestureRecognizerImpl.alloc().initWithTargetAction(owner, null);
		ret._owner = new WeakRef(owner);
		return ret;
	}

	mouseDown(event: NSEvent) {
		super.mouseDown(event);
		const owner = this._owner?.deref?.();

		if (!owner) {
			this._view?.deref?.()?.mouseDown?.(event);
			return;
		}
		const allTouches = event;
		if (allTouches) {
			// const touches = allTouches.allObjects as NSArray<NSTouch>;
			// const count = touches.count;
			// for (let i = 0; i < count; i++) {
			// 	const touch = touches.objectAtIndex(i);
			// 	const location = touch.locationInView(this.view);
			// 	const pointer = new Pointer(owner.nextId, location, touch, false);
			// 	owner.pointers.push(pointer);
			// 	owner.onPress(pointer.pointerId, location.x, location.y, this);
			// 	owner.nextId++;
			// }
		}

		//	this._view?.deref?.()?.mouseDown?.(event);
	}

	mouseUp(event: NSEvent) {
		super.mouseUp(event);
		// const owner = this._owner?.deref?.();
		// if (!owner) {
		// 	this._view?.deref?.()?.mouseUp?.(event);
		// 	return;
		// }
		//
		// const allTouches = event.allTouches();
		// if (allTouches) {
		// 	const touches = allTouches.allObjects as NSArray<NSTouch>;
		// 	const count = touches.count;
		// 	for (let i = 0; i < count; i++) {
		// 		const touch = touches.objectAtIndex(i);
		// 		const location = touch.locationInView(this.view);
		// 		const index = owner.pointers.findIndex((pointer) => {
		// 			return touch.isEqual(pointer.touch);
		// 		});
		// 		if (index === -1) {
		// 			continue;
		// 		}
		// 		const pointer = owner.pointers[index];
		// 		if (pointer.released) {
		// 			owner.pointers.splice(index, 1);
		// 			continue;
		// 		}
		//
		// 		owner.onRelease(pointer.pointerId, location.x, location.y, this);
		// 		pointer.released = true;
		// 		owner.pointers.splice(index, 1);
		// 	}
		// }
		//
		// this._view?.deref?.()?.mouseUp?.(event);
	}

	mouseDragged(event: NSEvent) {
		super.mouseDragged(event);
		// const owner = this._owner?.deref?.();
		// if (!owner) {
		// 	this._view?.deref?.()?.mouseDragged?.(event);
		// 	return;
		// }
		//
		// const view = this._view?.deref?.();
		// const allTouches = event.allTouches();
		// if (allTouches) {
		// 	const touches = allTouches.allObjects as NSArray<NSTouch>;
		// 	if (touches) {
		// 		const ret = {
		// 			event: 'move',
		// 			pointers: new Array(owner.pointers.length)
		// 		};
		// 		for (const [index, pointer] of owner.pointers.entries()) {
		// 			const location = pointer.touch.locationInView(view);
		// 			const pointerId = pointer.pointerId;
		// 			ret.pointers[index] = {
		// 				ptrId: pointerId,
		// 				isPrimary: index === 0,
		// 				x: location.x,
		// 				y: location.y
		//
		// 			};
		// 		}
		// 		view.touchEventListener(ret, this);
		// 	}
		// }
		//
		// view?.mouseDragged?.(event);
	}

	touchesCancelledWithEvent(event: NSEvent) {
		const owner = this._owner?.deref?.();
		if (!owner) {
			this._view?.deref?.()?.touchesCancelledWithEvent?.(event);
			return;
		}

		const allTouches = event.allTouches();
		if (allTouches) {
			const touches = allTouches.allObjects as NSArray<NSTouch>;
			const count = touches.count;
			for (let i = 0; i < count; i++) {
				const touch = touches.objectAtIndex(i);
				const location = touch.locationInView(this.view);
				const index = owner.pointers.findIndex((pointer) => {
					return touch.isEqual(pointer.touch);
				});
				if (index === -1) {
					continue;
				}
				const pointer = owner.pointers[index];
				if (pointer.released) {
					owner.pointers.splice(index, 1);
					continue;
				}

				owner.onCancel(pointer.pointerId, location.x, location.y, this);
				pointer.released = true;
				owner.pointers.splice(index, 1);
			}
		}

		this._view?.deref?.()?.touchesCancelledWithEvent?.(event);
	}
}

class NSCPanGestureRecognizerImpl extends NSPanGestureRecognizer {
	_owner: WeakRef<NSCTouchHandler>;
	static {
		NativeClass(this);
	}

	static initWithOwner(owner: NSCTouchHandler) {
		const ret = new NSCPanGestureRecognizerImpl();
		ret._owner = new WeakRef(owner);
		return ret;
	}
}

class NSCMagnificationGestureRecognizerImpl extends NSMagnificationGestureRecognizer {
	_owner: WeakRef<NSCTouchHandler>;
	static {
		NativeClass(this);
	}

	static initWithOwner(owner: NSCTouchHandler) {
		const ret = new NSCMagnificationGestureRecognizerImpl();
		ret._owner = new WeakRef(owner);
		return ret;
	}
}

class NSCTouchHandler extends NSObject {
	static {
		NativeClass(this);
	}
	view?: WeakRef<NSCCanvas>;
	nextId = 0;
	pointers: Array<Pointer> = [];
	gestureRecognizer: NSCClickGestureRecognizerImpl;
	panRecognizer: NSCPanGestureRecognizerImpl;
	magnificationRecognizer: NSCMagnificationGestureRecognizerImpl;

	static initWithOwner(owner: NSCCanvas) {
		const ret = NSCTouchHandler.new();
		ret.view = new WeakRef<NSCCanvas>(owner);
		ret.gestureRecognizer = NSCClickGestureRecognizerImpl.initWithOwner(ret);
		ret.panRecognizer = NSCPanGestureRecognizerImpl.initWithOwner(ret);
		ret.magnificationRecognizer = NSCMagnificationGestureRecognizerImpl.initWithOwner(ret);
		return ret;
	}

	onPress(ptrId: number, x: number, y: number, gestureRecognizer: NSGestureRecognizer) {
		const object = {
			event: 'down',
			ptrId,
			isPrimary: this.pointers[0]?.pointerId === ptrId,
			x,
			y,
		};
		this.view?.deref?.()?.touchEventListener?.(object, gestureRecognizer);
	}

	onRelease(ptrId: number, x: number, y: number, gestureRecognizer: NSGestureRecognizer) {
		const object = {
			event: 'up',
			ptrId,
			isPrimary: this.pointers[0]?.pointerId === ptrId,
			x,
			y,
		};
		this.view?.deref?.()?.touchEventListener?.(object, gestureRecognizer);
	}

	onCancel(ptrId: number, x: number, y: number, gestureRecognizer: NSGestureRecognizer) {
		const object = {
			event: 'cancel',
			ptrId,
			isPrimary: this.pointers[0]?.pointerId === ptrId,
			x,
			y,
		};

		this.view?.deref?.()?.touchEventListener?.(object, gestureRecognizer);
	}

	static ObjCExposedMethods = {
		mouseEntered: { returns: interop.types.void, params: [NSEvent] },
		mouseExited: { returns: interop.types.void, params: [NSEvent] },
	};
}

export default NSCTouchHandler;
