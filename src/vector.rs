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

        impl Mul<Vec2>for Vec2{
            type Output=Self;
            #[inline]
            fn mul(self,other:Self)->Self{
                Self::new(self.x*other.x,self.y*other.y)
            }
        }

        impl Div<f32>for Vec2{
            type Output=Self;
            #[inline]
            fn div(self,scalar:f32)->Self{
                Self::new(self.x/scalar,self.y/scalar)
            }
        }

        impl Div<Vec2>for Vec2{
            type Output=Self;
            #[inline]
            fn div(self,other:Self)->Self{
                Self::new(self.x/other.x,self.y/other.y)
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

        impl MulAssign<Vec2>for Vec2{
            #[inline]
            fn mul_assign(&mut self,other:Self){
                self.x*=other.x;
                self.y*=other.y;
            }
        }

        impl DivAssign<f32>for Vec2{
            #[inline]
            fn div_assign(&mut self,scalar:f32){
                self.x/=scalar;
                self.y/=scalar;
            }
        }

        impl DivAssign<Vec2>for Vec2{
            #[inline]
            fn div_assign(&mut self,other:Self){
                self.x/=other.x;
                self.y/=other.y;
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

        impl Mul<Vec3>for Vec3{
            type Output=Self;
            #[inline]
            fn mul(self,other:Self)->Self{
                Self::new(self.x*other.x,self.y*other.y,self.z*other.z)
            }
        }

        impl Div<f32>for Vec3{
            type Output=Self;
            #[inline]
            fn div(self,scalar:f32)->Self{
                Self::new(self.x/scalar,self.y/scalar,self.z/scalar)
            }
        }

        impl Div<Vec3>for Vec3{
            type Output=Self;
            #[inline]
            fn div(self,other:Self)->Self{
                Self::new(self.x/other.x,self.y/other.y,self.z/other.z)
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

        impl MulAssign<Vec3>for Vec3{
            #[inline]
            fn mul_assign(&mut self,other:Self){
                self.x*=other.x;
                self.y*=other.y;
                self.z*=other.z;
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

        impl DivAssign<Vec3>for Vec3{
            #[inline]
            fn div_assign(&mut self,other:Self){
                self.x/=other.x;
                self.y/=other.y;
                self.z/=other.z;
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

        impl Mul<Vec4>for Vec4{
            type Output=Self;
            #[inline]
            fn mul(self,other:Self)->Self{
                Self::new(self.x*other.x,self.y*other.y,self.z*other.z,self.w*other.w)
            }
        }

        impl Div<f32>for Vec4{
            type Output=Self;
            #[inline]
            fn div(self,scalar:f32)->Self{
                Self::new(self.x/scalar,self.y/scalar,self.z/scalar,self.w/scalar)
            }
        }

        impl Div<Vec4>for Vec4{
            type Output=Self;
            #[inline]
            fn div(self,other:Self)->Self{
                Self::new(self.x/other.x,self.y/other.y,self.z/other.z,self.w/other.w)
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

        impl MulAssign<Vec4>for Vec4{
            #[inline]
            fn mul_assign(&mut self,other:Self){
                self.x*=other.x;
                self.y*=other.y;
                self.z*=other.z;
                self.w*=other.w;
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

        impl DivAssign<Vec4>for Vec4{
            #[inline]
            fn div_assign(&mut self,other:Self){
                self.x/=other.x;
                self.y/=other.y;
                self.z/=other.z;
                self.w/=other.w;
            }
        }
