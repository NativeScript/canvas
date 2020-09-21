import {AddChildFromBuilder, Property, View} from "@nativescript/core";

export const xProperty = new Property<Text, any>({
    name: 'x'
});

export const yProperty = new Property<Text, any>({
    name: 'y'
});

export const dxProperty = new Property<Text, any>({
    name: 'dx'
});

export class Text extends View implements AddChildFromBuilder {
    x: any;
    y: any;
    dx: any;

    constructor() {
        super();
    }

    _addChildFromBuilder(name: string, value: any): void {
    }
}


xProperty.register(Text);
yProperty.register(Text);
dxProperty.register(Text);
