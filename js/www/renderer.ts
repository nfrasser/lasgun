declare const self: DedicatedWorkerGlobalScope

// @ts-ignore
import * as wasm from 'lasgun-js/lasgun_js_bg'
import * as lasgun from 'lasgun-js'

function capture(sceneFunctionBody: string) {
    const sceneFunction = new Function(
        'lasgun', 'exports',
        `"use strict";var ${unsafeGlobals.join(',')};\n${sceneFunctionBody}`
    );

    // Array of free-able items
    const allocations: Array<lasgun.Scene | lasgun.Aggregate> = []

    // Expose a new lasgun with just the bare essentials for use
    // in the user-scene code
    const lasgunLite = Object.freeze({
        scene(options: any) {
            let scene = lasgun.scene(options)
            allocations.push(scene)
            return scene
        },
        contents() {
            let contents = lasgun.Aggregate.new()
            allocations.push(contents)
            return contents
        }
    })

    const exports: { scene?: lasgun.Scene } = { scene: null }

    try {
        sceneFunction(lasgunLite, exports)
    } catch (e) {
        // TODO: Use this regexp to get line/column location of error
        // /<anonymous>:[0-9]+:[0-9]+/.exec(e.stack)
        console.log(e)
        return { error: e.toString() }
    }

    if (!exports.scene) {
        let message = 'No scene was exported from the given scene description. Did you forget to set `exports.scene = lasgun.scene({ ... })`?'
        console.error(message)
        return { error: message }
    }

    let scene: lasgun.Scene = exports.scene
    let film: lasgun.Film = lasgun.film(scene)

    let start = Date.now();
    lasgun.capture(scene, film)
    let end = Date.now()
    console.log(`Render time: ${end - start}ms (${(end - start)/1000}) sec`)

    return { scene, film, allocations }
}


/**
 * All the unsafe variables in DedicatedWorkerGlobalScope that user scripts
 * cannot have access to.
 */
const unsafeGlobals = [
    'addEventListener',
    'close',
    'onerror',
    'onmessage',
    'onmessageerror',
    'onrejectionhandled',
    'onunhandledrejection',
    'postMessage',
    'self',
    'DedicatedWorkerGlobalScope',
    'Function',
    'console',
    // Just to be safe
    'wasm',
    'capture',
    'unsafeGlobals',
    'sceneFunction'
]

self.addEventListener('message', (event) => {
    switch (event.data.type) {
    case 'scene':
        let result = capture(event.data.value)
        if (result.error) {
            self.postMessage({ type: 'error', value: result.error })
            break
        }

        let { film, allocations } = result
        self.postMessage({
            type: 'image',
            value: new Uint8ClampedArray(wasm.memory.buffer, film.data(), film.size()),
        })

        for (let alloc of allocations) {
            // Catch double-frees
            try { alloc.free() } catch (e) {}
        }

        break
    }
})

self.postMessage({ type: 'status', value: 'ready' })
