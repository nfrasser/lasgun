/* global resolve */
/* global reject */

const scene = lasgun.scene({
  ambient: [0.2, 0.2, 0.2],
  smoothing: true // smooth meshes with provided vertex normals, not applicable to this scene
})

const camera = lasgun.camera({
  projection: 'perspective', // or 'orthographic'
  fov: 47,
  origin: [0, 0, 800],
  look: [0, 0, 0],
  up: [0, 1, 0],
  supersampling: 0,
  aperture: 0
})

scene.set_camera(camera)

// Available Materials
const mat0 = lasgun.plastic({ kd: [0.7, 1.0, 0.7], ks: [0.5, 0.7, 0.5], roughness: 0.5 })
const mat1 = lasgun.plastic({ kd: [0.5, 0.5, 0.5], ks: [0.5, 0.7, 0.5], roughness: 0.5 })
const mat2 = lasgun.plastic({ kd: [1.0, 0.6, 0.1], ks: [0.5, 0.7, 0.5], roughness: 0.5 })
const mat3 = lasgun.plastic({ kd: [0.7, 0.6, 1.0], ks: [0.5, 0.4, 0.8], roughness: 0.5 })
// const metal = lasgun.metal({ eta: [0.9, 0.1, 0.9], k: [0.7, 1.0, 0.7], roughness: 0.25 })
// const glass = lasgun.glass({ kr: [0.9, 0.1, 0.4], kt: [0.5, 0.7, 0.5], eta: 1.25 });
// const mirror = lasgun.mirror({ kr: [0.5, 0.5, 0.5] });

// Available meshes
const smstdodeca = scene.add_obj(await lasgun.obj('/meshes/smstdodeca.obj'))
// const plane = scene.add_obj(await lasgun.obj("/meshes/plane.obj"))
// const skull = scene.add_obj(await lasgun.obj("/meshes/skull.obj"))
// const buckyball = scene.add_obj(await lasgun.obj("/meshes/buckyball.obj"))
// const bunny = scene.add_obj(await lasgun.obj("/meshes/bunny.obj"))

// Scene lighting
scene.add_point_light({
  position: [-100, 150, 400],
  intensity: [0.9, 0.9, 0.9],
  falloff: [1, 0, 0]
})
scene.add_point_light({ position: [400, 100, 150], intensity: [0.7, 0, 0.7], falloff: [1, 0, 0] })

// Background
scene.set_radial_background({ inner: [0.26, 0.78, 0.67], outer: [0.1, 0.09, 0.33] })

// Scene contents
const contents = lasgun.group()
contents.add_sphere({ origin: [0, 0, -400], radius: 100 }, mat0)
contents.add_sphere({ origin: [200, 50, -100], radius: 150 }, mat0)
contents.add_sphere({ origin: [0, -1200, -500], radius: 1000 }, mat1)
contents.add_sphere({ origin: [-100, 25, -300], radius: 50 }, mat2)
contents.add_sphere({ origin: [0, 100, -250], radius: 25 }, mat0)
contents.add_cube({ origin: [-200, -125, 0], dim: 100 }, mat3)

contents.add_obj(smstdodeca, mat2)
// contents.scale(0.5, 0.5, 0.5)
// contents.translate(-1.25, -0.5, 0)
// contents.rotate_z(45) // rotate 45 degress around the z axis
// contents.add_group(lasgun.group()) // Add another transformed group

scene.set_root(contents)
resolve(scene)
