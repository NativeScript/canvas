import {SVGItem} from "./SVGItem";
import {Svg} from "./SVG";
import {AddChildFromBuilder} from "@nativescript/core";
import {Stop} from "./Stop";

export class LinearGradient extends SVGItem {
	_views: any[];
	x1: any = '0%';
	y1: any = '0%';
	x2: any = '100%';
	y2: any = '0%';

	gradientTransform: string;
}
