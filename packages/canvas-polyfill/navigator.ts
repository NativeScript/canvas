import { Device } from '@nativescript/core';
import { GPU } from '@nativescript/canvas';

const gpu = new GPU();

export class Navigator {
	product = 'NativeScript';
	userAgent = 'chrome';
	vendor = '';
	vendorSub = '';
	platform = [];
	appVersion = Device.osVersion;
	maxTouchPoints = 5;
	standalone = true;
	get gpu() {
		return gpu;
	}
}
