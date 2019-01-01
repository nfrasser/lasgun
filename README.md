Lasgun [![Build Status](https://travis-ci.org/nfrasser/lasgun.svg?branch=master)](https://travis-ci.org/nfrasser/lasgun)
===

A ray tracer that works in the browser.

## Prerequisites

Install the latest releases of the following

- [Rust](https://www.rust-lang.org/en-US/install.html)
- [Node.js](https://nodejs.org/)
- [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/)

## Build the `lasgun` library

```
cargo build --release
```

## Render a sample scene

```
cargo run --example simple --release
```

Renders `simple.png`.
[See more examples](https://github.com/nfrasser/lasgun/tree/master/src/examples).

## Build lasgun for the browser

```
wasm-pack build js
```

Outputs to `js/pkg`

## Use lasgun in the browser

Use this in Node.js/Webpack via [NPM](https://npmjs.com) by linking the `pkg`
directory

```
cd js/pkg
npm link
cd -
```

Run the lasgun web client at [localhost:8080](http://localhost:8080)

```
npm install
npm link lasgun-js
npm start
```

Import in JavaScript with a module loader that supports WebAssembly.

```js
import * as lasgun from 'lasgun-js'

let scene = lasgun.scene({
    eye: [0, 0, 0],
    view: [0, 0, 1],
    up: [0, 1, 0],
    width: 512,
    height: 512,
    fov: 45.0
})

let mat = scene.add_phong_material({
    kd: [0.7, 1.0, 0.7],
    ks: [0.5, 0.7, 0.5],
    shininess: 25
})

let node = lasgun.group()
node.add_sphere({
    origin: [0, 0, 100],
    radius: 50
},  mat)
scene.set_root(node)

let film = lasgun.film(scene)
lasgun.capture(scene, film)

let pixels = film.pixels()
```

The resulting `pixels` is a `Uint8Array` buffer with image data for display in a
browser environment. Each consecutive group of four 8-bit integers represents a
pixel with RGBA channels. The pixels are arranged in row-major order.

Example pixel usage with HTML5 Canvas

```js
const canvas = document.getElementById('canvas')
const ctx = canvas.getContext('2d')

let imageData = ctx.getImageData(0, 0, 512, 512)
imageData.data.set(pixels)
ctx.putImageData(data)
```

## Resources

* [Rust and WebAssembly Book](https://rustwasm.github.io/book/)
* [`wasm-bindgen` API Reference](https://rustwasm.github.io/wasm-bindgen/)
* [WebAssembly](https://developer.mozilla.org/en-US/docs/WebAssembly)
* [Canvas API](https://developer.mozilla.org/en-US/docs/Web/API/Canvas_API)

## License

MIT
