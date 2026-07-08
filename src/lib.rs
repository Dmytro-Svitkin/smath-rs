#![no_std]

#[inline(always)]
fn floor(x:f32)->f32{
    let rcl:i32=x as i32;
    if x<rcl as f32{(rcl-1)as f32}else{rcl as f32}
}

#[inline(always)]
fn round(x:f32)->f32{
  if x>0.0{floor(x+0.5)}
  else {-floor(-x+0.5)}
}

pub mod trigonometry{
    use super::{floor,round};

    /// Simplified sinus function (degrees, 45.0 = 45°).
    #[inline]
    pub fn sin(x:f32)->f32{
        let x:f32=x-floor(x*0.0027777778)*360.0;
        if x<180.0{let rcl:f32=x*0.011111111-1.0;let rcl:f32=rcl*rcl;(1.0-rcl)*(1.0-0.2240081*rcl)}
        else{let rcl:f32=x*0.011111111-3.0;let rcl:f32=rcl*rcl;(rcl-1.0)*(1.0-0.2240081*rcl)}
    }

    /// Simplified sinus function (radians, 1.0 = π).
    #[inline]
    pub fn sinr(x:f32)->f32{
        let x:f32=x-floor(x*0.5)*2.0;
        if x<1.0{let rcl:f32=x*2.0-1.0;let rcl:f32=rcl*rcl;(1.0-rcl)*(1.0-0.2240081*rcl)}
        else{let rcl:f32=x*2.0-3.0;let rcl:f32=rcl*rcl;(rcl-1.0)*(1.0-0.2240081*rcl)}
    }

    /// Simplified cosinus function (degrees, 45.0 = 45°).
    #[inline]
    pub fn cos(x:f32)->f32{
        sin(x+90.0)
    }

    /// Simplified cosinus function (radians, 1.0 = π).
    #[inline]
    pub fn cosr(x:f32)->f32{
        sinr(x+0.5)
    }

    /// Simplified tangens function (degrees, 45.0 = 45°).
    #[inline]
    pub fn tan(x:f32)->f32{
        let x:f32=x*0.0055555556;let x:f32=(x-round(x))*180.0;
        let rcl:f32=x*x;(x*(141.37167-0.0031465*rcl))/(8100.0-rcl)
    }

    /// Simplified tangens function (degrees, 45.0 = 45°).
    #[inline(always)]
    pub fn tg(x:f32)->f32{tan(x)}

    /// Simplified tangens function (radians, 1.0 = π).
    #[inline]
    pub fn tanr(x:f32)->f32{
        let x:f32=x-round(x);
        let rcl:f32=x*x;(x*(0.78539816-0.5663706*rcl))/(0.25-rcl)
    }

    /// Simplified tangens function (radians, 1.0 = π).
    #[inline(always)]
    pub fn tgr(x:f32)->f32{
        tanr(x)
    }

    /// Simplified cotangens function (degrees, 45.0 = 45°).
    #[inline]
    pub fn cotan(x:f32)->f32{
        -tan(x+90.0)
    }

    /// Simplified cotangens function (degrees, 45.0 = 45°).
    #[inline(always)]
    pub fn ctg(x:f32)->f32{
        cotan(x)
    }

    /// Simplified cotangens function (radians, 1.0 = π).
    #[inline]
    pub fn cotanr(x:f32)->f32{
        -tanr(x+0.5)
    }

    /// Simplified cotangens function (radians, 1.0 = π).
    #[inline(always)]
    pub fn ctgr(x:f32)->f32{
        cotanr(x)
    }

    /// Simplified arc-sinus function (degrees, 45.0 = 45°).
    #[inline]
    pub fn arcsin(x:f32)->f32{
        let rcl:f32=x*x;let rcl:f32=rcl*rcl*rcl;
        x*(60.0+rcl*30.0)
    }

    /// Simplified arc-sinus function (radians, 1.0 = π).
    #[inline]
    pub fn arcsinr(x:f32)->f32{
        let rcl:f32=x*x;let rcl:f32=rcl*rcl*rcl;
        x*(0.33333334+rcl*0.16666667)
    }

    /// Simplified arc-cosinus function (degrees, 45.0 = 45°).
    #[inline]
    pub fn arccos(x:f32)->f32{
        90.0-arcsin(x)
    }

    /// Simplified arc-cosinus function (radians, 1.0 = π).
    #[inline]
    pub fn arccosr(x:f32)->f32{
        0.5-arcsinr(x)
    }

    /// Simplified arc-tangens function (degrees, 45.0 = 45°).
    #[inline]
    pub fn arctan(x:f32)->f32{
        if x>0.0&&x<2.0{let x:f32=1.3*x;let rcl:f32=x-2.0;(x/(x+1.0))*(90.0-(rcl*rcl*10.0))}
        else if x>0.0{90.0-90.0/(1.3*x+1.0)}
        else if x>-2.0{let x:f32=-1.3*x;let rcl:f32=2.0-x;-(x/(x+1.0))*(90.0-(rcl*rcl*10.0))}
        else{90.0/(-1.3*x+1.0)-90.0}
    }

    /// Simplified arc-tangens function (radians, 1.0 = π).
    #[inline]
    pub fn arctanr(x:f32)->f32{
        if x>0.0&&x<2.0{let x:f32=1.3*x;let rcl:f32=x-2.0;(x/(x+1.0))*(0.5-(rcl*rcl*0.055555556))}
        else if x>0.0{0.5-0.5/(1.3*x+1.0)}
        else if x>-2.0{let x:f32=-1.3*x;let rcl:f32=2.0-x;-(x/(x+1.0))*(0.5-(rcl*rcl*0.055555556))}
        else{1.0/(-2.6*x+2.0)-0.5}
    }

    /// Simplified arc-cotangens function (degrees, 45.0 = 45°).
    #[inline]
    pub fn arccotan(x:f32)->f32{
        90.0-arctan(x)
    }

    /// Simplified arc-cotangens function (degrees, 45.0 = 45°).
    #[inline]
    pub fn arccotanr(x:f32)->f32{
        0.5-arctanr(x)
    }
}
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

pub mod vector{
    use core::ops::{Add,AddAssign,Div,DivAssign,Mul,MulAssign,Neg,Sub,SubAssign};

    /// A 2D vector representing spatial coordinates (x, y).
    #[derive(Debug,Copy,Clone,Default)]
    pub struct Vec2{pub x:f32,pub y:f32}

    impl Vec2{
            /// Create a new Vec2 (2D vector).
            #[inline]
            pub const fn new(x:f32,y:f32)->Self{
                Self{x,y}
            }

            /// Returns a vector with all components set to 0.0.
            #[inline]
            pub const fn zero()->Self{
                Self{x:0.0,y:0.0}
            }

            /// Returns a vector with all components set to 1.0.
            #[inline]
            pub const fn one()->Self{
                Self{x:1.0,y:1.0}
            }

            /// Sets the vector's components to the given values.
            #[inline]
            pub fn set(&mut self,x_val:f32,y_val:f32){
                self.x=x_val;
                self.y=y_val
            }

            /// Shifts the vector's components by the given values.
            #[inline]
            pub fn shift(&mut self,x_val:f32,y_val:f32){
                self.x+=x_val;
                self.y+=y_val
            }
        }

        impl Add for Vec2{
            type Output=Self;
            #[inline]
            fn add(self,other:Self)->Self{
                Self::new(self.x+other.x,self.y+other.y)
            }
        }

        impl Sub for Vec2{
            type Output=Self;
            #[inline]
            fn sub(self,other:Self)->Self{
                Self::new(self.x-other.x,self.y-other.y)
            }
        }

        impl Mul<f32>for Vec2{
            type Output=Self;
            #[inline]
            fn mul(self,scalar:f32)->Self{
                Self::new(self.x*scalar,self.y*scalar)
            }
        }

        impl Div<f32>for Vec2{
            type Output=Self;
            #[inline]
            fn div(self,scalar:f32)->Self{
                Self::new(self.x/scalar,self.y/scalar)
            }
        }

        impl Neg for Vec2{
            type Output=Self;
            #[inline]
            fn neg(self)->Self{
                Self::new(-self.x,-self.y)
            }
        }

        impl AddAssign for Vec2{
            #[inline]
            fn add_assign(&mut self,other:Self){
                self.x+=other.x;
                self.y+=other.y;
            }
        }

        impl SubAssign for Vec2{
            #[inline]
            fn sub_assign(&mut self,other:Self){
                self.x-=other.x;
                self.y-=other.y;
            }
        }

        impl MulAssign<f32>for Vec2{
            #[inline]
            fn mul_assign(&mut self,scalar:f32){
                self.x*=scalar;
                self.y*=scalar;
            }
        }

        impl DivAssign<f32>for Vec2{
            #[inline]
            fn div_assign(&mut self,scalar:f32){
                self.x/=scalar;
                self.y/=scalar;
            }
        }

    /// A 3D vector representing spatial coordinates (x, y, z).
    #[derive(Debug,Copy,Clone,Default)]
    pub struct Vec3{pub x:f32,pub y:f32,pub z:f32}

    impl Vec3{
            /// Create a new Vec3 (3D vector).
            #[inline]
            pub const fn new(x:f32,y:f32,z:f32)->Self{
                Self{x,y,z}
            }

            /// Returns a vector with all components set to 0.0.
            #[inline]
            pub const fn zero()->Self{
                Self{x:0.0,y:0.0,z:0.0}
            }

            /// Returns a vector with all components set to 1.0.
            #[inline]
            pub const fn one()->Self{
                Self{x:1.0,y:1.0,z:1.0}
            }

            /// Sets the vector's components to the given values.
            #[inline]
            pub fn set(&mut self,x_val:f32,y_val:f32,z_val:f32){
                self.x=x_val;
                self.y=y_val;
                self.z=z_val
            }

            /// Shifts the vector's components by the given values.
            #[inline]
            pub fn shift(&mut self,x_val:f32,y_val:f32,z_val:f32){
                self.x+=x_val;
                self.y+=y_val;
                self.z+=z_val
            }
        }

        impl Add for Vec3{
            type Output=Self;
            #[inline]
            fn add(self,other:Self)->Self{
                Self::new(self.x+other.x,self.y+other.y,self.z+other.z)
            }
        }

        impl Sub for Vec3{
            type Output=Self;
            #[inline]
            fn sub(self,other:Self)->Self{
                Self::new(self.x-other.x,self.y-other.y,self.z-other.z)
            }
        }

        impl Mul<f32>for Vec3{
            type Output=Self;
            #[inline]
            fn mul(self,scalar:f32)->Self{
                Self::new(self.x*scalar,self.y*scalar,self.z*scalar)
            }
        }

        impl Div<f32>for Vec3{
            type Output=Self;
            #[inline]
            fn div(self,scalar:f32)->Self{
                Self::new(self.x/scalar,self.y/scalar,self.z/scalar)
            }
        }

        impl Neg for Vec3{
            type Output=Self;
            #[inline]
            fn neg(self)->Self{
                Self::new(-self.x,-self.y,-self.z)
            }
        }

        impl AddAssign for Vec3{
            #[inline]
            fn add_assign(&mut self,other:Self){
                self.x+=other.x;
                self.y+=other.y;
                self.z+=other.z;
            }
        }

        impl SubAssign for Vec3{
            #[inline]
            fn sub_assign(&mut self,other:Self){
                self.x-=other.x;
                self.y-=other.y;
                self.z-=other.z;
            }
        }

        impl MulAssign<f32>for Vec3{
            #[inline]
            fn mul_assign(&mut self,scalar:f32){
                self.x*=scalar;
                self.y*=scalar;
                self.z*=scalar;
            }
        }

        impl DivAssign<f32>for Vec3{
            #[inline]
            fn div_assign(&mut self,scalar:f32){
                self.x/=scalar;
                self.y/=scalar;
                self.z/=scalar;
            }
        }

    /// A 4D vector representing spatial coordinates (x, y, z, w).
    #[derive(Debug,Copy,Clone,Default)]
    pub struct Vec4{pub x:f32,pub y:f32,pub z:f32,pub w:f32}

    impl Vec4{
            /// Create a new Vec4 (4D vector).
            #[inline]
            pub const fn new(x:f32,y:f32,z:f32,w:f32)->Self{
                Self{x,y,z,w}
            }

            /// Returns a vector with all components set to 0.0.
            #[inline]
            pub const fn zero()->Self{
                Self{x:0.0,y:0.0,z:0.0,w:0.0}
            }

            /// Returns a vector with all components set to 1.0.
            #[inline]
            pub const fn one()->Self{
                Self{x:1.0,y:1.0,z:1.0,w:1.0}
            }

            /// Sets the vector's components to the given values.
            #[inline]
            pub fn set(&mut self,x_val:f32,y_val:f32,z_val:f32,w_val:f32){
                self.x=x_val;
                self.y=y_val;
                self.z=z_val;
                self.w=w_val
            }

            /// Shifts the vector's components by the given values.
            #[inline]
            pub fn shift(&mut self,x_val:f32,y_val:f32,z_val:f32,w_val:f32){
                self.x+=x_val;
                self.y+=y_val;
                self.z+=z_val;
                self.w+=w_val
            }
        }

        impl Add for Vec4{
            type Output=Self;
            #[inline]
            fn add(self,other:Self)->Self{
                Self::new(self.x+other.x,self.y+other.y,self.z+other.z,self.w+other.w)
            }
        }

        impl Sub for Vec4{
            type Output=Self;
            #[inline]
            fn sub(self,other:Self)->Self{
                Self::new(self.x-other.x,self.y-other.y,self.z-other.z,self.w-other.w)
            }
        }

        impl Mul<f32>for Vec4{
            type Output=Self;
            #[inline]
            fn mul(self,scalar:f32)->Self{
                Self::new(self.x*scalar,self.y*scalar,self.z*scalar,self.w*scalar)
            }
        }

        impl Div<f32>for Vec4{
            type Output=Self;
            #[inline]
            fn div(self,scalar:f32)->Self{
                Self::new(self.x/scalar,self.y/scalar,self.z/scalar,self.w/scalar)
            }
        }

        impl Neg for Vec4{
            type Output=Self;
            #[inline]
            fn neg(self)->Self{
                Self::new(-self.x,-self.y,-self.z,-self.w)
            }
        }

        impl AddAssign for Vec4{
            #[inline]
            fn add_assign(&mut self,other:Self){
                self.x+=other.x;
                self.y+=other.y;
                self.z+=other.z;
                self.w+=other.w;
            }
        }

        impl SubAssign for Vec4{
            #[inline]
            fn sub_assign(&mut self,other:Self){
                self.x-=other.x;
                self.y-=other.y;
                self.z-=other.z;
                self.w-=other.w;
            }
        }

        impl MulAssign<f32>for Vec4{
            #[inline]
            fn mul_assign(&mut self,scalar:f32){
                self.x*=scalar;
                self.y*=scalar;
                self.z*=scalar;
                self.w*=scalar;
            }
        }

        impl DivAssign<f32>for Vec4{
            #[inline]
            fn div_assign(&mut self,scalar:f32){
                self.x/=scalar;
                self.y/=scalar;
                self.z/=scalar;
                self.w/=scalar;
            }
        }
}
    pub use vector::*;