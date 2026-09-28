declare class NSCGamepadManager extends NSObject {
	static alloc(): NSCGamepadManager; // inherited from NSObject

	static new(): NSCGamepadManager; // inherited from NSObject

	static readonly shared: NSCGamepadManager;

	startWithBufferLengthListener(buffer: interop.Pointer | interop.Reference<any>, length: number, listener: (p1: number, p2: boolean, p3: string) => void): void;

	stop(): void;
}
