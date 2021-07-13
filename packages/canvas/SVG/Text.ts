import {AddChildFromBuilder, Property} from "@nativescript/core";
import { SVGItem } from "./SVGItem";

export const xProperty = new Property<Text, any>({
    name: 'x'
});

export const yProperty = new Property<Text, any>({
    name: 'y'
});

export const dxProperty = new Property<Text, any>({
    name: 'dx'
});

export const dyProperty = new Property<Text, any>({
    name: 'dy'
});
import { DOMParser } from 'xmldom';
export class Text extends SVGItem {
    x: any;
    y: any;
    dx: any;
    dy: any;
    text: string;

    constructor(){
		super();
		this._dom = new DOMParser().parseFromString('<text></text>');
	}
}


xProperty.register(Text);
yProperty.register(Text);
dxProperty.register(Text);
dyProperty.register(Text);
