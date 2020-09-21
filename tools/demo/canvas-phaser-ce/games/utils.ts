import {ImageSource, path, knownFolders} from "@nativescript/core";
import {ImageAsset} from '@nativescript/canvas';

const root = "~/assets/phaser-ce/si"
const icon = root + "/icon.png";
const bullet = root + "/files/bullet.png";
const enemyBullet = root + "/files/enemy-bullet.png";
const explode = root + "/files/explode.png";
const invader = root + "/files/invader32x32x4.png";
const player = root + "/files/player.png";
const starfield = root + "/files/starfield.png";

export const images = {
  icon,
  files: {
    bullet,
    enemyBullet,
    explode,
    invader,
    player,
    starfield,
  },
};

const imageSourceCache = {};

function cacheImages(images: any) {
  const imageArray = Object.keys(images).map((key) => images[key]);
  return imageArray.map((image) => {
    return new Promise((resolve, reject) => {
      const asset = new ImageAsset();
      asset.loadFileAsync(image)
        .then(loaded => {
          imageSourceCache[image] = asset;
          resolve();
        }).catch(e => {
        reject(e)
      });
      /*ImageSource.fromFile(image)
        .then((imageSrc) => {
          imageSourceCache[image] = imageSrc;
          resolve(imageSrc);
        })
        .catch((e) => {
          reject(e);
        });*/
    });
  });
}

function uri(value) {
  if (value.startsWith("~/")) {
    return path.join(
      knownFolders.currentApp().path,
      value.replace("~/", "")
    );
  }
  return imageSourceCache[value];
}

export const func = {
  cacheImages,
  uri,
};
