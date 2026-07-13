#![no_std]

#[inline(always)]
const fn floor(x:f32)->f32{
    let rcl:i32=x as i32;
    if x<rcl as f32{(rcl-1)as f32}else{rcl as f32}
}

#[inline(always)]
const fn round(x:f32)->f32{
  if x>0.0{floor(x+0.5)}
  else {-floor(-x+0.5)}
}

/// Simplified square root.
#[inline]
pub const fn sqrt(x:f32)->f32{
    if x<0.0{return f32::NAN;}if x==0.0||x==f32::INFINITY{return x;}
    let rcl:u32=x.to_bits();let rcl:f32=f32::from_bits(0x5f375a86-((rcl)>>1));let rcl:f32=rcl*(1.5-(0.5*x*rcl*rcl));let rcl:f32=x*rcl;0.5*(rcl+x/rcl)
}

/// Simplified inverse square root.
#[inline]
pub const fn isqrt(x:f32)->f32{
    if x<0.0{return f32::NAN;}if x==0.0||x==f32::INFINITY{return 0.0;}
    let rcl:u32=x.to_bits();let rcl:f32=f32::from_bits(0x5f375a86-((rcl)>>1));let rcl:f32=rcl*(1.5-(0.5*x*rcl*rcl));rcl
}

pub mod constant;
    pub use constant::*;

pub mod trigonometry;
    pub use trigonometry::*;

pub mod conversion;
    pub use conversion::*;
    pub use conversion::length::*;
    pub use conversion::weight::*;
    pub use conversion::volume::*;
    pub use conversion::speed::*;
    pub use conversion::pressure::*;
    pub use conversion::energy::*;
    pub use conversion::time::*;

pub mod vector;
    pub use vector::*;

pub mod matrix;
    pub use matrix::*;