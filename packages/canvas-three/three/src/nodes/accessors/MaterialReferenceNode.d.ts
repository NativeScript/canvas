import { Material } from '../../materials/Material.js';
import { NodeOrType, ShaderNodeObject } from '../shadernode/ShaderNode.js';
import ReferenceNode from './ReferenceNode.js';

export default class MaterialReferenceNode extends ReferenceNode<Material | null> {
	constructor(property: string, inputType: string, material?: Material | null);
}

export const materialReference: (name: string, nodeOrType: NodeOrType, material?: Material | null) => ShaderNodeObject<MaterialReferenceNode>;
