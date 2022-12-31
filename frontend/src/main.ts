import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import vuetify from './plugins/vuetify'
import { loadFonts } from './plugins/webfontloader'


// Array.at polyfill
function at(n: any) {
    // ToInteger() abstract op
    n = Math.trunc(n) || 0;
    // Allow negative indexing from the end
    // @ts-ignore
    if (n < 0) n += this.length;
    // OOB access is guaranteed to return undefined
    // @ts-ignore
    if (n < 0 || n >= this.length) return undefined;
    // Otherwise, this is just normal property access
    // @ts-ignore
    return this[n];
}

const TypedArray = Reflect.getPrototypeOf(Int8Array);
for (const C of [Array, String, TypedArray]) {
    Object.defineProperty((C as any).prototype, "at",
                          { value: at,
                            writable: true,
                            enumerable: false,
                            configurable: true });
}
// End Array.at polyfill

loadFonts()

createApp(App)
  .use(router)
  .use(vuetify)
  .mount('#app')
