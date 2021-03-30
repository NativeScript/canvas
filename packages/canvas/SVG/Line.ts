import {Property} from "@nativescript/core/ui/core/view";
import {Path2D} from "../Canvas2D";
import {SVGItem} from "./SVGItem";

export const x1Property = new Property<Line, any>({
	name: 'x1'
});
export const y1Property = new Property<Line, any>({
	name: 'y1'
});
export const x2Property = new Property<Line, any>({
	name: 'x2'
});
export const y2Property = new Property<Line, any>({
	name: 'y2'
});

export class Line extends SVGItem {
	x1: any;
	y1: any;
	x2: any;
	y2: any;
}

x1Property.register(Line);
x2Property.register(Line);
y1Property.register(Line);
y2Property.register(Line);
