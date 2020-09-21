const background_queue = dispatch_get_global_queue(21, 0);
const main_queue = dispatch_get_current_queue();
const core = require('@nativescript/core');

export class FileManager {
  static writeFile(bytes, path, callback) {
    dispatch_async(background_queue, () => {
      try {
        if (bytes instanceof NSData) {
          bytes.writeToFileAtomically(path, true);
        } else if (bytes instanceof ArrayBuffer) {
          NSData.dataWithData(bytes).writeToFileAtomically(path, true);
        }
        dispatch_async(main_queue, () => {
          callback(null, path);
        });
      } catch (e) {
        dispatch_async(main_queue, () => {
          callback(e, null);
        });
      }
    });
  }

  static readFile(path, options = {asStream: false}, callback) {
    dispatch_async(background_queue, () => {
      try {
        if (!core.File.exists(path)) {
          dispatch_async(main_queue, () => {
            callback(new Error('File not found.'), null);
          });
          return;
        }
        const data = NSData.dataWithContentsOfFile(path);
        dispatch_async(main_queue, () => {
          callback(null, data);
        });
      } catch (e) {
        dispatch_async(main_queue, () => {
          callback(e, null);
        });
      }
    });
  }

  static deleteFile(path, options = {asStream: false}, callback) {
    dispatch_async(background_queue, () => {
      try {
        NSFileManager.defaultManager.removeItemAtPathError(path);
        dispatch_async(main_queue, () => {
          callback(null, true);
        });
      } catch (e) {
        dispatch_async(main_queue, () => {
          callback(e, false);
        });
      }
    });
  }
}

//# sourceMappingURL=file.ios.js.map
