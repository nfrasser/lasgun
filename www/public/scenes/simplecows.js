/* global resolve */
/* global reject */

const scene = lasgun.scene({
  ambient: [0.2, 0.2, 0.2],
  width: 512,
  height: 512
})

const camera = lasgun.camera({
  projection: 'perspective',
  fov: 50,
  origin: [0, 2, 30],
  look: [0, 2, 0],
  up: [0, 1, 0],
  supersampling: 0
})

scene.set_camera(camera)
scene.set_radial_background({
  inner: [0.85, 0.82, 0.6],
  outer: [0.69, 0.85, 0.73]
})

scene.add_point_light({
  position: [200.0, 202.0, 430.0],
  intensity: [0.8, 0.8, 0.8],
  falloff: [1.0, 0.0, 0.0]
})

// Load shared object instances
let planemesh = scene.add_obj(await lasgun.obj('/meshes/plane.obj'))
let buckyballmesh = scene.add_obj(await lasgun.obj('/meshes/buckyball.obj'))

// Materials
let stone = lasgun.metal({ eta: [0.01, 0.01, 0.01], k: [0.7, 0.7, 0.7], roughness: 0.5 })
let grass = lasgun.plastic({ kd: [0.1, 0.7, 0.1], ks: [0.0, 0.0, 0.0], roughness: 0.75 })
let hide = lasgun.plastic({ kd: [0.84, 0.6, 0.53], ks: [0.3, 0.3, 0.3], roughness: 0.05 })

// Scene contents
let contents = lasgun.group()

// Ring of arches
for (let i = 0; i < 6; i++) {
  let p1 = lasgun.group()
  p1.add_cube({ origin: [0.0, 0.0, 0.0], dim: 1.0 }, stone)
  p1.scale(0.8, 4.0, 0.8)
  p1.translate(-2.4, 0.0, -0.4)

  let p2 = lasgun.group()
  p2.add_cube({ origin: [0.0, 0.0, 0.0], dim: 1.0 }, stone)
  p2.scale(0.8, 4.0, 0.8)
  p2.translate(1.6, 0.0, -0.4)

  let s = lasgun.group()
  s.add_sphere({ origin: [0.0, 0.0, 0.0], radius: 1.0 }, stone)
  s.scale(4.0, 0.6, 0.6)
  s.translate(0.0, 4.0, 0.0)

  let arc = lasgun.group()
  arc.add_group(p1)
  arc.add_group(p2)
  arc.add_group(s)

  arc.translate(0.0, 0.0, -10.0)
  arc.rotate_y((i - 1) * 60)

  contents.add_group(arc)
}

// Create some simple cows, transforming each one
for (let [translation, rotation] of [
  [[1.0, 1.3, 14.0], 20.0],
  [[5.0, 1.3, -11.0], 180.0],
  [[-5.5, 1.3, -3.0], -60.0]
]) {
  let cow = lasgun.group()
  cow.scale(1.4, 1.4, 1.4)
  cow.rotate_y(rotation)
  cow.translate(...translation)

  for (let [origin, radius] of [
    [[0.0, 0.0, 0.0], 1.0], // body
    [[0.9, 0.3, 0.0], 0.6], // head
    [[-0.94, 0.34, 0.0], 0.2], // tail
    [[0.7, -0.7, -0.7], 0.3], // lfleg
    [[-0.7, -0.7, -0.7], 0.3], // lrleg
    [[0.7, -0.7, 0.7], 0.3], // rfleg
    [[-0.7, -0.7, 0.7], 0.3] // rrleg
  ]) {
    cow.add_sphere({ origin, radius }, hide)
  }

  contents.add_group(cow)
}

// The Floor
let plane = lasgun.group()
plane.scale(30.0, 30.0, 30.0)
plane.add_obj(planemesh, grass)
contents.add_group(plane)

// Central altar
let buckyball = lasgun.group()
buckyball.scale(1.5, 1.5, 1.5)
buckyball.add_obj(buckyballmesh, stone)
contents.add_group(buckyball)

contents.rotate_x(23.0)
scene.set_root(contents)
resolve(scene)
