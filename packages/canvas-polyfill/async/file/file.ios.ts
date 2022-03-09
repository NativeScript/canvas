declare const qos_class_t;

const background_queue = dispatch_get_global_queue(qos_class_t.QOS_CLASS_DEFAULT, 0);
const main_queue = dispatch_get_current_queue();
export class FileManager {

    public static writeFile(bytes: any, path: string, callback: (...args) => void) {
        dispatch_async(background_queue, () => {
            try {
                if (bytes instanceof NSData) {
                    bytes.writeToFileAtomically(path, true);
                } else if (bytes instanceof ArrayBuffer) {
                    NSData.dataWithData(bytes as any).writeToFileAtomically(path, true);
                }
                dispatch_async(main_queue, ()=>{
                    callback(null, path);
                });
            } catch (e) {
                dispatch_async(main_queue, ()=>{
                    callback(e, null);
                });
            }
        });
    }

    public static readFile(path: string, options: Options = {asStream: false}, callback: (...args) => void) {
        dispatch_async(background_queue, () => {
            try {
                const data = NSData.dataWithContentsOfFile(path);
                dispatch_async(main_queue, ()=>{
                    callback(null, data);
                });
            } catch (e) {
                dispatch_async(main_queue, ()=>{
                    callback(e, null);
                });
            }
        });
    }

    public static deleteFile(path: string, options: Options = {asStream: false}, callback: (...args) => void) {
      dispatch_async(background_queue, () => {
          try {
              NSFileManager.defaultManager.removeItemAtPathError(path);
              dispatch_async(main_queue, ()=>{
                callback(null, true);
              });
          } catch (e) {
            dispatch_async(main_queue, ()=>{
                callback(e, false);
            });
          }
      });
    }
}

export interface Options {
    asStream?: boolean;
}
