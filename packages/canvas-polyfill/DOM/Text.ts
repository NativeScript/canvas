import { CharacterData } from './CharacterData';
import { Node } from './Node';
export class Text extends CharacterData {
	nodeType = Node.TEXT_NODE;
	constructor(data) {
		super(data);
	}
}
