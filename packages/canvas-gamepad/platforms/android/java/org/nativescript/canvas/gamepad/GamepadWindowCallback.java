package org.nativescript.canvas.gamepad;

import android.os.Build;
import android.view.ActionMode;
import android.view.KeyEvent;
import android.view.KeyboardShortcutGroup;
import android.view.Menu;
import android.view.MenuItem;
import android.view.MotionEvent;
import android.view.SearchEvent;
import android.view.View;
import android.view.Window;
import android.view.WindowManager;
import android.view.accessibility.AccessibilityEvent;

import androidx.annotation.NonNull;
import androidx.annotation.Nullable;
import androidx.annotation.RequiresApi;

import java.util.List;

// Joystick events only reach the focused view.
final class GamepadWindowCallback implements Window.Callback {
	private final Window.Callback wrapped;

	GamepadWindowCallback(Window.Callback wrapped) {
		this.wrapped = wrapped;
	}

	@Override
	public boolean dispatchKeyEvent(KeyEvent event) {
		return NSCGamepadManager.onKeyEvent(event) || wrapped.dispatchKeyEvent(event);
	}

	@Override
	public boolean dispatchGenericMotionEvent(MotionEvent event) {
		return NSCGamepadManager.onMotionEvent(event) || wrapped.dispatchGenericMotionEvent(event);
	}

	@Override
	public boolean dispatchKeyShortcutEvent(KeyEvent event) {
		return wrapped.dispatchKeyShortcutEvent(event);
	}

	@Override
	public boolean dispatchTouchEvent(MotionEvent event) {
		return wrapped.dispatchTouchEvent(event);
	}

	@Override
	public boolean dispatchTrackballEvent(MotionEvent event) {
		return wrapped.dispatchTrackballEvent(event);
	}

	@Override
	public boolean dispatchPopulateAccessibilityEvent(AccessibilityEvent event) {
		return wrapped.dispatchPopulateAccessibilityEvent(event);
	}

	@Nullable
	@Override
	public View onCreatePanelView(int featureId) {
		return wrapped.onCreatePanelView(featureId);
	}

	@Override
	public boolean onCreatePanelMenu(int featureId, @NonNull Menu menu) {
		return wrapped.onCreatePanelMenu(featureId, menu);
	}

	@Override
	public boolean onPreparePanel(int featureId, @Nullable View view, @NonNull Menu menu) {
		return wrapped.onPreparePanel(featureId, view, menu);
	}

	@Override
	public boolean onMenuOpened(int featureId, @NonNull Menu menu) {
		return wrapped.onMenuOpened(featureId, menu);
	}

	@Override
	public boolean onMenuItemSelected(int featureId, @NonNull MenuItem item) {
		return wrapped.onMenuItemSelected(featureId, item);
	}

	@Override
	public void onWindowAttributesChanged(WindowManager.LayoutParams attrs) {
		wrapped.onWindowAttributesChanged(attrs);
	}

	@Override
	public void onContentChanged() {
		wrapped.onContentChanged();
	}

	@Override
	public void onWindowFocusChanged(boolean hasFocus) {
		wrapped.onWindowFocusChanged(hasFocus);
	}

	@Override
	public void onAttachedToWindow() {
		wrapped.onAttachedToWindow();
	}

	@Override
	public void onDetachedFromWindow() {
		wrapped.onDetachedFromWindow();
	}

	@Override
	public void onPanelClosed(int featureId, @NonNull Menu menu) {
		wrapped.onPanelClosed(featureId, menu);
	}

	@Override
	public boolean onSearchRequested() {
		return wrapped.onSearchRequested();
	}

	@RequiresApi(Build.VERSION_CODES.M)
	@Override
	public boolean onSearchRequested(SearchEvent searchEvent) {
		return wrapped.onSearchRequested(searchEvent);
	}

	@Nullable
	@Override
	public ActionMode onWindowStartingActionMode(ActionMode.Callback callback) {
		return wrapped.onWindowStartingActionMode(callback);
	}

	@RequiresApi(Build.VERSION_CODES.M)
	@Nullable
	@Override
	public ActionMode onWindowStartingActionMode(ActionMode.Callback callback, int type) {
		return wrapped.onWindowStartingActionMode(callback, type);
	}

	@Override
	public void onActionModeStarted(ActionMode mode) {
		wrapped.onActionModeStarted(mode);
	}

	@Override
	public void onActionModeFinished(ActionMode mode) {
		wrapped.onActionModeFinished(mode);
	}

	@RequiresApi(Build.VERSION_CODES.N)
	@Override
	public void onProvideKeyboardShortcuts(List<KeyboardShortcutGroup> data, @Nullable Menu menu, int deviceId) {
		wrapped.onProvideKeyboardShortcuts(data, menu, deviceId);
	}

	@RequiresApi(Build.VERSION_CODES.O)
	@Override
	public void onPointerCaptureChanged(boolean hasCapture) {
		wrapped.onPointerCaptureChanged(hasCapture);
	}
}
