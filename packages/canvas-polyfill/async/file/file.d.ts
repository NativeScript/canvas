export interface Options {
	asStream?: boolean;
}

export class FileManager {
	static supportFastRead: boolean;
	public static writeFile(bytes: any, path: string, callback: (...args) => void);

	public static readFile(path: string, options: Options, callback: (...args) => void);
}
