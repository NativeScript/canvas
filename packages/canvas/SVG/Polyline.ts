import {Property} from "@nativescript/core/ui/core/view";
import {Path2D} from "../Canvas2D";
import {SVGItem} from "./SVGItem";

export const pointsProperty = new Property<Polyline, any>({
	name: 'points'
});

export class Polyline extends SVGItem {
	points: any;
}

pointsProperty.register(Polyline);
