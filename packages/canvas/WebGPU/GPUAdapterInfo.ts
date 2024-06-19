export class GPUAdapterInfo {
	_native;

	get architecture() {
		return this._native.architecture;
	}

	get description() {
		return this._native.description;
	}

	get device() {
		return this._native.device;
	}

	get vendor() {
		return this._native.vendor;
	}
}
