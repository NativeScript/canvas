module.exports = {
	message: 'NativeScript Plugins ~ made with ❤️  Choose a command to start...',
	pageSize: 32,
	scripts: {
		default: 'nps-i',
		nx: {
			script: 'nx',
			description: 'Execute any command with the @nrwl/cli',
		},
		format: {
			script: 'nx format:write',
			description: 'Format source code of the entire workspace (auto-run on precommit hook)',
		},
		'🔧': {
			script: `npx cowsay "NativeScript plugin demos make developers 😊"`,
			description: '_____________  Apps to demo plugins with  _____________',
		},
		// demos
		apps: {
			'...Vanilla...': {
				script: `npx cowsay "Nothing wrong with vanilla 🍦"`,
				description: ` 🔻 Vanilla`,
			},
			demo: {
				clean: {
					script: 'nx run demo:clean',
					description: '⚆  Clean  🧹',
				},
				ios: {
					script: 'nx run demo:ios',
					description: '⚆  Run iOS  ',
				},
				android: {
					script: 'nx run demo:android',
					description: '⚆  Run Android  🤖',
				},
			},
			'...Angular...': {
				script: `npx cowsay "Test all the Angles!"`,
				description: ` 🔻 Angular`,
			},
			'demo-angular': {
				clean: {
					script: 'nx run demo-angular:clean',
					description: '⚆  Clean  🧹',
				},
				ios: {
					script: 'nx run demo-angular:ios',
					description: '⚆  Run iOS  ',
				},
				android: {
					script: 'nx run demo-angular:android',
					description: '⚆  Run Android  🤖',
				},
			},
		},
		'⚙️': {
			script: `npx cowsay "@nativescript/* packages will keep your ⚙️ cranking"`,
			description: '_____________  @nativescript/*  _____________',
		},
		// packages
		// build output is always in dist/packages
		'@nativescript': {
			// @nativescript/canvas
			canvas: {
				build: {
					script: 'nx run canvas:build.all',
					description: '@nativescript/canvas: Build',
				},
			},
			// @nativescript/canvas-babylon
			'canvas-babylon': {
				build: {
					script: 'nx run canvas-babylon:build.all',
					description: '@nativescript/canvas-babylon: Build',
				},
			},
			// @nativescript/canvas-polyfill
			'canvas-polyfill': {
				build: {
					script: 'nx run canvas-polyfill:build.all',
					description: '@nativescript/canvas-polyfill: Build',
				},
			},
			// @nativescript/canvas-phaser
			'canvas-phaser': {
				build: {
					script: 'nx run canvas-phaser:build.all',
					description: '@nativescript/canvas-phaser: Build',
				},
			},
			// @nativescript/canvas-phaser-ce
			'canvas-phaser-ce': {
				build: {
					script: 'nx run canvas-phaser-ce:build.all',
					description: '@nativescript/canvas-phaser-ce: Build',
				},
			},
			// @nativescript/canvas-pixi
			'canvas-pixi': {
				build: {
					script: 'nx run canvas-pixi:build.all',
					description: '@nativescript/canvas-pixi: Build',
				},
			},
			// @nativescript/canvas-three
			'canvas-three': {
				build: {
					script: 'nx run canvas-three:build.all',
					description: '@nativescript/canvas-three: Build',
				},
			},
			// @nativescript/canvas-media
			'canvas-media': {
				build: {
					script: 'nx run canvas-media:build.all',
					description: '@nativescript/canvas-media: Build',
				},
			},
			'build-all': {
				script: 'nx run-many --target=build.all --all',
				description: 'Build all packages',
			},
		},
		'🧰': {
			script: `npx cowsay "Tools for the workspace 🧰"`,
			description: '_____________  Tools  _____________',
		},
		tools: {
			'sync-packages-with-demos': {
				script: 'nx workspace-schematic sync-packages-with-demos',
				description: 'Helps ensure all packages have views to test across all demo apps',
			},
		},
		'⚡': {
			script: `npx cowsay "Focus only on source you care about for efficiency ⚡"`,
			description: '_____________  Focus (VS Code supported)  _____________',
		},
		focus: {
			canvas: {
				script: 'nx run canvas:focus',
				description: 'Focus on @nativescript/canvas',
			},
			'canvas-babylon': {
				script: 'nx run canvas-babylon:focus',
				description: 'Focus on @nativescript/canvas-babylon',
			},
			'canvas-polyfill': {
				script: 'nx run canvas-polyfill:focus',
				description: 'Focus on @nativescript/canvas-polyfill',
			},
			'canvas-phaser': {
				script: 'nx run canvas-phaser:focus',
				description: 'Focus on @nativescript/canvas-phaser',
			},
			'canvas-phaser-ce': {
				script: 'nx run canvas-phaser-ce:focus',
				description: 'Focus on @nativescript/canvas-phaser-ce',
			},
			'canvas-pixi': {
				script: 'nx run canvas-pixi:focus',
				description: 'Focus on @nativescript/canvas-pixi',
			},
			'canvas-three': {
				script: 'nx run canvas-three:focus',
				description: 'Focus on @nativescript/canvas-three',
			},
			'canvas-media': {
				script: 'nx run canvas-media:focus',
				description: 'Focus on @nativescript/canvas-media',
			},
			reset: {
				script: 'nx g @nativescript/plugin-tools:focus-packages',
				description: 'Reset Focus',
			},
		},
		'.....................': {
			script: `npx cowsay "That's all for now folks ~"`,
			description: '.....................',
		},
	},
};
