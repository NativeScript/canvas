package org.nativescript.canvas.gamepad;

import android.app.Activity;
import android.app.Application;
import android.content.Context;
import android.hardware.input.InputManager;
import android.os.Bundle;
import android.os.Handler;
import android.os.Looper;
import android.view.InputDevice;
import android.view.KeyEvent;
import android.view.MotionEvent;
import android.view.Window;

import androidx.annotation.NonNull;
import androidx.annotation.Nullable;

import java.nio.FloatBuffer;
import java.util.Locale;

public final class NSCGamepadManager {
	public interface Listener {
		void onConnection(int index, boolean connected, String id);
	}

	// Keep in sync with packages/canvas-gamepad/common.ts.
	static final int MAX_GAMEPADS = 4;
	static final int MAX_AXES = 8;
	static final int SLOT_CONNECTED = 0;
	static final int SLOT_SEQUENCE = 1;
	static final int SLOT_MAPPING = 2;
	static final int SLOT_AXIS_COUNT = 3;
	static final int SLOT_BUTTON_COUNT = 4;
	static final int SLOT_AXES = 5;
	static final int SLOT_BUTTONS = SLOT_AXES + MAX_AXES;
	static final int SLOT_STRIDE = 80;
	static final int BUTTON_PRESSED = 1;
	static final int BUTTON_TOUCHED = 2;
	static final float MAPPING_STANDARD = 1;
	static final int STANDARD_AXES = 4;
	static final int STANDARD_BUTTONS = 17;
	// Float32 holds integers exactly up to 2^24.
	static final float SEQUENCE_WRAP = 16777216f;
	static final float TRIGGER_PRESSED = 0.1f;

	static final int BUTTON_LEFT_TRIGGER = 6;
	static final int BUTTON_RIGHT_TRIGGER = 7;
	static final int BUTTON_DPAD_UP = 12;
	static final int BUTTON_DPAD_DOWN = 13;
	static final int BUTTON_DPAD_LEFT = 14;
	static final int BUTTON_DPAD_RIGHT = 15;

	private static final class Slot {
		final int deviceId;
		final boolean hasHat;
		final boolean hasAnalogTriggers;
		final int rightX;
		final int rightY;

		Slot(InputDevice device) {
			deviceId = device.getId();
			hasHat = device.getMotionRange(MotionEvent.AXIS_HAT_X) != null;
			hasAnalogTriggers = device.getMotionRange(MotionEvent.AXIS_LTRIGGER) != null || device.getMotionRange(MotionEvent.AXIS_BRAKE) != null;
			boolean z = device.getMotionRange(MotionEvent.AXIS_Z) != null;
			boolean rx = device.getMotionRange(MotionEvent.AXIS_RX) != null;
			rightX = !z && rx ? MotionEvent.AXIS_RX : MotionEvent.AXIS_Z;
			rightY = !z && rx ? MotionEvent.AXIS_RY : MotionEvent.AXIS_RZ;
		}
	}

	private static final Slot[] slots = new Slot[MAX_GAMEPADS];
	@Nullable
	private static FloatBuffer buffer;
	@Nullable
	private static Listener listener;
	@Nullable
	private static InputManager inputManager;
	@Nullable
	private static Application application;

	private static final InputManager.InputDeviceListener deviceListener = new InputManager.InputDeviceListener() {
		@Override
		public void onInputDeviceAdded(int deviceId) {
			attach(deviceId);
		}

		@Override
		public void onInputDeviceRemoved(int deviceId) {
			detach(deviceId);
		}

		@Override
		public void onInputDeviceChanged(int deviceId) {
		}
	};

	private static final Application.ActivityLifecycleCallbacks lifecycle = new Application.ActivityLifecycleCallbacks() {
		@Override
		public void onActivityCreated(@NonNull Activity activity, @Nullable Bundle savedInstanceState) {
			hook(activity);
		}

		@Override
		public void onActivityStarted(@NonNull Activity activity) {
		}

		@Override
		public void onActivityResumed(@NonNull Activity activity) {
			hook(activity);
		}

		@Override
		public void onActivityPaused(@NonNull Activity activity) {
		}

		@Override
		public void onActivityStopped(@NonNull Activity activity) {
		}

		@Override
		public void onActivitySaveInstanceState(@NonNull Activity activity, @NonNull Bundle outState) {
		}

		@Override
		public void onActivityDestroyed(@NonNull Activity activity) {
		}
	};

	private NSCGamepadManager() {
	}

	public static void start(@NonNull Context context, @Nullable Activity activity, @NonNull FloatBuffer buffer, @NonNull Listener listener) {
		stop();
		if (buffer.capacity() < MAX_GAMEPADS * SLOT_STRIDE) {
			return;
		}
		NSCGamepadManager.buffer = buffer;
		NSCGamepadManager.listener = listener;
		for (int i = 0; i < MAX_GAMEPADS * SLOT_STRIDE; i++) {
			buffer.put(i, 0);
		}

		Context app = context.getApplicationContext();
		if (app instanceof Application) {
			application = (Application) app;
			application.registerActivityLifecycleCallbacks(lifecycle);
		}
		if (activity != null) {
			hook(activity);
		}

		inputManager = (InputManager) app.getSystemService(Context.INPUT_SERVICE);
		if (inputManager != null) {
			inputManager.registerInputDeviceListener(deviceListener, new Handler(Looper.getMainLooper()));
			for (int id : inputManager.getInputDeviceIds()) {
				attach(id);
			}
		}
	}

	public static void stop() {
		if (inputManager != null) {
			inputManager.unregisterInputDeviceListener(deviceListener);
			inputManager = null;
		}
		if (application != null) {
			application.unregisterActivityLifecycleCallbacks(lifecycle);
			application = null;
		}
		for (int i = 0; i < MAX_GAMEPADS; i++) {
			slots[i] = null;
		}
		buffer = null;
		listener = null;
	}

	static boolean isGamepad(int sources) {
		return (sources & InputDevice.SOURCE_GAMEPAD) == InputDevice.SOURCE_GAMEPAD || (sources & InputDevice.SOURCE_JOYSTICK) == InputDevice.SOURCE_JOYSTICK;
	}

	private static void hook(Activity activity) {
		Window window = activity.getWindow();
		if (window == null) {
			return;
		}
		Window.Callback callback = window.getCallback();
		if (callback != null && !(callback instanceof GamepadWindowCallback)) {
			window.setCallback(new GamepadWindowCallback(callback));
		}
	}

	private static int slotOf(int deviceId) {
		for (int i = 0; i < MAX_GAMEPADS; i++) {
			Slot slot = slots[i];
			if (slot != null && slot.deviceId == deviceId) {
				return i;
			}
		}
		return -1;
	}

	private static void attach(int deviceId) {
		FloatBuffer buffer = NSCGamepadManager.buffer;
		InputDevice device = InputDevice.getDevice(deviceId);
		if (buffer == null || device == null || device.isVirtual() || !isGamepad(device.getSources()) || slotOf(deviceId) != -1) {
			return;
		}
		int index = -1;
		for (int i = 0; i < MAX_GAMEPADS; i++) {
			if (slots[i] == null) {
				index = i;
				break;
			}
		}
		if (index == -1) {
			return;
		}
		slots[index] = new Slot(device);

		int base = index * SLOT_STRIDE;
		for (int i = 0; i < SLOT_STRIDE; i++) {
			buffer.put(base + i, 0);
		}
		buffer.put(base + SLOT_CONNECTED, 1);
		buffer.put(base + SLOT_MAPPING, MAPPING_STANDARD);
		buffer.put(base + SLOT_AXIS_COUNT, STANDARD_AXES);
		buffer.put(base + SLOT_BUTTON_COUNT, STANDARD_BUTTONS);

		if (listener != null) {
			String id = String.format(Locale.US, "%s (STANDARD GAMEPAD Vendor: %04x Product: %04x)", device.getName(), device.getVendorId(), device.getProductId());
			listener.onConnection(index, true, id);
		}
	}

	private static void detach(int deviceId) {
		FloatBuffer buffer = NSCGamepadManager.buffer;
		int index = slotOf(deviceId);
		if (buffer == null || index == -1) {
			return;
		}
		slots[index] = null;
		int base = index * SLOT_STRIDE;
		for (int i = 0; i < SLOT_STRIDE; i++) {
			buffer.put(base + i, 0);
		}
		if (listener != null) {
			listener.onConnection(index, false, "");
		}
	}

	static int buttonForKey(int keyCode) {
		switch (keyCode) {
			case KeyEvent.KEYCODE_BUTTON_A:
				return 0;
			case KeyEvent.KEYCODE_BUTTON_B:
				return 1;
			case KeyEvent.KEYCODE_BUTTON_X:
				return 2;
			case KeyEvent.KEYCODE_BUTTON_Y:
				return 3;
			case KeyEvent.KEYCODE_BUTTON_L1:
				return 4;
			case KeyEvent.KEYCODE_BUTTON_R1:
				return 5;
			case KeyEvent.KEYCODE_BUTTON_L2:
				return BUTTON_LEFT_TRIGGER;
			case KeyEvent.KEYCODE_BUTTON_R2:
				return BUTTON_RIGHT_TRIGGER;
			case KeyEvent.KEYCODE_BUTTON_SELECT:
				return 8;
			case KeyEvent.KEYCODE_BUTTON_START:
				return 9;
			case KeyEvent.KEYCODE_BUTTON_THUMBL:
				return 10;
			case KeyEvent.KEYCODE_BUTTON_THUMBR:
				return 11;
			case KeyEvent.KEYCODE_DPAD_UP:
				return BUTTON_DPAD_UP;
			case KeyEvent.KEYCODE_DPAD_DOWN:
				return BUTTON_DPAD_DOWN;
			case KeyEvent.KEYCODE_DPAD_LEFT:
				return BUTTON_DPAD_LEFT;
			case KeyEvent.KEYCODE_DPAD_RIGHT:
				return BUTTON_DPAD_RIGHT;
			case KeyEvent.KEYCODE_BUTTON_MODE:
				return 16;
			default:
				return -1;
		}
	}

	private static void writeButton(FloatBuffer buffer, int base, int button, float value, boolean pressed) {
		int offset = base + SLOT_BUTTONS + button * 2;
		buffer.put(offset, value);
		buffer.put(offset + 1, (pressed ? BUTTON_PRESSED : 0) | (pressed || value > 0 ? BUTTON_TOUCHED : 0));
	}

	private static void bump(FloatBuffer buffer, int base) {
		buffer.put(base + SLOT_SEQUENCE, (buffer.get(base + SLOT_SEQUENCE) + 1) % SEQUENCE_WRAP);
	}

	static boolean onKeyEvent(KeyEvent event) {
		FloatBuffer buffer = NSCGamepadManager.buffer;
		if (buffer == null || !isGamepad(event.getSource())) {
			return false;
		}
		int index = slotOf(event.getDeviceId());
		int button = buttonForKey(event.getKeyCode());
		if (index == -1 || button == -1) {
			return false;
		}
		int action = event.getAction();
		if (action != KeyEvent.ACTION_DOWN && action != KeyEvent.ACTION_UP) {
			return true;
		}
		if (event.getRepeatCount() > 0) {
			return true;
		}
		Slot slot = slots[index];
		boolean analogTrigger = slot.hasAnalogTriggers && (button == BUTTON_LEFT_TRIGGER || button == BUTTON_RIGHT_TRIGGER);
		boolean hatDpad = slot.hasHat && button >= BUTTON_DPAD_UP && button <= BUTTON_DPAD_RIGHT;
		if (!analogTrigger && !hatDpad) {
			int base = index * SLOT_STRIDE;
			boolean down = action == KeyEvent.ACTION_DOWN;
			writeButton(buffer, base, button, down ? 1 : 0, down);
			bump(buffer, base);
		}
		return true;
	}

	static boolean onMotionEvent(MotionEvent event) {
		FloatBuffer buffer = NSCGamepadManager.buffer;
		if (buffer == null || (event.getSource() & InputDevice.SOURCE_JOYSTICK) != InputDevice.SOURCE_JOYSTICK || event.getActionMasked() != MotionEvent.ACTION_MOVE) {
			return false;
		}
		int index = slotOf(event.getDeviceId());
		if (index == -1) {
			return false;
		}
		Slot slot = slots[index];
		int base = index * SLOT_STRIDE;

		buffer.put(base + SLOT_AXES, event.getAxisValue(MotionEvent.AXIS_X));
		buffer.put(base + SLOT_AXES + 1, event.getAxisValue(MotionEvent.AXIS_Y));
		buffer.put(base + SLOT_AXES + 2, event.getAxisValue(slot.rightX));
		buffer.put(base + SLOT_AXES + 3, event.getAxisValue(slot.rightY));

		if (slot.hasAnalogTriggers) {
			float left = Math.max(event.getAxisValue(MotionEvent.AXIS_LTRIGGER), event.getAxisValue(MotionEvent.AXIS_BRAKE));
			float right = Math.max(event.getAxisValue(MotionEvent.AXIS_RTRIGGER), event.getAxisValue(MotionEvent.AXIS_GAS));
			writeButton(buffer, base, BUTTON_LEFT_TRIGGER, left, left > TRIGGER_PRESSED);
			writeButton(buffer, base, BUTTON_RIGHT_TRIGGER, right, right > TRIGGER_PRESSED);
		}

		if (slot.hasHat) {
			float x = event.getAxisValue(MotionEvent.AXIS_HAT_X);
			float y = event.getAxisValue(MotionEvent.AXIS_HAT_Y);
			writeButton(buffer, base, BUTTON_DPAD_UP, y < -0.5f ? 1 : 0, y < -0.5f);
			writeButton(buffer, base, BUTTON_DPAD_DOWN, y > 0.5f ? 1 : 0, y > 0.5f);
			writeButton(buffer, base, BUTTON_DPAD_LEFT, x < -0.5f ? 1 : 0, x < -0.5f);
			writeButton(buffer, base, BUTTON_DPAD_RIGHT, x > 0.5f ? 1 : 0, x > 0.5f);
		}

		bump(buffer, base);
		return true;
	}
}
