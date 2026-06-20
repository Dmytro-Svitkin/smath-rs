#![no_std]

#[inline(always)]
fn floor(x:f32)->f32{
    let rcl:i32=x as i32;
    if x<rcl as f32{(rcl-1)as f32}else {rcl as f32}
}

#[inline(always)]
fn round(x:f32)->f32{
  if x>0.0{floor(x+0.5)}
  else {-floor(-x+0.5)}
}

pub use trigonometry::*;
pub use conversion::*;

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

pub mod conversion{
    pub mod length{
        /// Convert inches to millimeters.
        #[inline]
        pub const fn in_mm(x:f32)->f32{
            x*25.4
        }

        /// Convert inches to millimeters.
        #[inline(always)]
        pub const fn inches_millimeters(x:f32)->f32{
            in_mm(x)
        }

        /// Convert millimeters to inches.
        #[inline]
        pub const fn mm_in(x:f32)->f32{
            x*0.03937007874015748
        }

        /// Convert millimeters to inches.
        #[inline(always)]
        pub const fn millimeters_inches(x:f32)->f32{
            mm_in(x)
        }
        
        /// Convert feet to meters.
        #[inline]
        pub const fn ft_m(x:f32)->f32{
            x*0.3048
        }

        /// Convert feet to meters.
        #[inline(always)]
        pub const fn feet_meters(x:f32)->f32{
            ft_m(x)
        }

        /// Convert meters to feet.
        #[inline]
        pub const fn m_ft(x:f32)->f32{
            x*3.280839895013123
        }

        /// Convert meters to feet.
        #[inline(always)]
        pub const fn meters_feet(x:f32)->f32{
            m_ft(x)
        }

        /// Convert yards to meters.
        #[inline]
        pub const fn yd_m(x:f32)->f32{
            x*0.9144
        }

        /// Convert yards to meters.
        #[inline(always)]
        pub const fn yards_meters(x:f32)->f32{
            yd_m(x)
        }

        /// Convert meters to yards.
        #[inline]
        pub const fn m_yd(x:f32)->f32{
            x*1.0936132983377078
        }

        /// Convert meters to yards.
        #[inline(always)]
        pub const fn meters_yards(x:f32)->f32{
            m_yd(x)
        }
        
        /// Convert miles to kilometers.
        #[inline]
        pub const fn mi_km(x:f32)->f32{
            x*1.609344
        }

        /// Convert miles to kilometers.
        #[inline(always)]
        pub const fn miles_kilometers(x:f32)->f32{
            mi_km(x)
        }

        /// Convert kilometers to miles.
        #[inline]
        pub const fn km_mi(x:f32)->f32{
            x*0.621371192237334
        }

        /// Convert kilometers to miles.
        #[inline(always)]
        pub const fn kilometers_miles(x:f32)->f32{
            km_mi(x)
        }

        /// Convert nautical miles to kilometers.
        #[inline]
        pub const fn nmi_km(x:f32)->f32{
            x*1.852
        }

        /// Convert nautical miles to kilometers.
        #[inline(always)]
        pub const fn nautical_kilometers(x:f32)->f32{
            nmi_km(x)
        }

        /// Convert nautical miles to kilometers.
        #[inline(always)]
        pub const fn nauticalmiles_kilometers(x:f32)->f32{
            nmi_km(x)
        }

        /// Convert kilometers to nautical miles.
        #[inline]
        pub const fn km_nmi(x:f32)->f32{
            x*0.5399568034557235
        }

        /// Convert kilometers to nautical miles.
        #[inline(always)]
        pub const fn kilometers_nautical(x:f32)->f32{
            km_nmi(x)
        }

        /// Convert kilometers to nautical miles.
        #[inline(always)]
        pub const fn kilometers_nauticalmiles(x:f32)->f32{
            km_nmi(x)
        }
    }

    pub mod weight{

    }
}