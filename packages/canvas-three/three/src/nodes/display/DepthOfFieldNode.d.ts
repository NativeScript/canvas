import TextureNode from '../accessors/TextureNode.js';
import Node from '../core/Node.js';
import TempNode from '../core/TempNode.js';
import UniformNode from '../core/UniformNode.js';
import { NodeRepresentation, ShaderNodeObject } from '../shadernode/ShaderNode.js';

export default class DepthOfFieldNode extends TempNode {
	textureNode: TextureNode;
	viewZNode: Node;

	focus: UniformNode<number>;
	aperture: UniformNode<number>;
	maxblur: UniformNode<number>;

	constructor(textureNode: TextureNode, viewZNode: Node, focusNode: Node, apertureNode: Node, maxblurNode: Node);
}

export const dof: (node: NodeRepresentation, viewZNode: NodeRepresentation, focus?: NodeRepresentation, aperture?: NodeRepresentation, maxblur?: NodeRepresentation) => ShaderNodeObject<DepthOfFieldNode>;

declare module '../shadernode/ShaderNode.js' {
	interface NodeElements {
		dof: typeof dof;
	}
}
