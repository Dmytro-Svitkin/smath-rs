use super::{floor,round,sqrt};
use super::constant::{COS,SIN,TAN,COTAN};

/// Simplified sinus function (degrees, 45.0 = 45°).
#[inline]
pub fn sin<T:SinFn>(x:T)->f32{x.sin()}

pub trait SinFn{fn sin(self)->f32;}

impl SinFn for f32{
    #[inline]
    fn sin(self)->f32{
        #[inline(always)]
        const fn rcl(x:f32)->f32{
            let x:f32=x-floor(x*0.0027777778)*360.0;
            if x<180.0{let rcl:f32=x*0.011111111-1.0;let rcl:f32=rcl*rcl;(1.0-rcl)*(1.0-0.2240081*rcl)}
            else{let rcl:f32=x*0.011111111-3.0;let rcl:f32=rcl*rcl;(rcl-1.0)*(1.0-0.2240081*rcl)}
        }
        rcl(self)
    }
}

impl SinFn for isize{
    #[inline]
    fn sin(self)->f32{
        #[inline(always)]
        const fn rcl(x:isize)->f32{
            let x:isize=x.rem_euclid(360);let rcl:isize=x/90;let x:isize=x%90;
            match rcl{
                0=>SIN[x as usize],
                1=>COS[x as usize],
                2=>-SIN[x as usize],
                _=>-COS[x as usize]
            }
        }
        rcl(self)
    }
}

/// Simplified sinus function (radians, 1.0 = π).
#[inline]
pub const fn sinr(x:f32)->f32{
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
        #[inline(always)]
        const fn rcl(x:f32)->f32{
            let x:f32=x-floor(x*0.0027777778)*360.0;
            if x<180.0{let rcl:f32=x*0.011111111-1.0;let rcl:f32=rcl*rcl;(1.0-rcl)*(1.0-0.2240081*rcl)}
            else{let rcl:f32=x*0.011111111-3.0;let rcl:f32=rcl*rcl;(rcl-1.0)*(1.0-0.2240081*rcl)}
        }
        rcl(self+90.0)
    }
}

impl CosFn for isize{
    #[inline]
    fn cos(self)->f32{
        #[inline(always)]
        const fn rcl(x:isize)->f32{
            let x:isize=x.rem_euclid(360);let rcl:isize=x/90;let x:isize=x%90;
            match rcl{
                0=>COS[x as usize],
                1=>-SIN[x as usize],
                2=>-COS[x as usize],
                _=>SIN[x as usize]
            }
        }
        rcl(self)
    }
}

/// Simplified cosinus function (radians, 1.0 = π).
#[inline]
pub const fn cosr(x:f32)->f32{
    sinr(x+0.5)
}


/// Simplified tangens function (degrees, 45.0 = 45°).
#[inline]
pub fn tan<T:TanFn>(x:T)->f32{x.tan()}

pub trait TanFn{fn tan(self)->f32;}

impl TanFn for f32{
    #[inline]
    fn tan(self)->f32{
        #[inline(always)]
        const fn rcl(x:f32)->f32{
            let x:f32=x*0.0055555556;let x:f32=(x-round(x))*180.0;
            let rcl:f32=x*x;(x*(141.37167-0.0031465*rcl))/(8100.0-rcl)
        }
        rcl(self)
    }
}

impl TanFn for isize{
    #[inline]
    fn tan(self)->f32{
        #[inline(always)]
        const fn rcl(x:isize)->f32{
            let x:isize=x.rem_euclid(360);let rcl:isize=x/90;let x:isize=x%90;
            match rcl{
                0=>TAN[x as usize],
                1=>-COTAN[x as usize],
                2=>TAN[x as usize],
                _=>-COTAN[x as usize]
            }
        }
        rcl(self)
    }
}

/// Simplified tangens function (degrees, 45.0 = 45°).
#[inline(always)]
pub fn tg<T:TanFn>(x:T)->f32{
    tan(x)
}

/// Simplified tangens function (radians, 1.0 = π).
#[inline]
pub const fn tanr(x:f32)->f32{
    let x:f32=x-round(x);
    let rcl:f32=x*x;(x*(0.78539816-0.5663706*rcl))/(0.25-rcl)
}

/// Simplified tangens function (radians, 1.0 = π).
#[inline(always)]
pub const fn tgr(x:f32)->f32{
    tanr(x)
}

/// Simplified cotangens function (degrees, 45.0 = 45°).
#[inline]
pub fn cotan<T:CotanFn>(x:T)->f32{x.cotan()}

pub trait CotanFn{fn cotan(self)->f32;}

impl CotanFn for f32{
    #[inline]
    fn cotan(self)->f32{
        #[inline(always)]
        const fn rcl(x:f32)->f32{
            let x:f32=x*0.0055555556;let x:f32=(x-round(x))*180.0;
            let rcl:f32=x*x;(x*(141.37167-0.0031465*rcl))/(8100.0-rcl)
        }
        -rcl(self+90.0)
    }
}

impl CotanFn for isize{
    #[inline]
    fn cotan(self)->f32{
        #[inline(always)]
        const fn rcl(x:isize)->f32{
            let x:isize=x.rem_euclid(360);let rcl:isize=x/90;let x:isize=x%90;
            match rcl{
                0=>COTAN[x as usize],
                1=>-TAN[x as usize],
                2=>COTAN[x as usize],
                _=>-TAN[x as usize]
            }
        }
        rcl(self)
    }
}

/// Simplified cotangens function (degrees, 45.0 = 45°).
#[inline(always)]
pub fn ctg<T:CotanFn>(x:T)->f32{
    cotan(x)
}

/// Simplified cotangens function (radians, 1.0 = π).
#[inline]
pub const fn cotanr(x:f32)->f32{
    -tanr(x+0.5)
}

/// Simplified cotangens function (radians, 1.0 = π).
#[inline(always)]
pub const fn ctgr(x:f32)->f32{
    cotanr(x)
}

/// Simplified arc-sinus function (degrees, 45.0 = 45°).
#[inline]
pub const fn arcsin(x:f32)->f32{
    if x<0.0{sqrt(1.0-x)*(90.0+(12.15324+(4.25484+1.07311*x)*x)*x)-90.0}
    else{90.0-sqrt(1.0-x)*(90.0-(12.15324-(4.25484-1.07311*x)*x)*x)}
}

/// Simplified arc-sinus function (degrees, 45.0 = 45°).
#[inline]
pub const fn asin(x:f32)->f32{
    arcsin(x)
}

/// Simplified arc-sinus function (radians, 1.0 = π).
#[inline]
pub const fn arcsinr(x:f32)->f32{
    if x<0.0{sqrt(1.0-x)*(0.5+(0.0675181+(0.0236380+0.0059617*x)*x)*x)-0.5}
    else{0.5-sqrt(1.0-x)*(0.5-(0.0675181-(0.0236380-0.0059617*x)*x)*x)}
}

/// Simplified arc-sinus function (radians, 1.0 = π).
#[inline]
pub const fn asinr(x:f32)->f32{
    arcsinr(x)
}

/// Simplified arc-cosinus function (degrees, 45.0 = 45°).
#[inline]
pub const fn arccos(x:f32)->f32{
    90.0-arcsin(x)
}

/// Simplified arc-cosinus function (degrees, 45.0 = 45°).
#[inline]
pub const fn acos(x:f32)->f32{
    arccos(x)
}

/// Simplified arc-cosinus function (radians, 1.0 = π).
#[inline]
pub const fn arccosr(x:f32)->f32{
    0.5-arcsinr(x)
}

/// Simplified arc-cosinus function (radians, 1.0 = π).
#[inline]
pub const fn acosr(x:f32)->f32{
    arccosr(x)
}

/// Simplified arc-tangens function (degrees, 45.0 = 45°).
#[inline]
pub const fn arctan(x:f32)->f32{
    if x>0.0&&x<2.0{let x:f32=1.3*x;let rcl:f32=x-2.0;(x/(x+1.0))*(90.0-(rcl*rcl*10.0))}
    else if x>0.0{90.0-90.0/(1.3*x+1.0)}
    else if x>-2.0{let x:f32=-1.3*x;let rcl:f32=2.0-x;-(x/(x+1.0))*(90.0-(rcl*rcl*10.0))}
    else{90.0/(-1.3*x+1.0)-90.0}
}

/// Simplified arc-tangens function (degrees, 45.0 = 45°).
#[inline]
pub const fn atan(x:f32)->f32{
    arctan(x)
}

/// Simplified arc-tangens function (radians, 1.0 = π).
#[inline]
pub const fn arctanr(x:f32)->f32{
    if x>0.0&&x<2.0{let x:f32=1.3*x;let rcl:f32=x-2.0;(x/(x+1.0))*(0.5-(rcl*rcl*0.055555556))}
    else if x>0.0{0.5-0.5/(1.3*x+1.0)}
    else if x>-2.0{let x:f32=-1.3*x;let rcl:f32=2.0-x;-(x/(x+1.0))*(0.5-(rcl*rcl*0.055555556))}
    else{1.0/(-2.6*x+2.0)-0.5}
}

/// Simplified arc-tangens function (radians, 1.0 = π).
#[inline]
pub const fn atanr(x:f32)->f32{
    arctanr(x)
}

/// Simplified arc-cotangens function (degrees, 45.0 = 45°).
#[inline]
pub const fn arccotan(x:f32)->f32{
    90.0-arctan(x)
}

/// Simplified arc-cotangens function (degrees, 45.0 = 45°).
#[inline]
pub const fn acotan(x:f32)->f32{
    arccotan(x)
}

/// Simplified arc-cotangens function (degrees, 45.0 = 45°).
#[inline]
pub const fn arccotanr(x:f32)->f32{
    0.5-arctanr(x)
}

/// Simplified arc-cotangens function (degrees, 45.0 = 45°).
#[inline]
pub const fn acotanr(x:f32)->f32{
    arccotanr(x)
}