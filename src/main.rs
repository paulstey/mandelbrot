
extern crate num;
use num::Complex;

enum Option<T> {
    None,
    Some(T),
}



/// Try to determine if `c` is in the Mandelbrot set, using at most `limit` iterations to decide
///
/// If `c` is not a member, return `Some(i)`, where `i` is the number of iterations it toook for
/// `c` to leave the circle of radius two centered on the origin.  If `c` seems to be a member
/// (more precisely, if we reaced the iteration limit without being able to prove non-membership),
/// then we return `None`.
fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex {re: 0.0, im: 0.0};
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }
    None
}



fn main() {
    println!("no");
}
