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

scene.set_radial_background({ inner: [237, 222, 93], outer: [240, 152, 25] });

let mat0 = scene.add_metal_material({ eta: [0.9, 0.1, 0.9], k: [0.7, 1.0, 0.7], roughness: 0.25 })
let bunny = scene.add_obj(await lasgun.mesh("./meshes/bunny.obj"))

scene.add_point_light({ position: [0.0, 2.0, 3.0], intensity: [0.9, 0.9, 0.9], falloff: [1.0, 0.0, 0.0] })

let root = lasgun.group()
root.add_mesh(bunny, mat0)
scene.set_root(root)

resolve(scene)
