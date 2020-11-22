declare const self: DedicatedWorkerGlobalScope

// @ts-ignore
import * as wasm from 'lasgun-js/lasgun_js_bg'
import * as lasgun from 'lasgun-js'

type Vec3f = [ number, number, number ]

async function capture(sceneFunctionBody: string, width: number, height: number) {

    // Can't call directly because webpack rewrites this :(
    const AsyncFunction = (new Function("return Object.getPrototypeOf(async function () {}).constructor"))();
    const sceneFunction = new AsyncFunction(
        'lasgun', 'resolve', 'reject',
        `"use strict";var ${unsafeGlobals.join(',')};\n${sceneFunctionBody}`
    );

    // Array of free-able items
    const allocations: Array<lasgun.Scene | lasgun.Aggregate | lasgun.Material> = []

    // Expose a new lasgun with just the bare essentials for use
    // in the user-scene code
    const lasgunLite = Object.freeze({
        async obj(url: string): Promise<string> {
            return await (await fetch(url)).text()
        },
        scene(settings: any): lasgun.Scene {
            let scene = lasgun.scene(settings)
            allocations.push(scene)
            return scene
        },
        camera(settings: any): lasgun.Camera {
            let camera = lasgun.camera(settings)
            allocations.push(camera)
            return camera
        },
        group(): lasgun.Aggregate {
            let contents = lasgun.Aggregate.new()
            allocations.push(contents)
            return contents
        },
        matte(settings: { kd: Vec3f, sigma?: number }): lasgun.Material {
            let mat = lasgun.Material.matte(settings)
            allocations.push(mat)
            return mat
        },
        plastic(settings: { kd: Vec3f, ks: Vec3f, roughness?: number }): lasgun.Material {
            let mat = lasgun.Material.plastic(settings)
            allocations.push(mat)
            return mat
        },
        metal(settings: {
            eta: Vec3f, k: Vec3f,
            roughness?: number, u_roughness?: number, v_roughness?: number
        }): lasgun.Material {
            let mat = lasgun.Material.metal(settings)
            allocations.push(mat)
            return mat
        },
        mirror(settings?: { kr?: Vec3f }): lasgun.Material {
            let mat = lasgun.Material.mirror(settings || {})
            allocations.push(mat)
            return mat
        },
        glass(settings?: { kr?: Vec3f, kt?: Vec3f, eta?: number }): lasgun.Material {
            let mat = lasgun.Material.glass(settings || {})
            allocations.push(mat)
            return mat
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
        const message = 'No scene was exported from the given scene description. Did you forget to set `exports.scene = lasgun.scene({ ... })`?'
        console.error(message)
        return { error: message }
    }

    const scene: lasgun.Scene = exports.scene
    const start = Date.now();

    const area = width * height
    const subsets = shuffle(Array.from(Array(Math.min(100, area)).keys()))
    const film = lasgun.film(width, height)
    const startPtr = film.data_ptr()
    const endPtr = startPtr + area * 4;

    console.log(film.size())
    // let hunkCount = lasgun.hunk_count(scene)
    // let hunk = lasgun.Hunk.new()
    let root = lasgun.Accel.from(scene)

    // Stream bits of the scene over to main as we go
    for (const k of subsets) {
        lasgun.capture_subset(k, subsets.length, root, film)
        self.postMessage({
            type: 'progress',
            value: {
                output: wasm.memory.buffer.slice(startPtr, endPtr),
                progress: (k + 1) / subsets.length
            }
        })
    }

    film.free()
    root.free()
    let end = Date.now()

    // Deallocate everything
    for (let alloc of allocations) {
        // Catch double-frees
        try { alloc.free() } catch (e) {}
    }

    // Return the start/end timestamps
    return { start, end }
}

function shuffle(a: any[]) {
    var j, x, i;
    for (i = a.length - 1; i > 0; i--) {
        j = Math.floor(Math.random() * (i + 1));
        x = a[i];
        a[i] = a[j];
        a[j] = x;
    }
    return a;
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
        const { scene, width, height } = JSON.parse(event.data.value)
        const result = await capture(scene, width, height)
        if (result.error) {
            self.postMessage({ type: 'error', value: result.error })
        } else {
            self.postMessage({ type: 'done', value: result })
        }
        break
    }
})

self.postMessage({ type: 'status', value: 'ready' })
