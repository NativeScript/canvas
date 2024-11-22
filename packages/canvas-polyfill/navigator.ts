import { Device } from '@nativescript/core';
import { GPU } from '@nativescript/canvas';

const gpu = new GPU();

export class Navigator {
	product = 'NativeScript';
	userAgent = 'NativeScript';
	vendor = '';
	vendorSub = '';
	appVersion = Device.osVersion;
	maxTouchPoints = 5;
	standalone = true;
	get gpu() {
		return gpu;
	}
	get platform() {
		if (global.isIOS) {
			if (Device.os === 'iPadOS') {
				return 'MacIntel';
			}
			return Device.deviceType === 'Tablet' ? 'iPad' : 'iPhone';
		}
		return Device.os;
	}
}
