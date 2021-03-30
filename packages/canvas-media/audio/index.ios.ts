import {AudioBase, controlsProperty} from "./common";
import {Source} from "../common";
import {Button, Color, Frame, GridLayout, GridUnitType, ItemSpec, Label, Slider, Utils} from "@nativescript/core";


export class Audio extends AudioBase {
	#player: AVQueuePlayer;
	#sourceView: Source[];
	#view: GridLayout;
	#isReady: boolean = false;
	#playPauseBtn: Button;
	#isPaused: boolean = true;
	#slider: Slider;

	constructor() {
		super();
		this.#sourceView = [];
		this.#view = new GridLayout();
		this.#view.addColumn(
			new ItemSpec(1, GridUnitType.AUTO)
		);
		this.#view.addColumn(
			new ItemSpec(30, GridUnitType.PIXEL)
		);
		this.#view.addColumn(
			new ItemSpec(1, GridUnitType.STAR)
		);
		this.#view.addColumn(
			new ItemSpec(30, GridUnitType.PIXEL)
		);
		this.#view.addColumn(
			new ItemSpec(1, GridUnitType.AUTO)
		);
		this.#view.height = 60;
		this.#view.backgroundColor = new Color('#f1f3f4').ios;
		this.#player = AVQueuePlayer.new();
		this.#playPauseBtn = new Button();
		//this.#playBtn.color = 'black';
		this.#playPauseBtn.width = 50;
		this.#playPauseBtn.height = 50;
		this.#slider = new Slider();
	}


	initNativeView() {
		super.initNativeView();
		this._addControls();
	}

	private _addControls() {
		if (this.controls) {
			this._addView(this.#view);
			this.#view.addChild(this.#playPauseBtn);
			this.#playPauseBtn.nativeView.frame = CGRectMake(0, 0, 50, 50);
			this.#playPauseBtn.nativeView.setNeedsLayout()
			//this.#view.addChild(this.#slider);
			GridLayout.setColumn(this.#playPauseBtn, 0);
			//GridLayout.setColumn(this.#slider, 2);
			const layer = this._createPlayPause();
			const play = layer.play;
			const playBtnIcon = CAShapeLayer.new();
			playBtnIcon.lineWidth = 1;
			playBtnIcon.path = layer.play.CGPath;
			this.#playPauseBtn.nativeView.layer.addSublayer(
				playBtnIcon
			);
			this.#playPauseBtn.on('tap', args => {
				this._draw();
			});
		}
	}

	private _draw() {
		const path = this._createPlayPause();
		const view = this.#playPauseBtn.nativeView as UIButton;
		const animation = CABasicAnimation.animationWithKeyPath('path');
		if(this.#isPaused){
			if(this.#isReady){
				animation.fromValue = path.play.CGPath;
				animation.toValue = path.pause.CGPath;
			}else {
				animation.toValue = path.pause.CGPath;
			}
			animation.duration = .3;
			animation.fillMode = kCAFillModeForwards
			animation.removedOnCompletion = false
			view.layer.sublayers[0].addAnimationForKey(animation, animation.keyPath);
		}else {
			animation.fromValue = path.pause.CGPath;
			animation.toValue = path.play.CGPath;
			animation.duration = .3;
			animation.fillMode = kCAFillModeForwards
			animation.removedOnCompletion = false
			view.layer.sublayers[0].addAnimationForKey(animation, animation.keyPath);
		}
		this.#isPaused = !this.#isPaused;
	}

	private _createPlayPause() {
		const playBtnPath = UIBezierPath.bezierPath();
		playBtnPath.moveToPoint(CGPointZero);
		playBtnPath.addLineToPoint(CGPointMake(0, 25));
		playBtnPath.addLineToPoint(CGPointMake(6.25, 18.75));
		playBtnPath.addLineToPoint(CGPointMake(6.25, 6.25));
		playBtnPath.addLineToPoint(CGPointMake(0, 0));

		playBtnPath.moveToPoint(CGPointMake(6.25, 6.25));
		playBtnPath.addLineToPoint(CGPointMake(6.25, 18.75));
		playBtnPath.addLineToPoint(CGPointMake(12.5, 12.5));
		playBtnPath.moveToPoint(CGPointMake(6.25, 6.25));
		playBtnPath.closePath();


		//playBtnIcon.bounds = CGPathGetBoundingBox(playBtnIcon.path);


		const pauseBtnPath = UIBezierPath.bezierPath();
		pauseBtnPath.moveToPoint(CGPointZero);
		pauseBtnPath.addLineToPoint(CGPointMake(0, 25));
		pauseBtnPath.addLineToPoint(CGPointMake(5, 25));
		pauseBtnPath.addLineToPoint(CGPointMake(5, 0));
		pauseBtnPath.addLineToPoint(CGPointMake(0, 0));

		const offset = 2.5 + 5;

		pauseBtnPath.moveToPoint(CGPointMake(offset, 0));
		pauseBtnPath.addLineToPoint(CGPointMake(offset, 25));
		pauseBtnPath.addLineToPoint(CGPointMake(offset + 5, 25));
		pauseBtnPath.addLineToPoint(CGPointMake(offset + 5, 0));
		pauseBtnPath.addLineToPoint(CGPointMake(offset + 5, 0));
		pauseBtnPath.closePath();


		return {play: playBtnPath, pause: pauseBtnPath};
	}

	// private _createPauseBtn() {
	// 	const pauseBtnPath = UIBezierPath.bezierPath();
	// 	pauseBtnPath.moveToPoint(CGPointZero);
	// 	pauseBtnPath.addLineToPoint(CGPointMake(0, 25));
	// 	pauseBtnPath.addLineToPoint(CGPointMake(5, 25));
	// 	pauseBtnPath.addLineToPoint(CGPointMake(5, 0));
	// 	pauseBtnPath.addLineToPoint(CGPointMake(0, 0));
	//
	// 	const offset = 2.5 + 5;
	//
	// 	pauseBtnPath.moveToPoint(CGPointMake(offset, 0));
	// 	pauseBtnPath.addLineToPoint(CGPointMake(offset, 25));
	// 	pauseBtnPath.addLineToPoint(CGPointMake(offset + 5, 25));
	// 	pauseBtnPath.addLineToPoint(CGPointMake(offset + 5, 0));
	// 	pauseBtnPath.addLineToPoint(CGPointMake(offset + 5, 0));
	// 	pauseBtnPath.closePath();
	// 	const pauseBtnIcon = CAShapeLayer.new();
	// 	pauseBtnIcon.strokeColor = UIColor.redColor;
	// 	pauseBtnIcon.lineWidth = 1;
	// 	pauseBtnIcon.path = pauseBtnPath.CGPath;
	// 	this.#view.addChild(this.#pauseBtn);
	// 	GridLayout.setColumn(this.#pauseBtn, 0);
	// 	return pauseBtnIcon;
	// }

	[controlsProperty.setNative](enable: boolean) {

	}

	_addChildFromBuilder(name: string, value: any) {
		if (value instanceof Source) {
			this.#sourceView.push(value);
		}
	}

	onLoaded() {
		super.onLoaded();
		const item = this.#sourceView[0];
		if (item) {
			this.#player.insertItemAfterItem(
				AVPlayerItem.alloc().initWithURL(
					NSURL.URLWithString(item.src),
				),
				null
			);
		}
	}
}
