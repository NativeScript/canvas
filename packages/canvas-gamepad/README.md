# Nativescript canvas-gamepad

The [Gamepad API](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad_API) for NativeScript: `navigator.getGamepads()` plus `gamepadconnected` / `gamepaddisconnected`.

```javascript
ns plugin add @nativescript/canvas-gamepad
```

## Usage

With `@nativescript/canvas-polyfill`, web code works unchanged:

```ts
window.addEventListener('gamepadconnected', (e) => console.log(e.gamepad.id));

function frame() {
	const pad = navigator.getGamepads()[0];
	if (pad) {
		const [x, y] = pad.axes;
		const jump = pad.buttons[0].pressed;
	}
	requestAnimationFrame(frame);
}
requestAnimationFrame(frame);
```

Without the polyfill:

```ts
import { gamepads, getGamepads } from '@nativescript/canvas-gamepad';

gamepads.addListener((e) => console.log(e.type, e.gamepad.index));
const pads = getGamepads();
```

Monitoring starts on the first `getGamepads()` call or gamepad listener.

## Behaviour

- Controllers use the [standard mapping](https://w3c.github.io/gamepad/#remapping): 4 axes, 17 buttons (16 on Windows, which has no guide button). Up to 4 controllers.
- `Gamepad` objects are updated in place between calls, as in Firefox. Copy `axes` / `buttons` if you compare frames.
- `timestamp` is `performance.now()` at the first `getGamepads()` after the controller changed.
- `vibrationActuator` is `null`.

| Platform | Source | Notes |
| --- | --- | --- |
| iOS, tvOS, visionOS | GameController `GCExtendedGamepad` | The Siri Remote stays mapped to keyboard events. `valueChangedHandler` on each controller is taken over. |
| Android | `KeyEvent` / `MotionEvent` from gamepad and joystick sources | Events from connected controllers are consumed at the activity window, so B no longer triggers Back while the API is in use. |
| Windows | `Windows.Gaming.Input.Gamepad` | Build the native module with `make windows-gamepad`. |

## License

Apache License Version 2.0
