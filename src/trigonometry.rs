use super::{floor,round};
use super::constant::{COS,SIN,TAN,COTAN};

/// Simplified sinus function (degrees, 45.0 = 45°).
#[inline]
pub fn sin<T:SinFn>(x:T)->f32{x.sin()}

pub trait SinFn{fn sin(self)->f32;}

impl SinFn for f32{
    #[inline]
    fn sin(self)->f32{
        let x:f32=self-floor(self*0.0027777778)*360.0;
        if x<180.0{let rcl:f32=x*0.011111111-1.0;let rcl:f32=rcl*rcl;(1.0-rcl)*(1.0-0.2240081*rcl)}
        else{let rcl:f32=x*0.011111111-3.0;let rcl:f32=rcl*rcl;(rcl-1.0)*(1.0-0.2240081*rcl)}
    }
}

impl SinFn for isize{
    #[inline]
    fn sin(self)->f32{
        let rcl:isize=self.rem_euclid(360);let x:isize=rcl/90;let rcl:isize=rcl%90;
        match x{
            0=>SIN[rcl as usize],
            1=>SIN[(90-rcl) as usize],
            2=>-SIN[rcl as usize],
            _=>-SIN[(90-rcl) as usize]
        }
    }
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
pub fn cos<T:CosFn>(x:T)->f32{x.cos()}

pub trait CosFn{fn cos(self)->f32;}

impl CosFn for f32{
    #[inline]
    fn cos(self)->f32{
        sin(self+90.0)
    }
}

impl CosFn for isize{
    #[inline]
    fn cos(self)->f32{
        let rcl:isize=self.rem_euclid(360);let x:isize=rcl/90;let rcl:isize=rcl%90;
        match x{
            0=>COS[rcl as usize],
            1=>-COS[(90-rcl) as usize],
            2=>-COS[rcl as usize],
            _=>COS[(90-rcl) as usize]
        }
    }
}

/// Simplified cosinus function (radians, 1.0 = π).
#[inline]
pub fn cosr(x:f32)->f32{
    sinr(x+0.5)
}


/// Simplified tangens function (degrees, 45.0 = 45°).
#[inline]
pub fn tan<T:TanFn>(x:T)->f32{x.tan()}

pub trait TanFn{fn tan(self)->f32;}

impl TanFn for f32{
    #[inline]
    fn tan(self)->f32{
        let x:f32=self*0.0055555556;let x:f32=(x-round(x))*180.0;
        let rcl:f32=x*x;(x*(141.37167-0.0031465*rcl))/(8100.0-rcl)
    }
}

impl TanFn for isize{
    #[inline]
    fn tan(self)->f32{
        let rcl:isize=self.rem_euclid(360);let q:isize=rcl/90;let r:isize=rcl%90;
        match q{
            0=>TAN[r as usize],
            1=>-COTAN[r as usize],
            2=>TAN[r as usize],
            _=>-COTAN[r as usize]
        }
    }
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
pub fn cotan<T:CotanFn>(x:T)->f32{x.cotan()}

pub trait CotanFn{fn cotan(self)->f32;}

impl CotanFn for f32{
    #[inline]
    fn cotan(self)->f32{
        -tan(self+90.0)
    }
}

impl CotanFn for isize{
    #[inline]
    fn cotan(self)->f32{
        let rcl:isize=self.rem_euclid(360);let x:isize=rcl/90;let rcl:isize=rcl%90;
        match x{
            0=>COTAN[rcl as usize],
            1=>-TAN[rcl as usize],
            2=>COTAN[rcl as usize],
            _=>-TAN[rcl as usize]
        }
    }
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