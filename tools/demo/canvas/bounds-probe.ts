/**
 * Is `Canvas.getBoundingClientRect()` actually wired up on Android?
 *
 * The Android path is split in two: the JS side hands a `Float32Array` to
 * `NSCCanvas.setBoundsBuffer` once, and every later read calls the static
 * `NSCCanvas.getBoundingClientRect(canvas)`, which writes into that buffer.
 * Both halves have to hold for a caller (PixiJS, say) to see a real rect:
 *
 *   1. the runtime has to marshal the Float32Array into a *direct*
 *      java.nio.FloatBuffer that shares the JS backing store -- a copy would
 *      leave JS reading zeros forever;
 *   2. the buffer has to be registered *before* the first native fill.
 *
 * `NSCCanvas.getBoundingClientRectJSON` computes the same eight numbers from
 * the same View and returns them as a string, so it is the control: it needs
 * no buffer at all. Wherever the buffer disagrees with the JSON, the buffer
 * half is broken.
 *
 *   adb shell am start -n org.nativescript.plugindemo/com.tns.NativeScriptActivity \
 *     --es demo canvas-perf --es suite bounds
 *   adb logcat -d | grep 'BOUNDS|'
 */

declare const org: any;

function log(...parts: unknown[]) {
	console.log(['BOUNDS', ...parts].join('|'));
}

function rectToString(rect: any) {
	if (!rect) {
		return 'null';
	}
	const round = (v: number) => Math.round((v ?? NaN) * 100) / 100;
	return `x=${round(rect.x)} y=${round(rect.y)} w=${round(rect.width)} h=${round(rect.height)} t=${round(rect.top)} r=${round(rect.right)} b=${round(rect.bottom)} l=${round(rect.left)}`;
}

export function runBoundsProbe(canvas: any) {
	if (!__ANDROID__) {
		log('skip', 'android only');
		return;
	}

	const native = canvas.android;
	if (!native) {
		log('error', 'no native view');
		return;
	}

	log('parent', String(!!canvas.parent));
	log('isConnected', String((canvas as any).isConnected));
	log('client', `${canvas.clientWidth}x${canvas.clientHeight}`);
	log('surface', `${canvas.width}x${canvas.height}`);

	// The control: no buffer involved, straight from the same View.
	log('json', org.nativescript.canvas.NSCCanvas.getBoundingClientRectJSON(native));

	// Call 1 is the one that matters -- it is the call every library makes first.
	log('rect#1', rectToString(canvas.getBoundingClientRect()));
	log('rect#2', rectToString(canvas.getBoundingClientRect()));
	log('rect#3', rectToString(canvas.getBoundingClientRect()));

	// Does the marshalled FloatBuffer share memory with the JS Float32Array, or
	// did the runtime hand Java a copy? Write from JS, read back through Java.
	const probe = new Float32Array(8);
	native.setBoundsBuffer(probe);

	const registered = native.getBoundsBuffer();
	log('registered', String(!!registered));

	if (registered) {
		// Shared memory, JS -> Java: if the runtime handed Java a copy, Java
		// reads 0 here instead of what JS just wrote.
		probe[0] = 1234.5;
		let javaSaw = NaN;
		try {
			javaSaw = registered.get(0);
		} catch (e) {
			log('js->java', `threw ${e}`);
		}
		log('js->java', `wrote 1234.5, java read ${javaSaw}`);

		// Shared memory, Java -> JS: the direction getBoundingClientRect relies on.
		org.nativescript.canvas.NSCCanvas.getBoundingClientRect(native);
		log('java->js', `top=${probe[0]} right=${probe[1]} bottom=${probe[2]} left=${probe[3]} w=${probe[4]} h=${probe[5]} x=${probe[6]} y=${probe[7]}`);
	}

	// The state every Canvas is in before its buffer is registered -- what the
	// first getBoundingClientRect() used to hit. Kotlin's `boundsBuffer?.let`
	// skips silently, so the fill writes nowhere and the caller reads zeros.
	probe.fill(0);
	native.setBoundsBuffer(null);
	org.nativescript.canvas.NSCCanvas.getBoundingClientRect(native);
	log('nullbuffer', `fill was a silent no-op: ${probe.every((v) => v === 0)}`);

	// Hand the canvas its own buffer back (TS `private` is compile-time only),
	// so the page is left in a working state.
	native.setBoundsBuffer((canvas as any)._jsBuffer ?? null);
	log('rect#4', rectToString(canvas.getBoundingClientRect()));

	log('done');
}
