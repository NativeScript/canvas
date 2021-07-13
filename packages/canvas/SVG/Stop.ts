import {SVGItem} from "./SVGItem";
import { DOMParser } from 'xmldom';
export class Stop extends SVGItem {
    offset: any;
    constructor(){
		super();
		this._dom = new DOMParser().parseFromString('<stop></stop>');
	}
}