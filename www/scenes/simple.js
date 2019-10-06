/* global resolve */
/* global reject */

const scene = lasgun.scene({
  eye: [0.0, 0.0, 800.0],
  view: [0.0, 0.0, -800.0],
  up: [0.0, 1.0, 0.0],
  ambient: [0.2, 0.2, 0.2],
  width: 512,
  height: 512,
  fov: 47.0,
  sampling: 0
})

// Available Materials
let mat0 = lasgun.plastic({ kd: [0.7, 1.0, 0.7], ks: [0.5, 0.7, 0.5], roughness: 0.5 })
let mat1 = lasgun.plastic({ kd: [0.5, 0.5, 0.5], ks: [0.5, 0.7, 0.5], roughness: 0.5 })
let mat2 = lasgun.plastic({ kd: [1.0, 0.6, 0.1], ks: [0.5, 0.7, 0.5], roughness: 0.5 })
let mat3 = lasgun.plastic({ kd: [0.7, 0.6, 1.0], ks: [0.5, 0.4, 0.8], roughness: 0.5 })

let smstdodeca = scene.add_obj(await lasgun.obj("./meshes/smstdodeca.obj"))

// Scene lighting
scene.add_point_light({ position: [-100, 150, 400], intensity: [0.9, 0.9, 0.9], falloff: [1, 0, 0] })
scene.add_point_light({ position: [400, 100, 150], intensity: [0.7, 0, 0.7], falloff: [1, 0, 0] })

// Background
scene.set_radial_background({ inner: [0.26, 0.78, 0.67], outer: [0.1, 0.09, 0.33] })

// Scene contents
let contents = lasgun.group()
contents.add_sphere({ origin: [0, 0, -400], radius: 100 }, mat0)
contents.add_sphere({ origin: [200, 50, -100], radius: 150 }, mat0)
contents.add_sphere({ origin: [0, -1200, -500], radius: 1000 }, mat1)
contents.add_sphere({ origin: [-100, 25, -300], radius: 50 }, mat2)
contents.add_sphere({ origin: [0, 100, -250], radius: 25 }, mat0)
contents.add_cube({ origin: [-200, -125, 0], dim: 100 }, mat3)

contents.add_obj(smstdodeca, mat2)
scene.set_root(contents)
resolve(scene)

