#![no_std]

#[inline(always)]
fn floor(x:f32)->f32{
    let rcl:i32=x as i32;
    if x<rcl as f32{(rcl-1)as f32}else {rcl as f32}
}

#[inline(always)]
fn round(x:f32)->f32{
  floor(x+0.5)
}

pub use trigonometry::*;

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