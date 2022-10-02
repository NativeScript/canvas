import { BufferAttribute, BufferGeometry, FileLoader, Loader } from 'three';

// var DRACOLoader = function (manager) {
// 	Loader.call(this, manager);

// 	this.decoderPath = '';
// 	this.decoderConfig = {};
// 	this.decoderBinary = null;
// 	this.decoderPending = null;

// 	this.workerLimit = 5;
// 	this.workerPool = [];
// 	this.workerNextTaskID = 1;
// 	this.workerSourceURL = '';

// 	this.defaultAttributeIDs = {
// 		position: 'POSITION',
// 		normal: 'NORMAL',
// 		color: 'COLOR',
// 		uv: 'TEX_COORD',
// 	};
// 	this.defaultAttributeTypes = {
// 		position: 'Float32Array',
// 		normal: 'Float32Array',
// 		color: 'Float32Array',
// 		uv: 'Float32Array',
// 	};
// };


class DRACOLoader extends Loader {
	constructor(manager){
		super(manager);

	this.decoderPath = '';
	this.decoderConfig = {};
	this.decoderBinary = null;
	this.decoderPending = null;

	this.workerLimit = 1;
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

	/** @deprecated */
	setVerbosity() {
		console.warn('THREE.DRACOLoader: The .setVerbosity() method has been removed.');
	}

	/** @deprecated */
	setDrawMode() {
		console.warn('THREE.DRACOLoader: The .setDrawMode() method has been removed.');
	}

	/** @deprecated */
	setSkipDequantization() {
		console.warn('THREE.DRACOLoader: The .setSkipDequantization() method has been removed.');
	}

	load(url, onLoad, onProgress, onError) {
		var loader = new FileLoader(this.manager);

		loader.setPath(this.path);
		loader.setResponseType('arraybuffer');
		loader.setRequestHeader(this.requestHeader);

		if (this.crossOrigin === 'use-credentials') {
			loader.setWithCredentials(true);
		}

		loader.load(
			url,
			(buffer) => {
				var taskConfig = {
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
		var taskConfig = {
			attributeIDs: attributeIDs || this.defaultAttributeIDs,
			attributeTypes: attributeTypes || this.defaultAttributeTypes,
			useUniqueIDs: !!attributeIDs,
		};

		this.decodeGeometry(buffer, taskConfig).then(callback);
	}

	decodeGeometry(buffer, taskConfig) {
		// TODO: For backward-compatibility, support 'attributeTypes' objects containing
		// references (rather than names) to typed array constructors. These must be
		// serialized before sending them to the worker.
		for (var attribute in taskConfig.attributeTypes) {
			var type = taskConfig.attributeTypes[attribute];

			if (type.BYTES_PER_ELEMENT !== undefined) {
				taskConfig.attributeTypes[attribute] = type.name;
			}
		}

		//

		var taskKey = JSON.stringify(taskConfig);

		// Check for an existing task using this buffer. A transferred buffer cannot be transferred
		// again from this thread.
		if (DRACOLoader.taskCache.has(buffer)) {
			var cachedTask = DRACOLoader.taskCache.get(buffer);

			if (cachedTask.key === taskKey) {
				return cachedTask.promise;
			} else if (buffer.byteLength === 0) {
				// Technically, it would be possible to wait for the previous task to complete,
				// transfer the buffer back, and decode again with the second configuration. That
				// is complex, and I don't know of any reason to decode a Draco buffer twice in
				// different ways, so this is left unimplemented.
				throw new Error('THREE.DRACOLoader: Unable to re-decode a buffer with different ' + 'settings. Buffer has already been transferred.');
			}
		}

		//

		var worker;
		var taskID = this.workerNextTaskID++;
		var taskCost = buffer.byteLength;

		// Obtain a worker and assign a task, and construct a geometry instance
		// when the task completes.
		var geometryPending = this._getWorker(taskID, taskCost)
			.catch((e) => {
				console.error('geometryPending', e);
			})
			.then((_worker) => {
				worker = _worker;

				return new Promise((resolve, reject) => {
					try {
						worker._callbacks[taskID] = { resolve, reject };
						worker.postMessage({
							type: 'decode',
							id: taskID,
							taskConfig,
							buffer: JSON.stringify(new Uint8Array(buffer)),
						});
					} catch (e) {
						console.erro('geometryPending postMessage', e);
					}

					// this.debug();
				});
			})
			.then((message) => {
				const data = message.geometry;
				data.indexType = message.indexType;
				data.attributeTypes = message.attributeTypes;
				return this._createGeometry(data);
			});

		// Remove task from the task list.
		// Note: replaced '.finally()' with '.catch().then()' block - iOS 11 support (#19416)
		geometryPending
			.catch((e) => {
				return true;
			})
			.then(() => {
				if (worker && taskID) {
					this._releaseTask(worker, taskID);

					// this.debug();
				}
			});

		// Cache the task result.
		DRACOLoader.taskCache.set(buffer, {
			key: taskKey,
			promise: geometryPending,
		});

		return geometryPending;
	}

	_createGeometry(geometryData) {
		var geometry = new BufferGeometry();
			var getBuffer = function (buf, type) {
				if (type === 'Uint8Array') {
					return new Uint8Array(buf);
				} else if (type === 'Uint16Array') {
					return new Uint16Array(buf);
				} else if (type === 'Uint32Array') {
					return new Uint32Array(buf);
				} else if (type === 'Int8Array') {
					return new Int8Array(buf);
				} else if (type === 'Int16Array') {
					return new Int16Array(buf);
				} else if (type === 'Int32Array') {
					return new Int32Array(buf);
				} else if (type === 'Float32Array') {
					return new Float32Array(buf);
				} else {
					return buf;
				}
			};
			if (geometryData.index) {
				geometry.setIndex(new BufferAttribute(getBuffer(geometryData.index.array, geometryData.indexType), 1));
			}

			for (var i = 0; i < geometryData.attributes.length; i++) {
				var attribute = geometryData.attributes[i];
				var name = attribute.name;
				var array = getBuffer(attribute.array, geometryData.attributeTypes[i]);
				var itemSize = attribute.itemSize;

				geometry.setAttribute(name, new BufferAttribute(array, itemSize));
			}
		return geometry;
	}

	_loadLibrary(url, responseType) {
		var loader = new FileLoader(this.manager);
		loader.setPath(this.decoderPath);
		loader.setResponseType(responseType);

		return new Promise((resolve, reject) => {
			loader.load(url, resolve, undefined, reject);
		});
	}

	preload() {
		this._initDecoder();

		return this;
	}

	_initDecoder() {
		if (this.decoderPending) return this.decoderPending;

		var useJS = this.decoderConfig.type === 'js';
		var librariesPending = [];

		if (useJS) {
			librariesPending.push(this._loadLibrary('draco_decoder.js', 'text'));
		} else {
			librariesPending.push(this._loadLibrary('draco_wasm_wrapper.js', 'text'));
			librariesPending.push(this._loadLibrary('draco_decoder.wasm', 'arraybuffer'));
		}

		this.decoderPending = Promise.all(librariesPending).then((libraries) => {
			var jsContent = libraries[0];

			if (!useJS) {
				this.decoderConfig.wasmBinary = libraries[1];
			}

			var fn = DRACOLoader.DRACOWorker.toString();

			var body = ['/* draco decoder */', jsContent, '', '/* worker */', fn.substring(fn.indexOf('{') + 1, fn.lastIndexOf('}'))].join('\n');

			try {
				this.workerSourceURL = URL.createObjectURL(new Blob([body], { type: 'text/javascript' }), {appendExt: true, ext: 'js'});
				//require(this.workerSourceURL);
			} catch (e) {
				console.log('require fail', e);
			}
		});

		return this.decoderPending;
	}

	_getWorker(taskID, taskCost) {
		return this._initDecoder().then(() => {
			if (this.workerPool.length < this.workerLimit) {
				try {
					var worker = new Worker(URL.InternalAccessor.getPath(this.workerSourceURL));

					worker._callbacks = {};
					worker._taskCosts = {};
					worker._taskLoad = 0;

					worker.postMessage({ type: 'init', decoderConfig: this.decoderConfig });
					worker.onmessage = function (e) {
						var message = e.data;
						switch (message.type) {
							case 'decode':
								worker._callbacks[message.id].resolve(message);
								break;

							case 'error':
								worker._callbacks[message.id].reject(message);
								break;

							default:
								console.error('THREE.DRACOLoader: Unexpected message, "' + message.type + '"');
						}
					};

					this.workerPool.push(worker);
				} catch (e) {
					console.log('creating worker', e);
				}
			} else {
				this.workerPool.sort(function (a, b) {
					return a._taskLoad > b._taskLoad ? -1 : 1;
				});
			}

			var worker = this.workerPool[this.workerPool.length - 1];
			worker._taskCosts[taskID] = taskCost;
			worker._taskLoad += taskCost;
			return worker;
		});
	}

	_releaseTask(worker, taskID) {
		try {
			worker._taskLoad -= worker._taskCosts[taskID];
			delete worker._callbacks[taskID];
			delete worker._taskCosts[taskID];
		} catch (e) {
			console.log('_releaseTask', e);
		}
	}

	debug() {
		console.log(
			'Task load: ',
			this.workerPool.map((worker) => worker._taskLoad)
		);
	}

	dispose() {
		for (var i = 0; i < this.workerPool.length; ++i) {
			this.workerPool[i].terminate();
		}

		this.workerPool.length = 0;

		return this;
	}
};



/* WEB WORKER */

DRACOLoader.DRACOWorker = function () {
	var decoderConfig;
	var decoderPending;

	onmessage = function (e) {
		var message = e.data;

		switch (message.type) {
			case 'init':
				decoderConfig = message.decoderConfig;
				decoderPending = new Promise(function (resolve /*, reject*/) {
					decoderConfig.onModuleLoaded = function (draco) {
						// Module is Promise-like. Wrap before resolving to avoid loop.
						resolve({ draco: draco });
					};
					DracoDecoderModule(decoderConfig); // eslint-disable-line no-undef
				});
				break;

			case 'decode':
				const values = Object.values(JSON.parse(message.buffer));
				buffer = Int8Array.from(values);
				var taskConfig = message.taskConfig;
				decoderPending.then((module) => {
					try {
						var draco = module.draco;
						var decoder = new draco.Decoder();
						var decoderBuffer = new draco.DecoderBuffer();
						decoderBuffer.Init(buffer, buffer.byteLength);

						var geometry = decodeGeometry(draco, decoder, decoderBuffer, taskConfig);
						var bufferType = function (buf) {
							let type = 'Unknown';
							if (buf instanceof Uint8Array) {
								type = 'Uint8Array';
							} else if (buf instanceof Uint16Array) {
								type = 'Uint16Array';
							} else if (buf instanceof Uint32Array) {
								type = 'Uint32Array';
							} else if (buf instanceof Int8Array) {
								type = 'Int8Array';
							} else if (buf instanceof Int16Array) {
								type = 'Int16Array';
							} else if (buf instanceof Int32Array) {
								type = 'Int32Array';
							} else if (buf instanceof Float32Array) {
								type = 'Float32Array';
							}
							return type;
						};
						var attributeTypes = [];
						geometry.attributes = geometry.attributes.map((attr) => {
							var type = bufferType(attr.array);
							attr.array = Array.from(attr.array);
							attributeTypes.push(type);
							return attr;
						});
						var indexType;
						if (geometry.index) {
							indexType = bufferType(geometry.index.array);
							geometry.index.array = Array.from(geometry.index.array);
						}
						self.postMessage({ type: 'decode', id: message.id, geometry, indexType, attributeTypes });
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
		var attributeIDs = taskConfig.attributeIDs;
		var attributeTypes = taskConfig.attributeTypes;

		var dracoGeometry;
		var decodingStatus;

		var geometryType = decoder.GetEncodedGeometryType(decoderBuffer);

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

		var geometry = { index: null, attributes: [] };

		// Gather all vertex attributes.
		for (var attributeName in attributeIDs) {
			var attributeType = self[attributeTypes[attributeName]];

			var attribute;
			var attributeID;

			// A Draco file may be created with default vertex attributes, whose attribute IDs
			// are mapped 1:1 from their semantic name (POSITION, NORMAL, ...). Alternatively,
			// a Draco file may contain a custom set of attributes, identified by known unique
			// IDs. glTF files always do the latter, and `.drc` files typically do the former.
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

		// Add index.
		if (geometryType === draco.TRIANGULAR_MESH) {
			// Generate mesh faces.
			var numFaces = dracoGeometry.num_faces();
			var numIndices = numFaces * 3;
			var index = new Uint32Array(numIndices);
			var indexArray = new draco.DracoInt32Array();

			for (var i = 0; i < numFaces; ++i) {
				decoder.GetFaceFromMesh(dracoGeometry, i, indexArray);

				for (var j = 0; j < 3; ++j) {
					index[i * 3 + j] = indexArray.GetValue(j);
				}
			}

			geometry.index = { array: index, itemSize: 1 };

			draco.destroy(indexArray);
		}

		draco.destroy(dracoGeometry);

		return geometry;
	}

	function decodeAttribute(draco, decoder, dracoGeometry, attributeName, attributeType, attribute) {
		var numComponents = attribute.num_components();
		var numPoints = dracoGeometry.num_points();
		var numValues = numPoints * numComponents;
		var dracoArray;

		var array;

		switch (attributeType) {
			case Float32Array:
				dracoArray = new draco.DracoFloat32Array();
				decoder.GetAttributeFloatForAllPoints(dracoGeometry, attribute, dracoArray);
				array = new Float32Array(numValues);
				break;

			case Int8Array:
				dracoArray = new draco.DracoInt8Array();
				decoder.GetAttributeInt8ForAllPoints(dracoGeometry, attribute, dracoArray);
				array = new Int8Array(numValues);
				break;

			case Int16Array:
				dracoArray = new draco.DracoInt16Array();
				decoder.GetAttributeInt16ForAllPoints(dracoGeometry, attribute, dracoArray);
				array = new Int16Array(numValues);
				break;

			case Int32Array:
				dracoArray = new draco.DracoInt32Array();
				decoder.GetAttributeInt32ForAllPoints(dracoGeometry, attribute, dracoArray);
				array = new Int32Array(numValues);
				break;

			case Uint8Array:
				dracoArray = new draco.DracoUInt8Array();
				decoder.GetAttributeUInt8ForAllPoints(dracoGeometry, attribute, dracoArray);
				array = new Uint8Array(numValues);
				break;

			case Uint16Array:
				dracoArray = new draco.DracoUInt16Array();
				decoder.GetAttributeUInt16ForAllPoints(dracoGeometry, attribute, dracoArray);
				array = new Uint16Array(numValues);
				break;

			case Uint32Array:
				dracoArray = new draco.DracoUInt32Array();
				decoder.GetAttributeUInt32ForAllPoints(dracoGeometry, attribute, dracoArray);
				array = new Uint32Array(numValues);
				break;

			default:
				throw new Error('THREE.DRACOLoader: Unexpected attribute type.');
		}

		for (var i = 0; i < numValues; i++) {
			array[i] = dracoArray.GetValue(i);
		}

		draco.destroy(dracoArray);

		return {
			name: attributeName,
			array: array,
			itemSize: numComponents,
		};
	}
};

DRACOLoader.taskCache = new WeakMap();

/** Deprecated static methods */

/** @deprecated */
DRACOLoader.setDecoderPath = function () {
	console.warn('THREE.DRACOLoader: The .setDecoderPath() method has been removed. Use instance methods.');
};

/** @deprecated */
DRACOLoader.setDecoderConfig = function () {
	console.warn('THREE.DRACOLoader: The .setDecoderConfig() method has been removed. Use instance methods.');
};

/** @deprecated */
DRACOLoader.releaseDecoderModule = function () {
	console.warn('THREE.DRACOLoader: The .releaseDecoderModule() method has been removed. Use instance methods.');
};

/** @deprecated */
DRACOLoader.getDecoderModule = function () {
	console.warn('THREE.DRACOLoader: The .getDecoderModule() method has been removed. Use instance methods.');
};

export { DRACOLoader };
