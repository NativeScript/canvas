const { cancelAnimationFrame, requestAnimationFrame, time } = require('./raf');
module.exports.cancelAnimationFrame = cancelAnimationFrame;
module.exports.requestAnimationFrame = requestAnimationFrame;
module.exports.time = time;

const { dispatchToMainThread, dispatchToUIThread, isMainThread } = require('./mainthread-helper');

const { queueMacrotask } = require('./macrotask-scheduler');

module.exports.dispatchToMainThread = dispatchToMainThread;
module.exports.dispatchToUIThread = dispatchToUIThread;
module.exports.isMainThread = isMainThread;
module.exports.queueMacrotask = queueMacrotask;
