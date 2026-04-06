if (!(global as any).performance || !(global as any).performance.now) {
	(global as any).performance = {
		timeOrigin: -1,
		timing: {
			connectEnd: -1,
			connectStart: -1,
			domComplete: -1,
			domContentLoadedEventEnd: -1,
			domContentLoadedEventStart: -1,
			domInteractive: -1,
			domLoading: -1,
			domainLookupEnd: -1,
			domainLookupStart: -1,
			fetchStart: -1,
			loadEventEnd: -1,
			loadEventStart: -1,
			navigationStart: -1,
			redirectEnd: -1,
			redirectStart: -1,
			requestStart: -1,
			responseEnd: -1,
			responseStart: -1,
			secureConnectionStart: -1,
			unloadEventEnd: -1,
			unloadEventStart: -1,
		},
		now() {
			if (global.isIOS) {
				return performance.now();
			}
			return __time();
		},
		toJSON() {
			return {
				timing: this.timing,
				navigation: this.navigation,
				timeOrigin: this.timeOrigin,
			};
		},
		navigation: {
			redirectCount: -1,
			type: -1,
		},
		memory: {
			jsHeapSizeLimit: -1,
			totalJSHeapSize: -1,
			usedJSHeapSize: -1,
		},
		_marks: new Map<string, number>(),
		_measures: new Map<string, { duration: number; startTime: number }>(),
		measure(name: string, startMark?: string, endMark?: string) {
			const start = startMark ? (this._marks.get(startMark) ?? 0) : 0;
			const end = endMark ? (this._marks.get(endMark) ?? this.now()) : this.now();
			this._measures.set(name, { duration: end - start, startTime: start });
		},
		mark(name: string) {
			this._marks.set(name, this.now());
		},
		clearMeasures(name?: string) {
			if (name) {
				this._measures.delete(name);
			} else {
				this._measures.clear();
			}
		},
		clearMarks(name?: string) {
			if (name) {
				this._marks.delete(name);
			} else {
				this._marks.clear();
			}
		},
		clearResourceTimings() {},
		setResourceTimingBufferSize() {},
		getEntriesByType(type: string) {
			if (type === 'measure') {
				return Array.from(this._measures.entries()).map(([name, v]) => ({ name, ...v, entryType: 'measure' }));
			}
			if (type === 'mark') {
				return Array.from(this._marks.entries()).map(([name, startTime]) => ({ name, startTime, duration: 0, entryType: 'mark' }));
			}
			return [];
		},
		getEntriesByName(name: string) {
			const entries = [];
			if (this._marks.has(name)) {
				entries.push({ name, startTime: this._marks.get(name), duration: 0, entryType: 'mark' });
			}
			if (this._measures.has(name)) {
				entries.push({ name, ...this._measures.get(name), entryType: 'measure' });
			}
			return entries;
		},
		getEntries() {
			const marks = Array.from(this._marks.entries()).map(([name, startTime]) => ({ name, startTime, duration: 0, entryType: 'mark' }));
			const measures = Array.from(this._measures.entries()).map(([name, v]) => ({ name, ...v, entryType: 'measure' }));
			return [...marks, ...measures];
		},
	};
}
