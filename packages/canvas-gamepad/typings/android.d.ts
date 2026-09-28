declare namespace org {
	export namespace nativescript {
		export namespace canvas {
			export namespace gamepad {
				export class NSCGamepadManager {
					static start(context: android.content.Context, activity: android.app.Activity | null, buffer: java.nio.FloatBuffer, listener: NSCGamepadManager.Listener): void;
					static stop(): void;
				}
				export namespace NSCGamepadManager {
					export class Listener {
						constructor(implementation: { onConnection(index: number, connected: boolean, id: string): void });
						onConnection(index: number, connected: boolean, id: string): void;
					}
				}
			}
		}
	}
}
