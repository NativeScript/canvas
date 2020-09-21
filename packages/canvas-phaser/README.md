# @nativescript/canvas-phaser

Tools for using Phaser3 to build native 2D games in NativeScript 👾👾

- [Installation](#installation)
- [Usage](#usage)
- [Functions](#functions)

### Installation

```bash
npm i @nativescript/canvas-phaser
```

### Usage

Import the library into your JavaScript file:

```js
import TNSPhaser from "@nativescript/canvas-phaser";
```

## Functions

### `TNSPhaser.game({ canvas, renderer: Phaser.WEBGL || Phaser.CANVAS, ...extras })`

Given a `canvas` from a
[`Canvas`](https://github.com/nativescript/canvas), return a
[`Phaser.Game`](https://photonstorm.github.io/phaser3-docs/Phaser.Game.html)
that draws into it.

#### Props

| Property    |         Type          | Description                                                                 |         Default Value         |
| ----------- | :-------------------: | --------------------------------------------------------------------------- | :---------------------------: |
| canvas     | TNSCanvas| Required: canvas that the `Phaser.Game` will render to                     |            `null`             |
| renderer     | number?| Optional: choose the renderer type e.g Phaser.CANVAS (1) , Phaser.WEBGL(2)             |            `1`             |
| width       |        number?        | Optional: height of the `Phaser.Game`                                       | `canvas height`  |
| height      |        number?        | Optional: width of the `Phaser.Game`                                        | `canvas width` |
| title       |        string?        | Optional: title of the `Phaser.Game`                                        |     `"tns-phaser-game"`      |

#### Returns

| Property |                              Type                              | Description                                      |
| -------- | :------------------------------------------------------------: | ------------------------------------------------ |
| game     | [`Phaser.Game`](https://photonstorm.github.io/phaser3-docs/Phaser.Game.html) | The Phaser3 game used for rendering game logic |

## Example

```js
const game = TNSPhaser.game({ canvas });
```

## What does it do?

Under the hood, TNSPhaser is maintaining global instances of a few libraries.

- [Phaser3](https://github.com/photonstorm/phaser)

```js
window.Phaser = require("phaser");
```

Finally when a new instance of `TNSPhaser.Game` is created, we set the `document.readyState` to `'complete'`

```js
global.document.readyState = "complete";
```

## License

Apache License Version 2.0, January 2004
