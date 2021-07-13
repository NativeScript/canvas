import {SVGItem} from "./SVGItem";
import {AddChildFromBuilder} from "@nativescript/core";
import { DOMParser } from 'xmldom';
export class Symbol extends SVGItem {
    constructor(){
		super();
		this._dom = new DOMParser().parseFromString('<symbol></symbol>');
	}
}
