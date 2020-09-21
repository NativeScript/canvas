import {DemoSharedBase} from '../utils';
import {Screen, Utils} from "@nativescript/core";
import {func, images} from "./games/utils";

declare let Phaser: any, UIDevice;

interface AccelerometerData {
	x: number;
	y: number;
	z: number;
}

type SensorDelay = "normal" | "game" | "ui" | "fastest";

interface AccelerometerOptions {
	sensorDelay?: SensorDelay;
}

export const stopButNotStarted = "[nativescript-accelerometer] stopAccelerometerUpdates() called, but currently not listening. Ignoring...";
export const startButNotStopped = "[nativescript-accelerometer] startAccelerometerUpdates() called, but there is active listener. Will stop the current listener and switch to the new one.";

class Accelerometer {
	static baseAcceleration = -9.81;
	static sensorListener: android.hardware.SensorEventListener;
	static sensorManager: android.hardware.SensorManager;
	static accelerometerSensor: android.hardware.Sensor;

	static accManager;
	static isListeningForUpdates = false;
	static main_queue = global.isIOS ? dispatch_get_current_queue() : null;


	static getNativeDelay(options?: AccelerometerOptions): number {
		if (global.isAndroid) {
			if (!options || !options.sensorDelay) {
				return android.hardware.SensorManager.SENSOR_DELAY_NORMAL;
			}

			switch (options.sensorDelay) {
				case "normal":
					return android.hardware.SensorManager.SENSOR_DELAY_NORMAL;

				case "game":
					return android.hardware.SensorManager.SENSOR_DELAY_GAME;

				case "ui":
					return android.hardware.SensorManager.SENSOR_DELAY_UI;

				case "fastest":
					return android.hardware.SensorManager.SENSOR_DELAY_FASTEST;
			}
		} else {
			if (!options || !options.sensorDelay) {
				return 0.2;
			}

			switch (options.sensorDelay) {
				case "normal":
					return 0.2;
				case "ui":
					return 0.06;
				case "game":
					return 0.02
				case "fastest":
					return 0.001;
			}
		}
	}

	static startAccelerometerUpdates(callback: (data: AccelerometerData) => void, options?: AccelerometerOptions) {
		if (global.isAndroid) {
			if (Accelerometer.isListening()) {
				console.log(startButNotStopped);
				Accelerometer.stopAccelerometerUpdates();
			}

			const wrappedCallback = zonedCallback(callback);
			const context: android.content.Context = Utils.ad.getApplicationContext();
			if (!context) {
				throw Error("Could not get Android application context.")
			}

			if (!Accelerometer.sensorManager) {
				Accelerometer.sensorManager = context.getSystemService(android.content.Context.SENSOR_SERVICE);

				if (!Accelerometer.sensorManager) {
					throw Error("Could not initialize SensorManager.")
				}
			}

			if (!Accelerometer.accelerometerSensor) {
				Accelerometer.accelerometerSensor = Accelerometer.sensorManager.getDefaultSensor(android.hardware.Sensor.TYPE_ACCELEROMETER);
				if (!Accelerometer.accelerometerSensor) {
					throw Error("Could get accelerometer sensor.")
				}
			}


			Accelerometer.sensorListener = new android.hardware.SensorEventListener({
				onAccuracyChanged: (sensor, accuracy) => {
				},
				onSensorChanged: (event) => {
					wrappedCallback({
						x: event.values[0] / Accelerometer.baseAcceleration,
						y: event.values[1] / Accelerometer.baseAcceleration,
						z: event.values[2] / Accelerometer.baseAcceleration
					})
				}
			});

			const nativeDelay = Accelerometer.getNativeDelay(options);
			Accelerometer.sensorManager.registerListener(
				Accelerometer.sensorListener,
				Accelerometer.accelerometerSensor,
				nativeDelay
			);
		} else {
			if (Accelerometer.isListeningForUpdates) {
				console.log(startButNotStopped);
				Accelerometer.stopAccelerometerUpdates();
			}

			const wrappedCallback = zonedCallback(callback);

			if (!Accelerometer.accManager) {
				Accelerometer.accManager = CMMotionManager.alloc().init();
				if (!Accelerometer.accManager.gyroAvailable) {
					return;
				}
			}

			Accelerometer.accManager.accelerometerUpdateInterval = Accelerometer.getNativeDelay(options);

			if (Accelerometer.accManager.accelerometerAvailable) {
				var queue = NSOperationQueue.alloc().init();
				Accelerometer.accManager.startAccelerometerUpdatesToQueueWithHandler(queue, (data, error) => {
					dispatch_async(Accelerometer.main_queue, () => {
						wrappedCallback({
							x: data.acceleration.x,
							y: data.acceleration.y,
							z: data.acceleration.z
						})
					})
				});

				Accelerometer.isListeningForUpdates = true;
			} else {
				throw new Error("Accelerometer not available.")
			}
		}
	}

	static stopAccelerometerUpdates() {
		if (global.isAndroid) {
			if (Accelerometer.sensorListener) {
				Accelerometer.sensorManager.unregisterListener(Accelerometer.sensorListener);
				Accelerometer.sensorListener = undefined;
			} else {
				console.log(stopButNotStarted);
			}
		} else {
			if (Accelerometer.isListeningForUpdates) {
				Accelerometer.accManager.stopAccelerometerUpdates();
				Accelerometer.isListeningForUpdates = false;
			} else {
				console.log(stopButNotStarted);
			}
		}
	}

	static isListening(): boolean {
		if (global.isAndroid) {
			return !!Accelerometer.sensorListener;
		} else {
			return Accelerometer.isListeningForUpdates;
		}
	}
}

export class DemoSharedCanvasPhaserCe extends DemoSharedBase {
	canvas;
	fps = 0;

	canvasLoaded(args) {
		this.canvas = args.object;
		this.setupGame(this.canvas);
	}

	gamePause: boolean = false;
	isLoading: boolean = true;
	kills: number = 0;
	score: number = 0;
	shotsFired: number = 0;
	displayAccuracy: string | null = null;
	game: CustomGame;
	didInit: boolean = false;
	didSubscribe: boolean = false;
	useAccelerometer: boolean = true;

	constructor() {
		super();
		this.on("propertyChange", (args) => {
			if (!this.isLoading && !this.didSubscribe) {
				this.subscribe();
				this.set("didSubscribe", true);
			}
		});
		this.updateStats = this.updateStats.bind(this);
		//this.preloadAssetsAsync = this.preloadAssetsAsync.bind(this);
		this.preloadAssetsAsync();
		this.handleTogglePause = this.handleTogglePause.bind(this);

		this.onTouchesBegan = this.onTouchesBegan.bind(this);
		this.onTouchesEnded = this.onTouchesEnded.bind(this);
	}

	onTouch(event) {
		if (event.eventName === "touch") {
			switch (event.action) {
				case "down":
					this.onTouchesBegan();
					break;
				case "up":
					this.onTouchesEnded();
					break;
				case "cancel":
					this.onTouchesEnded();
					break;
				default:
					break;
			}
		} else if (event.eventName === "pan") {
			if (!this.useAccelerometer) {
				// TODO allow update position with pan
				//this.game.updateControls(event.deltaX);
			}
		}
	}

	subscribe() {
		if (!this.useAccelerometer) {
			return;
		}
		if (global.isIOS) {

			if (
				!CMMotionManager.alloc().init().gyroAvailable
			) {
				return;
			}
		}
		if (Accelerometer.isListening) {
			Accelerometer.stopAccelerometerUpdates();
		}
		Accelerometer.startAccelerometerUpdates(
			(changed) => {
				if (this.game) {
					this.game.updateControls(changed.x);
				}
			},
			{
				sensorDelay: "ui",
			}
		);

	}

	unsubscribe() {
		if (!this.useAccelerometer) {
			return;
		}
		if (global.isIOS) {
			return (
				UIDevice.currentDevice.name
					.toLowerCase()
					.indexOf("simulator") !== -1
			);
		}
		if (Accelerometer.isListening) {
			Accelerometer.stopAccelerometerUpdates();
		}
	}

	async preloadAssetsAsync() {
		const imageAssets = func.cacheImages(images.files);
		await Promise.all([...imageAssets]).then((image) => {
			this.set("isLoading", false);
		}).catch(e => {
			console.log('e:', e);
		});
	}

	updateStats(data) {
		this.set("kills", data.kills);
		this.set("score", data.score);
		this.set("shotsFired", data.shotsFired);
	}

	handleTogglePause() {
		this.set("gamePause", !this.gamePause);
	}

	onTouchesBegan() {
		if (!this.game) {
			return false;
		}

		return this.game.onTouchesBegan();
	}

	onTouchesEnded() {
		if (!this.game) {
			return false;
		}

		return this.game.onTouchesEnded();
	}

	setupGame(canvas) {
		this.game = new CustomGame({
			canvas,
			gamePause: this.gamePause,
			updateStats: this.updateStats,
		});

	}

}


export class CustomGame {
	game;
	playable: Playable;

	constructor({canvas, gamePause, updateStats}) {
		const TNSPhaser = require("@nativescript/canvas-phaser-ce");
		this.game = TNSPhaser.Game({canvas});
		this.playable = new Playable({
			game: this.game,
			gamePause,
			updateStats,
		});
		this.game.state.add("Playable", this.playable);
		this.game.state.start("Playable");

		//this.onTouchesBegan = this.onTouchesBegan.bind(this);
		//this.onTouchesEnded = this.onTouchesEnded.bind(this);
	}

	updateControls(velocity) {
		if (this.playable) {
			this.playable.updateControls({velocity});
		}
	}

	onTouchesBegan() {
		if (!this.playable) {
			return false;
		}

		return this.playable.onTouchesBegan();
	}

	onTouchesEnded() {
		if (!this.playable) {
			return false;
		}

		return this.playable.onTouchesEnded();
	}

	onTogglePause(paused) {
		this.playable.pauseGame(paused);
	}
}

const scale = Screen.mainScreen.scale;

class SettingsConfig {
	explosion: number;
	invader: number;
	playerSpeed: number;

	constructor() {
		this.explosion = 128; // * scale;
		this.invader = 32; // * scale;
		this.playerSpeed = 600;
	}
}

const Settings = new SettingsConfig();

export class Playable {
	context;
	game;
	gamePause;
	updateStats;
	startLives: number;
	alienRows: number;
	initialGameState: any;
	aliens: any;
	bullets: any;
	bulletTime: number;
	enemyBullet: any;
	explosions: any;
	firingTimer: number;
	lives: any;
	livingEnemies: any[];
	player: any;
	score: number;
	shotsFired: number;
	kills: number;
	starfield: any;
	stateText: any;
	pressing: boolean;
	enemyBullets: any;
	live: any;
	bullet: any;

	constructor({game, gamePause, updateStats}) {
		// prevent warnings for Phaser.Cache
		(console as any).disableYellowBox = true;

		this.updateStats = updateStats;

		// game config
		this.startLives = 3;
		this.alienRows = 4;
		this.initialGameState = gamePause;

		// default states
		this.aliens = null;
		this.bullets = null;
		this.bulletTime = 0;
		// this.cursors = null;
		this.enemyBullet = null;
		this.explosions = null;
		// this.fireButton = null;
		this.firingTimer = 0;
		this.lives = null;
		this.livingEnemies = [];
		this.player = null;
		this.starfield = null;
		this.stateText = null;

		// player stats
		this.kills = 0;
		this.shotsFired = 0;
		this.score = 0;

		this.game = game;
	}

	preload() {

		const {files} = images;
		this.game.load.image("bullet", func.uri(files.bullet));
		this.game.load.image("enemyBullet", func.uri(files.enemyBullet));
		this.game.load.spritesheet(
			"invader",
			func.uri(files.invader),
			Settings.invader,
			Settings.invader
		);
		this.game.load.image("ship", func.uri(files.player));
		this.game.load.spritesheet(
			"kaboom",
			func.uri(files.explode),
			Settings.explosion,
			Settings.explosion
		);

		this.game.load.image("starfield", func.uri(files.starfield));
	}

	updateControls({velocity}) {
		const {player} = this;

		if (player && player.alive) {
			const speed = Math.floor(velocity * Settings.playerSpeed);

			// reset the player, then check for movement keys
			player.body.velocity.setTo(0, 0);
			player.body.velocity.x = speed;
		}
	}

	scaleNode(node) {
		node.width *= scale;
		node.height *= scale;
	}

	onTouchesBegan() {
		this.pressing = true;
	}

	onTouchesEnded() {
		this.pressing = false;
		if (this.player) {
			if (this.player.alive) {
				this.fireBullet();
			} else {
				this.restart();
			}
		}
	}

	pauseGame(paused) {
		const {game} = this;
		game.paused = paused;
	}

	create() {
		const {game, startLives} = this;
		const {world} = game;
		const {height, width} = world;

		//Why ios needs this ?
		world.alpha = 2;
		// game.stage.backgroundColor = '#4488AA';
		game.stage.backgroundColor = "#000";
		game.physics.startSystem(Phaser.Physics.ARCADE);

		// initial game state paused?
		game.paused = this.initialGameState;

		/**
		 *
		 *  A TileSprite is a Sprite that has a repeating texture.
		 *  The texture can be scrolled and scaled independently of the TileSprite itself.
		 *  Textures will automatically wrap and are designed so that you can create game
		 *  backdrops using seamless textures as a source.
		 *
		 * */

		// the scrolling starfield background
		// this.starfield = game.add.tileSprite(0, 0, width, height, 'starfield');
		this.starfield = game.add.sprite(0, 0, "starfield");
		this.starfield.height = height;
		this.starfield.width = width;

		// our bullet group
		this.bullets = game.add.group();
		this.bullets.enableBody = true;
		this.bullets.physicsBodyType = Phaser.Physics.ARCADE;
		this.bullets.createMultiple(30, "bullet");
		this.bullets.setAll("anchor.x", 0.5);
		this.bullets.setAll("anchor.y", 1);
		this.bullets.setAll("width", 6 * scale);
		this.bullets.setAll("height", 36 * scale);
		this.bullets.setAll("outOfBoundsKill", true);
		this.bullets.setAll("checkWorldBounds", true);

		// the enemy's bullets
		this.enemyBullets = game.add.group();
		this.enemyBullets.enableBody = true;
		this.enemyBullets.physicsBodyType = Phaser.Physics.ARCADE;
		this.enemyBullets.createMultiple(30, "enemyBullet");
		this.enemyBullets.setAll("anchor.x", 0.5);
		this.enemyBullets.setAll("anchor.y", 1);
		this.enemyBullets.setAll("width", 9 * scale);
		this.enemyBullets.setAll("height", 9 * scale);
		this.enemyBullets.setAll("outOfBoundsKill", true);
		this.enemyBullets.setAll("checkWorldBounds", true);

		// the hero!
		this.player = game.add.sprite(
			width * 0.5,
			height * 0.833333333,
			"ship"
		);


		this.player.anchor.setTo(0.5, 0.5);
		this.scaleNode(this.player);
		game.physics.enable(this.player, Phaser.Physics.ARCADE);

		// the baddies!
		this.aliens = game.add.group();
		this.aliens.enableBody = true;
		this.aliens.physicsBodyType = Phaser.Physics.ARCADE;

		this.createAliens();

		// lives
		this.lives = game.add.group();
		// game.add.text(game.world.width - 100, 10, 'Lives : ', { font: '34px Arial', fill: '#fff' });

		// text
		// this.stateText = game.add.text(game.world.centerX,game.world.centerY,' ', { font: '84px Arial', fill: '#fff' });
		// this.stateText.anchor.setTo(0.5, 0.5);
		// this.stateText.visible = false;

		const shipOffset = width * 0.125;
		const initialshipXoffset = width - shipOffset * startLives;
		const shipInterval = 30 * scale;
		const shipY = 60 * scale;

		for (let i = 0; i < startLives; i += 1) {
			const ship = this.lives.create(
				initialshipXoffset + shipInterval * i,
				shipY,
				"ship"
			);
			this.scaleNode(ship);
			ship.anchor.setTo(0.5, 0.5);
			ship.angle = 90;
			ship.alpha = 0.4;
		}

		// an explosion pool
		this.explosions = game.add.group();

		// this.explosions.scale = scale;
		this.explosions.createMultiple(30, "kaboom");
		this.explosions.setAll("height", 128 * scale);
		this.explosions.setAll("width", 128 * scale);
		this.explosions.setAll("transparent", true);

		this.explosions.forEach(this.setupInvader, this);

		// and some controls to play the game with
		// this.cursors = game.input.keyboard.createCursorKeys();
		// this.fireButton = game.input.keyboard.addKey(Phaser.Keyboard.SPACEBAR);
	}

	createAliens() {
		const {alienRows, game} = this;
		const {world} = game;
		const {height, width} = world;

		const alienDelta = width * 0.25;
		const alienAvailableSpace = width - alienDelta;
		const alienWidth = 32 * scale;
		const alienPadding = 12;
		const aliens = Math.floor(
			alienAvailableSpace / (alienPadding + alienWidth)
		);

		const dimensions = {
			columns: aliens,
			rows: alienRows,
		};
		const alienOffset = {
			x: alienAvailableSpace / dimensions.columns,
		};

		for (let y = 0; y < dimensions.rows; y += 1) {
			for (let x = 0; x < dimensions.columns; x += 1) {
				const alien = this.aliens.create(
					x * alienOffset.x,
					y * alienOffset.x,
					"invader"
				);
				this.scaleNode(alien);
				alien.anchor.setTo(0.5, 0.5);
				alien.animations.add("fly", [0, 1, 2, 3], 20, true);
				alien.play("fly");
				alien.body.moves = false;
			}
		}

		// const alienOffset = this.game.world.width
		this.aliens.x = alienWidth / 2;
		this.aliens.y = height * 0.0625;

		// all this does is basically start the invaders moving. notice we're
		// moving the Group they belong to, rather than the invaders directly.
		const tween = this.game.add
			.tween(this.aliens)
			.to(
				{x: width - alienAvailableSpace + alienWidth / 2},
				2000,
				Phaser.Easing.Linear.None,
				true,
				0,
				1000,
				true
			);

		// when the tween loops it calls descend
		tween.onRepeat.add(this.descend, this);
	}

	setupInvader(invader) {
		invader.anchor.x = 0.5;
		invader.anchor.y = 0.5;
		invader.animations.add("kaboom");
	}

	descend() {
		const {game} = this;
		const {world} = game;
		const {height} = world;

		console.log("Loop");
		this.aliens.y += height * 0.0166666667;
	}

	collisionHandler(bullet, alien) {
		// when a bullet hits an alien we kill them both
		bullet.kill();
		alien.kill();

		// increase the kills
		this.kills += 1;

		// increase the score
		this.score += 20;

		// and create an explosion :)
		const explosion = this.explosions.getFirstExists(false);

		if (explosion) {
			explosion.reset(alien.body.x, alien.body.y);
			explosion.play("kaboom", 30, false, true);
		}

		if (this.aliens.countLiving() === 0) {
			this.score += 1000;

			this.player.kill();
			this.enemyBullets.callAll("kill");

			// this.stateText.text = " You Won, \n Click to restart";
			// this.stateText.visible = true;

			// the "click to restart" handler
			console.log("--------------------");
			console.log("you beat this level!");
			console.log("--------------------");
			this.game.input.onTap.addOnce(this.restart, this);
		}
	}

	enemyHitsPlayer(player, bullet) {
		bullet.kill();

		this.live = this.lives.getFirstAlive();

		if (this.live) {
			this.live.kill();
		}

		// and create an explosion :)
		const explosion = this.explosions.getFirstExists(false);
		if (explosion) {
			explosion.reset(player.body.x, player.body.y);
			explosion.play("kaboom", 30, false, true);
		}

		// when the player dies
		if (this.lives.countLiving() < 1) {
			player.kill();
			this.enemyBullets.callAll("kill");

			// this.stateText.text=" GAME OVER \n Click to restart";
			// this.stateText.visible = true;

			// the "click to restart" handler
			console.log("--------------------");
			console.log("you lost, game over!");
			console.log("--------------------");
			this.game.input.onTap.addOnce(this.restart, this);
			// game.input.onTap.addOnce(this.restart, this);
		}
	}

	enemyFires() {
		const {game} = this;

		// grab the first bullet we can from the pool
		this.enemyBullet = this.enemyBullets.getFirstExists(false);
		this.livingEnemies.length = 0;

		this.aliens.forEachAlive((alien) => {
			// put every living enemy in an array
			this.livingEnemies.push(alien);
		});

		if (this.enemyBullet && this.livingEnemies.length > 0) {
			const random = game.rnd.integerInRange(
				0,
				this.livingEnemies.length - 1
			);

			// randomly select one of them
			const shooter = this.livingEnemies[random];
			// and fire the bullet from this enemy
			this.enemyBullet.reset(shooter.body.x, shooter.body.y);

			game.physics.arcade.moveToObject(
				this.enemyBullet,
				this.player,
				120
			);
			this.firingTimer = game.time.now + 2000;
		}
	}

	fireBullet() {
		//  const { bullets, game, player } = this;
		// let { bulletTime, bullet } = this;

		// to avoid them being allowed to fire too fast we set a time limit
		if (this.game.time.now > this.bulletTime) {
			// grab the first bullet we can from the pool
			this.bullet = this.bullets.getFirstExists(false);

			if (this.bullet) {
				this.shotsFired += 1;
				// and fire it
				this.bullet.reset(this.player.x, this.player.y + 8 * scale);
				this.bullet.body.velocity.y = -400 * scale;
				this.bulletTime = this.game.time.now + 200 * scale;
			}
		}
	}

	// resetBullet(bullet) {
	resetBullet() {
		// called if the bullet goes out of the screen
		this.bullet.kill();
	}

	restart() {
		const {lives, aliens, player} = this;
		// a new level starts

		// resets the life count
		lives.callAll("revive");
		// and brings the aliens back from the dead :)
		aliens.removeAll();
		this.createAliens();

		// revives the player
		player.revive();

		// hides the text
		// stateText.visible = false;
	}

	cycleNode(node) {
		const {game} = this;
		const {world} = game;
		const {width} = world;

		const half = node.width / 2;

		if (node.x < -half) {
			node.x = width + half;
		} else if (node.x > width + half) {
			node.x = -half;
		}
	}

	update() {
		const {
			aliens,
			bullets,
			collisionHandler,
			enemyBullets,
			enemyHitsPlayer,
			firingTimer,
			game,
			player,
			starfield,
		} = this;
		// scroll the background

		if (starfield.tilePosition) {
			starfield.tilePosition.y += 2;
		}

		this.updateStats({
			kills: this.kills,
			score: this.score,
			shotsFired: this.shotsFired,
		});

		if (player.alive) {
			// firing?
			if (game.time.now > firingTimer) {
				this.enemyFires();
			}
			this.cycleNode(player);

			if (this.aliens.y >= player.y && this.lives.countLiving() > 0) {
				player.kill();
				this.enemyBullets.callAll("kill");
				// this.stateText.text=" GAME OVER \n Click to restart";
				// this.stateText.visible = true;

				// the "click to restart" handler
				game.input.onTap.addOnce(this.restart, this);
			}

			// run collision
			game.physics.arcade.overlap(
				bullets,
				aliens,
				collisionHandler,
				null,
				this
			);

			game.physics.arcade.overlap(
				enemyBullets,
				player,
				enemyHitsPlayer,
				null,
				this
			);
		}
	}
}
