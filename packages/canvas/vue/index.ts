export default {
	install(Vue) {
		Vue.registerElement('Canvas', () => require('../').Canvas);
	}
}