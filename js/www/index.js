import { scene } from './scenes/simple'
import * as lasgun from 'lasgun-js'
import * as wasm from 'lasgun-js/lasgun_js_bg'

const canvas = document.getElementById('output')
const ctx = canvas.getContext('2d')
const film = lasgun.film(scene)

let start = Date.now();
lasgun.capture(scene, film)
let end = Date.now();
console.log(`Time ${end - start}ms (${(end - start)/1000}) sec`)

const data = new Uint8ClampedArray(wasm.memory.buffer, film.data(), film.size())
const imageData = new ImageData(data, 512, 512)
ctx.putImageData(imageData, 0, 0)

scene.free()
film.free()
