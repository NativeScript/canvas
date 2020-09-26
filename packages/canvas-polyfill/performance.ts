import {isIOS} from "@nativescript/core";

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
            if (isIOS) {
                return CACurrentMediaTime() * 1000;
            }

            return java.lang.System.nanoTime() / 1000000;
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
        measure() {
            console.warn("window.performance.measure is not implemented");
        },
        mark() {
            console.warn("window.performance.mark is not implemented");
        },
        clearMeasures() {
            console.warn("window.performance.clearMeasures is not implemented");
        },
        clearMarks() {
            console.warn("window.performance.clearMarks is not implemented");
        },
        clearResourceTimings() {
            console.warn(
                "window.performance.clearResourceTimings is not implemented"
            );
        },
        setResourceTimingBufferSize() {
            console.warn(
                "window.performance.setResourceTimingBufferSize is not implemented"
            );
        },
        getEntriesByType() {
            console.warn(
                "window.performance.getEntriesByType is not implemented"
            );
            return [];
        },
        getEntriesByName() {
            console.warn(
                "window.performance.getEntriesByName is not implemented"
            );
            return [];
        },
        getEntries() {
            console.warn("window.performance.getEntries is not implemented");
        },
    };
}
