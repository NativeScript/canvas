require('@nativescript/core/globals');

if (__ANDROID__ && !(global as any).__canvasLoaded) {
	try {
		// load canvas lib if polyfill is called before
		java.lang.System.loadLibrary('canvasnative');
		(global as any).__canvasLoaded = true;
	} catch (e) {}
}
import { TNSXMLHttpRequest, FileReader, Blob } from './async/async';
import { Element } from './DOM/Element';
import { Document } from './DOM/Document';
import './window';
import './resize';
import './process';
import './localStorage';
import { TextDecoder, TextEncoder, ImageBitmap } from '@nativescript/canvas';
import './workerPatch';
import { FontFaceSet } from '@nativescript/canvas';

if (!global.Document) {
	Object.defineProperty(global, 'Document', {
		value: Document,
		configurable: true,
		writable: true,
	});
}

(global as any).document = (global as any).window.document = (global as any).document || new Document();

if (!global.document.fonts) {
	(global as any).document.fonts = global.fonts || new FontFaceSet();
}

if (!(global as any).createImageBitmap) {
	(global as any).window.createImageBitmap = (global as any).createImageBitmap = (...args) => {
		const image = args[0];
		const sx_or_options = args[1];
		const sy = args[2];
		const sw = args[3];
		const sh = args[4];
		if ((typeof sw === 'number' && sw === 0) || (typeof sh === 'number' && sh === 0)) {
			return Promise.reject(new RangeError(`Failed to execute 'createImageBitmap' : The crop rect width is 0`));
		}

		if (args.length === 1) {
			//@ts-ignore
			return ImageBitmap.createFrom(image);
		} else if (args.length === 2) {
			//@ts-ignore
			return ImageBitmap.createFrom(image, sx_or_options);
		} else if (args.length === 5) {
			//@ts-ignore
			return ImageBitmap.createFromRect(image, sx_or_options, sy, sw, sh);
		} else if (args.length === 6) {
			//@ts-ignore
			return ImageBitmap.createFromRect(image, sx_or_options, sy, sw, sh, args[5]);
		}
	};
}

if (typeof (global as any).Intl === 'undefined') {
	(global as any).Intl = (global as any).window?.Intl ?? {};
} else if (typeof (global as any).window?.Intl === 'undefined') {
	(global as any).window.Intl = (global as any).Intl;
}

// Linux V8 ships Intl.Segmenter without ICU data and segment() crashes, so hide it.
{
	const intl = (global as any).Intl;
	if (typeof intl?.Segmenter === 'function') {
		let usable = false;
		try {
			usable = intl.Segmenter.supportedLocalesOf(['en']).length > 0;
		} catch (e) {}
		if (!usable) {
			delete intl.Segmenter;
		}
	}
}

// Without ICU data toLocaleString throws; fall back to en-US formatting.
{
	let icu = true;
	try {
		(1234.5).toLocaleString();
	} catch (e) {
		icu = false;
	}
	if (!icu) {
		Number.prototype.toLocaleString = function (this: number, _locales?: unknown, options?: Intl.NumberFormatOptions) {
			let value = Number(this);
			if (!Number.isFinite(value)) {
				return Number.isNaN(value) ? 'NaN' : value > 0 ? '∞' : '-∞';
			}
			const percent = options?.style === 'percent';
			if (percent) {
				value *= 100;
			}
			const min = options?.minimumFractionDigits ?? 0;
			const max = Math.max(min, options?.maximumFractionDigits ?? (percent ? 0 : 3));
			let [whole, fraction = ''] = Math.abs(value).toFixed(max).split('.');
			fraction = fraction.replace(/0+$/, '').padEnd(min, '0');
			if (options?.useGrouping !== false) {
				whole = whole.replace(/\B(?=(\d{3})+(?!\d))/g, ',');
			}
			return `${value < 0 ? '-' : ''}${whole}${fraction ? `.${fraction}` : ''}${percent ? '%' : ''}`;
		};
	}
}

import { MutationObserver } from './MutationObserver';

if (!global.MutationObserver) {
	Object.defineProperty(global, 'MutationObserver', {
		value: MutationObserver,
		configurable: true,
		writable: true,
	});
}

function defineGlobalIfAbsent(name: string, value: any, overwrite = false) {
	const g: any = global as any;
	if (typeof g[name] === 'undefined' || overwrite) {
		try {
			Object.defineProperty(g, name, {
				value,
				configurable: true,
				writable: true,
			});
		} catch (e) {
			g[name] = value;
		}
	}

	try {
		if (g.window && (typeof g.window[name] === 'undefined' || overwrite)) {
			Object.defineProperty(g.window, name, {
				value,
				configurable: true,
				writable: true,
			});
		}
	} catch (e) {
		if (g.window) g.window[name] = g.window[name] || value;
	}
}

defineGlobalIfAbsent('Element', Element);
defineGlobalIfAbsent('XMLHttpRequest', TNSXMLHttpRequest, true);
defineGlobalIfAbsent('Blob', Blob, true);
defineGlobalIfAbsent('FileReader', FileReader, true);
defineGlobalIfAbsent('TextDecoder', TextDecoder, true);
defineGlobalIfAbsent('TextEncoder', TextEncoder, true);
defineGlobalIfAbsent('TextEncoder', TextEncoder, true);

import './urlBlobPatch';
import './localFetch';

try {
	// @ts-ignore
	const canvasMedia = require('@nativescript/canvas-media');
	defineGlobalIfAbsent('VideoFrame', canvasMedia.VideoFrame);
	defineGlobalIfAbsent('VideoColorSpace', canvasMedia.VideoColorSpace);
} catch (_e) {
	defineGlobalIfAbsent(
		'VideoFrame',
		class VideoFrame {
			readonly displayWidth: number = 0;
			readonly displayHeight: number = 0;
			readonly codedWidth: number = 0;
			readonly codedHeight: number = 0;
			close() {}
		},
	);
}

try {
	// @ts-ignore
	const canvasMedia = require('@nativescript/audio-context');
	defineGlobalIfAbsent('AudioParam', canvasMedia.AudioParam, true);
	defineGlobalIfAbsent('AudioNode', canvasMedia.AudioNode, true);
	defineGlobalIfAbsent('AudioBuffer', canvasMedia.AudioBuffer, true);
	defineGlobalIfAbsent('GainNode', canvasMedia.GainNode, true);
	defineGlobalIfAbsent('BiquadFilterNode', canvasMedia.BiquadFilterNode, true);
	defineGlobalIfAbsent('PannerNode', canvasMedia.PannerNode, true);
	defineGlobalIfAbsent('AudioDestinationNode', canvasMedia.AudioDestinationNode, true);
	defineGlobalIfAbsent('AudioScheduledSourceNode', canvasMedia.AudioScheduledSourceNode, true);
	defineGlobalIfAbsent('OscillatorNode', canvasMedia.OscillatorNode, true);
	defineGlobalIfAbsent('StereoPannerNode', canvasMedia.StereoPannerNode, true);
	defineGlobalIfAbsent('AudioContext', canvasMedia.AudioContext, true);
	defineGlobalIfAbsent('webkitAudioContext', canvasMedia.AudioContext, true);
	defineGlobalIfAbsent('DelayNode', canvasMedia.DelayNode, true);
	defineGlobalIfAbsent('ConstantSourceNode', canvasMedia.ConstantSourceNode, true);
	defineGlobalIfAbsent('AnalyserNode', canvasMedia.AnalyserNode, true);
	defineGlobalIfAbsent('WaveShaperNode', canvasMedia.WaveShaperNode, true);
	defineGlobalIfAbsent('IIRFilterNode', canvasMedia.IIRFilterNode, true);
	defineGlobalIfAbsent('ConvolverNode', canvasMedia.ConvolverNode, true);
	defineGlobalIfAbsent('PeriodicWave', canvasMedia.PeriodicWave, true);
	defineGlobalIfAbsent('AudioBufferSourceNode', canvasMedia.AudioBufferSourceNode, true);
	defineGlobalIfAbsent('OfflineAudioContext', canvasMedia.OfflineAudioContext, true);
	defineGlobalIfAbsent('AudioListener', canvasMedia.AudioListener, true);
} catch (_e) {}
