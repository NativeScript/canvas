import {Property} from "@nativescript/core";
import {Path2D} from "../Canvas2D";
import {SVGItem} from "./SVGItem";

export const pointsProperty = new Property<Polygon, any>({
	name: 'points'
});
import { DOMParser } from 'xmldom';
export class Polygon extends SVGItem {
	points: any;

	constructor(){
		super();
		this._dom = new DOMParser().parseFromString('<polygon></polygon>');
	}
}

pointsProperty.register(Polygon);
