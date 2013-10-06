use std::num;
use std::num::NumCast;

struct Complex {
    r : float,
    j : float
}

impl Complex {
    fn conjugate(&self) -> Complex { Complex { j : -self.j, .. *self } }
    fn magnitude(&self) -> float { num::sqrt( self.r * self.r + self.j * self.j ) }
}

impl ToStr for Complex {
    fn to_str(&self) -> ~str { format!("({:f} + {:f}i)", self.r, self.j) }
}

trait ToComplex { fn to_complex(&self) -> Complex; }

impl ToComplex for float {
    fn to_complex(&self) -> Complex { Complex { r : *self, j : 0.0f } }
}

// Can't do this:

// impl Add<float,Complex> for Complex {
//     fn add(&self, rhs : &float) -> Complex {
//         Complex { r : self.r + *rhs, j : self.j }
//     }
// }

// complex.rs:52:0: 56:1 error: conflicting implementations for a trait
// ...
// complex.rs:24:0: 28:1 error: conflicting implementations for a trait
// ...
// complex.rs:81:12: 81:17 error: multiple applicable methods in scope
// ...

// But can use double dispatch:

trait ComplexRhs {
    fn add_complex(&self, lhs: &Complex) -> Complex;
    fn sub_complex(&self, lhs: &Complex) -> Complex;
    fn mul_complex(&self, lhs: &Complex) -> Complex;
}

// Implementation of Add, etc. using double dispatch on ComplexRhs.
impl<R : ComplexRhs> Add<R, Complex> for Complex {
    fn add(&self, rhs: &R) -> Complex { rhs.add_complex(self) }
}
impl<R : ComplexRhs> Sub<R, Complex> for Complex {
    fn sub(&self, rhs: &R) -> Complex { rhs.sub_complex(self) }
}
impl<R : ComplexRhs> Mul<R, Complex> for Complex {
    fn mul(&self, rhs: &R) -> Complex { rhs.mul_complex(self) }
}

impl ComplexRhs for Complex {
    fn add_complex(&self, lhs: &Complex) -> Complex {
        Complex { r : self.r + lhs.r, j : self.j + lhs.j }
    }
    fn sub_complex(&self, lhs: &Complex) -> Complex {
        Complex { r : lhs.r - self.r, j : lhs.j - self.j }
    }
    fn mul_complex(&self, lhs: &Complex) -> Complex {
        Complex { r : (lhs.r * self.r) - (lhs.j * self.j),
                  j : (lhs.r * self.j) + (lhs.j * self.r) }
    }
}

// Implement ComplexRhs for generic scalar rhs's
macro_rules! scalar_impl(
    ($foo:ty) => (
        impl ComplexRhs for $foo {
            fn add_complex(&self, lhs: &Complex) -> Complex {
                Complex { r : lhs.r + (*self as float), j : lhs.j }
            }
            fn sub_complex(&self, lhs: &Complex) -> Complex {
                Complex { r : lhs.r - (*self as float), j : lhs.j }
            }
            fn mul_complex(&self, lhs: &Complex) -> Complex {
                Complex { r : (lhs.r * (*self as float)), j : (lhs.j * (*self as float)) }
            }
        }
    )
)

scalar_impl!(float)
scalar_impl!(int)
scalar_impl!(uint)

impl Div<Complex,Complex> for Complex {
    fn div(&self, rhs : &Complex) -> Complex {
        let rhs_conj = rhs.conjugate();
        let num = self * rhs_conj;
        let denom = rhs * rhs_conj;
        Complex { r : num.r / denom.r, j : num.j / denom.r }
    }
}

impl NumCast for Complex {
    fn from<N:NumCast>(n: N) -> Complex { n.to_float().to_complex() }

    fn to_u8(&self)    -> u8    { self.magnitude() as u8    }
    fn to_u16(&self)   -> u16   { self.magnitude() as u16   }
    fn to_u32(&self)   -> u32   { self.magnitude() as u32   }
    fn to_u64(&self)   -> u64   { self.magnitude() as u64   }
    fn to_uint(&self)  -> uint  { self.magnitude() as uint  }

    fn to_i8(&self)    -> i8    { self.magnitude() as i8    }
    fn to_i16(&self)   -> i16   { self.magnitude() as i16   }
    fn to_i32(&self)   -> i32   { self.magnitude() as i32   }
    fn to_i64(&self)   -> i64   { self.magnitude() as i64   }
    fn to_int(&self)   -> int   { self.magnitude() as int   }

    fn to_f32(&self)   -> f32   { self.magnitude() as f32   }
    fn to_f64(&self)   -> f64   { self.magnitude() as f64   }
    fn to_float(&self) -> float { self.magnitude()          }
}

fn main() {
    let x = Complex { r : 1.0, j : 0.0 };
    let y = Complex { r : 3.0, j : 0.0 };
    let z = x + y;
    let w = NumCast::from(2);
    println(( y + 3.0f                ).to_str());
    println(( y + 3.0f.to_complex()   ).to_str());
    println(( y + 3                   ).to_str());
    println(( x * 3.0f                ).to_str());
    println(( x * 4                   ).to_str());
    println(( z / w                   ).to_str());

    let n = Complex { r : 0.0, j : 1.0 };
    println(( n * n                   ).to_str());
}

// Notes: i => j, suggested by englabenny and dfjkfskjhfshdfjhsdjl on reddit.
