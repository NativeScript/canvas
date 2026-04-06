import { native_ } from './Constants';
import { GPUCompilationMessage } from './GPUCompilationMessage';
const messages_ = Symbol('messages');

export class GPUCompilationInfo {
	[native_];
	[messages_]: GPUCompilationMessage[];
	get messages(): GPUCompilationMessage[] {
		if (!this[messages_]) {
			this[messages_] = (this[native_]?.messages ?? []).map((msg) => GPUCompilationMessage.fromNative(msg));
		}
		return this[messages_];
	}

	static fromNative(info) {
		if (info) {
			const ret = new GPUCompilationInfo();
			ret[native_] = info;
			return ret;
		}
		return null;
	}
}
