class FrameHandlerImpl extends NSObject {
	/**
	 * @type {WeakRef<FPSCallback>}
	 */
	_owner;

	static {
		NativeClass(this);
	}

	/**
	 *
	 * @param {WeakRef<FPSCallback>} owner
	 * @returns {FrameHandlerImpl}
	 */
	static initWithOwner(owner) {
		const handler = FrameHandlerImpl.new();
		handler._owner = owner;

		return handler;
	}

	/**
	 *
	 * @param {CADisplayLink} sender
	 */
	handleFrame(sender) {
		const owner = this._owner?.deref();
		if (owner) {
			owner._handleFrame(sender);
		}
	}

	static ObjCExposedMethods = {
		handleFrame: { returns: interop.types.void, params: [CADisplayLink] },
	};
}

class FPSCallback {
	/**
	 * @type {boolean}
	 */
	running;
	/**
	 * @type {Function}
	 */
	onFrame;
	/**
	 * @type {CADisplayLink}
	 */
	displayLink;
	/**
	 * @type {FrameHandlerImpl}
	 */
	impl;

	/**
	 *
	 * @param {function(number): void}  onFrame
	 */
	constructor(onFrame) {
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

	/**
	 *
	 * @param  {CADisplayLink} sender
	 */
	_handleFrame(sender) {
		if (!this.running) {
			return;
		}

		// timestamp is CFTimeInterval, which is in seconds, the onFrame callback expects millis, so multiply by 1000
		this.onFrame(sender.timestamp * 1000);
	}
}

module.exports.FPSCallback = FPSCallback;
