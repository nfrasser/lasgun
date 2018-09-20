import * as lasgun from "lasgun-js"
import * as smstdodeca from "./smstdodeca"

let scene = lasgun.scene({
    eye: [0.0, 0.0, 800.0],
    view: [0.0, 0.0, -800.0],
    up: [0.0, 1.0, 0.0],
    ambient: [0.3, 0.3, 0.3],
    width: 512,
    height: 512,
    fov: 50.0,
    supersampling: 2,
    threads: 0
})

let mat0 = scene.add_phong_material({
    kd: [0.7, 1.0, 0.7],  ks: [0.5, 0.7, 0.5], shininess: 25
})
let mat1 = scene.add_phong_material({
    kd: [0.5, 0.5, 0.5], ks: [0.5, 0.7, 0.5], shininess: 25
})
let mat2 = scene.add_phong_material({
    kd: [1.0, 0.6, 0.1], ks: [0.5, 0.7, 0.5], shininess: 25
})
let mat3 = scene.add_phong_material({
    kd: [0.7, 0.6, 1.0], ks: [0.5, 0.4, 0.8], shininess: 25
})

scene.add_point_light({
    position: [-100.0, 150.0, 400.0],
    intensity: [0.9, 0.9, 0.9],
    falloff: [1.0, 0.0, 0.0]
})

scene.add_point_light({
    position: [400.0, 100.0, 150.0],
    intensity: [0.7, 0.0, 0.7],
    falloff: [1.0, 0.0, 0.0]
})

let contents = lasgun.Aggregate.new()
contents.add_sphere({
    origin: [0.0, 0.0, -400.0],
    radius: 100.0
}, mat0)
contents.add_sphere({
    origin: [200.0, 50.0, -100.0],
    radius: 150.0
}, mat0)
contents.add_sphere({
    origin: [0.0, -1200.0, -500.0],
    radius: 1000.0
}, mat1)
contents.add_sphere({
    origin: [-100.0, 25.0, -300.0],
    radius: 50.0
}, mat2)
contents.add_sphere({
    origin: [0.0, 100.0, -250.0],
    radius: 25.0
}, mat0)
contents.add_cube({
    origin: [-200.0, -125.0, 0.0],
    dim: 100.0,
}, mat3)
contents.add_mesh(smstdodeca, mat2)

scene.set_contents(contents)
