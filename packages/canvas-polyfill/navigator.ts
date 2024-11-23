import { Device } from '@nativescript/core';
import { GPU } from '@nativescript/canvas';

const gpu = new GPU();

export class Navigator {
	appCodeName = 'Mozilla';
	appName = 'Netscape';
	product = 'NativeScript';
	userAgent = `Mozilla/5.0 (${this.platform === 'Android' ? 'Linux' : this.platform}; ${this.platform} ${Device.osVersion};) (KHTML, like Gecko) Mobile Safari/537.36 NativeScript/` + `${global.__runtimeVersion ?? '8.0.0'}`.replaceAll('"', '');
	vendor = '';
	vendorSub = '';
	appVersion = `5.0 (${this.platform === 'Android' ? 'Linux' : this.platform}; ${this.platform} ${Device.osVersion};) (KHTML, like Gecko) Mobile Safari/537.36 NativeScript/` + `${global.__runtimeVersion ?? '8.0.0'}`.replaceAll('"', '');
	maxTouchPoints = 5;
	standalone = true;
	language() {
		return Device.language;
	}
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
