import { Property } from "@nativescript/core";
import { Path2D, CanvasGradient } from "../Canvas2D";
import { SVGItem } from "./SVGItem";

export const dProperty = new Property<Path, string>({
	name: 'd'
});
import { DOMParser } from 'xmldom';
export class Path extends SVGItem {
	public d: string;

	constructor(){
		super();
		this._dom = new DOMParser().parseFromString('<path></path>');
	}
}

dProperty.register(Path);
