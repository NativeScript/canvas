class FrameHandlerImpl extends NSObject {
	_owner?: WeakRef<FPSCallback>;

	static {
		NativeClass(this);
	}

	static initWithOwner(owner: WeakRef<FPSCallback>): FrameHandlerImpl {
		const handler = FrameHandlerImpl.new();
		handler._owner = owner;

		return handler;
	}

	/**
	 *
	 * @param {CADisplayLink} sender
	 */
	handleFrame(sender: CADisplayLink) {
		const owner = this._owner?.deref();
		if (owner) {
			owner._handleFrame(sender);
		}
	}

	static ObjCExposedMethods = {
		handleFrame: { returns: interop.types.void, params: [CADisplayLink] },
	};
}

export class FPSCallback {
	running = false;
	onFrame: (time: number) => void;
	displayLink: CADisplayLink;
	impl: FrameHandlerImpl;

	constructor(onFrame: (time: number) => void) {
		this.onFrame = onFrame;

		this.impl = FrameHandlerImpl.initWithOwner(new WeakRef(this));

		this.displayLink = NSScreen.mainScreen.displayLinkWithTargetSelector(this.impl, 'handleFrame');
		this.displayLink.isPaused = true;
		this.displayLink.addToRunLoopForMode(NSRunLoop.currentRunLoop, NSDefaultRunLoopMode);
	}

	start() {
		if (this.running) {
			return;
		}

		this.running = true;
		this.displayLink.isPaused = false;
	}

	stop() {
		if (!this.running) {
			return;
		}

		this.displayLink.isPaused = true;
		this.running = false;
	}

	_handleFrame(sender: CADisplayLink) {
		if (!this.running) {
			return;
		}

		// timestamp is CFTimeInterval, which is in seconds, the onFrame callback expects millis, so multiply by 1000
		this.onFrame(sender.timestamp * 1000);
	}
}
