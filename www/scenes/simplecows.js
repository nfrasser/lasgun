/* global resolve */
/* global reject */

const scene = lasgun.scene({
    eye: [0.0, 2.0, 30.0],
    view: [0.0, 0.0, -1.0],
    up: [0.0, 1.0, 0.0],
    ambient: [0.4, 0.4, 0.4],
    width: 512,
    height: 512,
    fov: 50.0,
    sampling: 0
})
let contents = lasgun.contents();

scene.add_point_light({
    position: [200.0, 202.0, 430.0],
    intensity: [0.8, 0.8, 0.8],
    falloff: [1.0, 0.0, 0.0]
})

let stone = scene.add_phong_material({ kd: [0.8, 0.7, 0.7], ks: [0.0, 0.0, 0.0], shininess: 0 })
let grass = scene.add_phong_material({ kd: [0.1, 0.7, 0.1], ks: [0.0, 0.0, 0.0], shininess: 0 })
let hide = scene.add_phong_material({ kd: [0.84, 0.6, 0.53], ks: [0.3, 0.3, 0.3], shininess: 20 })

// Ring of arches
for (let i = 0; i < 6; i++) {

    let p1 = lasgun.contents()
    p1.add_cube({ origin: [0.0, 0.0, 0.0], dim: 1.0 }, stone)
    p1.scale(0.8, 4.0, 0.8)
    p1.translate(-2.4, 0.0, -0.4)

    let p2 = lasgun.contents()
    p2.add_cube({ origin: [0.0, 0.0, 0.0], dim: 1.0 }, stone)
    p2.scale(0.8, 4.0, 0.8)
    p2.translate(1.6, 0.0, -0.4)

    let s = lasgun.contents()
    s.add_sphere({ origin: [0.0, 0.0, 0.0], radius: 1.0 }, stone)
    s.scale(4.0, 0.6, 0.6)
    s.translate(0.0, 4.0, 0.0)

    let arc = lasgun.contents()
    arc.add_node(p1)
    arc.add_node(p2)
    arc.add_node(s)

    arc.translate(0.0, 0.0, -10.0)
    arc.rotate_y(((i-1) * 60))

    contents.add_node(arc)
}

// Create some simple cows, transforming each one
for (let [translation, rotation] of [
    [[1.0, 1.3, 14.0], 20.0],
    [[5.0, 1.3, -11.0], 180.0],
    [[-5.5, 1.3, -3.0], -60.0],
]) {
    let cow = lasgun.contents()
    cow.scale(1.4, 1.4, 1.4)
    cow.rotate_y(rotation)
    cow.translate(...translation)

    for (let [origin, radius] of [
        [[0.0, 0.0, 0.0], 1.0],           // body
        [[0.9, 0.3, 0.0], 0.6],       // head
        [[-0.94, 0.34, 0.0], 0.2],    // tail
        [[0.7, -0.7, -0.7], 0.3],      // lfleg
        [[-0.7, -0.7, -0.7], 0.3],     // lrleg
        [[0.7, -0.7, 0.7], 0.3],       // rfleg
        [[-0.7, -0.7, 0.7], 0.3],      // rrleg
    ]) {
        cow.add_sphere({ origin, radius }, hide)
    }

    contents.add_node(cow)
}

// The Floor
let plane = lasgun.contents()
plane.scale(30.0, 30.0, 30.0)
plane.add_mesh(await lasgun.mesh('./meshes/plane.obj'), grass)
contents.add_node(plane)

// Central altar
let buckyball = lasgun.contents()
buckyball.scale(1.5, 1.5, 1.5)
buckyball.add_mesh(await lasgun.mesh('./meshes/buckyball.obj'), stone)
contents.add_node(buckyball)

contents.rotate_x(23.0)
scene.set_contents(contents)
resolve(scene)
