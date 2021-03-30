import { knownFolders, File as NSFile, isIOS, path } from '@nativescript/core';

export class URL {
	public static createObjectURL(object: any): string {
		if (object instanceof Blob || object instanceof File) {
			const filePath = path.join(knownFolders.temp().path, this.getUUID() + '.js');
			const buf = (Blob as any).InternalAccessor.getBuffer(object);
			if (isIOS) {
				NSFile.fromPath(filePath).writeSync(NSData.dataWithData(buf));
			} else {
				try {
					const fos = new java.io.FileOutputStream(new java.io.File(filePath));
					fos.write(Array.from(buf) as any);
					fos.flush();
					fos.close();
				} catch (e) {
					return null;
				}
			}
			return filePath;
		}
		return null;
	}

	public static revokeObjectURL(url: string) {
		if (typeof url === 'string') {
			if (NSFile.exists(url)) {
				const file = NSFile.fromPath(url);
				file.removeSync();
			}
		}
	}

	private static getUUID() {
		if (isIOS) {
			return NSUUID.UUID().UUIDString;
		}
		return java.util.UUID.randomUUID().toString();
	}
}
