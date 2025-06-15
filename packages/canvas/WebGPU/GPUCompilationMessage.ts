import { native_ } from './Constants';

export class GPUCompilationMessage {
	[native_];

	static fromNative(message) {
		if (message) {
			const ret = new GPUCompilationMessage();
			ret[native_] = message;
			return ret;
		}
		return null;
	}

	get length(): number {
		return this[native_]?.length ?? 0;
	}

	get lineNum(): number {
		return this[native_]?.lineNum ?? 0;
	}
	get linePos(): number {
		return this[native_]?.linePos ?? 0;
	}
	get message(): string {
		return this[native_]?.message ?? '';
	}
	get offset(): number {
		return this[native_]?.offset ?? 0;
	}
	get type(): 'error' | 'warning' | 'info' {
		return this[native_]?.type ?? 'error';
	}
}
