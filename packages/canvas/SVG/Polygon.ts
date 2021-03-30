import {Property} from "@nativescript/core";
import {Path2D} from "../Canvas2D";
import {SVGItem} from "./SVGItem";

export const pointsProperty = new Property<Polygon, any>({
	name: 'points'
});

export class Polygon extends SVGItem {
	points: any;
}

pointsProperty.register(Polygon);
