use std::mem;
use typed_arena::Arena;
use partition::partition;
use crate::{
    space::*,
    shape::*,
    ray::Ray,
    primitive::{Primitive, geometry::Geometry},
    interaction::SurfaceInteraction,
    scene::{Scene, MaterialRef, ObjRef, description::{self, SceneNode}}
};

// Hiding my ugly dynamic dispatch type.
// Should have the lifetime of the referenced Scene instance.
type PrimBox<'s> = Box<dyn Primitive + 's>;

// (In)convenience types, mostly for documentation
type BVHSplitAxis = usize;
type BVHPrimNumber = usize;
type BVHPrimCount = usize;

// Upper SAH buckets
const BVH_NBUCKETS: usize = 12;

// Morton enconding constants
// see PBRT v3 p268
const MORTON_BITS: u32 = 10;
const MORTON_SCALE: u32 = 1 << MORTON_BITS;

// Radix sort constants for sorting morton constants
const RADIX_BITS_PER_PASS: u32 = 6;
const RADIX_NBITS: u32 = 30;
const RADIX_NPASSES: u32 = RADIX_NBITS / RADIX_BITS_PER_PASS;
const RADIX_NBUCKETS: usize = 1 << RADIX_BITS_PER_PASS as usize;
const RADIX_BITMASK: u32 = (1 << RADIX_BITS_PER_PASS) - 1;


/// Bounding Volume Hierarchy Acceleration structure
/// Its lifetime depends on the scene whose content it holds
/// Uses Linear Bounding Volume hierarchy strategy
pub struct BVHAccel<'s> {
    primitives: Vec<PrimBox<'s>>,

    /// BVH tree nodes arranged in linear memory
    nodes: Vec<LinearBVHNode>,

    /// Transform matrix reference
    transform: &'s Transformation,

    // Used if all the primitives share the same material
    // This is generally for triangle meshes
    material: Option<MaterialRef>,

    // The order in which primitives are accessed following BVH construction.
    // Each element is an index into the primitives vec. The offset indeces on
    // each nodes member referes to an index into this vec.
    order: Vec<BVHPrimNumber>,

    // Limit to how many primitives there may be per node
    max_prims_per_node: u8
}

/// Deterministic sorting construct for objects in 3D space
/// See PBRT book v3 page 268
#[derive(Copy, Clone, PartialEq)]
struct MortonPrimitive {
    pub index: BVHPrimNumber,
    pub code: u32 // morton code
}

/// Information about each primitive stored in a BVHAccel
struct BVHPrimitiveInfo {
    number: BVHPrimNumber,
    bounds: Bounds,
    centroid: Point
}

/// For Upper SAH buckets
#[derive(Copy, Clone)]
struct BVHBucketInfo {
    count: usize,
    bounds: Bounds
}

// The lifetime of this is tied to the memory area where this is allocated
enum BVHNodeType<'a> {

    /// Holds an index into the primitives array in the parent BVHAccel
    /// and the total number of primitives in this node
    Leaf(BVHPrimNumber, BVHPrimCount),

    // i.e., not a child node.
    Interior(BVHSplitAxis, &'a BVHBuildNode<'a>, &'a BVHBuildNode<'a>)
}

/// A BVHAccel tree entry. Tied to the lifetime of the memory arena used to
/// allocate this node.
struct BVHBuildNode<'a> {
    content: BVHNodeType<'a>,
    bounds: Bounds
}

// Cluster of primitives that can processeded for bounds checks independently
// Lifetime is tied to the referenced build nodes (allocated in an arena)
struct LBVHTreelet<'a> {
    pub start: BVHPrimNumber,
    pub node: &'a BVHBuildNode<'a>
}

#[derive(Copy, Clone)]
enum LinearBVHNodeType {
    // First/second child offset and prim count
    Leaf(u32, u16),
    // split axis and offset into parent array
    Interior(u8, u32)
}

#[derive(Copy, Clone)]
struct LinearBVHNode {
    pub bounds: Bounds,
    pub content: LinearBVHNodeType
}


impl<'s> BVHAccel<'s> {
    pub fn from(scene: &'s Scene) -> BVHAccel<'s> {
        BVHAccel::from_aggregate(scene, &scene.root)
    }

    /// Create a new BVH structure from the given triangle mesh
    /// This structure will be composed entirely of Triangles
    fn from_mesh(scene: &'s Scene, mesh: &ObjRef, material: &MaterialRef)
    -> BVHAccel<'s> {
        let triangles: Vec<PrimBox<'s>> = scene.mesh(mesh).unwrap()
            .into_iter()
            .map(|t| -> PrimBox<'s> { Box::new(t) })
            .collect();
        let per_node = triangles.len();
        BVHAccel::new(triangles, &transform::ID, Some(*material), per_node)
    }

    fn from_aggregate(scene: &'s Scene, aggregate: &'s description::Aggregate) -> BVHAccel<'s> {
        let primitives: Vec<PrimBox<'s>> = aggregate.contents.iter()
        .map(|node| match node {
            SceneNode::Geometry(shape, mat) =>
                geometry(shape, mat),
            SceneNode::Mesh(obj, mat) =>
                Box::new(BVHAccel::from_mesh(scene, obj, mat)),
            SceneNode::Group(aggregate) =>
                Box::new(BVHAccel::from_aggregate(scene, aggregate))
        }).collect();
        let per_node = primitives.len();
        BVHAccel::new(primitives, &aggregate.transform, None, per_node)
    }

    fn new(
        primitives: Vec<PrimBox<'s>>,
        transform: &'s Transformation,
        material: Option<MaterialRef>,
        max_prims_per_node: usize
    ) -> BVHAccel<'s> {

        let arena = Arena::with_capacity(1024 * 1024);
        let nprims = primitives.len();
        let prim_info: Vec<BVHPrimitiveInfo> = primitives.iter()
            .enumerate()
            .map(|(i, prim)| BVHPrimitiveInfo::new(i, prim.bound()))
            .collect();

        let mut accel = BVHAccel {
            primitives,
            nodes: vec![],
            order: vec![std::usize::MAX; nprims], // Fill with dummy values
            transform,
            material,
            max_prims_per_node: max_prims_per_node.min(255) as u8
        };

        let mut total_nodes = 0;
        let node = accel.build(&arena, &prim_info, &mut total_nodes);
        accel.nodes = vec![ // Fill with dummy nodes
            LinearBVHNode {
                bounds: Bounds::none(),
                content: LinearBVHNodeType::Leaf(0, 0)
            }
        ; total_nodes];

        accel.flatten_bvh_tree(node, &mut 0);
        accel
    }

    /// Build the BVH tree with the hierarchical linear bounding volume hierachy algorithm
    fn build<'a>(
        &mut self,
        arena: &'a Arena<BVHBuildNode<'a>>,
        prim_info: &Vec<BVHPrimitiveInfo>,
        total_nodes: &mut BVHPrimCount
    ) -> &'a BVHBuildNode<'a> {
        // Compute bounding box of all primitive centroids
        let bounds = prim_info.iter()
            .fold(Bounds::none(), |bounds, info| bounds.union(&info.bounds));

        // Compute Morton indeces of primitives
        // TODO: Parallelize
        let mut morton_prims = prim_info.iter().map(|info| {
            let centroid_offset = bounds.offset(&info.centroid);
            let morton = MortonPrimitive {
                index: info.number,
                code: encode_morton_3(&(centroid_offset * MORTON_SCALE.into()))
            };
            morton
        }).collect();

        // Constant-time sort. Once this is done, the morton primitives are
        // arranged in a recursive pattern such that primitives in opposite
        // subdivisions of the vector are in spatially different quadrants
        radix_sort(&mut morton_prims);

        // Create LBVH treelets at bottom of BVH
        // Find invervals for primitives for each treelet
        let mut treelets: Vec<LBVHTreelet> = Vec::new();
        let mut start = 0;
        let mut ordered_prims_offset = 0;

        // Create and return SAH BVH from LBVH treelets
        // TODO: Parallelize
        let mut total = 0;
        for end in 1..=(morton_prims.len()) {
            let mask = 0b00111111111111000000000000000000;
            if end == morton_prims.len() || (
                (morton_prims[start].code & mask) != (morton_prims[end].code & mask)
            ) {
                // Add entry to treelets for this treelet
                let mut nodes_created = 0;
                let nprims = end - start;
                let maxnodes = 2 * nprims;
                let nodes = arena.alloc_extend((0..maxnodes)
                    .map(|_| BVHBuildNode {
                        content: BVHNodeType::Leaf(0, 0),
                        bounds: Bounds::none()
                    }));

                let first_bit_index = 29 - 12; // Something to do with Morton encoding bit positions, I think
                let (node, _) = self.emit_lbvh(
                    nodes, &morton_prims[start..], nprims, prim_info,
                    &mut nodes_created, &mut ordered_prims_offset, first_bit_index);

                total += nodes_created;

                treelets.push(LBVHTreelet { start: start as BVHPrimNumber, node });
                start = end;
            }
        }
        *total_nodes += total;

        // Create and return Surface Area Heuristic BVH from LBVH treelets
        let mut finished_treelets: Vec<&'a BVHBuildNode<'a>> = treelets.iter()
        .map(|treelet| treelet.node).collect();

        BVHAccel::build_upper_sah(arena, &mut finished_treelets[..], total_nodes)
    }

    /// Creates and returns LBVH nodes and returns the the total of nodes
    /// created. Also calculates the prim_order order and returns yet-unused
    /// build nodes.
    fn emit_lbvh<'a>(
        &mut self,
        nodes: &'a mut [BVHBuildNode<'a>],
        morton_prims: &[MortonPrimitive],
        nprims: BVHPrimCount,
        prim_info: &Vec<BVHPrimitiveInfo>,
        total_nodes: &mut BVHPrimCount,
        ordered_prims_offset: &mut usize,
        bit_index: i32
    ) -> (&'a BVHBuildNode<'a>, &'a mut [BVHBuildNode<'a>]) {

        if bit_index == -1 || nprims < self.max_prims_per_node as usize {
            // Create and return leaf node of LBVH treelet
            let first_prim_offset = *ordered_prims_offset;
            let (node, rest) = nodes.split_at_mut(1);
            let node = &mut node[0];
            *ordered_prims_offset += nprims;
            *total_nodes += 1;

            let bounds = (0..nprims).fold(Bounds::none(), |bounds, i| {
                let prim_index = morton_prims[i].index;
                self.order[first_prim_offset + i] = prim_index;
                bounds.union(&prim_info[prim_index].bounds)
            });

            node.init_leaf(first_prim_offset, nprims, bounds);
            return (node, rest)
        }

        let mask = 1 << bit_index;

        // Advance to next subtree level if there's no LBVH split for this bit
        if (morton_prims[0].code & mask)
        == (morton_prims[nprims - 1].code & mask) {
            return self.emit_lbvh(nodes, morton_prims, nprims, prim_info, total_nodes,
                ordered_prims_offset, bit_index - 1);
        }

        // Find LVBH split point for this dimension
        let (mut search_start, mut search_end) = (0, nprims - 1);
        while search_start + 1 != search_end {
            let mid = (search_start + search_end) / 2;
            if (morton_prims[search_start].code & mask)
            == (morton_prims[mid].code & mask) {
                search_start = mid
            } else {
                search_end = mid
            }
        }

        let split_offset = search_end;
        let (node, nodes) = nodes.split_at_mut(1);
        let node = &mut node[0];
        *total_nodes += 1;

        // Create and return interial LBVH node
        let (lbvh0, nodes) = self.emit_lbvh(
            nodes, morton_prims, split_offset,
            prim_info, total_nodes,
            ordered_prims_offset, bit_index - 1);

        let (lbvh1, nodes) = self.emit_lbvh(
            nodes, &morton_prims[split_offset..], nprims - split_offset,
            prim_info, total_nodes,
            ordered_prims_offset, bit_index - 1);

        let axis = (bit_index % 3) as BVHSplitAxis;
        node.init_interior(axis, lbvh0, lbvh1);
        (node, nodes)
    }

    /// Use surface area heuristic to build BVH
    fn build_upper_sah<'a>(
        arena: &'a Arena<BVHBuildNode<'a>>,
        treelet_roots: &mut [&'a BVHBuildNode<'a>],
        total_nodes: &mut BVHPrimCount
    ) -> &'a BVHBuildNode<'a> {
        let ncount = treelet_roots.len(); // node count
        if ncount == 1 { return treelet_roots[0] }; // Base case

        let node = arena.alloc(BVHBuildNode {
            content: BVHNodeType::Leaf(0, 0),
            bounds: Bounds::none()
        });
        *total_nodes += 1;

        // Compute bounds of all nodes under this HLBVH node
        let bounds = treelet_roots.iter()
        .fold(Bounds::none(), |bounds, root| bounds.union(&root.bounds));

        // Compute bound of HLBVH node centroids
        let centroid_bounds = treelet_roots.iter()
        .fold(Bounds::none(), |bounds, root| {
            let centroid = 0.5 * (root.bounds.min + root.bounds.max.to_vec());
            bounds.point_union(&centroid)
        });

        // Choose split dimension
        let dim = centroid_bounds.maximum_extent();

        // Allocate and initialize BucketInfo for SAH partition buckets
        let mut buckets: [BVHBucketInfo; BVH_NBUCKETS] = [
            BVHBucketInfo { count: 0, bounds: Bounds::none() }; BVH_NBUCKETS
        ];
        for root in treelet_roots.iter() {
            let centroid = (root.bounds.min[dim] + root.bounds.max[dim]) * 0.5;
            let b0 = (centroid - centroid_bounds.min[dim]) /
                (centroid_bounds.max[dim] - centroid_bounds.min[dim]);
            let mut b = ((BVH_NBUCKETS as f64 * b0) as u32) as usize;
            if b == BVH_NBUCKETS { b = BVH_NBUCKETS - 1 };
            buckets[b].count += 1;
            buckets[b].bounds = buckets[b].bounds.union(&root.bounds);
        }

        // Compute costs for splitting after each bucket
        let mut cost: [f64; BVH_NBUCKETS] = [0.0; BVH_NBUCKETS];
        for i in 0..BVH_NBUCKETS {
            let (b0, count0) = (0..=i).fold((Bounds::none(), 0), |(b, count), j| {
                (b.union(&buckets[j].bounds), count + buckets[j].count)
            });

            let (b1, count1) = ((i+1)..BVH_NBUCKETS).fold((Bounds::none(), 0), |(b, count), j| {
                (b.union(&buckets[j].bounds), count + buckets[j].count)
            });

            cost[i] = 0.125 + (
                count0 as f64 * b0.surface_area() + count1 as f64 * b1.surface_area()
            ) / bounds.surface_area();
        }

        // Find bucket to split at that minimizes SAH metric
        let min_cost_split_bucket = cost.iter().enumerate().fold(0, |bucket, (i, c)| {
            if *c < cost[bucket] { i } else { bucket }
        });

        // Split nodes and create interior HLBVH SAH node
        let (lo_roots, hi_roots) = partition(treelet_roots, |node| {
            let centroid = 0.5 * (node.bounds.min[dim] + node.bounds.max[dim]);
            let b0 = (centroid - centroid_bounds.min[dim]) /
                (centroid_bounds.max[dim] - centroid_bounds.min[dim]);
            let mut b = ((BVH_NBUCKETS as f64 * b0) as u32) as usize;
            if b == BVH_NBUCKETS { b = BVH_NBUCKETS - 1 };
            b <= min_cost_split_bucket
        });
        node.init_interior(dim,
            BVHAccel::build_upper_sah(arena, lo_roots, total_nodes),
            BVHAccel::build_upper_sah(arena, hi_roots, total_nodes));

        node
    }

    // a is the lifetime of the arena as usual
    // v is the lifetime of the parent LinearBVHNode vec
    fn flatten_bvh_tree<'a, 'v>(
        &mut self,
        node: &'a BVHBuildNode<'a>,
        offset: &mut usize
    ) -> usize {
        let my_offset = *offset; *offset += 1;
        self.nodes[my_offset].bounds = node.bounds;
        match node.content {
            BVHNodeType::Leaf(prim_offset, nprims) => {
                self.nodes[my_offset].content =
                    LinearBVHNodeType::Leaf(prim_offset as u32, nprims as u16);
            }
            BVHNodeType::Interior(axis, c0, c1) => {
                let (_, second_offset) = (
                    self.flatten_bvh_tree(c0, offset),
                    self.flatten_bvh_tree(c1, offset)
                );
                self.nodes[my_offset].content
                    = LinearBVHNodeType::Interior(axis as u8, second_offset as u32)
            }
        }

        my_offset
    }
}

impl<'s> Primitive for BVHAccel<'s> {
    fn bound(&self) -> Bounds {
        self.transform.transform_bounds(self.nodes[0].bounds)
    }

    fn intersect(&self, ray: &Ray, interaction: &mut SurfaceInteraction) -> bool {
        let ray = self.transform.inverse_transform_ray(*ray);
        let dir_is_neg = [ray.dinv.x < 0.0, ray.dinv.y < 0.0, ray.dinv.z < 0.0];
        let mut isect = self.transform.inverse_transform_surface_interaction(interaction);

        let mut hit = false;
        let mut to_visit_offset = 0;
        let mut current_node_index = 0;
        let mut nodes_to_visit: [usize; 64] = [0; 64];

        loop {
            let node = &self.nodes[current_node_index];
            if !node.bounds.intersects(&ray) {
                if to_visit_offset == 0 { break };
                to_visit_offset -= 1;
                current_node_index = nodes_to_visit[to_visit_offset];
                continue;
            }

            match node.content {
                LinearBVHNodeType::Leaf(prim_offset, nprims) => {
                    // intersect with primitives in leaf node
                    for i in 0..(nprims as u32) {
                        let prim_index = self.order[(prim_offset + i) as usize];
                        if self.primitives[prim_index].intersect(&ray, &mut isect) {
                            hit = true
                        }
                    }
                    if to_visit_offset == 0 { break };
                    to_visit_offset -= 1;
                    current_node_index = nodes_to_visit[to_visit_offset];
                }
                LinearBVHNodeType::Interior(axis, child_offset) => {
                    // Put far BVH node on nodes_to_visit stack, advance to near
                    // node. Node direction helps determine which way to go.
                    if dir_is_neg[axis as usize] {
                        nodes_to_visit[to_visit_offset] = current_node_index + 1;
                        current_node_index = child_offset as usize;
                    } else {
                        nodes_to_visit[to_visit_offset] = child_offset as usize;
                        current_node_index += 1;
                    }
                    to_visit_offset += 1;
                }
            }
        }

        // Transform normal before sending it back
        if hit {
            interaction.t = isect.t;
            interaction.n = self.transform.transform_normal(isect.n);
            interaction.p = self.transform.transform_point(isect.p);

            // Assign the uniform material if it hit
            if let Some(material) = self.material {
                interaction.material = Some(material);
            } else {
                interaction.material = isect.material;
            }
        }

        hit
    }
}

impl BVHPrimitiveInfo {
    pub fn new(number: BVHPrimNumber, bounds: Bounds) -> BVHPrimitiveInfo {
        BVHPrimitiveInfo {
            number,
            bounds,
            centroid: 0.5 * bounds.min + 0.5 * bounds.max.to_vec()
        }
    }
}

impl<'a> BVHBuildNode<'a> {
    /*
    pub fn leaf(first: BVHPrimNumber, n: BVHPrimCount, bounds: Bounds) -> BVHBuildNode<'a> {
        BVHBuildNode {
            content: BVHNodeType::Leaf(first, n),
            bounds,
        }
    }

    pub fn interior(axis: BVHSplitAxis, c0: &'a BVHBuildNode<'a>, c1: &'a BVHBuildNode<'a>) -> BVHBuildNode<'a> {
        BVHBuildNode {
            content: BVHNodeType::Interior(axis, c0, c1),
            bounds: c0.bounds.union(&c1.bounds)
        }
    }
    */

    pub fn init_leaf(&mut self, first: BVHPrimNumber, n: BVHPrimCount, bounds: Bounds) {
        self.content = BVHNodeType::Leaf(first, n);
        self.bounds = bounds;
    }

    pub fn init_interior(&mut self, axis: BVHSplitAxis, c0: &'a BVHBuildNode<'a>, c1: &'a BVHBuildNode<'a>) {
        self.content = BVHNodeType::Interior(axis, c0, c1);
        self.bounds = c0.bounds.union(&c1.bounds);
    }
}

fn geometry<'s>(shape: &description::Shape, mat: &MaterialRef) -> PrimBox<'s> {
    match shape {
        description::Shape::Sphere(o, r) =>
            Box::new(Geometry { shape: Sphere::new(*o, *r), material: *mat }),
        description::Shape::Cube(o, d) =>
            Box::new(Geometry { shape: Cuboid::cube(*o, *d), material: *mat }),
        description::Shape::Cuboid(c0, c1) =>
            Box::new(Geometry { shape: Cuboid::new(*c0, *c1), material: *mat }),
    }
}

#[inline]
fn encode_morton_3(v: &Vector) -> u32 {
    (left_shift_3(v.z as u32) << 2)
    | (left_shift_3(v.y as u32) << 1)
    | (left_shift_3(v.z as u32))
}

/// "Spreads" out the bottom 10 bits over the 32 bit range. The
/// lowest-significat bit stays in place, the next moves 3 spots ahead, the next
/// 6, etc.
///
/// e.g.,
/// Before: ----------------------abcdefghij
/// After:  ----a--b--c--d--e--f--g--h--i--j
/// Where each letter is some big, and `-` is don't-care
#[inline]
fn left_shift_3(x: u32) -> u32 {
    let mut x = x;
    if x == (1 << 10) { x -= 1 };
    x = (x | (x << 16)) & 0b00000011000000000000000011111111;
    x = (x | (x <<  8)) & 0b00000011000000001111000000001111;
    x = (x | (x <<  4)) & 0b00000011000011000011000011000011;
    x = (x | (x <<  2)) & 0b00001001001001001001001001001001;
    x
}

fn radix_sort(v: &mut Vec<MortonPrimitive>) {
    let mut temp: Vec<MortonPrimitive> = vec![
        MortonPrimitive { index: 0, code: 0}; v.len()
    ];

    for pass in 0..RADIX_NPASSES {
        let lowbit = pass * RADIX_BITS_PER_PASS;
        let (input, output): (&mut Vec<MortonPrimitive>, &mut Vec<MortonPrimitive>) =
            if pass & 1 == 0 {
                (v, &mut temp)
            } else {
                (&mut temp, v)
            };

        let mut bucket_count: [usize; RADIX_NBUCKETS] = [0; RADIX_NBUCKETS];
        for mp in input.iter() {
            let bucket = ((mp.code >> lowbit) & RADIX_BITMASK) as usize;
            bucket_count[bucket] += 1;
        }

        let mut out_index: [usize; RADIX_NBUCKETS] = [0; RADIX_NBUCKETS];
        for i in 1..RADIX_NBUCKETS {
            out_index[i] = out_index[i - 1] + bucket_count[i - 1];
        }

        for mp in input.iter() {
            let bucket = ((mp.code >> lowbit) & RADIX_BITMASK) as usize;
            output[out_index[bucket]] = *mp;
            out_index[bucket] += 1;
        }
    }

    if RADIX_NPASSES & 1 == 1 {
        mem::swap(v, &mut temp)
    }
}
