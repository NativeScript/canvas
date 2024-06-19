export class GPUBuffer {
	native_;

	get label() {
		return this.native_.label;
	}
	get size() {
		return this.native_?.size ?? 0;
	}

	get usage() {
		return this.native_?.usage ?? 0;
	}

	destroy() {
		this.native_?.destroy?.();
	}

	private _mapState = 'unmapped';

	get mapState() {
		return this._mapState;
	}

    mapAsync(){}
    unmap(){}
}
