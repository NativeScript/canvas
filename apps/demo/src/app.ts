require('@nativescript/canvas-polyfill');
import { Application, Utils } from '@nativescript/core';
declare const jp, GDPerformanceMonitor, org;
let monitor;
// uncomment for fps monitor
/*
Application.on('launch', (args) => {
	if (global.isAndroid) {
		jp.wasabeef.takt.Takt.stock(Utils.android.getApplicationContext()).seat(jp.wasabeef.takt.Seat.TOP_CENTER).color(-65536);
	} else {
		monitor = GDPerformanceMonitor.new();

		monitor.startMonitoringWithConfiguration((label) => {
			label.backgroundColor = UIColor.blackColor;
			label.textColor = UIColor.whiteColor;
			label.layer.borderColor = UIColor.redColor;
		});

		monitor.appVersionHidden = true;
		monitor.deviceVersionHidden = true;
	}
});
*/
Application.run({ moduleName: 'app-root' });
