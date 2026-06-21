#![no_std]

pub use conversion::*;

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
        /// Convert ounces to grams.
        #[inline]
        pub const fn oz_g(x:f32)->f32{
            x*28.349523
        }

        /// Convert ounces to grams.
        #[inline(always)]
        pub const fn ounces_grams(x:f32)->f32{
            oz_g(x)
        }

        /// Convert grams to ounces.
        #[inline]
        pub const fn g_oz(x:f32)->f32{
            x*0.03527396
        }

        /// Convert grams to ounces.
        #[inline(always)]
        pub const fn grams_ounces(x:f32)->f32{
            g_oz(x)
        }

        /// Convert pounds to kilograms.
        #[inline]
        pub const fn lb_kg(x:f32)->f32{
            x*0.45359237
        }

        /// Convert pounds to kilograms.
        #[inline(always)]
        pub const fn pounds_kilograms(x:f32)->f32{
            lb_kg(x)
        }

        /// Convert kilograms to pounds.
        #[inline]
        pub const fn kg_lb(x:f32)->f32{
            x*2.20462262
        }

        /// Convert kilograms to pounds.
        #[inline(always)]
        pub const fn kilograms_pounds(x:f32)->f32{
            kg_lb(x)
        }

        /// Convert stones to kilograms.
        #[inline]
        pub const fn st_kg(x:f32)->f32{
            x*6.35029318
        }

        /// Convert stones to kilograms.
        #[inline(always)]
        pub const fn stones_kilograms(x:f32)->f32{
            st_kg(x)
        }

        /// Convert kilograms to stones.
        #[inline]
        pub const fn kg_st(x:f32)->f32{
            x*0.15747304
        }

        /// Convert kilograms to stones.
        #[inline(always)]
        pub const fn kilograms_stones(x:f32)->f32{
            kg_st(x)
        }

        /// Convert tons to tonnes.
        #[inline]
        pub const fn tn_t(x:f32)->f32{
            x*0.90718474
        }

        /// Convert tons to tonnes.
        #[inline(always)]
        pub const fn tons_tonnes(x:f32)->f32{
            tn_t(x)
        }

        /// Convert tonnes to tons.
        #[inline]
        pub const fn t_tn(x:f32)->f32{
            x*1.10231131
        }

        /// Convert tonnes to tons.
        #[inline(always)]
        pub const fn tonnes_tons(x:f32)->f32{
            t_tn(x)
        }
    }

    pub mod volume{
        /// Convert fluid ounces to milliliters.
        #[inline]
        pub const fn floz_ml(x:f32)->f32{
            x*29.57353
        }

        /// Convert fluid ounces to milliliters.
        #[inline(always)]
        pub const fn fluidounces_milliliters(x:f32)->f32{
            floz_ml(x)
        }

        /// Convert milliliters to fluid ounces.
        #[inline]
        pub const fn ml_floz(x:f32)->f32{
            x*0.033814022
        }

        /// Convert milliliters to fluid ounces.
        #[inline(always)]
        pub const fn milliliters_fluidounces(x:f32)->f32{
            ml_floz(x)
        }

        /// Convert cups to milliliters.
        #[inline]
        pub const fn cp_ml(x:f32)->f32{
            x*236.58824
        }

        /// Convert cups to milliliters.
        #[inline(always)]
        pub const fn cups_milliliters(x:f32)->f32{
            cp_ml(x)
        }

        /// Convert milliliters to cups.
        #[inline]
        pub const fn ml_cp(x:f32)->f32{
            x*0.0042267527
        }

        /// Convert milliliters to cups.
        #[inline(always)]
        pub const fn milliliters_cups(x:f32)->f32{
            ml_cp(x)
        }

        /// Convert pints to liters.
        #[inline]
        pub const fn pt_l(x:f32)->f32{
            x*0.47317648
        }

        /// Convert pints to liters.
        #[inline(always)]
        pub const fn pints_liters(x:f32)->f32{
            pt_l(x)
        }

        /// Convert liters to pints.
        #[inline]
        pub const fn l_pt(x:f32)->f32{
            x*2.1133764
        }

        /// Convert liters to pints.
        #[inline(always)]
        pub const fn liters_pints(x:f32)->f32{
            l_pt(x)
        }

        /// Convert quarts to liters.
        #[inline]
        pub const fn qt_l(x:f32)->f32{
            x*0.94635295
        }

        /// Convert quarts to liters.
        #[inline(always)]
        pub const fn quarts_liters(x:f32)->f32{
            qt_l(x)
        }

        /// Convert liters to quarts.
        #[inline]
        pub const fn l_qt(x:f32)->f32{
            x*1.0566882
        }

        /// Convert liters to quarts.
        #[inline(always)]
        pub const fn liters_quarts(x:f32)->f32{
            l_qt(x)
        }

        /// Convert gallons to liters.
        #[inline]
        pub const fn gal_l(x:f32)->f32{
            x*3.7854118
        }

        /// Convert gallons to liters.
        #[inline(always)]
        pub const fn gallons_liters(x:f32)->f32{
            gal_l(x)
        }

        /// Convert liters to gallons.
        #[inline]
        pub const fn l_gal(x:f32)->f32{
            x*0.26417205
        }

        /// Convert liters to gallons.
        #[inline(always)]
        pub const fn liters_gallons(x:f32)->f32{
            l_gal(x)
        }
    }

    pub mod temperature{
        /// Convert Celsius to Fahrenheit.
        #[inline]
        pub const fn c_f(x:f32)->f32{
            (x*1.8)+32.0
        }

        /// Convert Celsius to Fahrenheit.
        #[inline(always)]
        pub const fn celsius_fahrenheit(x:f32)->f32{
            c_f(x)
        }

        /// Convert Fahrenheit to Celsius.
        #[inline]
        pub const fn f_c(x:f32)->f32{
            (x-32.0)/1.8
        }

        /// Convert Fahrenheit to Celsius.
        #[inline(always)]
        pub const fn fahrenheit_celsius(x:f32)->f32{
            f_c(x)
        }

        /// Convert Celsius to Kelvin.
        #[inline]
        pub const fn c_k(x:f32)->f32{
            x+273.15
        }

        /// Convert Celsius to Kelvin.
        #[inline(always)]
        pub const fn celsius_kelvin(x:f32)->f32{
            c_k(x)
        }

        /// Convert Kelvin to Celsius.
        #[inline]
        pub const fn k_c(x:f32)->f32{
            x-273.15
        }

        /// Convert Kelvin to Celsius.
        #[inline(always)]
        pub const fn kelvin_celsius(x:f32)->f32{
            k_c(x)
        }

        /// Convert Fahrenheit to Kelvin.
        #[inline]
        pub const fn f_k(x:f32)->f32{
            ((x-32.0)/1.8)+273.15
        }

        /// Convert Fahrenheit to Kelvin.
        #[inline(always)]
        pub const fn fahrenheit_kelvin(x:f32)->f32{
            f_k(x)
        }

        /// Convert Kelvin to Fahrenheit.
        #[inline]
        pub const fn k_f(x:f32)->f32{
            ((x-273.15)*1.8)+32.0
        }

        /// Convert Kelvin to Fahrenheit.
        #[inline(always)]
        pub const fn kelvin_fahrenheit(x:f32)->f32{
            k_f(x)
        }
    }

    pub mod speed{
        /// Convert miles per hour to kilometers per hour.
        #[inline]
        pub const fn mph_kmh(x:f32)->f32{
            x*1.609344
        }

        /// Convert miles per hour to kilometers per hour.
        #[inline(always)]
        pub const fn mph_kilometershour(x:f32)->f32{
            mph_kmh(x)
        }

        /// Convert kilometers per hour to miles per hour.
        #[inline]
        pub const fn kmh_mph(x:f32)->f32{
            x*0.6213712
        }

        /// Convert kilometers per hour to miles per hour.
        #[inline(always)]
        pub const fn kilometershour_mph(x:f32)->f32{
            kmh_mph(x)
        }

        /// Convert knots to kilometers per hour.
        #[inline]
        pub const fn kt_kmh(x:f32)->f32{
            x*1.852
        }

        /// Convert knots to kilometers per hour.
        #[inline(always)]
        pub const fn knots_kilometershour(x:f32)->f32{
            kt_kmh(x)
        }

        /// Convert kilometers per hour to knots.
        #[inline]
        pub const fn kmh_kt(x:f32)->f32{
            x*0.5399568
        }

        /// Convert kilometers per hour to knots.
        #[inline(always)]
        pub const fn kilometershour_knots(x:f32)->f32{
            kmh_kt(x)
        }

        /// Convert Mach to meters per second.
        #[inline]
        pub const fn mach_ms(x:f32)->f32{
            x*343.0
        }

        /// Convert Mach to meters per second.
        #[inline(always)]
        pub const fn mach_meterssecond(x:f32)->f32{
            mach_ms(x)
        }

        /// Convert meters per second to Mach.
        #[inline]
        pub const fn ms_mach(x:f32)->f32{
            x*0.002915452
        }

        /// Convert meters per second to Mach.
        #[inline(always)]
        pub const fn meterssecond_mach(x:f32)->f32{
            ms_mach(x)
        }
    }

    pub mod pressure{
        /// Convert pounds per square inch to pascals.
        #[inline]
        pub const fn psi_pa(x:f32)->f32{
            x*6894.7573
        }

        /// Convert pounds per square inch to pascals.
        #[inline(always)]
        pub const fn psi_pascals(x:f32)->f32{
            psi_pa(x)
        }

        /// Convert pascals to pounds per square inch.
        #[inline]
        pub const fn pa_psi(x:f32)->f32{
            x*0.00014503774
        }

        /// Convert pascals to pounds per square inch.
        #[inline(always)]
        pub const fn pascals_psi(x:f32)->f32{
            pa_psi(x)
        }

        /// Convert bar to pascals.
        #[inline]
        pub const fn bar_pa(x:f32)->f32{
            x*100000.0
        }

        /// Convert bar to pascals.
        #[inline(always)]
        pub const fn bar_pascals(x:f32)->f32{
            bar_pa(x)
        }

        /// Convert pascals to bar.
        #[inline]
        pub const fn pa_bar(x:f32)->f32{
            x*0.00001
        }

        /// Convert pascals to bar.
        #[inline(always)]
        pub const fn pascals_bar(x:f32)->f32{
            pa_bar(x)
        }

        /// Convert atmospheres to pascals.
        #[inline]
        pub const fn atm_pa(x:f32)->f32{
            x*101325.0
        }

        /// Convert atmospheres to pascals.
        #[inline(always)]
        pub const fn atmospheres_pascals(x:f32)->f32{
            atm_pa(x)
        }

        /// Convert pascals to atmospheres.
        #[inline]
        pub const fn pa_atm(x:f32)->f32{
            x*0.000009869233
        }

        /// Convert pascals to atmospheres.
        #[inline(always)]
        pub const fn pascals_atm(x:f32)->f32{
            pa_atm(x)
        }
    }

    pub mod energy{
        /// Convert Joules to calories.
        #[inline]
        pub const fn j_cal(x:f32)->f32{
            x*0.23900574
        }

        /// Convert Joules to calories.
        #[inline(always)]
        pub const fn joules_calories(x:f32)->f32{
            j_cal(x)
        }

        /// Convert calories to Joules.
        #[inline]
        pub const fn cal_j(x:f32)->f32{
            x*4.184
        }

        /// Convert calories to Joules.
        #[inline(always)]
        pub const fn calories_joules(x:f32)->f32{
            cal_j(x)
        }

        /// Convert horsepower to Watts.
        #[inline]
        pub const fn hp_w(x:f32)->f32{
            x*745.6999
        }

        /// Convert horsepower to Watts.
        #[inline(always)]
        pub const fn horsepower_watts(x:f32)->f32{
            hp_w(x)
        }

        /// Convert Watts to horsepower.
        #[inline]
        pub const fn w_hp(x:f32)->f32{
            x*0.0013410221
        }

        /// Convert Watts to horsepower.
        #[inline(always)]
        pub const fn watts_horsepower(x:f32)->f32{
            w_hp(x)
        }
    }
}