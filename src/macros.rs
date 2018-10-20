/// Permute the components of a 3-space vector or point (set each index to the other given index)
#[macro_export]
macro_rules! permute {
    ($t:ty, $s:expr, $x:expr, $y:expr, $z:expr) => {
        <$t>::new($s[$x], $s[$y], $s[$z])
    }
}

/// Check that each pair of components for the pair of points matches the
/// expression
macro_rules! all_coords_match {
    ($p0:expr, $p1:expr, $chk:expr) => {
        $chk($p0.x, $p1.x) &&
        $chk($p0.y, $p1.y) &&
        $chk($p0.z, $p1.z)
    };
}

/// Create a new point by joining the components
macro_rules! zip_points {
    ($p0:expr, $p1:expr, $cb:expr) => {
        Point3::new($cb($p0.x, $p1.x), $cb($p0.y, $p1.y), $cb($p0.z, $p1.z))
    }
}

/// Create a new point by joining the components
macro_rules! zip_vectors {
    ($p0:expr, $p1:expr, $cb:expr) => {
        Vector3::new($cb($p0.x, $p1.x), $cb($p0.y, $p1.y), $cb($p0.z, $p1.z))
    }
}
