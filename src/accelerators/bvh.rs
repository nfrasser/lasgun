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

// For BVH transformation references
const ID: Transformation = Transformation::identity();

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

    // The order in which primitives are accessed following BVH construction
    // Each element is an index into the primitives Vec
    order: Vec<BVHPrimNumber>,

    // Limit to how many primitives there may be per node
    max_prims_per_node: u8
}

/// Information about each primitive stored in a BVHAccel
struct BVHPrimitiveInfo {
    pub number: BVHPrimNumber,
    pub bounds: Bounds,
    pub centroid: Point
}

/// For Upper SAH buckets
#[derive(Copy, Clone)]
struct BVHBucketInfo {
    pub count: usize,
    pub bounds: Bounds
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

#[derive(Copy, Clone)]
struct LinearBVHNode {
    pub bounds: Bounds,
    pub offset: (u32, u32), // First and second child offsets
    pub nprims: u16,
    pub axis: u8
}

/// Deterministic sorting construct for objects in 3D space
/// See PBRT book v3 page 268
#[derive(Copy, Clone, PartialEq)]
struct MortonPrimitive {
    pub index: BVHPrimNumber,
    pub code: u32 // morton code
}

// Cluster of primitives that can processeded for bounds checks independently
// Lifetime is tied to the referenced build nodes (allocated in an arean)
struct LBVHTreelet<'a> {
    pub start: BVHPrimNumber,
    pub nodes: &'a [BVHBuildNode<'a>]
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
        BVHAccel::new(scene, triangles, &ID, Some(*material), 255)
    }

    fn from_aggregate(scene: &'s Scene, aggregate: &'s description::Aggregate) -> BVHAccel<'s> {
        let primitives = Vec::with_capacity(aggregate.contents.len());
        for node in aggregate.contents.iter() {
            match node {
                SceneNode::Geometry(shape, mat) => {
                    primitives.push(geometry(shape, mat));
                },
                SceneNode::Mesh(obj, mat) => {
                    primitives.push(Box::new(BVHAccel::from_mesh(scene, obj, mat)))
                },
                SceneNode::Group(aggregate) => {
                    primitives.push(Box::new(BVHAccel::from_aggregate(scene, aggregate)));
                }
            }
        };

        BVHAccel::new(scene, primitives, &ID, None, 255)
    }

    fn new(
        scene: &'s Scene,
        primitives: Vec<PrimBox<'s>>,
        transform: &'s Transformation,
        material: Option<MaterialRef>,
        max_prims_per_node: u8
    ) -> BVHAccel<'s> {

        let arena = Arena::with_capacity(1024 * 1024);
        let nprims = primitives.len();
        let prim_info: Vec<BVHPrimitiveInfo> = primitives.iter()
            .enumerate()
            .map(|(i, prim)| BVHPrimitiveInfo::new(i, prim.object_bound()))
            .collect();

        let order: Vec<BVHPrimNumber> = vec![std::usize::MAX; nprims];
        let mut total_nodes = 0;

        let node = BVHAccel::build(&arena, &primitives, &prim_info, &mut order, &mut total_nodes);
        let nodes: Vec<LinearBVHNode> = vec![
            LinearBVHNode {
                bounds: Bounds::none(),
                offset: (0xffffffff, 0xffffffff),
                nprims: 0xffff,
                axis: 0xff
            }
        ; total_nodes];

        BVHAccel::flatten_bvh_tree(node, &mut nodes[..], &mut 0);

        BVHAccel {
            primitives,
            nodes,
            order,
            transform,
            material,
            max_prims_per_node
        }
    }

    /// Build the BVH tree with the hierarchical linear bounding volume hierachy algorithm
    fn build<'a>(
        arena: &'a Arena<BVHBuildNode<'a>>,
        primitives: &Vec<PrimBox<'s>>,
        prim_info: &Vec<BVHPrimitiveInfo>,
        prim_order: &mut Vec<BVHPrimNumber>,
        total_nodes: &mut BVHPrimCount
    ) -> &'a BVHBuildNode<'a> {
        // Compute bounding box of all primitive centroids
        let bounds = prim_info.iter()
            .fold(Bounds::none(), |bounds, info| bounds.union(&info.bounds));

        // Compute Morton indeces of primitives
        // TODO: Parallelize
        let mut morton_prims = prim_info.iter().map(|info| {
            let centroid_offset = bounds.offset(&info.centroid);
            MortonPrimitive {
                index: info.number,
                code: encode_morton_3(&(centroid_offset * MORTON_SCALE.into()))
            }
        }).collect();

        // Constant-time sort. Once this is done, the morton primitives are
        // arranged in a recursive pattern such that primitives in opposite
        // subdivisions of the vector are in spatially different quadrants
        radix_sort(&mut morton_prims);

        // Create LBVH treelets at bottom of BVH
        // Find invervals for primitives for each treelet
        let mut treelets: Vec<LBVHTreelet> = Vec::new();
        let mut start = 0;
        for end in 1..=(morton_prims.len()) {
            let mask = 0b00111111111111000000000000000000;
            if end == morton_prims.len() || (
                (morton_prims[start].code & mask) != (morton_prims[end].code & mask)
            ) {
                // Add entry to treelets for this treelet
                let nprims = end - start;
                let maxnodes = 2 * nprims;
                let nodes = arena.alloc_extend((0..maxnodes)
                .map(|_| BVHBuildNode {
                    content: BVHNodeType::Leaf(0, 0),
                    bounds: Bounds::none()
                }));
                treelets.push(LBVHTreelet { start: start as BVHPrimNumber, nodes });
                start = end;
            }
        }

        // Create and return SAH BVH from LBVH treelets
        // TODO: Parallelize
        let mut ordered_prims_offset = 0;
        *total_nodes = treelets.iter_mut().enumerate().fold(0, |total, (i, treelet)| {
            let first_bit_index = 29 - 12; // Something to do with Morton encoding bit positions, I think
            treelet.nodes = BVHAccel::emit_lbvh(
                &mut treelet.nodes, &morton_prims, primitives, prim_info, prim_order,
                &mut ordered_prims_offset, first_bit_index);
            total + treelet.nodes.len()
        });

        // Create and return Surface Area Heuristic BVH from LBVH treelets
        let mut finished_treelets: Vec<&'a BVHBuildNode<'a>> = treelets.iter()
        .map(|treelet| &treelet.nodes[0]).collect();

        BVHAccel::build_upper_sah(arena, &mut finished_treelets[..], total_nodes)
    }

    /// Creates and returns LBVH nodes and returns the the total of nodes created
    /// Also calculates the prim_order order
    fn emit_lbvh<'a>(
        nodes: &'a mut [BVHBuildNode<'a>],
        morton_prims: &[MortonPrimitive],
        primitives: &Vec<PrimBox<'s>>,
        prim_info: &Vec<BVHPrimitiveInfo>,
        prim_order: &mut Vec<BVHPrimNumber>,
        ordered_prims_offset: &mut usize,
        bit_index: i32
    ) -> &'a mut [BVHBuildNode<'a>] {

        let mask = 1 << bit_index;
        let nprims = nodes.len();

        if bit_index == -1 || nprims == 1 { // FIXME - nprims check should be from defined min
            // Create and return leaf node of LBVH treelet
            let first_prim_offset = *ordered_prims_offset;
            *ordered_prims_offset += nprims;
            let node = &mut nodes[0];
            let bounds = (0..nprims).fold(Bounds::none(), |bounds, i| {
                let prim_index = morton_prims[i].index;
                prim_order[first_prim_offset + i] = prim_index;
                bounds.union(&prim_info[prim_index].bounds)
            });
            node.set_leaf(first_prim_offset, nprims, bounds);
            return &mut nodes[0..1]
        }

        // Advance to next subtree level if there's no LBVH split for this bit
        if (morton_prims[0].code & mask)
        == (morton_prims[nprims - 1].code & mask) {
            return BVHAccel::emit_lbvh(
                nodes, morton_prims, primitives, prim_info,
                prim_order, ordered_prims_offset, bit_index - 1);
        }

        // Find LVBH split point for this dimension
        let (mut search_start, mut search_end) = (0, nprims - 1);
        while search_start + 1 != search_end {
            let mid = (search_start - search_end) / 2;
            if (morton_prims[search_start].code & mask)
            == (morton_prims[mid].code & mask) {
                search_start = mid
            } else {
                search_end = mid
            }
        }
        let split_offset = search_end;

        // Create and return interial LBVH node
        let lbvh0 = BVHAccel::emit_lbvh(
            &mut nodes[1..split_offset], morton_prims, primitives, prim_info,
            prim_order, ordered_prims_offset, bit_index - 1);

        let lbvh1 = BVHAccel::emit_lbvh(
            &mut nodes[split_offset..], &morton_prims[split_offset..], primitives, prim_info,
            prim_order, ordered_prims_offset, bit_index - 1);

        let axis = (bit_index % 3) as BVHSplitAxis;
        nodes[0].set_interior(axis, &lbvh0[0], &lbvh1[0]);
        &mut nodes[0..(lbvh0.len() + lbvh1.len() + 1)]
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
            let centroid = 0.5 * root.bounds.min + 0.5 * root.bounds.min.to_vec();
            bounds.point_union(&centroid)
        });

        // Choose split dimension
        let dim = centroid_bounds.maximum_extent();

        // Allocate and initialize BucketInfo for SAH partition buckets
        let mut buckets: [BVHBucketInfo; BVH_NBUCKETS] = [
            BVHBucketInfo { count: 0, bounds: Bounds::none() }; BVH_NBUCKETS
        ];
        for root in treelet_roots {
            let centroid = root.bounds.min[dim] + root.bounds.max[dim] * 0.5;
            let mut b = (BVH_NBUCKETS as f64 * (
                (centroid - centroid_bounds.min[dim]) /
                (centroid_bounds.max[dim] - centroid_bounds.min[dim])
            )) as usize;
            if b == BVH_NBUCKETS { b = BVH_NBUCKETS - 1 };
            buckets[b].count += 1;
            buckets[b].bounds = buckets[b].bounds.union(&root.bounds);
        }

        // Compute costs for splitting after each bucket
        let cost: [f64; BVH_NBUCKETS];
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
            let centroid = 0.5 * node.bounds.min[dim] + 0.5 * node.bounds.max[dim];
            let mut b = (BVH_NBUCKETS as f64 * (
                (centroid - centroid_bounds.min[dim]) /
                (centroid_bounds.max[dim] - centroid_bounds.min[dim])
            )) as usize;
            if b == BVH_NBUCKETS { b = BVH_NBUCKETS - 1 };
            b <= min_cost_split_bucket
        });
        node.set_interior(dim,
            BVHAccel::build_upper_sah(arena, lo_roots, total_nodes),
            BVHAccel::build_upper_sah(arena, hi_roots, total_nodes));

        node
    }

    // a is the lifetime of the arena as usual
    // v is the lifetime of the parent LinearBVHNode vec
    fn flatten_bvh_tree<'a, 'v>(
        node: &'a BVHBuildNode<'a>,
        linear_nodes: &'v mut [LinearBVHNode],
        offset: &mut usize
    ) -> usize {
        let linear_node = &mut linear_nodes[0];
        linear_node.bounds = node.bounds;
        let my_offset = *offset; *offset += 1;
        match node.content {
            BVHNodeType::Leaf(offset, nprims) => {
                linear_node.offset.0 = offset as u32;
                linear_node.nprims = nprims as u16;
            }
            BVHNodeType::Interior(axis, c0, c1) => {
                linear_node.axis = axis as u8;
                linear_node.nprims = 0;

                let (_, second_offset) = (
                    BVHAccel::flatten_bvh_tree(c0, linear_nodes, offset),
                    BVHAccel::flatten_bvh_tree(c1, linear_nodes, offset)
                );

                linear_node.offset.1 = second_offset as u32;
            }
        }

        my_offset
    }
}

impl<'s> Primitive for BVHAccel<'s> {
    fn object_bound(&self) -> Bounds {
        self.transform.transform_bounds(self.nodes[0].bounds)
    }

    fn intersect(&self, ray: &Ray, interaction: &mut SurfaceInteraction) -> bool {
        let ray = self.transform.inverse_transform_ray(*ray);
        let mut i = self.transform.inverse_transform_surface_interaction(interaction);
        let mut exists = false;

        // Find the closest child with which this node intersects
        // let exists = self.nodes.intersect(&ray, interaction);
        let exists = false;

        // Transform normal before sending it back
        if exists {
            interaction.t = i.t;
            interaction.n = self.transform.transform_normal(i.n);
            interaction.p = self.transform.transform_point(i.p);

            // Assign the uniform material if it exists
            if let Some(material) = self.material {
                interaction.material = Some(material);
            } else {
                interaction.material = i.material;
            }
        }

        exists
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

    pub fn set_leaf(&mut self, first: BVHPrimNumber, n: BVHPrimCount, bounds: Bounds) {
        self.content = BVHNodeType::Leaf(first, n);
        self.bounds = bounds;
    }

    pub fn set_interior(&mut self, axis: BVHSplitAxis, c0: &'a BVHBuildNode<'a>, c1: &'a BVHBuildNode<'a>) {
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
    // Unsafely create a bunch of morton prims filled with gibberish.
    // This is okay because they're never logically read from
    let len = v.len();
    let mut temp: Vec<MortonPrimitive> = Vec::with_capacity(len);
    unsafe { temp.set_len(len); }

    for pass in 0..RADIX_NPASSES {
        let lowbit = pass * RADIX_BITS_PER_PASS;
        let input = if pass & 1 == 1 { &mut temp } else { v };
        let output = if pass & 1 == 1 { v } else { &mut temp  };

        let mut bucket_count: [usize; RADIX_NBUCKETS] = unsafe { mem::zeroed() }; // This is safe, I promise
        for mp in input {
            let bucket = ((mp.code >> lowbit) & RADIX_BITMASK) as usize;
            bucket_count[bucket] += 1;
        }

        let mut out_index: [usize; RADIX_NBUCKETS];
        out_index[0] = 0;
        for i in 1..RADIX_NBUCKETS {
            out_index[i] = out_index[i - 1] + bucket_count[i - 1];
        }

        for mp in input {
            let bucket = ((mp.code >> lowbit) & RADIX_BITMASK) as usize;
            output[out_index[bucket]] = *mp;
        }
    }

    if RADIX_NPASSES & 1 == 1 {
        mem::swap(v, &mut temp)
    }
}
