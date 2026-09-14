import './callbound.js';

/**
 * Native half of the call-bound A/B. The benchmark body lives in `callbound.js`,
 * which the WebView page loads with a <script src> -- one source, so the two
 * halves cannot drift apart.
 *
 *   adb shell am start -n org.nativescript.plugindemo/com.tns.NativeScriptActivity \
 *     --es demo canvas-perf --es suite callbound
 *   adb logcat -d | grep 'NVCALL|'
 *
 * The WebView control is the same ops via:
 *   --es demo canvas-webview-compare --es suite callbound
 */
export function runCallBound(canvas: any) {
	const ctx = canvas.getContext('2d');
	if (!ctx) {
		console.log('NVCALL|error|no 2d context');
		return;
	}
	console.log('NVCALL|op|ns/op|ms per 20k');
	(globalThis as any).__callBound(ctx, (line: string) => console.log('NVCALL|' + line));
	console.log('NVCALL|done');
}
