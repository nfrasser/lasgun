/* global resolve */
/* global reject */

let scene = lasgun.scene({
    eye:  [0.0, 1.0, 4.0],
    view: [-0.1, 0.0, -1.0],
    up: [0.0, 1.0, 0.0],
    ambient: [0.1, 0.1, 0.1],
    width: 512,
    height: 512,
    fov: 60.0,
    sampling: 0,
})

scene.set_radial_background({
    inner: [0.93, 0.87, 0.36],
    outer: [0.94, 0.6, 0.1]
});

let mat0 = lasgun.metal({ eta: [0.9, 0.1, 0.9], k: [0.7, 1.0, 0.7], roughness: 0.25 })
let bunny = scene.add_obj(await lasgun.obj("./meshes/bunny.obj"))

scene.add_point_light({ position: [0.0, 2.0, 3.0], intensity: [0.9, 0.9, 0.9], falloff: [1.0, 0.0, 0.0] })

let root = lasgun.group()
root.add_obj(bunny, mat0)
scene.set_root(root)

resolve(scene)
