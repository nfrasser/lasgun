use std::f64::NAN;

/**
    Finds the roots of a quadratic polynomial.
    Returns the roots and how many there range [0, 2]
*/
pub fn quad_roots(a: f64, b: f64, c: f64) -> ([f64; 2], u8) {
    if a == 0.0 {
        if b == 0.0 {
            ([NAN, NAN], 0) // No roots
        } else {
            ([-c/b, NAN], 1) // One root
        }
    } else {
        // Compute discrimanant D = b^2 - 4ac
        let d = b*b - 4.0*a*c;
        if d < 0.0 {
            ([NAN, NAN], 0) // No roots

        } else {
            // Two real roots
            let q = -(b + b.signum() * d.sqrt()) / 2.0;
            let q_over_a = q / a;
            ([
                q_over_a,
                if q == 0.0 { q_over_a } else { c / q }
            ], 2) // Two roots
        }
    }
}
