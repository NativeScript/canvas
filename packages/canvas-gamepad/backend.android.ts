import { Application, Utils } from '@nativescript/core';
import type { GamepadBackend } from './common';

export function createBackend(): GamepadBackend {
	return {
		start(buffer, onConnection) {
			org.nativescript.canvas.gamepad.NSCGamepadManager.start(
				Utils.android.getApplicationContext(),
				Application.android.foregroundActivity ?? Application.android.startActivity ?? null,
				buffer as never,
				new org.nativescript.canvas.gamepad.NSCGamepadManager.Listener({
					onConnection(index: number, connected: boolean, id: string) {
						onConnection(index, connected, id);
					},
				}),
			);
		},
	};
}
