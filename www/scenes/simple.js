/* global resolve */
/* global reject */

const scene = lasgun.scene({
  eye: [0.0, 0.0, 800.0],
  view: [0.0, 0.0, -800.0],
  up: [0.0, 1.0, 0.0],
  ambient: [0.3, 0.3, 0.3],
  width: 512,
  height: 512,
  fov: 50.0,
  sampling: 0
})

// Available Materials
let mat0 = scene.add_phong_material({ kd: [0.7, 1.0, 0.7], ks: [0.5, 0.7, 0.5], shininess: 25 })
let mat1 = scene.add_phong_material({ kd: [0.5, 0.5, 0.5], ks: [0.5, 0.7, 0.5], shininess: 25 })
let mat2 = scene.add_phong_material({ kd: [1.0, 0.6, 0.1], ks: [0.5, 0.7, 0.5], shininess: 25 })
let mat3 = scene.add_phong_material({ kd: [0.7, 0.6, 1.0], ks: [0.5, 0.4, 0.8], shininess: 25 })

let smstdodeca = scene.add_obj(await lasgun.mesh("./meshes/smstdodeca.obj"))

// Scene lighting
scene.add_point_light({ position: [-100, 150, 400], intensity: [0.9, 0.9, 0.9], falloff: [1, 0, 0] })
scene.add_point_light({ position: [400, 100, 150], intensity: [0.7, 0, 0.7], falloff: [1, 0, 0] })

// Background
scene.set_radial_background({ inner: [67, 198, 172], outer: [25, 22, 84] })

// Scene contents
let contents = lasgun.group()
contents.add_sphere({ origin: [0, 0, -400], radius: 100 }, mat0)
contents.add_sphere({ origin: [200, 50, -100], radius: 150 }, mat0)
contents.add_sphere({ origin: [0, -1200, -500], radius: 1000 }, mat1)
contents.add_sphere({ origin: [-100, 25, -300], radius: 50 }, mat2)
contents.add_sphere({ origin: [0, 100, -250], radius: 25 }, mat0)
contents.add_cube({ origin: [-200, -125, 0], dim: 100 }, mat3)

contents.add_mesh(smstdodeca, mat2)
scene.set_root(contents)
resolve(scene)

