module.exports = function install(Vue) {
	Vue.registerElement('Canvas', () => require('../').Canvas);
}
