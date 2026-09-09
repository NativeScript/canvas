import { knownFolders } from '@nativescript/core';

/**
 * Where the polyfill keeps its files (localStorage.db, materialised Blobs, downloaded images).
 * tvOS apps have no writable Documents directory (only Library/Caches and tmp); on a physical Apple TV
 * creating a file under Documents aborts the app, while the simulator tolerates it. Caches may be purged
 * by the system under storage pressure, which is the documented tvOS persistence model.
 */
export function storageFolderPath(): string {
	const isTvOS = typeof (global as any).__TVOS__ !== 'undefined' ? !!(global as any).__TVOS__ : typeof UIDevice !== 'undefined' && UIDevice.currentDevice.userInterfaceIdiom === UIUserInterfaceIdiom.TV;
	return isTvOS ? knownFolders.ios.library().getFolder('Caches').path : knownFolders.documents().path;
}
