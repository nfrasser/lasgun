/// Permute the components of a 3-space vector or point (set each index to the other given index)
#[macro_export]
macro_rules! permute {
    ($t:ty, $s:expr, $x:expr, $y:expr, $z:expr) => {
        <$t>::new($s[$x], $s[$y], $s[$z])
    }
}
