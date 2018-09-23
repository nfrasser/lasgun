import * as lasgun from 'lasgun-js'
import * as wasm from 'lasgun-js/lasgun_js_bg'
import { scene } from './scenes/simple'

self.addEventListener('message', (event) => {
    console.log(event.data)
})

const film = lasgun.film(scene)

let start = Date.now();
lasgun.capture(scene, film)
let end = Date.now()

console.log(`Time ${end - start}ms (${(end - start)/1000}) sec`)
const data = new Uint8ClampedArray(wasm.memory.buffer, film.data(), film.size())

self.postMessage(data)

scene.free()
film.free()
