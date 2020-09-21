import {NativeScriptProps, registerElement, ViewAttributes} from "react-nativescript";

registerElement('Canvas', () => require('@nativescript/canvas').Canvas);
import {CanvasRenderingContext2D} from "../Canvas2D/CanvasRenderingContext2D";
import {WebGLRenderingContext} from "../WebGL/WebGLRenderingContext";
import {WebGL2RenderingContext} from "../WebGL2/WebGL2RenderingContext";
import {Canvas} from "../Canvas";
import {EventData} from "@nativescript/core";

export declare type CanvasAttributes = ViewAttributes & {
	readonly clientWidth?: number;
	readonly clientHeight?: number;

	flush?(): void;

	toDataURL?(type?: string, encoderOptions?: number): any;

	getContext?(type: string, options?: any): CanvasRenderingContext2D | WebGLRenderingContext | WebGL2RenderingContext | null;

	onReady?: (arg: EventData) => void;
};

declare global {
	module JSX {
		interface IntrinsicElements {
			tnsCanvas: NativeScriptProps<CanvasAttributes, Canvas>;
		}
	}
}
