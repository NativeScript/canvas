import { Utils, Device } from '@nativescript/core';
declare const sysctlbyname;
const appleModelsPPIManifest = new Map(
	Object.entries({
		/// iPhone
		// iPhone 1
		'iPhone1,1': 163,

		// iPhone 3G
		'iPhone1,2': 163,

		// iPhone 3GS
		'iPhone2,1': 163,

		// iPhone 4
		'iPhone3,1': 326,
		'iPhone3,2': 326,
		'iPhone3,3': 326,

		// iPhone 4S
		'iPhone4,1': 326,

		// iPhone 5
		'iPhone5,1': 326,
		'iPhone5,2': 326,

		// iPhone 5c
		'iPhone5,3': 326,
		'iPhone5,4': 326,

		// iPhone 5s
		'iPhone6,1': 326,
		'iPhone6,2': 326,

		// iPhone 6
		'iPhone7,2': 326,

		// iPhone 6 Plus
		'iPhone7,1': 401,

		// iPhone 6s
		'iPhone8,1': 326,

		// iPhone 6s Plus
		'iPhone8,2': 401,

		// iPhone SE
		'iPhone8,4': 326,

		// iPhone 7
		'iPhone9,1': 326,
		'iPhone9,3': 326,

		// iPhone 7 Plus
		'iPhone9,2': 401,
		'iPhone9,4': 401,

		// iPhone 8
		'iPhone10,1': 326,
		'iPhone10,4': 326,

		// iPhone 8 Plus
		'iPhone10,2': 401,
		'iPhone10,5': 401,

		// iPhone X
		'iPhone10,3': 458,
		'iPhone10,6': 458,

		// iPhone XR
		'iPhone11,8': 326,

		// iPhone XS
		'iPhone11,2': 458,

		// iPhone XS Max
		'iPhone11,4': 458,
		'iPhone11,6': 458,

		// iPhone 11
		'iPhone12,1': 326,

		// iPhone 11 Pro
		'iPhone12,3': 458,

		// iPhone 11 Pro Max
		'iPhone12,5': 458,

		// iPhone SE 2
		'iPhone12,8': 326,

		// iPhone 12 mini
		'iPhone13,1': 476,

		// iPhone 12
		'iPhone13,2': 460,

		// iPhone 12 Pro
		'iPhone13,3': 460,

		// iPhone 12 Pro Max
		'iPhone13,4': 458,

		// iPhone 13 mini
		'iPhone14,4': 476,

		// iPhone 13
		'iPhone14,5': 460,

		// iPhone 13 Pro
		'iPhone14,2': 460,

		// iPhone 13 Pro Max
		'iPhone14,3': 458,

		// iPhone SE 3rd Gen
		'iPhone14,6': 326,

		// iPhone 14
		'iPhone14,7': 460,

		// iPhone 14 Plus
		'iPhone14,8': 458,

		// iPhone 14 Pro
		'iPhone15,2': 460,

		// iPhone 14 Pro Max
		'iPhone15,3': 460,

		// iPhone 15
		'iPhone15,4': 460,

		// iPhone 15 Plus
		'iPhone15,5': 460,

		// iPhone 15 Pro
		'iPhone16,1': 460,

		// iPhone 15 Pro Max
		'iPhone16,2': 460,

		/// iPad
		// iPad 1
		'iPad1,1': 132,

		// iPad 2
		'iPad2,1': 132,
		'iPad2,2': 132,
		'iPad2,3': 132,
		'iPad2,4': 132,

		// iPad mini
		'iPad2,5': 163,
		'iPad2,6': 163,
		'iPad2,7': 163,

		// iPad 3
		'iPad3,1': 264,
		'iPad3,2': 264,
		'iPad3,3': 264,

		// iPad 4
		'iPad3,4': 264,
		'iPad3,5': 264,
		'iPad3,6': 264,

		// iPad Air
		'iPad4,1': 264,
		'iPad4,2': 264,
		'iPad4,3': 264,

		// iPad mini 2
		'iPad4,4': 326,
		'iPad4,5': 326,
		'iPad4,6': 326,

		// iPad mini 3
		'iPad4,7': 326,
		'iPad4,8': 326,
		'iPad4,9': 326,

		// iPad mini 4
		'iPad5,1': 326,
		'iPad5,2': 326,

		// iPad Air 2
		'iPad5,3': 264,
		'iPad5,4': 264,

		// iPad Pro 12.9-inch
		'iPad6,7': 264,
		'iPad6,8': 264,

		// iPad Pro 9.7-inch
		'iPad6,3': 264,
		'iPad6,4': 264,

		// iPad 5th Gen, 2017
		'iPad6,11': 264,
		'iPad6,12': 264,

		// iPad Pro 12.9-inch, 2017
		'iPad7,1': 264,
		'iPad7,2': 264,

		// iPad Pro 10.5-inch, 2017
		'iPad7,3': 264,
		'iPad7,4': 264,

		// iPad 6th Gen, 2018
		'iPad7,5': 264,
		'iPad7,6': 264,

		// iPad 7th Gen, 2019
		'iPad7,11': 264,
		'iPad7,12': 264,

		// iPad Pro 11-inch, 2018
		'iPad8,1': 264,
		'iPad8,3': 264,

		// iPad Pro 11-inch 1TB, 2018
		'iPad8,2': 264,
		'iPad8,4': 264,

		// iPad Pro 3rd Gen 12.9-inch, 2018
		'iPad8,5': 264,
		'iPad8,7': 264,

		// iPad Pro 3rd Gen 12.9-inch 1TB, 2018
		'iPad8,6': 264,
		'iPad8,8': 264,

		// iPad Pro 2nd Gen 11-inch, 2020
		'iPad8,9': 264,
		'iPad8,10': 264,

		// iPad Pro 4th Gen 12.9-inch, 2020
		'iPad8,11': 264,
		'iPad8,12': 264,

		// iPad mini 5
		'iPad11,1': 326,
		'iPad11,2': 326,

		// iPad Air 3
		'iPad11,3': 264,
		'iPad11,4': 264,

		// iPad 8th Gen, 2020
		'iPad11,6': 264,
		'iPad11,7': 264,

		// iPad 9th Gen, 2021
		'iPad12,1': 264,
		'iPad12,2': 264,

		// iPad Air 4
		'iPad13,1': 264,
		'iPad13,2': 264,

		// iPad Pro 3rd Gen 11-inch, 2021
		'iPad13,4': 264,
		'iPad13,5': 264,
		'iPad13,6': 264,
		'iPad13,7': 264,

		// iPad Pro 5th Gen 12.9-inch, 2021
		'iPad13,8': 264,
		'iPad13,9': 264,
		'iPad13,10': 264,
		'iPad13,11': 264,

		// iPad Air 5, 2022
		'iPad13,16': 264,
		'iPad13,17': 264,

		// iPad mini 6
		'iPad14,1': 326,
		'iPad14,2': 326,

		// iPad 10th Gen, 2022
		'iPad18,18': 264,
		'iPad13,19': 264,

		// iPad Pro 4th Gen 11-inch, 2022
		'iPad14,3': 264,
		'iPad14,4': 264,

		// iPad Pro 6th Gen 12.9-inch, 2022
		'iPad14,5': 264,
		'iPad14,6': 264,

		/// iPod
		// iPod Touch 1st Gen
		'iPod1,1': 163,

		// iPod Touch 2nd Gen
		'iPod2,1': 163,

		// iPod Touch 3rd Gen
		'iPod3,1': 163,

		// iPod Touch 4th Gen
		'iPod4,1': 326,

		// iPod Touch 5th Gen
		'iPod5,1': 326,

		// iPod Touch 6th Gen
		'iPod7,1': 326,

		// iPod Touch 7th Gen
		'iPod9,1': 326,

		/// Apple Watch
		// Apple Watch 1st Gen
		'Watch1,1': 326,
		'Watch1,2': 326,

		// Apple Watch Series 1
		'Watch2,6': 326,
		'Watch2,7': 326,

		// Apple Watch Series 2
		'Watch2,3': 326,
		'Watch2,4': 326,

		// Apple Watch Series 3
		'Watch3,1': 326,
		'Watch3,2': 326,
		'Watch3,3': 326,
		'Watch3,4': 326,

		// Apple Watch Series 4
		'Watch4,1': 326,
		'Watch4,2': 326,
		'Watch4,3': 326,
		'Watch4,4': 326,

		// Apple Watch Series 5
		'Watch5,1': 326,
		'Watch5,2': 326,
		'Watch5,3': 326,
		'Watch5,4': 326,

		// Apple Watch SE
		'Watch5,9': 326,
		'Watch5,10': 326,
		'Watch5,11': 326,
		'Watch5,12': 326,

		// Apple Watch Series 6
		'Watch6,1': 326,
		'Watch6,2': 326,
		'Watch6,3': 326,
		'Watch6,4': 326,

		// Apple Watch Series 7
		'Watch6,6': 326,
		'Watch6,7': 326,
		'Watch6,8': 326,
		'Watch6,9': 326,

		// SE 2nd gen
		'Watch6,10': 326,
		'Watch6,11': 326,
		'Watch6,12': 326,
		'Watch6,13': 326,

		// Apple Watch Series 8
		'Watch6,14': 326,
		'Watch6,15': 326,
		'Watch6,16': 326,
		'Watch6,17': 326,

		// Apple Watch Ultra
		'Watch6,18': 338,

		// Apple Watch Series 9
		'Watch7,1': 326,
		'Watch7,2': 326,
		'Watch7,3': 326,
		'Watch7,4': 326,

		// Apple Watch Ultra 2
		'Watch7,5': 338,
	})
);

let ppi: number;
export function getPixelsPerInchForCurrentDevice() {
	if (global.isAndroid) {
		if (ppi === undefined) {
			const ctx = Utils.android.getApplicationContext() as android.content.Context;
			const metrics = new android.util.DisplayMetrics();
			ctx.getSystemService(android.content.Context.WINDOW_SERVICE).getDefaultDisplay().getRealMetrics(metrics);
			ppi = metrics.densityDpi;
		}
	}
	if (global.isIOS) {
		if (ppi === undefined) {
			const size = new interop.Reference<number>(0);
			sysctlbyname('hw.machine', null, size, null, 0);
			const machine = interop.alloc(size.value);
			sysctlbyname('hw.machine', machine, size, null, 0);
			const platform = NSString.stringWithUTF8String(machine);
			switch (platform.toString()) {
				case '86':
				case 'x86_64':
				case 'arm64':
					ppi = 132;
					break;
			}

			if (ppi) {
				return ppi;
			}

			ppi = appleModelsPPIManifest.get(platform as any);

			if (ppi === undefined) {
				if (platform.hasPrefix('iPhone')) {
					ppi = 401;
				} else if (platform.hasPrefix('iPod')) {
					ppi = 326;
				} else if (platform.hasPrefix('iPad')) {
					ppi = 264;
				} else if (platform.hasPrefix('iWatch')) {
					ppi = 326;
				}
			}

			// if all fails
			ppi = 96;
		}
	}

	return ppi;
}
