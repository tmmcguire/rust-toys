use std::fmt;
use std::fmt::{Display,Formatter};
use std::ops::{Add,Div,Mul};

#[derive(Debug,Copy,Clone)]
pub struct Complex {
    r : f64,
    j : f64
}

impl Complex {
    fn conjugate(&self) -> Complex { Complex { j : -self.j, .. *self } }
    fn magnitude(&self) -> f64 { ( self.r * self.r + self.j * self.j ).sqrt() }
}

impl Display for Complex {
    fn fmt(&self, formatter : &mut Formatter) -> fmt::Result {
        write!(formatter, "{} + {}j", self.r, self.j)
    }
}

// Conversions to Complex from f64

trait ToComplex { fn to_complex(&self) -> Complex; }

impl ToComplex for f64 {
    fn to_complex(&self) -> Complex { Complex { r : *self, j : 0.0f64 } }
}

// Implementation of addition and multiplication for Complex and Complex

impl Add<Complex> for Complex {
    type Output = Complex;
    fn add(self, rhs : Complex) -> Complex {
        Complex { r : self.r + rhs.r, j : self.j + rhs.j }
    }
}

impl Mul<Complex> for Complex {
    type Output = Complex;
    fn mul(self, rhs : Complex) -> Complex {
        Complex {
            r : (self.r * rhs.r) - (self.j * rhs.j),
            j : (self.r * rhs.j) + (self.j * rhs.r)
        }
    }
}

// Implementation of division for Complex

impl Div<Complex> for Complex {
    type Output = Complex;
    fn div(self, rhs : Complex) -> Complex {
        let rhs_conj = rhs.conjugate();
        let num = self * rhs_conj;
        let den = rhs * rhs_conj;
        Complex { r : num.r / den.r, j : num.j / den.r }
    }
}

// Implementation of Addition for Complex and f64

// // LHS: Complex, RHS: f64
impl Add<f64> for Complex {
    type Output = Complex;
    fn add(self, rhs : f64) -> Complex {
        Complex { r : self.r + rhs, j : self.j }
    }
}

// // LHS: f64, RHS: Complex
impl Add<Complex> for f64 {
    type Output = Complex;
    fn add(self, rhs : Complex) -> Complex {
        Complex { r : self + rhs.r, j : rhs.j }
    }
}

// // Etc., etc., etc.

// Implementation of multiplication for most other primitives
// // Note: probably not safe for u64 and i64.

macro_rules! scalar_impl(
    ($foo:ty) => (

        // Implementation of multiplication for Complex and $foo
        impl Mul<$foo> for Complex {
            type Output = Complex;
            fn mul(self, rhs : $foo) -> Complex {
                Complex { r : self.r * (rhs as f64), j : self.j * (rhs as f64) }
            }
        }
        impl Mul<Complex> for $foo {
            type Output = Complex;
            fn mul(self, rhs : Complex) -> Complex {
                Complex { r : (self as f64) * rhs.r, j : (self as f64) * rhs.j }
            }
        }

    )
);

scalar_impl!(i8);
scalar_impl!(i16);
scalar_impl!(i32);
scalar_impl!(i64);
scalar_impl!(isize);
scalar_impl!(u8);
scalar_impl!(u16);
scalar_impl!(u32);
scalar_impl!(u64);
scalar_impl!(usize);
scalar_impl!(f64);
scalar_impl!(f32);

fn main() {
    let w = 2.0.to_complex();

    let x = Complex { r : 1.0, j : 0.0 };
    let y = Complex { r : 3.0, j : 0.0 };
    let z = x + y;
    println!("  z: {:?}", z);
    // =>   z: Complex { r: 4, j: 0 }
    println!("{}", ( z / w                  ));
    // => 2 + 0j

    println!("{}", ( y + 3.0                ));
    // => 6 + 0j
    println!("{}", ( 3.0 + y                ));
    // => 6 + 0j

    println!("{}", ( y * 3isize             ));
    // => 9 + 0j
    println!("{}", ( y * 3.0f64             ));
    // => 9 + 0j
    println!("{}", ( 4u8 * y                ));
    // => 12 + 0j

    let n = Complex { r : 0.0, j : 1.0 };
    println!("{}", (  n * n                 ));
    // => -1 + 0j
    println!("{}", ( (n * n) * 2            ));
    // => -2 + 0j

    let mu : Complex = (n * n) * 2;
    println!("{}", mu.magnitude() );
    // => 2

    // Without type annotation:
    // src/bin/complex.rs:146:20: 146:34 error: the type of this value must be known in this context
    // src/bin/complex.rs:146     println!("{}", mu.magnitude() );
    //                                           ^~~~~~~~~~~~~~
}

// Notes: i => j, suggested by englabenny and dfjkfskjhfshdfjhsdjl on reddit.
