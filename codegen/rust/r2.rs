// 3D Projective Geometric Algebra
// Written by a generator written by enki.
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![feature(const_slice_len)]

use std::fmt;
use std::ops::{Index,IndexMut,Add,Sub,Mul,BitAnd,BitOr,BitXor,Not};

type float_t = f64;

// use std::f64::consts::PI;
const PI: float_t = 3.14159265358979323846;

const basis: &'static [&'static str] = &[ "1","e1","e2","e12" ];
const basis_count: usize = basis.len();

#[derive(Default,Debug,Clone,Copy,PartialEq)]
struct R2 {
    mvec: [float_t; basis_count]
}

impl R2 {
    pub const fn zero() -> Self {
        Self {
            mvec: [0.0; basis_count]
        }
    }

    pub const fn new(f: float_t, idx: usize) -> Self {
        let mut ret = Self::zero();
        ret.mvec[idx] = f;
        ret
    }
}

// basis vectors are available as global constants.
const e1: R2 = R2::new(1.0, 1);
const e2: R2 = R2::new(1.0, 2);
const e12: R2 = R2::new(1.0, 3);

impl Index<usize> for R2 {
    type Output = float_t;

    fn index<'a>(&'a self, index: usize) -> &'a Self::Output {
        &self.mvec[index]
    }
}

impl IndexMut<usize> for R2 {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut Self::Output {
        &mut self.mvec[index]
    }
}

impl fmt::Display for R2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut n = 0;
        let ret = self.mvec.iter().enumerate().filter_map(|(i, &coeff)| {
            if coeff > 0.00001 || coeff < -0.00001 {
                n = 1;
                Some(format!("{}{}", 
                        format!("{:.*}", 7, coeff).trim_end_matches('0').trim_end_matches('.'),
                        if i > 0 { basis[i] } else { "" }
                    )
                )
            } else {
                None
            }
        }).collect::<Vec<String>>().join(" + ");
        if n==0 { write!(f,"0") } else { write!(f, "{}", ret) }
    }
}

// Reverse
// Reverse the order of the basis blades.
impl R2 {
    pub fn Reverse(self: Self) -> R2 {
        let mut res = R2::zero();
        let a = self;
        res[0]=a[0];
        res[1]=a[1];
        res[2]=a[2];
        res[3]=-a[3];
        res
    }
}

// Dual
// Poincare duality operator.
impl R2 {
    pub fn Dual(self: Self) -> R2 {
        let mut res = R2::zero();
        let a = self;
        res[0]=-a[3];
        res[1]=a[2];
        res[2]=-a[1];
        res[3]=a[0];
        res
    }
}

impl Not for R2 {
    type Output = R2;

    fn not(self: Self) -> R2 {
        let mut res = R2::zero();
        let a = self;
        res[0]=-a[3];
        res[1]=a[2];
        res[2]=-a[1];
        res[3]=a[0];
        res
    }
}

// Conjugate
// Clifford Conjugation
impl R2 {
    pub fn Conjugate(self: Self) -> R2 {
        let mut res = R2::zero();
        let a = self;
        res[0]=a[0];
        res[1]=-a[1];
        res[2]=-a[2];
        res[3]=-a[3];
        res
    }
}

// Involute
// Main involution
impl R2 {
    pub fn Involute(self: Self) -> R2 {
        let mut res = R2::zero();
        let a = self;
        res[0]=a[0];
        res[1]=-a[1];
        res[2]=-a[2];
        res[3]=a[3];
        res
    }
}

// Mul
// The geometric product.
impl Mul for R2 {
    type Output = R2;

    fn mul(self: R2, b: R2) -> R2 {
        let mut res = R2::zero();
        let a = self;
        res[0]=b[0]*a[0]+b[1]*a[1]+b[2]*a[2]-b[3]*a[3];
		res[1]=b[1]*a[0]+b[0]*a[1]-b[3]*a[2]+b[2]*a[3];
		res[2]=b[2]*a[0]+b[3]*a[1]+b[0]*a[2]-b[1]*a[3];
		res[3]=b[3]*a[0]+b[2]*a[1]-b[1]*a[2]+b[0]*a[3];
        res
    }
}

// Wedge
// The outer product. (MEET)
impl BitXor for R2 {
    type Output = R2;

    fn bitxor(self: R2, b: R2) -> R2 {
        let mut res = R2::zero();
        let a = self;
        res[0]=b[0]*a[0];
		res[1]=b[1]*a[0]+b[0]*a[1];
		res[2]=b[2]*a[0]+b[0]*a[2];
		res[3]=b[3]*a[0]+b[2]*a[1]-b[1]*a[2]+b[0]*a[3];
        res
    }
}

// Vee
// The regressive product. (JOIN)
impl BitAnd for R2 {
    type Output = R2;

    fn bitand(self: R2, b: R2) -> R2 {
        let mut res = R2::zero();
        let a = self;
        res[3]=b[3]*a[3];
		res[2]=b[2]*a[3]+b[3]*a[2];
		res[1]=b[1]*a[3]+b[3]*a[1];
		res[0]=b[0]*a[3]+b[1]*a[2]-b[2]*a[1]+b[3]*a[0];
        res
    }
}

// Dot
// The inner product.
impl BitOr for R2 {
    type Output = R2;

    fn bitor(self: R2, b: R2) -> R2 {
        let mut res = R2::zero();
        let a = self;
        res[0]=b[0]*a[0]+b[1]*a[1]+b[2]*a[2]-b[3]*a[3];
		res[1]=b[1]*a[0]+b[0]*a[1]-b[3]*a[2]+b[2]*a[3];
		res[2]=b[2]*a[0]+b[3]*a[1]+b[0]*a[2]-b[1]*a[3];
		res[3]=b[3]*a[0]+b[0]*a[3];
        res
    }
}

// Add
// Multivector addition
impl Add for R2 {
    type Output = R2;

    fn add(self: R2, b: R2) -> R2 {
        let mut res = R2::zero();
        let a = self;
        res[0] = a[0]+b[0];
		res[1] = a[1]+b[1];
		res[2] = a[2]+b[2];
		res[3] = a[3]+b[3];
        res
    }
}

// Sub
// Multivector subtraction
impl Sub for R2 {
    type Output = R2;

    fn sub(self: R2, b: R2) -> R2 {
        let mut res = R2::zero();
        let a = self;
        res[0] = a[0]-b[0];
		res[1] = a[1]-b[1];
		res[2] = a[2]-b[2];
		res[3] = a[3]-b[3];
        res
    }
}

// smul
// scalar/multivector multiplication
impl Mul<R2> for float_t {
    type Output = R2;

    fn mul(self: float_t, b: R2) -> R2 {
        let mut res = R2::zero();
        let a = self;
        res[0] = a*b[0];
        res[1] = a*b[1];
        res[2] = a*b[2];
        res[3] = a*b[3];
        res
    }
}

// muls
// multivector/scalar multiplication
impl Mul<float_t> for R2 {
    type Output = R2;

    fn mul(self: R2, b: float_t) -> R2 {
        let mut res = R2::zero();
        let a = self;
        res[0] = a[0]*b;
        res[1] = a[1]*b;
        res[2] = a[2]*b;
        res[3] = a[3]*b;
        res
    }
    }

// sadd
// scalar/multivector addition
impl Add<R2> for float_t {
    type Output = R2;

    fn add(self: float_t, b: R2) -> R2 {
        let mut res = R2::zero();
        let a = self;
        res[0] = a+b[0];
        res[1] = b[1];
        res[2] = b[2];
        res[3] = b[3];
        res
    }
}

// adds
// multivector/scalar addition
impl Add<float_t> for R2 {
    type Output = R2;

    fn add(self: R2, b: float_t) -> R2 {
        let mut res = R2::zero();
        let a = self;
        res[0] = a[0]+b;
        res[1] = a[1];
        res[2] = a[2];
        res[3] = a[3];
        res
    }
    }

impl R2 {
    pub fn norm(self: Self) -> float_t {
        let scalar_part = (self * self.Conjugate())[0];

        scalar_part.abs().sqrt()
    }

    pub fn inorm(self: Self) -> float_t {
        self.Dual().norm()
    }

    pub fn normalized(self: Self) -> Self {
        self * (1.0 / self.norm())
    }
    
    

}


fn main() {

  println!("e1*e1         : {}", e1 * e1);
  println!("pss           : {}", e12);
  println!("pss*pss       : {}", e12*e12);

}