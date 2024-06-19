class GPUDeviceLostInfo {
	_native;

	get message() {
		return this._native.message;
	}

	get reason() {
		return this._native.reason;
	}
}
