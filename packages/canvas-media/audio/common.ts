import { CSSType } from '@nativescript/core';
import { MediaBase } from '../common';

@CSSType('Audio')
export abstract class AudioBase extends MediaBase {
	public controls: boolean;
	public loop: boolean;
	public autoPlay: boolean;
}
