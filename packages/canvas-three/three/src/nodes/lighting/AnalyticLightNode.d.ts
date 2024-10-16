import { Light } from '../../lights/Light.js';
import LightingNode from './LightingNode.js';

export default class AnalyticLightNode<T extends Light> extends LightingNode {
	light: T | null;

	constructor(light?: T | null);
}
