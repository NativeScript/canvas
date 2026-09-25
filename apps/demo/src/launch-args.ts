/**
 * Demo requested on the command line, so a run is one command instead of a tap
 * at fixed screen coordinates:
 *
 *   adb shell am start -n org.nativescript.plugindemo/com.tns.NativeScriptActivity \
 *     --es demo canvas-spec --es suite 2d
 *
 * `--es surface true` swaps the canvas backing view from a TextureView to a
 * SurfaceView, which skips a compositor copy -- the A/B for "is the extra copy
 * costing us anything on this scene".
 *
 *   xcrun simctl launch <udid> org.nativescript.plugindemo --demo=canvas-spec --suite=2d
 *
 * Captured at launch rather than read in the page, because the first page
 * navigates during bootstrap -- before Application.android.foregroundActivity
 * is assigned.
 */
export const launchArgs: { demo?: string; profile?: string; frames?: number; suite?: string; surface?: string } = {};

const KEYS = ['demo', 'profile', 'suite', 'surface'] as const;

export function captureLaunchArgs(androidIntent?: any) {
	try {
		if (__ANDROID__) {
			captureFromIntent(androidIntent);
		} else if (__APPLE__) {
			captureFromProcessArguments();
		}
	} catch (e) {
		// A launch without arguments is the normal case; nothing to do.
	}
}

function captureFromIntent(intent: any) {
	if (!intent) {
		return;
	}
	for (const key of KEYS) {
		const value = intent.getStringExtra?.(key);
		if (value) {
			launchArgs[key] = value;
			// Consume it so navigating Back does not bounce straight back in.
			intent.removeExtra?.(key);
		}
	}
	const frames = intent.getIntExtra?.('frames', 0);
	if (frames) {
		launchArgs.frames = frames;
		intent.removeExtra?.('frames');
	}
}

/** `--demo=canvas-spec`, as simctl and Xcode's scheme arguments pass them. */
function captureFromProcessArguments() {
	const args = NSProcessInfo.processInfo.arguments;
	if (!args) {
		return;
	}
	for (let i = 0; i < args.count; i++) {
		const arg = String(args.objectAtIndex(i));
		const eq = arg.indexOf('=');
		if (arg.indexOf('--') !== 0 || eq === -1) {
			continue;
		}
		const key = arg.substring(2, eq);
		const value = arg.substring(eq + 1);
		if (!value) {
			continue;
		}
		if (key === 'frames') {
			launchArgs.frames = parseInt(value, 10) || undefined;
		} else if ((KEYS as readonly string[]).indexOf(key) !== -1) {
			launchArgs[key] = value;
		}
	}
}
