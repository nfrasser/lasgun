/**
Permute the components of a 3-space vector or point (set each index to the other given index)
*/
#[macro_export]
macro_rules! permute {
    ($t:ty, $s:expr, $x:expr, $y:expr, $z:expr) => {
        <$t>::new($s[$x], $s[$y], $s[$z])
    }
}

#[macro_export]
macro_rules! iamax {
    ($v:expr) => {{
        let mut max = $v.x;
        let mut i = 0;
        let mut imax = 0;
        $v.map(|c| {
            imax = if c > max { max = c; i } else { imax };
            i += 1
        });
        imax
    }}
}
