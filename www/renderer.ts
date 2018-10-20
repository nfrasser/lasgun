declare const self: DedicatedWorkerGlobalScope

// @ts-ignore
import * as wasm from 'lasgun-js/lasgun_js_bg'
import * as lasgun from 'lasgun-js'

async function capture(sceneFunctionBody: string) {

    // Can't call directly because webpack rewrites this :(
    const AsyncFunction = (new Function("return Object.getPrototypeOf(async function () {}).constructor"))();
    const sceneFunction = new AsyncFunction(
        'lasgun', 'resolve', 'reject',
        `"use strict";var ${unsafeGlobals.join(',')};\n${sceneFunctionBody}`
    );

    // Array of free-able items
    const allocations: Array<lasgun.Scene | lasgun.Aggregate> = []

    // Expose a new lasgun with just the bare essentials for use
    // in the user-scene code
    const lasgunLite = Object.freeze({
        scene(options: any): lasgun.Scene {
            let scene = lasgun.scene(options)
            allocations.push(scene)
            return scene
        },
        group(): lasgun.Aggregate {
            let contents = lasgun.Aggregate.new()
            allocations.push(contents)
            return contents
        },
        async mesh(url: string): Promise<string> {
            let response = await fetch(url)
            return await response.text()
        }
    })

    const exports: { scene?: lasgun.Scene } = { scene: null };

    try {
        exports.scene = await new Promise<lasgun.Scene>(async (resolve, reject) => {
            try {
                await sceneFunction(lasgunLite, resolve, reject)
            } catch (e) {
                reject(e)
            }
        })
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

    // Prepare the browser for rendering the scene
    self.postMessage({
        type: 'scene',
        value: {
            width: scene.width(),
            height: scene.height()
        }
    })

    let start = Date.now();

    let hunkCount = lasgun.hunk_count(scene)
    let hunk = lasgun.Hunk.new()
    let root = lasgun.Accel.from(scene)
    let mem: ArrayBuffer = wasm.memory.buffer

    // Stream bits of the scene over to main as we go
    for (let i = 0; i < hunkCount; i++) {
        lasgun.capture_hunk(i, scene, root, hunk)
        let start = hunk.as_ptr();
        let end = start + 1024;
        self.postMessage({
            type: 'hunk',
            value: {
                x: hunk.x,
                y: hunk.y,
                data: new Uint8ClampedArray(mem.slice(start, end))
            }
        })
    }

    hunk.free()
    let end = Date.now()

    // Deallocate everything
    for (let alloc of allocations) {
        // Catch double-frees
        try { alloc.free() } catch (e) {}
    }

    // Return the start/end timestamps
    return { start, end }
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
    'AsyncFunction',
    'console',
    // Just to be safe
    'wasm',
    'capture',
    'unsafeGlobals',
    'sceneFunction'
]

self.addEventListener('message', async (event) => {
    switch (event.data.type) {
    case 'scene':
        let result = await capture(event.data.value)
        if (result.error) {
            self.postMessage({ type: 'error', value: result.error })
        } else {
            self.postMessage({ type: 'done', value: result })
        }
        break
    }
})

self.postMessage({ type: 'status', value: 'ready' })
