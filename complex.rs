#![ feature(macro_rules) ]

use std::num::NumCast;
use std::fmt::{Formatter,FormatError,Show};

struct MComplex {
    r : f64,
    j : f64
}

impl MComplex {
    fn conjugate(&self) -> MComplex { MComplex { j : -self.j, .. *self } }
    fn magnitude(&self) -> f64 { ( self.r * self.r + self.j * self.j ).sqrt() }
}

impl Show for MComplex {
    fn fmt(&self, formatter : &mut Formatter) -> Result<(),FormatError> {
        write!(formatter, "{} + {}i", self.r, self.j)
    }
}

trait ToMComplex { fn to_complex(&self) -> MComplex; }

impl ToMComplex for f64 {
    fn to_complex(&self) -> MComplex { MComplex { r : *self, j : 0.0f64 } }
}

// Can't do this:

// impl Add<f64,MComplex> for MComplex {
//     fn add(&self, rhs : &f64) -> MComplex {
//         MComplex { r : self.r + *rhs, j : self.j }
//     }
// }

// complex.rs:52:0: 56:1 error: conflicting implementations for a trait
// ...
// complex.rs:24:0: 28:1 error: conflicting implementations for a trait
// ...
// complex.rs:81:12: 81:17 error: multiple applicable methods in scope
// ...

// But can use double dispatch:

trait MComplexRhs {
    fn add_complex(&self, lhs: &MComplex) -> MComplex;
    fn sub_complex(&self, lhs: &MComplex) -> MComplex;
    fn mul_complex(&self, lhs: &MComplex) -> MComplex;
}

// Implementation of Add, etc. using double dispatch on MComplexRhs.
impl<R : MComplexRhs> Add<R, MComplex> for MComplex {
    fn add(&self, rhs: &R) -> MComplex { rhs.add_complex(self) }
}
impl<R : MComplexRhs> Sub<R, MComplex> for MComplex {
    fn sub(&self, rhs: &R) -> MComplex { rhs.sub_complex(self) }
}
impl<R : MComplexRhs> Mul<R, MComplex> for MComplex {
    fn mul(&self, rhs: &R) -> MComplex { rhs.mul_complex(self) }
}

impl MComplexRhs for MComplex {
    fn add_complex(&self, lhs: &MComplex) -> MComplex {
        MComplex { r : self.r + lhs.r, j : self.j + lhs.j }
    }
    fn sub_complex(&self, lhs: &MComplex) -> MComplex {
        MComplex { r : lhs.r - self.r, j : lhs.j - self.j }
    }
    fn mul_complex(&self, lhs: &MComplex) -> MComplex {
        MComplex { r : (lhs.r * self.r) - (lhs.j * self.j),
                  j : (lhs.r * self.j) + (lhs.j * self.r) }
    }
}

// Implement MComplexRhs for generic scalar rhs's
macro_rules! scalar_impl(
    ($foo:ty) => (
        impl MComplexRhs for $foo {
            fn add_complex(&self, lhs: &MComplex) -> MComplex {
                MComplex { r : lhs.r + (*self as f64), j : lhs.j }
            }
            fn sub_complex(&self, lhs: &MComplex) -> MComplex {
                MComplex { r : lhs.r - (*self as f64), j : lhs.j }
            }
            fn mul_complex(&self, lhs: &MComplex) -> MComplex {
                MComplex { r : (lhs.r * (*self as f64)), j : (lhs.j * (*self as f64)) }
            }
        }
    )
)

scalar_impl!(f64)
scalar_impl!(int)
scalar_impl!(uint)

impl Div<MComplex,MComplex> for MComplex {
    fn div(&self, rhs : &MComplex) -> MComplex {
        let rhs_conj = rhs.conjugate();
        let num = self * rhs_conj;
        let denom = rhs * rhs_conj;
        MComplex { r : num.r / denom.r, j : num.j / denom.r }
    }
}

impl ToPrimitive for MComplex {
    fn to_i64(&self)   -> Option<i64>   { Some(self.magnitude() as i64)   }
    fn to_u64(&self)   -> Option<u64>   { Some(self.magnitude() as u64)   }
    fn to_f64(&self)   -> Option<f64>   { Some(self.magnitude() as f64)   }
}

impl NumCast for MComplex {
    fn from<N:NumCast>(n: N) -> Option<MComplex> { n.to_f64().map( |v| v.to_complex() ) }
}

fn main() {
    let x = MComplex { r : 1.0, j : 0.0 };
    let y = MComplex { r : 3.0, j : 0.0 };
    let z = x + y;
    let w = NumCast::from(2i).unwrap();
    println!("{}", ( y + 3.0f64                ));
    println!("{}", ( y + 3.0f64.to_complex()   ));
    println!("{}", ( y + 3i                    ));
    println!("{}", ( x * 3.0f64                ));
    println!("{}", ( x * 4i                    ));
    println!("{}", ( z / w                     ));

    let n = MComplex { r : 0.0, j : 1.0 };
    println!("{}", ( n * n                     ));
}

// Notes: i => j, suggested by englabenny and dfjkfskjhfshdfjhsdjl on reddit.
