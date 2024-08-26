import { Loader, FileLoader, BufferGeometry, BufferAttribute } from 'three';
const _taskCache = /* @__PURE__ */ new WeakMap();

var getJSBuffer = function (buf, type) {
	if (type === 'Uint8Array') {
		return new Uint8Array(buf);
	} else if (type === 'Uint8ClampedArray') {
		return new Uint8ClampedArray(buf);
	} else if (type === 'Uint16Array') {
		return new Uint16Array(buf);
	} else if (type === 'Uint32Array') {
		return new Uint32Array(buf);
	} else if (type === 'BigUint64Array') {
		return new BigUint64Array(buf);
	} else if (type === 'Int8Array') {
		return new Int8Array(buf);
	} else if (type === 'Int16Array') {
		return new Int16Array(buf);
	} else if (type === 'Int32Array') {
		return new Int32Array(buf);
	} else if (type === 'BigInt64Array') {
		return new BigInt64Array(buf);
	} else if (type === 'Float32Array') {
		return new Float32Array(buf);
	} else if (type === 'Float64Array') {
		return new Float64Array(buf);
	} else {
		return buf;
	}
};

function storeBuffer(key, buffer) {
	if (global.isAndroid) {
		org.nativescript.canvas.NSCCanvas.storeBuffer(key, buffer);
	}

	if (global.isIOS) {
		NSCCanvas.store.setObjectForKey(NSData.dataWithData(buffer), key);
	}
}

function removeBuffer(key) {
	if (global.isAndroid) {
		org.nativescript.canvas.NSCCanvas.removeBuffer(key);
	}

	if (global.isIOS) {
		NSCCanvas.store.removeObjectForKey(key);
	}
}

function getBuffer(key) {
	if (global.isAndroid) {
		const buffer = org.nativescript.canvas.NSCCanvas.getBuffer(key);
		if (buffer) {
			return ArrayBuffer.from(buffer);
		}

		return buffer;
	}

	if (global.isIOS) {
		const buffer = NSCCanvas.store.objectForKey(key);

		if (buffer) {
			return interop.bufferFromData(buffer);
		}

		return buffer;
	}
}

class DRACOLoader extends Loader {
	constructor(manager) {
		super(manager);
		this.decoderPath = '';
		this.decoderConfig = {};
		this.decoderBinary = null;
		this.decoderPending = null;
		this.workerLimit = 4;
		this.workerPool = [];
		this.workerNextTaskID = 1;
		this.workerSourceURL = '';
		this.defaultAttributeIDs = {
			position: 'POSITION',
			normal: 'NORMAL',
			color: 'COLOR',
			uv: 'TEX_COORD',
		};
		this.defaultAttributeTypes = {
			position: 'Float32Array',
			normal: 'Float32Array',
			color: 'Float32Array',
			uv: 'Float32Array',
		};
	}
	setDecoderPath(path) {
		this.decoderPath = path;
		return this;
	}
	setDecoderConfig(config) {
		this.decoderConfig = config;
		return this;
	}
	setWorkerLimit(workerLimit) {
		this.workerLimit = workerLimit;
		return this;
	}
	load(url, onLoad, onProgress, onError) {
		const loader = new FileLoader(this.manager);
		loader.setPath(this.path);
		loader.setResponseType('arraybuffer');
		loader.setRequestHeader(this.requestHeader);
		loader.setWithCredentials(this.withCredentials);
		loader.load(
			url,
			(buffer) => {
				const taskConfig = {
					attributeIDs: this.defaultAttributeIDs,
					attributeTypes: this.defaultAttributeTypes,
					useUniqueIDs: false,
				};
				this.decodeGeometry(buffer, taskConfig).then(onLoad).catch(onError);
			},
			onProgress,
			onError
		);
	}
	/** @deprecated Kept for backward-compatibility with previous DRACOLoader versions. */
	decodeDracoFile(buffer, callback, attributeIDs, attributeTypes) {
		const taskConfig = {
			attributeIDs: attributeIDs || this.defaultAttributeIDs,
			attributeTypes: attributeTypes || this.defaultAttributeTypes,
			useUniqueIDs: !!attributeIDs,
		};
		this.decodeGeometry(buffer, taskConfig).then(callback);
	}
	decodeGeometry(buffer, taskConfig) {
		for (const attribute in taskConfig.attributeTypes) {
			const type = taskConfig.attributeTypes[attribute];
			if (type.BYTES_PER_ELEMENT !== void 0) {
				taskConfig.attributeTypes[attribute] = type.name;
			}
		}
		const taskKey = JSON.stringify(taskConfig);
		if (_taskCache.has(buffer)) {
			const cachedTask = _taskCache.get(buffer);
			if (cachedTask.key === taskKey) {
				return cachedTask.promise;
			} else if (buffer.byteLength === 0) {
				throw new Error('THREE.DRACOLoader: Unable to re-decode a buffer with different settings. Buffer has already been transferred.');
			}
		}
		let worker;
		const taskID = this.workerNextTaskID++;
		const taskCost = buffer.byteLength;
		const geometryPending = this._getWorker(taskID, taskCost)
			.then((_worker) => {
				worker = _worker;
				return new Promise((resolve, reject) => {
					worker._callbacks[taskID] = { resolve, reject };

					storeBuffer(taskID, buffer);

					worker.postMessage({ type: 'decode', id: taskID, taskConfig });
				});
			})
			.then((message) => this._createGeometry(message.geometry));
		geometryPending
			.catch(() => true)
			.then(() => {
				if (worker && taskID) {
					this._releaseTask(worker, taskID);
				}
			});
		_taskCache.set(buffer, {
			key: taskKey,
			promise: geometryPending,
		});
		return geometryPending;
	}
	_createGeometry(geometryData) {
		const geometry = new BufferGeometry();
		if (geometryData.index) {
			const geometryIndexData = geometryData.index.array;
			const id = geometryIndexData.id;
			const nativeBuffer = getBuffer(id);

			const buffer = getJSBuffer(nativeBuffer, geometryIndexData.type);

			removeBuffer(id);

			geometryData.index.array = buffer;

			geometry.setIndex(new BufferAttribute(geometryData.index.array, 1));
		}
		for (let i = 0; i < geometryData.attributes.length; i++) {
			const attribute = geometryData.attributes[i];
			const name = attribute.name;

			const arrayData = geometryData.attributes[i].array;

			const id = arrayData.id;

			const nativeBuffer = getBuffer(id);

			const array = getJSBuffer(nativeBuffer, arrayData.type);

			removeBuffer(id);

			const itemSize = attribute.itemSize;
			geometry.setAttribute(name, new BufferAttribute(array, itemSize));
		}
		return geometry;
	}
	_loadLibrary(url, responseType) {
		const loader = new FileLoader(this.manager);
		loader.setPath(this.decoderPath);
		loader.setResponseType(responseType);
		loader.setWithCredentials(this.withCredentials);
		return new Promise((resolve, reject) => {
			loader.load(url, resolve, void 0, reject);
		});
	}
	preload() {
		this._initDecoder();
		return this;
	}
	_initDecoder() {
		if (this.decoderPending) return this.decoderPending;
		const useJS = typeof WebAssembly !== 'object' || this.decoderConfig.type === 'js';
		const librariesPending = [];
		if (useJS) {
			librariesPending.push(this._loadLibrary('draco_decoder.js', 'text'));
		} else {
			librariesPending.push(this._loadLibrary('draco_wasm_wrapper.js', 'text'));
			librariesPending.push(this._loadLibrary('draco_decoder.wasm', 'arraybuffer'));
		}
		this.decoderPending = Promise.all(librariesPending).then((libraries) => {
			const jsContent = libraries[0];
			if (!useJS) {
				this.decoderConfig.wasmBinary = libraries[1];
			}
			const fn = DRACOWorker.toString();
			const body = ['/* draco decoder */', jsContent, '', '/* worker */', fn.substring(fn.indexOf('{') + 1, fn.lastIndexOf('}'))].join('\n');
			// this.workerSourceURL = URL.createObjectURL(new Blob([body]));

			this.workerSourceURL = URL.createObjectURL(new Blob([body], { type: 'text/javascript' }), { appendExt: true, ext: 'js' });
		});
		return this.decoderPending;
	}
	_getWorker(taskID, taskCost) {
		return this._initDecoder().then(() => {
			if (this.workerPool.length < this.workerLimit) {
				// const worker2 = new Worker(this.workerSourceURL);
				const worker2 = new Worker(URL.InternalAccessor.getPath(this.workerSourceURL));
				worker2._callbacks = {};
				worker2._taskCosts = {};
				worker2._taskLoad = 0;
				worker2.postMessage({ type: 'init', decoderConfig: this.decoderConfig });
				worker2.onmessage = function (e) {
					const message = e.data;
					switch (message.type) {
						case 'decode':
							worker2._callbacks[message.id].resolve(message);
							break;
						case 'error':
							worker2._callbacks[message.id].reject(message);
							break;
						default:
							console.error('THREE.DRACOLoader: Unexpected message, "' + message.type + '"');
					}
				};
				this.workerPool.push(worker2);
			} else {
				this.workerPool.sort(function (a, b) {
					return a._taskLoad > b._taskLoad ? -1 : 1;
				});
			}
			const worker = this.workerPool[this.workerPool.length - 1];
			worker._taskCosts[taskID] = taskCost;
			worker._taskLoad += taskCost;
			return worker;
		});
	}
	_releaseTask(worker, taskID) {
		worker._taskLoad -= worker._taskCosts[taskID];
		delete worker._callbacks[taskID];
		delete worker._taskCosts[taskID];
	}
	debug() {
		console.log(
			'Task load: ',
			this.workerPool.map((worker) => worker._taskLoad)
		);
	}
	dispose() {
		for (let i = 0; i < this.workerPool.length; ++i) {
			this.workerPool[i].terminate();
		}

		if (this.workerSourceURL !== '') {
			URL.revokeObjectURL(this.workerSourceURL);
		}

		this.workerPool.length = 0;
		return this;
	}
}
function DRACOWorker() {
	let decoderConfig;
	let decoderPending;

	function storeBuffer(key, buffer) {
		if (global.isAndroid) {
			org.nativescript.canvas.NSCCanvas.storeBuffer(key, buffer);
		}

		if (global.isIOS) {
			NSCCanvas.store.setObjectForKey(NSData.dataWithData(buffer), key);
		}
	}

	function removeBuffer(key) {
		if (global.isAndroid) {
			org.nativescript.canvas.NSCCanvas.removeBuffer(key);
		}

		if (global.isIOS) {
			NSCCanvas.store.removeObjectForKey(key);
		}
	}

	function getBuffer(key) {
		if (global.isAndroid) {
			const buffer = org.nativescript.canvas.NSCCanvas.getBuffer(key);
			if (buffer) {
				return ArrayBuffer.from(buffer);
			}

			return buffer;
		}

		if (global.isIOS) {
			const buffer = NSCCanvas.store.objectForKey(key);

			if (buffer) {
				return interop.bufferFromData(buffer);
			}

			return buffer;
		}
	}

	onmessage = function (e) {
		const message = e.data;
		switch (message.type) {
			case 'init':
				decoderConfig = message.decoderConfig;
				decoderPending = new Promise(function (resolve) {
					decoderConfig.onModuleLoaded = function (draco) {
						resolve({ draco });
					};
					DracoDecoderModule(decoderConfig);
				});
				break;
			case 'decode':
				const buffer = getBuffer(message.id); // message.buffer;
				const taskConfig = message.taskConfig;
				removeBuffer(message.id);

				var bufferType = function (buf) {
					let type = 'Unknown';
					if (buf instanceof Uint8Array) {
						type = 'Uint8Array';
					} else if (buf instanceof Uint8ClampedArray) {
						type = 'Uint8ClampedArray';
					} else if (buf instanceof Uint16Array) {
						type = 'Uint16Array';
					} else if (buf instanceof Uint32Array) {
						type = 'Uint32Array';
					} else if (buf instanceof BigUint64Array) {
						type = 'BigUint64Array';
					} else if (buf instanceof Int8Array) {
						type = 'Int8Array';
					} else if (buf instanceof Int16Array) {
						type = 'Int16Array';
					} else if (buf instanceof Int32Array) {
						type = 'Int32Array';
					} else if (buf instanceof BigInt64Array) {
						type = 'BigInt64Array';
					} else if (buf instanceof Float32Array) {
						type = 'Float32Array';
					} else if (buf instanceof Float64Array) {
						type = 'Float64Array';
					}
					return type;
				};

				decoderPending.then((module) => {
					const draco = module.draco;
					const decoder = new draco.Decoder();
					const decoderBuffer = new draco.DecoderBuffer();
					decoderBuffer.Init(new Int8Array(buffer), buffer.byteLength);
					try {
						const geometry = decodeGeometry(draco, decoder, decoderBuffer, taskConfig);
						// const buffers = geometry.attributes.map((attr) => attr.array.buffer);

						const buffers = geometry.attributes.map((attr, index) => {
							const array = attr.array;
							const buffer = attr.array.buffer;
							const id = `${message.id}_${index}`;
							storeBuffer(id, buffer);
							geometry.attributes[index].array = {
								type: bufferType(array),
								id,
							};
							return buffer;
						});

						// if (geometry.index)
						//   buffers.push(geometry.index.array.buffer);

						if (geometry.index) {
							const array = geometry.index.array;
							const buffer = geometry.index.array.buffer;
							const id = `${message.id}_geometry_index`;

							const index = {
								type: bufferType(array),
								id,
							};

							geometry.index.array = index;

							storeBuffer(id, buffer);

							buffers.push(index);
						}

						self.postMessage({ type: 'decode', id: message.id, geometry });
					} catch (error) {
						self.postMessage({ type: 'error', id: message.id, error: error.message });
					} finally {
						draco.destroy(decoderBuffer);
						draco.destroy(decoder);
					}
				});
				break;
		}
	};
	function decodeGeometry(draco, decoder, decoderBuffer, taskConfig) {
		const attributeIDs = taskConfig.attributeIDs;
		const attributeTypes = taskConfig.attributeTypes;
		let dracoGeometry;
		let decodingStatus;
		const geometryType = decoder.GetEncodedGeometryType(decoderBuffer);
		if (geometryType === draco.TRIANGULAR_MESH) {
			dracoGeometry = new draco.Mesh();
			decodingStatus = decoder.DecodeBufferToMesh(decoderBuffer, dracoGeometry);
		} else if (geometryType === draco.POINT_CLOUD) {
			dracoGeometry = new draco.PointCloud();
			decodingStatus = decoder.DecodeBufferToPointCloud(decoderBuffer, dracoGeometry);
		} else {
			throw new Error('THREE.DRACOLoader: Unexpected geometry type.');
		}
		if (!decodingStatus.ok() || dracoGeometry.ptr === 0) {
			throw new Error('THREE.DRACOLoader: Decoding failed: ' + decodingStatus.error_msg());
		}
		const geometry = { index: null, attributes: [] };
		for (const attributeName in attributeIDs) {
			const attributeType = self[attributeTypes[attributeName]];
			let attribute;
			let attributeID;
			if (taskConfig.useUniqueIDs) {
				attributeID = attributeIDs[attributeName];
				attribute = decoder.GetAttributeByUniqueId(dracoGeometry, attributeID);
			} else {
				attributeID = decoder.GetAttributeId(dracoGeometry, draco[attributeIDs[attributeName]]);
				if (attributeID === -1) continue;
				attribute = decoder.GetAttribute(dracoGeometry, attributeID);
			}
			geometry.attributes.push(decodeAttribute(draco, decoder, dracoGeometry, attributeName, attributeType, attribute));
		}
		if (geometryType === draco.TRIANGULAR_MESH) {
			geometry.index = decodeIndex(draco, decoder, dracoGeometry);
		}
		draco.destroy(dracoGeometry);
		return geometry;
	}
	function decodeIndex(draco, decoder, dracoGeometry) {
		const numFaces = dracoGeometry.num_faces();
		const numIndices = numFaces * 3;
		const byteLength = numIndices * 4;
		const ptr = draco._malloc(byteLength);
		decoder.GetTrianglesUInt32Array(dracoGeometry, byteLength, ptr);
		const index = new Uint32Array(draco.HEAPF32.buffer, ptr, numIndices).slice();
		draco._free(ptr);
		return { array: index, itemSize: 1 };
	}
	function decodeAttribute(draco, decoder, dracoGeometry, attributeName, attributeType, attribute) {
		const numComponents = attribute.num_components();
		const numPoints = dracoGeometry.num_points();
		const numValues = numPoints * numComponents;
		const byteLength = numValues * attributeType.BYTES_PER_ELEMENT;
		const dataType = getDracoDataType(draco, attributeType);
		const ptr = draco._malloc(byteLength);
		decoder.GetAttributeDataArrayForAllPoints(dracoGeometry, attribute, dataType, byteLength, ptr);
		const array = new attributeType(draco.HEAPF32.buffer, ptr, numValues).slice();
		draco._free(ptr);
		return {
			name: attributeName,
			array,
			itemSize: numComponents,
		};
	}
	function getDracoDataType(draco, attributeType) {
		switch (attributeType) {
			case Float32Array:
				return draco.DT_FLOAT32;
			case Int8Array:
				return draco.DT_INT8;
			case Int16Array:
				return draco.DT_INT16;
			case Int32Array:
				return draco.DT_INT32;
			case Uint8Array:
				return draco.DT_UINT8;
			case Uint16Array:
				return draco.DT_UINT16;
			case Uint32Array:
				return draco.DT_UINT32;
		}
	}
}
export { DRACOLoader };
//# sourceMappingURL=DRACOLoader.js.map
