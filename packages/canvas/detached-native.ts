/**
 * Stand-in for a native context object whose backing memory the platform view has released.
 *
 * The JS context wrappers (2D, WebGL, WebGL2) borrow a native object created from the view's
 * raw context pointer; the view frees that memory in disposeNativeView, but app code can keep
 * calling the wrapper (frame loops, or a component whose view tree was rebuilt). Swapping the
 * native object for this stand-in turns every method the live object exposed into a no-op and
 * every accessor into `undefined`, so a wrapper that outlives its canvas goes inert instead of
 * reading freed memory.
 *
 * Descriptors are read, never invoked, so building the stand-in touches no native code.
 */
export function createDetachedNative(native: any): any {
	const stub: Record<PropertyKey, any> = Object.create(null);
	const noop = () => undefined;
	const seen = new Set<PropertyKey>();
	let proto: any = native;
	while (proto && proto !== Object.prototype && proto !== Function.prototype) {
		let names: PropertyKey[] = [];
		try {
			names = [...Object.getOwnPropertyNames(proto), ...Object.getOwnPropertySymbols(proto)];
		} catch {}
		for (const name of names) {
			if (seen.has(name) || name === 'constructor') {
				continue;
			}
			seen.add(name);
			let descriptor: PropertyDescriptor | undefined;
			try {
				descriptor = Object.getOwnPropertyDescriptor(proto, name);
			} catch {}
			if (descriptor && typeof descriptor.value === 'function') {
				stub[name] = noop;
			} else {
				Object.defineProperty(stub, name, { get: () => undefined, set: () => {}, configurable: true });
			}
		}
		try {
			proto = Object.getPrototypeOf(proto);
		} catch {
			break;
		}
	}
	return stub;
}
