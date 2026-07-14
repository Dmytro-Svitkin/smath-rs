pub mod length{
    /// Convert millimeters to centimeters.
    #[inline]
    pub const fn mm_cm(x:f32)->f32{
        x*0.1
    }

    /// Convert millimeters to decimeters.
    #[inline]
    pub const fn mm_dm(x:f32)->f32{
        x*0.01
    }

    /// Convert millimeters to meters.
    #[inline]
    pub const fn mm_m(x:f32)->f32{
        x*0.001
    }

    /// Convert millimeters to kilometers.
    #[inline]
    pub const fn mm_km(x:f32)->f32{
        x*0.000001
    }

    /// Convert millimeters to inches.
    #[inline]
    pub const fn mm_in(x:f32)->f32{
        x*0.03937008
    }

    /// Convert millimeters to feet.
    #[inline]
    pub const fn mm_ft(x:f32)->f32{
        x*0.00328084
    }

    /// Convert millimeters to yards.
    #[inline]
    pub const fn mm_yd(x:f32)->f32{
        x*0.00109361
    }

    /// Convert millimeters to miles.
    #[inline]
    pub const fn mm_mi(x:f32)->f32{
        x*0.00000062
    }

    /// Convert millimeters to nautical miles.
    #[inline]
    pub const fn mm_nmi(x:f32)->f32{
        x*0.00000054
    }

    /// Convert centimeters to millimeters.
    #[inline]
    pub const fn cm_mm(x:f32)->f32{
        x*10.0
    }

    /// Convert centimeters to decimeters.
    #[inline]
    pub const fn cm_dm(x:f32)->f32{
        x*0.1
    }

    /// Convert centimeters to meters.
    #[inline]
    pub const fn cm_m(x:f32)->f32{
        x*0.01
    }

    /// Convert centimeters to kilometers.
    #[inline]
    pub const fn cm_km(x:f32)->f32{
        x*0.00001
    }

    /// Convert centimeters to inches.
    #[inline]
    pub const fn cm_in(x:f32)->f32{
        x*0.39370079
    }

    /// Convert centimeters to feet.
    #[inline]
    pub const fn cm_ft(x:f32)->f32{
        x*0.0328084
    }

    /// Convert centimeters to yards.
    #[inline]
    pub const fn cm_yd(x:f32)->f32{
        x*0.01093613
    }

    /// Convert centimeters to miles.
    #[inline]
    pub const fn cm_mi(x:f32)->f32{
        x*0.00000621
    }

    /// Convert centimeters to nautical miles.
    #[inline]
    pub const fn cm_nmi(x:f32)->f32{
        x*0.0000054
    }

    /// Convert decimeters to millimeters.
    #[inline]
    pub const fn dm_mm(x:f32)->f32{
        x*100.0
    }

    /// Convert decimeters to centimeters.
    #[inline]
    pub const fn dm_cm(x:f32)->f32{
        x*10.0
    }

    /// Convert decimeters to meters.
    #[inline]
    pub const fn dm_m(x:f32)->f32{
        x*0.1
    }

    /// Convert decimeters to kilometers.
    #[inline]
    pub const fn dm_km(x:f32)->f32{
        x*0.0001
    }

    /// Convert decimeters to inches.
    #[inline]
    pub const fn dm_in(x:f32)->f32{
        x*3.93700787
    }

    /// Convert decimeters to feet.
    #[inline]
    pub const fn dm_ft(x:f32)->f32{
        x*0.32808399
    }

    /// Convert decimeters to yards.
    #[inline]
    pub const fn dm_yd(x:f32)->f32{
        x*0.10936133
    }

    /// Convert decimeters to miles.
    #[inline]
    pub const fn dm_mi(x:f32)->f32{
        x*0.00006214
    }

    /// Convert decimeters to nautical miles.
    #[inline]
    pub const fn dm_nmi(x:f32)->f32{
        x*0.000054
    }

    /// Convert meters to millimeters.
    #[inline]
    pub const fn m_mm(x:f32)->f32{
        x*1000.0
    }

    /// Convert meters to centimeters.
    #[inline]
    pub const fn m_cm(x:f32)->f32{
        x*100.0
    }

    /// Convert meters to decimeters.
    #[inline]
    pub const fn m_dm(x:f32)->f32{
        x*10.0
    }

    /// Convert meters to kilometers.
    #[inline]
    pub const fn m_km(x:f32)->f32{
        x*0.001
    }

    /// Convert meters to inches.
    #[inline]
    pub const fn m_in(x:f32)->f32{
        x*39.37007874
    }

    /// Convert meters to feet.
    #[inline]
    pub const fn m_ft(x:f32)->f32{
        x*3.2808399
    }

    /// Convert meters to yards.
    #[inline]
    pub const fn m_yd(x:f32)->f32{
        x*1.0936133
    }

    /// Convert meters to miles.
    #[inline]
    pub const fn m_mi(x:f32)->f32{
        x*0.00062137
    }

    /// Convert meters to nautical miles.
    #[inline]
    pub const fn m_nmi(x:f32)->f32{
        x*0.00053996
    }

    /// Convert kilometers to millimeters.
    #[inline]
    pub const fn km_mm(x:f32)->f32{
        x*1000000.0
    }

    /// Convert kilometers to centimeters.
    #[inline]
    pub const fn km_cm(x:f32)->f32{
        x*100000.0
    }

    /// Convert kilometers to decimeters.
    #[inline]
    pub const fn km_dm(x:f32)->f32{
        x*10000.0
    }

    /// Convert kilometers to meters.
    #[inline]
    pub const fn km_m(x:f32)->f32{
        x*1000.0
    }

    /// Convert kilometers to inches.
    #[inline]
    pub const fn km_in(x:f32)->f32{
        x*39370.07874016
    }

    /// Convert kilometers to feet.
    #[inline]
    pub const fn km_ft(x:f32)->f32{
        x*3280.83989501
    }

    /// Convert kilometers to yards.
    #[inline]
    pub const fn km_yd(x:f32)->f32{
        x*1093.61329834
    }

    /// Convert kilometers to miles.
    #[inline]
    pub const fn km_mi(x:f32)->f32{
        x*0.62137119
    }

    /// Convert kilometers to nautical miles.
    #[inline]
    pub const fn km_nmi(x:f32)->f32{
        x*0.5399568
    }

    /// Convert inches to millimeters.
    #[inline]
    pub const fn in_mm(x:f32)->f32{
        x*25.4
    }

    /// Convert inches to centimeters.
    #[inline]
    pub const fn in_cm(x:f32)->f32{
        x*2.54
    }

    /// Convert inches to decimeters.
    #[inline]
    pub const fn in_dm(x:f32)->f32{
        x*0.254
    }

    /// Convert inches to meters.
    #[inline]
    pub const fn in_m(x:f32)->f32{
        x*0.0254
    }

    /// Convert inches to kilometers.
    #[inline]
    pub const fn in_km(x:f32)->f32{
        x*0.0000254
    }

    /// Convert inches to feet.
    #[inline]
    pub const fn in_ft(x:f32)->f32{
        x*0.08333333
    }

    /// Convert inches to yards.
    #[inline]
    pub const fn in_yd(x:f32)->f32{
        x*0.02777778
    }

    /// Convert inches to miles.
    #[inline]
    pub const fn in_mi(x:f32)->f32{
        x*0.00001578
    }

    /// Convert inches to nautical miles.
    #[inline]
    pub const fn in_nmi(x:f32)->f32{
        x*0.00001371
    }

    /// Convert feet to millimeters.
    #[inline]
    pub const fn ft_mm(x:f32)->f32{
        x*304.8
    }

    /// Convert feet to centimeters.
    #[inline]
    pub const fn ft_cm(x:f32)->f32{
        x*30.48
    }

    /// Convert feet to decimeters.
    #[inline]
    pub const fn ft_dm(x:f32)->f32{
        x*3.048
    }

    /// Convert feet to meters.
    #[inline]
    pub const fn ft_m(x:f32)->f32{
        x*0.3048
    }

    /// Convert feet to kilometers.
    #[inline]
    pub const fn ft_km(x:f32)->f32{
        x*0.0003048
    }

    /// Convert feet to inches.
    #[inline]
    pub const fn ft_in(x:f32)->f32{
        x*12.0
    }

    /// Convert feet to yards.
    #[inline]
    pub const fn ft_yd(x:f32)->f32{
        x*0.33333333
    }

    /// Convert feet to miles.
    #[inline]
    pub const fn ft_mi(x:f32)->f32{
        x*0.00018939
    }

    /// Convert feet to nautical miles.
    #[inline]
    pub const fn ft_nmi(x:f32)->f32{
        x*0.00016458
    }

    /// Convert yards to millimeters.
    #[inline]
    pub const fn yd_mm(x:f32)->f32{
        x*914.4
    }

    /// Convert yards to centimeters.
    #[inline]
    pub const fn yd_cm(x:f32)->f32{
        x*91.44
    }

    /// Convert yards to decimeters.
    #[inline]
    pub const fn yd_dm(x:f32)->f32{
        x*9.144
    }

    /// Convert yards to meters.
    #[inline]
    pub const fn yd_m(x:f32)->f32{
        x*0.9144
    }

    /// Convert yards to kilometers.
    #[inline]
    pub const fn yd_km(x:f32)->f32{
        x*0.0009144
    }

    /// Convert yards to inches.
    #[inline]
    pub const fn yd_in(x:f32)->f32{
        x*36.0
    }

    /// Convert yards to feet.
    #[inline]
    pub const fn yd_ft(x:f32)->f32{
        x*3.0
    }

    /// Convert yards to miles.
    #[inline]
    pub const fn yd_mi(x:f32)->f32{
        x*0.00056818
    }

    /// Convert yards to nautical miles.
    #[inline]
    pub const fn yd_nmi(x:f32)->f32{
        x*0.00049374
    }

    /// Convert miles to millimeters.
    #[inline]
    pub const fn mi_mm(x:f32)->f32{
        x*1609344.0
    }

    /// Convert miles to centimeters.
    #[inline]
    pub const fn mi_cm(x:f32)->f32{
        x*160934.4
    }

    /// Convert miles to decimeters.
    #[inline]
    pub const fn mi_dm(x:f32)->f32{
        x*16093.44
    }

    /// Convert miles to meters.
    #[inline]
    pub const fn mi_m(x:f32)->f32{
        x*1609.344
    }

    /// Convert miles to kilometers.
    #[inline]
    pub const fn mi_km(x:f32)->f32{
        x*1.609344
    }

    /// Convert miles to inches.
    #[inline]
    pub const fn mi_in(x:f32)->f32{
        x*63360.0
    }

    /// Convert miles to feet.
    #[inline]
    pub const fn mi_ft(x:f32)->f32{
        x*5280.0
    }

    /// Convert miles to yards.
    #[inline]
    pub const fn mi_yd(x:f32)->f32{
        x*1760.0
    }

    /// Convert miles to nautical miles.
    #[inline]
    pub const fn mi_nmi(x:f32)->f32{
        x*0.86897624
    }

    /// Convert nautical miles to millimeters.
    #[inline]
    pub const fn nmi_mm(x:f32)->f32{
        x*1852000.0
    }

    /// Convert nautical miles to centimeters.
    #[inline]
    pub const fn nmi_cm(x:f32)->f32{
        x*185200.0
    }

    /// Convert nautical miles to decimeters.
    #[inline]
    pub const fn nmi_dm(x:f32)->f32{
        x*18520.0
    }

    /// Convert nautical miles to meters.
    #[inline]
    pub const fn nmi_m(x:f32)->f32{
        x*1852.0
    }

    /// Convert nautical miles to kilometers.
    #[inline]
    pub const fn nmi_km(x:f32)->f32{
        x*1.852
    }

    /// Convert nautical miles to inches.
    #[inline]
    pub const fn nmi_in(x:f32)->f32{
        x*72913.38582677
    }

    /// Convert nautical miles to feet.
    #[inline]
    pub const fn nmi_ft(x:f32)->f32{
        x*6076.11548556
    }

    /// Convert nautical miles to yards.
    #[inline]
    pub const fn nmi_yd(x:f32)->f32{
        x*2025.37182852
    }

    /// Convert nautical miles to miles.
    #[inline]
    pub const fn nmi_mi(x:f32)->f32{
        x*1.15077945
    }

    /// Convert millimeters to centimeters.
    #[inline(always)]
    pub const fn mm_centimeter(x:f32)->f32{
        mm_cm(x)
    }

    /// Convert millimeters to centimeters.
    #[inline(always)]
    pub const fn mm_centimeters(x:f32)->f32{
        mm_cm(x)
    }

    /// Convert millimeters to centimeters.
    #[inline(always)]
    pub const fn millimeter_cm(x:f32)->f32{
        mm_cm(x)
    }

    /// Convert millimeters to centimeters.
    #[inline(always)]
    pub const fn millimeter_centimeter(x:f32)->f32{
        mm_cm(x)
    }

    /// Convert millimeters to centimeters.
    #[inline(always)]
    pub const fn millimeter_centimeters(x:f32)->f32{
        mm_cm(x)
    }

    /// Convert millimeters to centimeters.
    #[inline(always)]
    pub const fn millimeters_cm(x:f32)->f32{
        mm_cm(x)
    }

    /// Convert millimeters to centimeters.
    #[inline(always)]
    pub const fn millimeters_centimeter(x:f32)->f32{
        mm_cm(x)
    }

    /// Convert millimeters to centimeters.
    #[inline(always)]
    pub const fn millimeters_centimeters(x:f32)->f32{
        mm_cm(x)
    }

    /// Convert millimeters to decimeters.
    #[inline(always)]
    pub const fn mm_decimeter(x:f32)->f32{
        mm_dm(x)
    }

    /// Convert millimeters to decimeters.
    #[inline(always)]
    pub const fn mm_decimeters(x:f32)->f32{
        mm_dm(x)
    }

    /// Convert millimeters to decimeters.
    #[inline(always)]
    pub const fn millimeter_dm(x:f32)->f32{
        mm_dm(x)
    }

    /// Convert millimeters to decimeters.
    #[inline(always)]
    pub const fn millimeter_decimeter(x:f32)->f32{
        mm_dm(x)
    }

    /// Convert millimeters to decimeters.
    #[inline(always)]
    pub const fn millimeter_decimeters(x:f32)->f32{
        mm_dm(x)
    }

    /// Convert millimeters to decimeters.
    #[inline(always)]
    pub const fn millimeters_dm(x:f32)->f32{
        mm_dm(x)
    }

    /// Convert millimeters to decimeters.
    #[inline(always)]
    pub const fn millimeters_decimeter(x:f32)->f32{
        mm_dm(x)
    }

    /// Convert millimeters to decimeters.
    #[inline(always)]
    pub const fn millimeters_decimeters(x:f32)->f32{
        mm_dm(x)
    }

    /// Convert millimeters to meters.
    #[inline(always)]
    pub const fn mm_meter(x:f32)->f32{
        mm_m(x)
    }

    /// Convert millimeters to meters.
    #[inline(always)]
    pub const fn mm_meters(x:f32)->f32{
        mm_m(x)
    }

    /// Convert millimeters to meters.
    #[inline(always)]
    pub const fn millimeter_m(x:f32)->f32{
        mm_m(x)
    }

    /// Convert millimeters to meters.
    #[inline(always)]
    pub const fn millimeter_meter(x:f32)->f32{
        mm_m(x)
    }

    /// Convert millimeters to meters.
    #[inline(always)]
    pub const fn millimeter_meters(x:f32)->f32{
        mm_m(x)
    }

    /// Convert millimeters to meters.
    #[inline(always)]
    pub const fn millimeters_m(x:f32)->f32{
        mm_m(x)
    }

    /// Convert millimeters to meters.
    #[inline(always)]
    pub const fn millimeters_meter(x:f32)->f32{
        mm_m(x)
    }

    /// Convert millimeters to meters.
    #[inline(always)]
    pub const fn millimeters_meters(x:f32)->f32{
        mm_m(x)
    }

    /// Convert millimeters to kilometers.
    #[inline(always)]
    pub const fn mm_kilometer(x:f32)->f32{
        mm_km(x)
    }

    /// Convert millimeters to kilometers.
    #[inline(always)]
    pub const fn mm_kilometers(x:f32)->f32{
        mm_km(x)
    }

    /// Convert millimeters to kilometers.
    #[inline(always)]
    pub const fn millimeter_km(x:f32)->f32{
        mm_km(x)
    }

    /// Convert millimeters to kilometers.
    #[inline(always)]
    pub const fn millimeter_kilometer(x:f32)->f32{
        mm_km(x)
    }

    /// Convert millimeters to kilometers.
    #[inline(always)]
    pub const fn millimeter_kilometers(x:f32)->f32{
        mm_km(x)
    }

    /// Convert millimeters to kilometers.
    #[inline(always)]
    pub const fn millimeters_km(x:f32)->f32{
        mm_km(x)
    }

    /// Convert millimeters to kilometers.
    #[inline(always)]
    pub const fn millimeters_kilometer(x:f32)->f32{
        mm_km(x)
    }

    /// Convert millimeters to kilometers.
    #[inline(always)]
    pub const fn millimeters_kilometers(x:f32)->f32{
        mm_km(x)
    }

    /// Convert millimeters to inches.
    #[inline(always)]
    pub const fn mm_inch(x:f32)->f32{
        mm_in(x)
    }

    /// Convert millimeters to inches.
    #[inline(always)]
    pub const fn mm_inches(x:f32)->f32{
        mm_in(x)
    }

    /// Convert millimeters to inches.
    #[inline(always)]
    pub const fn millimeter_in(x:f32)->f32{
        mm_in(x)
    }

    /// Convert millimeters to inches.
    #[inline(always)]
    pub const fn millimeter_inch(x:f32)->f32{
        mm_in(x)
    }

    /// Convert millimeters to inches.
    #[inline(always)]
    pub const fn millimeter_inches(x:f32)->f32{
        mm_in(x)
    }

    /// Convert millimeters to inches.
    #[inline(always)]
    pub const fn millimeters_in(x:f32)->f32{
        mm_in(x)
    }

    /// Convert millimeters to inches.
    #[inline(always)]
    pub const fn millimeters_inch(x:f32)->f32{
        mm_in(x)
    }

    /// Convert millimeters to inches.
    #[inline(always)]
    pub const fn millimeters_inches(x:f32)->f32{
        mm_in(x)
    }

    /// Convert millimeters to feet.
    #[inline(always)]
    pub const fn mm_foot(x:f32)->f32{
        mm_ft(x)
    }

    /// Convert millimeters to feet.
    #[inline(always)]
    pub const fn mm_feet(x:f32)->f32{
        mm_ft(x)
    }

    /// Convert millimeters to feet.
    #[inline(always)]
    pub const fn millimeter_ft(x:f32)->f32{
        mm_ft(x)
    }

    /// Convert millimeters to feet.
    #[inline(always)]
    pub const fn millimeter_foot(x:f32)->f32{
        mm_ft(x)
    }

    /// Convert millimeters to feet.
    #[inline(always)]
    pub const fn millimeter_feet(x:f32)->f32{
        mm_ft(x)
    }

    /// Convert millimeters to feet.
    #[inline(always)]
    pub const fn millimeters_ft(x:f32)->f32{
        mm_ft(x)
    }

    /// Convert millimeters to feet.
    #[inline(always)]
    pub const fn millimeters_foot(x:f32)->f32{
        mm_ft(x)
    }

    /// Convert millimeters to feet.
    #[inline(always)]
    pub const fn millimeters_feet(x:f32)->f32{
        mm_ft(x)
    }

    /// Convert millimeters to yards.
    #[inline(always)]
    pub const fn mm_yard(x:f32)->f32{
        mm_yd(x)
    }

    /// Convert millimeters to yards.
    #[inline(always)]
    pub const fn mm_yards(x:f32)->f32{
        mm_yd(x)
    }

    /// Convert millimeters to yards.
    #[inline(always)]
    pub const fn millimeter_yd(x:f32)->f32{
        mm_yd(x)
    }

    /// Convert millimeters to yards.
    #[inline(always)]
    pub const fn millimeter_yard(x:f32)->f32{
        mm_yd(x)
    }

    /// Convert millimeters to yards.
    #[inline(always)]
    pub const fn millimeter_yards(x:f32)->f32{
        mm_yd(x)
    }

    /// Convert millimeters to yards.
    #[inline(always)]
    pub const fn millimeters_yd(x:f32)->f32{
        mm_yd(x)
    }

    /// Convert millimeters to yards.
    #[inline(always)]
    pub const fn millimeters_yard(x:f32)->f32{
        mm_yd(x)
    }

    /// Convert millimeters to yards.
    #[inline(always)]
    pub const fn millimeters_yards(x:f32)->f32{
        mm_yd(x)
    }

    /// Convert millimeters to miles.
    #[inline(always)]
    pub const fn mm_mile(x:f32)->f32{
        mm_mi(x)
    }

    /// Convert millimeters to miles.
    #[inline(always)]
    pub const fn mm_miles(x:f32)->f32{
        mm_mi(x)
    }

    /// Convert millimeters to miles.
    #[inline(always)]
    pub const fn millimeter_mi(x:f32)->f32{
        mm_mi(x)
    }

    /// Convert millimeters to miles.
    #[inline(always)]
    pub const fn millimeter_mile(x:f32)->f32{
        mm_mi(x)
    }

    /// Convert millimeters to miles.
    #[inline(always)]
    pub const fn millimeter_miles(x:f32)->f32{
        mm_mi(x)
    }

    /// Convert millimeters to miles.
    #[inline(always)]
    pub const fn millimeters_mi(x:f32)->f32{
        mm_mi(x)
    }

    /// Convert millimeters to miles.
    #[inline(always)]
    pub const fn millimeters_mile(x:f32)->f32{
        mm_mi(x)
    }

    /// Convert millimeters to miles.
    #[inline(always)]
    pub const fn millimeters_miles(x:f32)->f32{
        mm_mi(x)
    }

    /// Convert millimeters to nautical miles.
    #[inline(always)]
    pub const fn mm_nautical(x:f32)->f32{
        mm_nmi(x)
    }

    /// Convert millimeters to nautical miles.
    #[inline(always)]
    pub const fn mm_nauticalmile(x:f32)->f32{
        mm_nmi(x)
    }

    /// Convert millimeters to nautical miles.
    #[inline(always)]
    pub const fn mm_nauticalmiles(x:f32)->f32{
        mm_nmi(x)
    }

    /// Convert millimeters to nautical miles.
    #[inline(always)]
    pub const fn millimeter_nmi(x:f32)->f32{
        mm_nmi(x)
    }

    /// Convert millimeters to nautical miles.
    #[inline(always)]
    pub const fn millimeter_nautical(x:f32)->f32{
        mm_nmi(x)
    }

    /// Convert millimeters to nautical miles.
    #[inline(always)]
    pub const fn millimeter_nauticalmile(x:f32)->f32{
        mm_nmi(x)
    }

    /// Convert millimeters to nautical miles.
    #[inline(always)]
    pub const fn millimeter_nauticalmiles(x:f32)->f32{
        mm_nmi(x)
    }

    /// Convert millimeters to nautical miles.
    #[inline(always)]
    pub const fn millimeters_nmi(x:f32)->f32{
        mm_nmi(x)
    }

    /// Convert millimeters to nautical miles.
    #[inline(always)]
    pub const fn millimeters_nautical(x:f32)->f32{
        mm_nmi(x)
    }

    /// Convert millimeters to nautical miles.
    #[inline(always)]
    pub const fn millimeters_nauticalmile(x:f32)->f32{
        mm_nmi(x)
    }

    /// Convert millimeters to nautical miles.
    #[inline(always)]
    pub const fn millimeters_nauticalmiles(x:f32)->f32{
        mm_nmi(x)
    }

    /// Convert centimeters to millimeters.
    #[inline(always)]
    pub const fn cm_millimeter(x:f32)->f32{
        cm_mm(x)
    }

    /// Convert centimeters to millimeters.
    #[inline(always)]
    pub const fn cm_millimeters(x:f32)->f32{
        cm_mm(x)
    }

    /// Convert centimeters to millimeters.
    #[inline(always)]
    pub const fn centimeter_mm(x:f32)->f32{
        cm_mm(x)
    }

    /// Convert centimeters to millimeters.
    #[inline(always)]
    pub const fn centimeter_millimeter(x:f32)->f32{
        cm_mm(x)
    }

    /// Convert centimeters to millimeters.
    #[inline(always)]
    pub const fn centimeter_millimeters(x:f32)->f32{
        cm_mm(x)
    }

    /// Convert centimeters to millimeters.
    #[inline(always)]
    pub const fn centimeters_mm(x:f32)->f32{
        cm_mm(x)
    }

    /// Convert centimeters to millimeters.
    #[inline(always)]
    pub const fn centimeters_millimeter(x:f32)->f32{
        cm_mm(x)
    }

    /// Convert centimeters to millimeters.
    #[inline(always)]
    pub const fn centimeters_millimeters(x:f32)->f32{
        cm_mm(x)
    }

    /// Convert centimeters to decimeters.
    #[inline(always)]
    pub const fn cm_decimeter(x:f32)->f32{
        cm_dm(x)
    }

    /// Convert centimeters to decimeters.
    #[inline(always)]
    pub const fn cm_decimeters(x:f32)->f32{
        cm_dm(x)
    }

    /// Convert centimeters to decimeters.
    #[inline(always)]
    pub const fn centimeter_dm(x:f32)->f32{
        cm_dm(x)
    }

    /// Convert centimeters to decimeters.
    #[inline(always)]
    pub const fn centimeter_decimeter(x:f32)->f32{
        cm_dm(x)
    }

    /// Convert centimeters to decimeters.
    #[inline(always)]
    pub const fn centimeter_decimeters(x:f32)->f32{
        cm_dm(x)
    }

    /// Convert centimeters to decimeters.
    #[inline(always)]
    pub const fn centimeters_dm(x:f32)->f32{
        cm_dm(x)
    }

    /// Convert centimeters to decimeters.
    #[inline(always)]
    pub const fn centimeters_decimeter(x:f32)->f32{
        cm_dm(x)
    }

    /// Convert centimeters to decimeters.
    #[inline(always)]
    pub const fn centimeters_decimeters(x:f32)->f32{
        cm_dm(x)
    }

    /// Convert centimeters to meters.
    #[inline(always)]
    pub const fn cm_meter(x:f32)->f32{
        cm_m(x)
    }

    /// Convert centimeters to meters.
    #[inline(always)]
    pub const fn cm_meters(x:f32)->f32{
        cm_m(x)
    }

    /// Convert centimeters to meters.
    #[inline(always)]
    pub const fn centimeter_m(x:f32)->f32{
        cm_m(x)
    }

    /// Convert centimeters to meters.
    #[inline(always)]
    pub const fn centimeter_meter(x:f32)->f32{
        cm_m(x)
    }

    /// Convert centimeters to meters.
    #[inline(always)]
    pub const fn centimeter_meters(x:f32)->f32{
        cm_m(x)
    }

    /// Convert centimeters to meters.
    #[inline(always)]
    pub const fn centimeters_m(x:f32)->f32{
        cm_m(x)
    }

    /// Convert centimeters to meters.
    #[inline(always)]
    pub const fn centimeters_meter(x:f32)->f32{
        cm_m(x)
    }

    /// Convert centimeters to meters.
    #[inline(always)]
    pub const fn centimeters_meters(x:f32)->f32{
        cm_m(x)
    }

    /// Convert centimeters to kilometers.
    #[inline(always)]
    pub const fn cm_kilometer(x:f32)->f32{
        cm_km(x)
    }

    /// Convert centimeters to kilometers.
    #[inline(always)]
    pub const fn cm_kilometers(x:f32)->f32{
        cm_km(x)
    }

    /// Convert centimeters to kilometers.
    #[inline(always)]
    pub const fn centimeter_km(x:f32)->f32{
        cm_km(x)
    }

    /// Convert centimeters to kilometers.
    #[inline(always)]
    pub const fn centimeter_kilometer(x:f32)->f32{
        cm_km(x)
    }

    /// Convert centimeters to kilometers.
    #[inline(always)]
    pub const fn centimeter_kilometers(x:f32)->f32{
        cm_km(x)
    }

    /// Convert centimeters to kilometers.
    #[inline(always)]
    pub const fn centimeters_km(x:f32)->f32{
        cm_km(x)
    }

    /// Convert centimeters to kilometers.
    #[inline(always)]
    pub const fn centimeters_kilometer(x:f32)->f32{
        cm_km(x)
    }

    /// Convert centimeters to kilometers.
    #[inline(always)]
    pub const fn centimeters_kilometers(x:f32)->f32{
        cm_km(x)
    }

    /// Convert centimeters to inches.
    #[inline(always)]
    pub const fn cm_inch(x:f32)->f32{
        cm_in(x)
    }

    /// Convert centimeters to inches.
    #[inline(always)]
    pub const fn cm_inches(x:f32)->f32{
        cm_in(x)
    }

    /// Convert centimeters to inches.
    #[inline(always)]
    pub const fn centimeter_in(x:f32)->f32{
        cm_in(x)
    }

    /// Convert centimeters to inches.
    #[inline(always)]
    pub const fn centimeter_inch(x:f32)->f32{
        cm_in(x)
    }

    /// Convert centimeters to inches.
    #[inline(always)]
    pub const fn centimeter_inches(x:f32)->f32{
        cm_in(x)
    }

    /// Convert centimeters to inches.
    #[inline(always)]
    pub const fn centimeters_in(x:f32)->f32{
        cm_in(x)
    }

    /// Convert centimeters to inches.
    #[inline(always)]
    pub const fn centimeters_inch(x:f32)->f32{
        cm_in(x)
    }

    /// Convert centimeters to inches.
    #[inline(always)]
    pub const fn centimeters_inches(x:f32)->f32{
        cm_in(x)
    }

    /// Convert centimeters to feet.
    #[inline(always)]
    pub const fn cm_foot(x:f32)->f32{
        cm_ft(x)
    }

    /// Convert centimeters to feet.
    #[inline(always)]
    pub const fn cm_feet(x:f32)->f32{
        cm_ft(x)
    }

    /// Convert centimeters to feet.
    #[inline(always)]
    pub const fn centimeter_ft(x:f32)->f32{
        cm_ft(x)
    }

    /// Convert centimeters to feet.
    #[inline(always)]
    pub const fn centimeter_foot(x:f32)->f32{
        cm_ft(x)
    }

    /// Convert centimeters to feet.
    #[inline(always)]
    pub const fn centimeter_feet(x:f32)->f32{
        cm_ft(x)
    }

    /// Convert centimeters to feet.
    #[inline(always)]
    pub const fn centimeters_ft(x:f32)->f32{
        cm_ft(x)
    }

    /// Convert centimeters to feet.
    #[inline(always)]
    pub const fn centimeters_foot(x:f32)->f32{
        cm_ft(x)
    }

    /// Convert centimeters to feet.
    #[inline(always)]
    pub const fn centimeters_feet(x:f32)->f32{
        cm_ft(x)
    }

    /// Convert centimeters to yards.
    #[inline(always)]
    pub const fn cm_yard(x:f32)->f32{
        cm_yd(x)
    }

    /// Convert centimeters to yards.
    #[inline(always)]
    pub const fn cm_yards(x:f32)->f32{
        cm_yd(x)
    }

    /// Convert centimeters to yards.
    #[inline(always)]
    pub const fn centimeter_yd(x:f32)->f32{
        cm_yd(x)
    }

    /// Convert centimeters to yards.
    #[inline(always)]
    pub const fn centimeter_yard(x:f32)->f32{
        cm_yd(x)
    }

    /// Convert centimeters to yards.
    #[inline(always)]
    pub const fn centimeter_yards(x:f32)->f32{
        cm_yd(x)
    }

    /// Convert centimeters to yards.
    #[inline(always)]
    pub const fn centimeters_yd(x:f32)->f32{
        cm_yd(x)
    }

    /// Convert centimeters to yards.
    #[inline(always)]
    pub const fn centimeters_yard(x:f32)->f32{
        cm_yd(x)
    }

    /// Convert centimeters to yards.
    #[inline(always)]
    pub const fn centimeters_yards(x:f32)->f32{
        cm_yd(x)
    }

    /// Convert centimeters to miles.
    #[inline(always)]
    pub const fn cm_mile(x:f32)->f32{
        cm_mi(x)
    }

    /// Convert centimeters to miles.
    #[inline(always)]
    pub const fn cm_miles(x:f32)->f32{
        cm_mi(x)
    }

    /// Convert centimeters to miles.
    #[inline(always)]
    pub const fn centimeter_mi(x:f32)->f32{
        cm_mi(x)
    }

    /// Convert centimeters to miles.
    #[inline(always)]
    pub const fn centimeter_mile(x:f32)->f32{
        cm_mi(x)
    }

    /// Convert centimeters to miles.
    #[inline(always)]
    pub const fn centimeter_miles(x:f32)->f32{
        cm_mi(x)
    }

    /// Convert centimeters to miles.
    #[inline(always)]
    pub const fn centimeters_mi(x:f32)->f32{
        cm_mi(x)
    }

    /// Convert centimeters to miles.
    #[inline(always)]
    pub const fn centimeters_mile(x:f32)->f32{
        cm_mi(x)
    }

    /// Convert centimeters to miles.
    #[inline(always)]
    pub const fn centimeters_miles(x:f32)->f32{
        cm_mi(x)
    }

    /// Convert centimeters to nautical miles.
    #[inline(always)]
    pub const fn cm_nautical(x:f32)->f32{
        cm_nmi(x)
    }

    /// Convert centimeters to nautical miles.
    #[inline(always)]
    pub const fn cm_nauticalmile(x:f32)->f32{
        cm_nmi(x)
    }

    /// Convert centimeters to nautical miles.
    #[inline(always)]
    pub const fn cm_nauticalmiles(x:f32)->f32{
        cm_nmi(x)
    }

    /// Convert centimeters to nautical miles.
    #[inline(always)]
    pub const fn centimeter_nmi(x:f32)->f32{
        cm_nmi(x)
    }

    /// Convert centimeters to nautical miles.
    #[inline(always)]
    pub const fn centimeter_nautical(x:f32)->f32{
        cm_nmi(x)
    }

    /// Convert centimeters to nautical miles.
    #[inline(always)]
    pub const fn centimeter_nauticalmile(x:f32)->f32{
        cm_nmi(x)
    }

    /// Convert centimeters to nautical miles.
    #[inline(always)]
    pub const fn centimeter_nauticalmiles(x:f32)->f32{
        cm_nmi(x)
    }

    /// Convert centimeters to nautical miles.
    #[inline(always)]
    pub const fn centimeters_nmi(x:f32)->f32{
        cm_nmi(x)
    }

    /// Convert centimeters to nautical miles.
    #[inline(always)]
    pub const fn centimeters_nautical(x:f32)->f32{
        cm_nmi(x)
    }

    /// Convert centimeters to nautical miles.
    #[inline(always)]
    pub const fn centimeters_nauticalmile(x:f32)->f32{
        cm_nmi(x)
    }

    /// Convert centimeters to nautical miles.
    #[inline(always)]
    pub const fn centimeters_nauticalmiles(x:f32)->f32{
        cm_nmi(x)
    }

    /// Convert decimeters to millimeters.
    #[inline(always)]
    pub const fn dm_millimeter(x:f32)->f32{
        dm_mm(x)
    }

    /// Convert decimeters to millimeters.
    #[inline(always)]
    pub const fn dm_millimeters(x:f32)->f32{
        dm_mm(x)
    }

    /// Convert decimeters to millimeters.
    #[inline(always)]
    pub const fn decimeter_mm(x:f32)->f32{
        dm_mm(x)
    }

    /// Convert decimeters to millimeters.
    #[inline(always)]
    pub const fn decimeter_millimeter(x:f32)->f32{
        dm_mm(x)
    }

    /// Convert decimeters to millimeters.
    #[inline(always)]
    pub const fn decimeter_millimeters(x:f32)->f32{
        dm_mm(x)
    }

    /// Convert decimeters to millimeters.
    #[inline(always)]
    pub const fn decimeters_mm(x:f32)->f32{
        dm_mm(x)
    }

    /// Convert decimeters to millimeters.
    #[inline(always)]
    pub const fn decimeters_millimeter(x:f32)->f32{
        dm_mm(x)
    }

    /// Convert decimeters to millimeters.
    #[inline(always)]
    pub const fn decimeters_millimeters(x:f32)->f32{
        dm_mm(x)
    }

    /// Convert decimeters to centimeters.
    #[inline(always)]
    pub const fn dm_centimeter(x:f32)->f32{
        dm_cm(x)
    }

    /// Convert decimeters to centimeters.
    #[inline(always)]
    pub const fn dm_centimeters(x:f32)->f32{
        dm_cm(x)
    }

    /// Convert decimeters to centimeters.
    #[inline(always)]
    pub const fn decimeter_cm(x:f32)->f32{
        dm_cm(x)
    }

    /// Convert decimeters to centimeters.
    #[inline(always)]
    pub const fn decimeter_centimeter(x:f32)->f32{
        dm_cm(x)
    }

    /// Convert decimeters to centimeters.
    #[inline(always)]
    pub const fn decimeter_centimeters(x:f32)->f32{
        dm_cm(x)
    }

    /// Convert decimeters to centimeters.
    #[inline(always)]
    pub const fn decimeters_cm(x:f32)->f32{
        dm_cm(x)
    }

    /// Convert decimeters to centimeters.
    #[inline(always)]
    pub const fn decimeters_centimeter(x:f32)->f32{
        dm_cm(x)
    }

    /// Convert decimeters to centimeters.
    #[inline(always)]
    pub const fn decimeters_centimeters(x:f32)->f32{
        dm_cm(x)
    }

    /// Convert decimeters to meters.
    #[inline(always)]
    pub const fn dm_meter(x:f32)->f32{
        dm_m(x)
    }

    /// Convert decimeters to meters.
    #[inline(always)]
    pub const fn dm_meters(x:f32)->f32{
        dm_m(x)
    }

    /// Convert decimeters to meters.
    #[inline(always)]
    pub const fn decimeter_m(x:f32)->f32{
        dm_m(x)
    }

    /// Convert decimeters to meters.
    #[inline(always)]
    pub const fn decimeter_meter(x:f32)->f32{
        dm_m(x)
    }

    /// Convert decimeters to meters.
    #[inline(always)]
    pub const fn decimeter_meters(x:f32)->f32{
        dm_m(x)
    }

    /// Convert decimeters to meters.
    #[inline(always)]
    pub const fn decimeters_m(x:f32)->f32{
        dm_m(x)
    }

    /// Convert decimeters to meters.
    #[inline(always)]
    pub const fn decimeters_meter(x:f32)->f32{
        dm_m(x)
    }

    /// Convert decimeters to meters.
    #[inline(always)]
    pub const fn decimeters_meters(x:f32)->f32{
        dm_m(x)
    }

    /// Convert decimeters to kilometers.
    #[inline(always)]
    pub const fn dm_kilometer(x:f32)->f32{
        dm_km(x)
    }

    /// Convert decimeters to kilometers.
    #[inline(always)]
    pub const fn dm_kilometers(x:f32)->f32{
        dm_km(x)
    }

    /// Convert decimeters to kilometers.
    #[inline(always)]
    pub const fn decimeter_km(x:f32)->f32{
        dm_km(x)
    }

    /// Convert decimeters to kilometers.
    #[inline(always)]
    pub const fn decimeter_kilometer(x:f32)->f32{
        dm_km(x)
    }

    /// Convert decimeters to kilometers.
    #[inline(always)]
    pub const fn decimeter_kilometers(x:f32)->f32{
        dm_km(x)
    }

    /// Convert decimeters to kilometers.
    #[inline(always)]
    pub const fn decimeters_km(x:f32)->f32{
        dm_km(x)
    }

    /// Convert decimeters to kilometers.
    #[inline(always)]
    pub const fn decimeters_kilometer(x:f32)->f32{
        dm_km(x)
    }

    /// Convert decimeters to kilometers.
    #[inline(always)]
    pub const fn decimeters_kilometers(x:f32)->f32{
        dm_km(x)
    }

    /// Convert decimeters to inches.
    #[inline(always)]
    pub const fn dm_inch(x:f32)->f32{
        dm_in(x)
    }

    /// Convert decimeters to inches.
    #[inline(always)]
    pub const fn dm_inches(x:f32)->f32{
        dm_in(x)
    }

    /// Convert decimeters to inches.
    #[inline(always)]
    pub const fn decimeter_in(x:f32)->f32{
        dm_in(x)
    }

    /// Convert decimeters to inches.
    #[inline(always)]
    pub const fn decimeter_inch(x:f32)->f32{
        dm_in(x)
    }

    /// Convert decimeters to inches.
    #[inline(always)]
    pub const fn decimeter_inches(x:f32)->f32{
        dm_in(x)
    }

    /// Convert decimeters to inches.
    #[inline(always)]
    pub const fn decimeters_in(x:f32)->f32{
        dm_in(x)
    }

    /// Convert decimeters to inches.
    #[inline(always)]
    pub const fn decimeters_inch(x:f32)->f32{
        dm_in(x)
    }

    /// Convert decimeters to inches.
    #[inline(always)]
    pub const fn decimeters_inches(x:f32)->f32{
        dm_in(x)
    }

    /// Convert decimeters to feet.
    #[inline(always)]
    pub const fn dm_foot(x:f32)->f32{
        dm_ft(x)
    }

    /// Convert decimeters to feet.
    #[inline(always)]
    pub const fn dm_feet(x:f32)->f32{
        dm_ft(x)
    }

    /// Convert decimeters to feet.
    #[inline(always)]
    pub const fn decimeter_ft(x:f32)->f32{
        dm_ft(x)
    }

    /// Convert decimeters to feet.
    #[inline(always)]
    pub const fn decimeter_foot(x:f32)->f32{
        dm_ft(x)
    }

    /// Convert decimeters to feet.
    #[inline(always)]
    pub const fn decimeter_feet(x:f32)->f32{
        dm_ft(x)
    }

    /// Convert decimeters to feet.
    #[inline(always)]
    pub const fn decimeters_ft(x:f32)->f32{
        dm_ft(x)
    }

    /// Convert decimeters to feet.
    #[inline(always)]
    pub const fn decimeters_foot(x:f32)->f32{
        dm_ft(x)
    }

    /// Convert decimeters to feet.
    #[inline(always)]
    pub const fn decimeters_feet(x:f32)->f32{
        dm_ft(x)
    }

    /// Convert decimeters to yards.
    #[inline(always)]
    pub const fn dm_yard(x:f32)->f32{
        dm_yd(x)
    }

    /// Convert decimeters to yards.
    #[inline(always)]
    pub const fn dm_yards(x:f32)->f32{
        dm_yd(x)
    }

    /// Convert decimeters to yards.
    #[inline(always)]
    pub const fn decimeter_yd(x:f32)->f32{
        dm_yd(x)
    }

    /// Convert decimeters to yards.
    #[inline(always)]
    pub const fn decimeter_yard(x:f32)->f32{
        dm_yd(x)
    }

    /// Convert decimeters to yards.
    #[inline(always)]
    pub const fn decimeter_yards(x:f32)->f32{
        dm_yd(x)
    }

    /// Convert decimeters to yards.
    #[inline(always)]
    pub const fn decimeters_yd(x:f32)->f32{
        dm_yd(x)
    }

    /// Convert decimeters to yards.
    #[inline(always)]
    pub const fn decimeters_yard(x:f32)->f32{
        dm_yd(x)
    }

    /// Convert decimeters to yards.
    #[inline(always)]
    pub const fn decimeters_yards(x:f32)->f32{
        dm_yd(x)
    }

    /// Convert decimeters to miles.
    #[inline(always)]
    pub const fn dm_mile(x:f32)->f32{
        dm_mi(x)
    }

    /// Convert decimeters to miles.
    #[inline(always)]
    pub const fn dm_miles(x:f32)->f32{
        dm_mi(x)
    }

    /// Convert decimeters to miles.
    #[inline(always)]
    pub const fn decimeter_mi(x:f32)->f32{
        dm_mi(x)
    }

    /// Convert decimeters to miles.
    #[inline(always)]
    pub const fn decimeter_mile(x:f32)->f32{
        dm_mi(x)
    }

    /// Convert decimeters to miles.
    #[inline(always)]
    pub const fn decimeter_miles(x:f32)->f32{
        dm_mi(x)
    }

    /// Convert decimeters to miles.
    #[inline(always)]
    pub const fn decimeters_mi(x:f32)->f32{
        dm_mi(x)
    }

    /// Convert decimeters to miles.
    #[inline(always)]
    pub const fn decimeters_mile(x:f32)->f32{
        dm_mi(x)
    }

    /// Convert decimeters to miles.
    #[inline(always)]
    pub const fn decimeters_miles(x:f32)->f32{
        dm_mi(x)
    }

    /// Convert decimeters to nautical miles.
    #[inline(always)]
    pub const fn dm_nautical(x:f32)->f32{
        dm_nmi(x)
    }

    /// Convert decimeters to nautical miles.
    #[inline(always)]
    pub const fn dm_nauticalmile(x:f32)->f32{
        dm_nmi(x)
    }

    /// Convert decimeters to nautical miles.
    #[inline(always)]
    pub const fn dm_nauticalmiles(x:f32)->f32{
        dm_nmi(x)
    }

    /// Convert decimeters to nautical miles.
    #[inline(always)]
    pub const fn decimeter_nmi(x:f32)->f32{
        dm_nmi(x)
    }

    /// Convert decimeters to nautical miles.
    #[inline(always)]
    pub const fn decimeter_nautical(x:f32)->f32{
        dm_nmi(x)
    }

    /// Convert decimeters to nautical miles.
    #[inline(always)]
    pub const fn decimeter_nauticalmile(x:f32)->f32{
        dm_nmi(x)
    }

    /// Convert decimeters to nautical miles.
    #[inline(always)]
    pub const fn decimeter_nauticalmiles(x:f32)->f32{
        dm_nmi(x)
    }

    /// Convert decimeters to nautical miles.
    #[inline(always)]
    pub const fn decimeters_nmi(x:f32)->f32{
        dm_nmi(x)
    }

    /// Convert decimeters to nautical miles.
    #[inline(always)]
    pub const fn decimeters_nautical(x:f32)->f32{
        dm_nmi(x)
    }

    /// Convert decimeters to nautical miles.
    #[inline(always)]
    pub const fn decimeters_nauticalmile(x:f32)->f32{
        dm_nmi(x)
    }

    /// Convert decimeters to nautical miles.
    #[inline(always)]
    pub const fn decimeters_nauticalmiles(x:f32)->f32{
        dm_nmi(x)
    }

    /// Convert meters to millimeters.
    #[inline(always)]
    pub const fn m_millimeter(x:f32)->f32{
        m_mm(x)
    }

    /// Convert meters to millimeters.
    #[inline(always)]
    pub const fn m_millimeters(x:f32)->f32{
        m_mm(x)
    }

    /// Convert meters to millimeters.
    #[inline(always)]
    pub const fn meter_mm(x:f32)->f32{
        m_mm(x)
    }

    /// Convert meters to millimeters.
    #[inline(always)]
    pub const fn meter_millimeter(x:f32)->f32{
        m_mm(x)
    }

    /// Convert meters to millimeters.
    #[inline(always)]
    pub const fn meter_millimeters(x:f32)->f32{
        m_mm(x)
    }

    /// Convert meters to millimeters.
    #[inline(always)]
    pub const fn meters_mm(x:f32)->f32{
        m_mm(x)
    }

    /// Convert meters to millimeters.
    #[inline(always)]
    pub const fn meters_millimeter(x:f32)->f32{
        m_mm(x)
    }

    /// Convert meters to millimeters.
    #[inline(always)]
    pub const fn meters_millimeters(x:f32)->f32{
        m_mm(x)
    }

    /// Convert meters to centimeters.
    #[inline(always)]
    pub const fn m_centimeter(x:f32)->f32{
        m_cm(x)
    }

    /// Convert meters to centimeters.
    #[inline(always)]
    pub const fn m_centimeters(x:f32)->f32{
        m_cm(x)
    }

    /// Convert meters to centimeters.
    #[inline(always)]
    pub const fn meter_cm(x:f32)->f32{
        m_cm(x)
    }

    /// Convert meters to centimeters.
    #[inline(always)]
    pub const fn meter_centimeter(x:f32)->f32{
        m_cm(x)
    }

    /// Convert meters to centimeters.
    #[inline(always)]
    pub const fn meter_centimeters(x:f32)->f32{
        m_cm(x)
    }

    /// Convert meters to centimeters.
    #[inline(always)]
    pub const fn meters_cm(x:f32)->f32{
        m_cm(x)
    }

    /// Convert meters to centimeters.
    #[inline(always)]
    pub const fn meters_centimeter(x:f32)->f32{
        m_cm(x)
    }

    /// Convert meters to centimeters.
    #[inline(always)]
    pub const fn meters_centimeters(x:f32)->f32{
        m_cm(x)
    }

    /// Convert meters to decimeters.
    #[inline(always)]
    pub const fn m_decimeter(x:f32)->f32{
        m_dm(x)
    }

    /// Convert meters to decimeters.
    #[inline(always)]
    pub const fn m_decimeters(x:f32)->f32{
        m_dm(x)
    }

    /// Convert meters to decimeters.
    #[inline(always)]
    pub const fn meter_dm(x:f32)->f32{
        m_dm(x)
    }

    /// Convert meters to decimeters.
    #[inline(always)]
    pub const fn meter_decimeter(x:f32)->f32{
        m_dm(x)
    }

    /// Convert meters to decimeters.
    #[inline(always)]
    pub const fn meter_decimeters(x:f32)->f32{
        m_dm(x)
    }

    /// Convert meters to decimeters.
    #[inline(always)]
    pub const fn meters_dm(x:f32)->f32{
        m_dm(x)
    }

    /// Convert meters to decimeters.
    #[inline(always)]
    pub const fn meters_decimeter(x:f32)->f32{
        m_dm(x)
    }

    /// Convert meters to decimeters.
    #[inline(always)]
    pub const fn meters_decimeters(x:f32)->f32{
        m_dm(x)
    }

    /// Convert meters to kilometers.
    #[inline(always)]
    pub const fn m_kilometer(x:f32)->f32{
        m_km(x)
    }

    /// Convert meters to kilometers.
    #[inline(always)]
    pub const fn m_kilometers(x:f32)->f32{
        m_km(x)
    }

    /// Convert meters to kilometers.
    #[inline(always)]
    pub const fn meter_km(x:f32)->f32{
        m_km(x)
    }

    /// Convert meters to kilometers.
    #[inline(always)]
    pub const fn meter_kilometer(x:f32)->f32{
        m_km(x)
    }

    /// Convert meters to kilometers.
    #[inline(always)]
    pub const fn meter_kilometers(x:f32)->f32{
        m_km(x)
    }

    /// Convert meters to kilometers.
    #[inline(always)]
    pub const fn meters_km(x:f32)->f32{
        m_km(x)
    }

    /// Convert meters to kilometers.
    #[inline(always)]
    pub const fn meters_kilometer(x:f32)->f32{
        m_km(x)
    }

    /// Convert meters to kilometers.
    #[inline(always)]
    pub const fn meters_kilometers(x:f32)->f32{
        m_km(x)
    }

    /// Convert meters to inches.
    #[inline(always)]
    pub const fn m_inch(x:f32)->f32{
        m_in(x)
    }

    /// Convert meters to inches.
    #[inline(always)]
    pub const fn m_inches(x:f32)->f32{
        m_in(x)
    }

    /// Convert meters to inches.
    #[inline(always)]
    pub const fn meter_in(x:f32)->f32{
        m_in(x)
    }

    /// Convert meters to inches.
    #[inline(always)]
    pub const fn meter_inch(x:f32)->f32{
        m_in(x)
    }

    /// Convert meters to inches.
    #[inline(always)]
    pub const fn meter_inches(x:f32)->f32{
        m_in(x)
    }

    /// Convert meters to inches.
    #[inline(always)]
    pub const fn meters_in(x:f32)->f32{
        m_in(x)
    }

    /// Convert meters to inches.
    #[inline(always)]
    pub const fn meters_inch(x:f32)->f32{
        m_in(x)
    }

    /// Convert meters to inches.
    #[inline(always)]
    pub const fn meters_inches(x:f32)->f32{
        m_in(x)
    }

    /// Convert meters to feet.
    #[inline(always)]
    pub const fn m_foot(x:f32)->f32{
        m_ft(x)
    }

    /// Convert meters to feet.
    #[inline(always)]
    pub const fn m_feet(x:f32)->f32{
        m_ft(x)
    }

    /// Convert meters to feet.
    #[inline(always)]
    pub const fn meter_ft(x:f32)->f32{
        m_ft(x)
    }

    /// Convert meters to feet.
    #[inline(always)]
    pub const fn meter_foot(x:f32)->f32{
        m_ft(x)
    }

    /// Convert meters to feet.
    #[inline(always)]
    pub const fn meter_feet(x:f32)->f32{
        m_ft(x)
    }

    /// Convert meters to feet.
    #[inline(always)]
    pub const fn meters_ft(x:f32)->f32{
        m_ft(x)
    }

    /// Convert meters to feet.
    #[inline(always)]
    pub const fn meters_foot(x:f32)->f32{
        m_ft(x)
    }

    /// Convert meters to feet.
    #[inline(always)]
    pub const fn meters_feet(x:f32)->f32{
        m_ft(x)
    }

    /// Convert meters to yards.
    #[inline(always)]
    pub const fn m_yard(x:f32)->f32{
        m_yd(x)
    }

    /// Convert meters to yards.
    #[inline(always)]
    pub const fn m_yards(x:f32)->f32{
        m_yd(x)
    }

    /// Convert meters to yards.
    #[inline(always)]
    pub const fn meter_yd(x:f32)->f32{
        m_yd(x)
    }

    /// Convert meters to yards.
    #[inline(always)]
    pub const fn meter_yard(x:f32)->f32{
        m_yd(x)
    }

    /// Convert meters to yards.
    #[inline(always)]
    pub const fn meter_yards(x:f32)->f32{
        m_yd(x)
    }

    /// Convert meters to yards.
    #[inline(always)]
    pub const fn meters_yd(x:f32)->f32{
        m_yd(x)
    }

    /// Convert meters to yards.
    #[inline(always)]
    pub const fn meters_yard(x:f32)->f32{
        m_yd(x)
    }

    /// Convert meters to yards.
    #[inline(always)]
    pub const fn meters_yards(x:f32)->f32{
        m_yd(x)
    }

    /// Convert meters to miles.
    #[inline(always)]
    pub const fn m_mile(x:f32)->f32{
        m_mi(x)
    }

    /// Convert meters to miles.
    #[inline(always)]
    pub const fn m_miles(x:f32)->f32{
        m_mi(x)
    }

    /// Convert meters to miles.
    #[inline(always)]
    pub const fn meter_mi(x:f32)->f32{
        m_mi(x)
    }

    /// Convert meters to miles.
    #[inline(always)]
    pub const fn meter_mile(x:f32)->f32{
        m_mi(x)
    }

    /// Convert meters to miles.
    #[inline(always)]
    pub const fn meter_miles(x:f32)->f32{
        m_mi(x)
    }

    /// Convert meters to miles.
    #[inline(always)]
    pub const fn meters_mi(x:f32)->f32{
        m_mi(x)
    }

    /// Convert meters to miles.
    #[inline(always)]
    pub const fn meters_mile(x:f32)->f32{
        m_mi(x)
    }

    /// Convert meters to miles.
    #[inline(always)]
    pub const fn meters_miles(x:f32)->f32{
        m_mi(x)
    }

    /// Convert meters to nautical miles.
    #[inline(always)]
    pub const fn m_nautical(x:f32)->f32{
        m_nmi(x)
    }

    /// Convert meters to nautical miles.
    #[inline(always)]
    pub const fn m_nauticalmile(x:f32)->f32{
        m_nmi(x)
    }

    /// Convert meters to nautical miles.
    #[inline(always)]
    pub const fn m_nauticalmiles(x:f32)->f32{
        m_nmi(x)
    }

    /// Convert meters to nautical miles.
    #[inline(always)]
    pub const fn meter_nmi(x:f32)->f32{
        m_nmi(x)
    }

    /// Convert meters to nautical miles.
    #[inline(always)]
    pub const fn meter_nautical(x:f32)->f32{
        m_nmi(x)
    }

    /// Convert meters to nautical miles.
    #[inline(always)]
    pub const fn meter_nauticalmile(x:f32)->f32{
        m_nmi(x)
    }

    /// Convert meters to nautical miles.
    #[inline(always)]
    pub const fn meter_nauticalmiles(x:f32)->f32{
        m_nmi(x)
    }

    /// Convert meters to nautical miles.
    #[inline(always)]
    pub const fn meters_nmi(x:f32)->f32{
        m_nmi(x)
    }

    /// Convert meters to nautical miles.
    #[inline(always)]
    pub const fn meters_nautical(x:f32)->f32{
        m_nmi(x)
    }

    /// Convert meters to nautical miles.
    #[inline(always)]
    pub const fn meters_nauticalmile(x:f32)->f32{
        m_nmi(x)
    }

    /// Convert meters to nautical miles.
    #[inline(always)]
    pub const fn meters_nauticalmiles(x:f32)->f32{
        m_nmi(x)
    }

    /// Convert kilometers to millimeters.
    #[inline(always)]
    pub const fn km_millimeter(x:f32)->f32{
        km_mm(x)
    }

    /// Convert kilometers to millimeters.
    #[inline(always)]
    pub const fn km_millimeters(x:f32)->f32{
        km_mm(x)
    }

    /// Convert kilometers to millimeters.
    #[inline(always)]
    pub const fn kilometer_mm(x:f32)->f32{
        km_mm(x)
    }

    /// Convert kilometers to millimeters.
    #[inline(always)]
    pub const fn kilometer_millimeter(x:f32)->f32{
        km_mm(x)
    }

    /// Convert kilometers to millimeters.
    #[inline(always)]
    pub const fn kilometer_millimeters(x:f32)->f32{
        km_mm(x)
    }

    /// Convert kilometers to millimeters.
    #[inline(always)]
    pub const fn kilometers_mm(x:f32)->f32{
        km_mm(x)
    }

    /// Convert kilometers to millimeters.
    #[inline(always)]
    pub const fn kilometers_millimeter(x:f32)->f32{
        km_mm(x)
    }

    /// Convert kilometers to millimeters.
    #[inline(always)]
    pub const fn kilometers_millimeters(x:f32)->f32{
        km_mm(x)
    }

    /// Convert kilometers to centimeters.
    #[inline(always)]
    pub const fn km_centimeter(x:f32)->f32{
        km_cm(x)
    }

    /// Convert kilometers to centimeters.
    #[inline(always)]
    pub const fn km_centimeters(x:f32)->f32{
        km_cm(x)
    }

    /// Convert kilometers to centimeters.
    #[inline(always)]
    pub const fn kilometer_cm(x:f32)->f32{
        km_cm(x)
    }

    /// Convert kilometers to centimeters.
    #[inline(always)]
    pub const fn kilometer_centimeter(x:f32)->f32{
        km_cm(x)
    }

    /// Convert kilometers to centimeters.
    #[inline(always)]
    pub const fn kilometer_centimeters(x:f32)->f32{
        km_cm(x)
    }

    /// Convert kilometers to centimeters.
    #[inline(always)]
    pub const fn kilometers_cm(x:f32)->f32{
        km_cm(x)
    }

    /// Convert kilometers to centimeters.
    #[inline(always)]
    pub const fn kilometers_centimeter(x:f32)->f32{
        km_cm(x)
    }

    /// Convert kilometers to centimeters.
    #[inline(always)]
    pub const fn kilometers_centimeters(x:f32)->f32{
        km_cm(x)
    }

    /// Convert kilometers to decimeters.
    #[inline(always)]
    pub const fn km_decimeter(x:f32)->f32{
        km_dm(x)
    }

    /// Convert kilometers to decimeters.
    #[inline(always)]
    pub const fn km_decimeters(x:f32)->f32{
        km_dm(x)
    }

    /// Convert kilometers to decimeters.
    #[inline(always)]
    pub const fn kilometer_dm(x:f32)->f32{
        km_dm(x)
    }

    /// Convert kilometers to decimeters.
    #[inline(always)]
    pub const fn kilometer_decimeter(x:f32)->f32{
        km_dm(x)
    }

    /// Convert kilometers to decimeters.
    #[inline(always)]
    pub const fn kilometer_decimeters(x:f32)->f32{
        km_dm(x)
    }

    /// Convert kilometers to decimeters.
    #[inline(always)]
    pub const fn kilometers_dm(x:f32)->f32{
        km_dm(x)
    }

    /// Convert kilometers to decimeters.
    #[inline(always)]
    pub const fn kilometers_decimeter(x:f32)->f32{
        km_dm(x)
    }

    /// Convert kilometers to decimeters.
    #[inline(always)]
    pub const fn kilometers_decimeters(x:f32)->f32{
        km_dm(x)
    }

    /// Convert kilometers to meters.
    #[inline(always)]
    pub const fn km_meter(x:f32)->f32{
        km_m(x)
    }

    /// Convert kilometers to meters.
    #[inline(always)]
    pub const fn km_meters(x:f32)->f32{
        km_m(x)
    }

    /// Convert kilometers to meters.
    #[inline(always)]
    pub const fn kilometer_m(x:f32)->f32{
        km_m(x)
    }

    /// Convert kilometers to meters.
    #[inline(always)]
    pub const fn kilometer_meter(x:f32)->f32{
        km_m(x)
    }

    /// Convert kilometers to meters.
    #[inline(always)]
    pub const fn kilometer_meters(x:f32)->f32{
        km_m(x)
    }

    /// Convert kilometers to meters.
    #[inline(always)]
    pub const fn kilometers_m(x:f32)->f32{
        km_m(x)
    }

    /// Convert kilometers to meters.
    #[inline(always)]
    pub const fn kilometers_meter(x:f32)->f32{
        km_m(x)
    }

    /// Convert kilometers to meters.
    #[inline(always)]
    pub const fn kilometers_meters(x:f32)->f32{
        km_m(x)
    }

    /// Convert kilometers to inches.
    #[inline(always)]
    pub const fn km_inch(x:f32)->f32{
        km_in(x)
    }

    /// Convert kilometers to inches.
    #[inline(always)]
    pub const fn km_inches(x:f32)->f32{
        km_in(x)
    }

    /// Convert kilometers to inches.
    #[inline(always)]
    pub const fn kilometer_in(x:f32)->f32{
        km_in(x)
    }

    /// Convert kilometers to inches.
    #[inline(always)]
    pub const fn kilometer_inch(x:f32)->f32{
        km_in(x)
    }

    /// Convert kilometers to inches.
    #[inline(always)]
    pub const fn kilometer_inches(x:f32)->f32{
        km_in(x)
    }

    /// Convert kilometers to inches.
    #[inline(always)]
    pub const fn kilometers_in(x:f32)->f32{
        km_in(x)
    }

    /// Convert kilometers to inches.
    #[inline(always)]
    pub const fn kilometers_inch(x:f32)->f32{
        km_in(x)
    }

    /// Convert kilometers to inches.
    #[inline(always)]
    pub const fn kilometers_inches(x:f32)->f32{
        km_in(x)
    }

    /// Convert kilometers to feet.
    #[inline(always)]
    pub const fn km_foot(x:f32)->f32{
        km_ft(x)
    }

    /// Convert kilometers to feet.
    #[inline(always)]
    pub const fn km_feet(x:f32)->f32{
        km_ft(x)
    }

    /// Convert kilometers to feet.
    #[inline(always)]
    pub const fn kilometer_ft(x:f32)->f32{
        km_ft(x)
    }

    /// Convert kilometers to feet.
    #[inline(always)]
    pub const fn kilometer_foot(x:f32)->f32{
        km_ft(x)
    }

    /// Convert kilometers to feet.
    #[inline(always)]
    pub const fn kilometer_feet(x:f32)->f32{
        km_ft(x)
    }

    /// Convert kilometers to feet.
    #[inline(always)]
    pub const fn kilometers_ft(x:f32)->f32{
        km_ft(x)
    }

    /// Convert kilometers to feet.
    #[inline(always)]
    pub const fn kilometers_foot(x:f32)->f32{
        km_ft(x)
    }

    /// Convert kilometers to feet.
    #[inline(always)]
    pub const fn kilometers_feet(x:f32)->f32{
        km_ft(x)
    }

    /// Convert kilometers to yards.
    #[inline(always)]
    pub const fn km_yard(x:f32)->f32{
        km_yd(x)
    }

    /// Convert kilometers to yards.
    #[inline(always)]
    pub const fn km_yards(x:f32)->f32{
        km_yd(x)
    }

    /// Convert kilometers to yards.
    #[inline(always)]
    pub const fn kilometer_yd(x:f32)->f32{
        km_yd(x)
    }

    /// Convert kilometers to yards.
    #[inline(always)]
    pub const fn kilometer_yard(x:f32)->f32{
        km_yd(x)
    }

    /// Convert kilometers to yards.
    #[inline(always)]
    pub const fn kilometer_yards(x:f32)->f32{
        km_yd(x)
    }

    /// Convert kilometers to yards.
    #[inline(always)]
    pub const fn kilometers_yd(x:f32)->f32{
        km_yd(x)
    }

    /// Convert kilometers to yards.
    #[inline(always)]
    pub const fn kilometers_yard(x:f32)->f32{
        km_yd(x)
    }

    /// Convert kilometers to yards.
    #[inline(always)]
    pub const fn kilometers_yards(x:f32)->f32{
        km_yd(x)
    }

    /// Convert kilometers to miles.
    #[inline(always)]
    pub const fn km_mile(x:f32)->f32{
        km_mi(x)
    }

    /// Convert kilometers to miles.
    #[inline(always)]
    pub const fn km_miles(x:f32)->f32{
        km_mi(x)
    }

    /// Convert kilometers to miles.
    #[inline(always)]
    pub const fn kilometer_mi(x:f32)->f32{
        km_mi(x)
    }

    /// Convert kilometers to miles.
    #[inline(always)]
    pub const fn kilometer_mile(x:f32)->f32{
        km_mi(x)
    }

    /// Convert kilometers to miles.
    #[inline(always)]
    pub const fn kilometer_miles(x:f32)->f32{
        km_mi(x)
    }

    /// Convert kilometers to miles.
    #[inline(always)]
    pub const fn kilometers_mi(x:f32)->f32{
        km_mi(x)
    }

    /// Convert kilometers to miles.
    #[inline(always)]
    pub const fn kilometers_mile(x:f32)->f32{
        km_mi(x)
    }

    /// Convert kilometers to miles.
    #[inline(always)]
    pub const fn kilometers_miles(x:f32)->f32{
        km_mi(x)
    }

    /// Convert kilometers to nautical miles.
    #[inline(always)]
    pub const fn km_nautical(x:f32)->f32{
        km_nmi(x)
    }

    /// Convert kilometers to nautical miles.
    #[inline(always)]
    pub const fn km_nauticalmile(x:f32)->f32{
        km_nmi(x)
    }

    /// Convert kilometers to nautical miles.
    #[inline(always)]
    pub const fn km_nauticalmiles(x:f32)->f32{
        km_nmi(x)
    }

    /// Convert kilometers to nautical miles.
    #[inline(always)]
    pub const fn kilometer_nmi(x:f32)->f32{
        km_nmi(x)
    }

    /// Convert kilometers to nautical miles.
    #[inline(always)]
    pub const fn kilometer_nautical(x:f32)->f32{
        km_nmi(x)
    }

    /// Convert kilometers to nautical miles.
    #[inline(always)]
    pub const fn kilometer_nauticalmile(x:f32)->f32{
        km_nmi(x)
    }

    /// Convert kilometers to nautical miles.
    #[inline(always)]
    pub const fn kilometer_nauticalmiles(x:f32)->f32{
        km_nmi(x)
    }

    /// Convert kilometers to nautical miles.
    #[inline(always)]
    pub const fn kilometers_nmi(x:f32)->f32{
        km_nmi(x)
    }

    /// Convert kilometers to nautical miles.
    #[inline(always)]
    pub const fn kilometers_nautical(x:f32)->f32{
        km_nmi(x)
    }

    /// Convert kilometers to nautical miles.
    #[inline(always)]
    pub const fn kilometers_nauticalmile(x:f32)->f32{
        km_nmi(x)
    }

    /// Convert kilometers to nautical miles.
    #[inline(always)]
    pub const fn kilometers_nauticalmiles(x:f32)->f32{
        km_nmi(x)
    }

    /// Convert inches to millimeters.
    #[inline(always)]
    pub const fn in_millimeter(x:f32)->f32{
        in_mm(x)
    }

    /// Convert inches to millimeters.
    #[inline(always)]
    pub const fn in_millimeters(x:f32)->f32{
        in_mm(x)
    }

    /// Convert inches to millimeters.
    #[inline(always)]
    pub const fn inch_mm(x:f32)->f32{
        in_mm(x)
    }

    /// Convert inches to millimeters.
    #[inline(always)]
    pub const fn inch_millimeter(x:f32)->f32{
        in_mm(x)
    }

    /// Convert inches to millimeters.
    #[inline(always)]
    pub const fn inch_millimeters(x:f32)->f32{
        in_mm(x)
    }

    /// Convert inches to millimeters.
    #[inline(always)]
    pub const fn inches_mm(x:f32)->f32{
        in_mm(x)
    }

    /// Convert inches to millimeters.
    #[inline(always)]
    pub const fn inches_millimeter(x:f32)->f32{
        in_mm(x)
    }

    /// Convert inches to millimeters.
    #[inline(always)]
    pub const fn inches_millimeters(x:f32)->f32{
        in_mm(x)
    }

    /// Convert inches to centimeters.
    #[inline(always)]
    pub const fn in_centimeter(x:f32)->f32{
        in_cm(x)
    }

    /// Convert inches to centimeters.
    #[inline(always)]
    pub const fn in_centimeters(x:f32)->f32{
        in_cm(x)
    }

    /// Convert inches to centimeters.
    #[inline(always)]
    pub const fn inch_cm(x:f32)->f32{
        in_cm(x)
    }

    /// Convert inches to centimeters.
    #[inline(always)]
    pub const fn inch_centimeter(x:f32)->f32{
        in_cm(x)
    }

    /// Convert inches to centimeters.
    #[inline(always)]
    pub const fn inch_centimeters(x:f32)->f32{
        in_cm(x)
    }

    /// Convert inches to centimeters.
    #[inline(always)]
    pub const fn inches_cm(x:f32)->f32{
        in_cm(x)
    }

    /// Convert inches to centimeters.
    #[inline(always)]
    pub const fn inches_centimeter(x:f32)->f32{
        in_cm(x)
    }

    /// Convert inches to centimeters.
    #[inline(always)]
    pub const fn inches_centimeters(x:f32)->f32{
        in_cm(x)
    }

    /// Convert inches to decimeters.
    #[inline(always)]
    pub const fn in_decimeter(x:f32)->f32{
        in_dm(x)
    }

    /// Convert inches to decimeters.
    #[inline(always)]
    pub const fn in_decimeters(x:f32)->f32{
        in_dm(x)
    }

    /// Convert inches to decimeters.
    #[inline(always)]
    pub const fn inch_dm(x:f32)->f32{
        in_dm(x)
    }

    /// Convert inches to decimeters.
    #[inline(always)]
    pub const fn inch_decimeter(x:f32)->f32{
        in_dm(x)
    }

    /// Convert inches to decimeters.
    #[inline(always)]
    pub const fn inch_decimeters(x:f32)->f32{
        in_dm(x)
    }

    /// Convert inches to decimeters.
    #[inline(always)]
    pub const fn inches_dm(x:f32)->f32{
        in_dm(x)
    }

    /// Convert inches to decimeters.
    #[inline(always)]
    pub const fn inches_decimeter(x:f32)->f32{
        in_dm(x)
    }

    /// Convert inches to decimeters.
    #[inline(always)]
    pub const fn inches_decimeters(x:f32)->f32{
        in_dm(x)
    }

    /// Convert inches to meters.
    #[inline(always)]
    pub const fn in_meter(x:f32)->f32{
        in_m(x)
    }

    /// Convert inches to meters.
    #[inline(always)]
    pub const fn in_meters(x:f32)->f32{
        in_m(x)
    }

    /// Convert inches to meters.
    #[inline(always)]
    pub const fn inch_m(x:f32)->f32{
        in_m(x)
    }

    /// Convert inches to meters.
    #[inline(always)]
    pub const fn inch_meter(x:f32)->f32{
        in_m(x)
    }

    /// Convert inches to meters.
    #[inline(always)]
    pub const fn inch_meters(x:f32)->f32{
        in_m(x)
    }

    /// Convert inches to meters.
    #[inline(always)]
    pub const fn inches_m(x:f32)->f32{
        in_m(x)
    }

    /// Convert inches to meters.
    #[inline(always)]
    pub const fn inches_meter(x:f32)->f32{
        in_m(x)
    }

    /// Convert inches to meters.
    #[inline(always)]
    pub const fn inches_meters(x:f32)->f32{
        in_m(x)
    }

    /// Convert inches to kilometers.
    #[inline(always)]
    pub const fn in_kilometer(x:f32)->f32{
        in_km(x)
    }

    /// Convert inches to kilometers.
    #[inline(always)]
    pub const fn in_kilometers(x:f32)->f32{
        in_km(x)
    }

    /// Convert inches to kilometers.
    #[inline(always)]
    pub const fn inch_km(x:f32)->f32{
        in_km(x)
    }

    /// Convert inches to kilometers.
    #[inline(always)]
    pub const fn inch_kilometer(x:f32)->f32{
        in_km(x)
    }

    /// Convert inches to kilometers.
    #[inline(always)]
    pub const fn inch_kilometers(x:f32)->f32{
        in_km(x)
    }

    /// Convert inches to kilometers.
    #[inline(always)]
    pub const fn inches_km(x:f32)->f32{
        in_km(x)
    }

    /// Convert inches to kilometers.
    #[inline(always)]
    pub const fn inches_kilometer(x:f32)->f32{
        in_km(x)
    }

    /// Convert inches to kilometers.
    #[inline(always)]
    pub const fn inches_kilometers(x:f32)->f32{
        in_km(x)
    }

    /// Convert inches to feet.
    #[inline(always)]
    pub const fn in_foot(x:f32)->f32{
        in_ft(x)
    }

    /// Convert inches to feet.
    #[inline(always)]
    pub const fn in_feet(x:f32)->f32{
        in_ft(x)
    }

    /// Convert inches to feet.
    #[inline(always)]
    pub const fn inch_ft(x:f32)->f32{
        in_ft(x)
    }

    /// Convert inches to feet.
    #[inline(always)]
    pub const fn inch_foot(x:f32)->f32{
        in_ft(x)
    }

    /// Convert inches to feet.
    #[inline(always)]
    pub const fn inch_feet(x:f32)->f32{
        in_ft(x)
    }

    /// Convert inches to feet.
    #[inline(always)]
    pub const fn inches_ft(x:f32)->f32{
        in_ft(x)
    }

    /// Convert inches to feet.
    #[inline(always)]
    pub const fn inches_foot(x:f32)->f32{
        in_ft(x)
    }

    /// Convert inches to feet.
    #[inline(always)]
    pub const fn inches_feet(x:f32)->f32{
        in_ft(x)
    }

    /// Convert inches to yards.
    #[inline(always)]
    pub const fn in_yard(x:f32)->f32{
        in_yd(x)
    }

    /// Convert inches to yards.
    #[inline(always)]
    pub const fn in_yards(x:f32)->f32{
        in_yd(x)
    }

    /// Convert inches to yards.
    #[inline(always)]
    pub const fn inch_yd(x:f32)->f32{
        in_yd(x)
    }

    /// Convert inches to yards.
    #[inline(always)]
    pub const fn inch_yard(x:f32)->f32{
        in_yd(x)
    }

    /// Convert inches to yards.
    #[inline(always)]
    pub const fn inch_yards(x:f32)->f32{
        in_yd(x)
    }

    /// Convert inches to yards.
    #[inline(always)]
    pub const fn inches_yd(x:f32)->f32{
        in_yd(x)
    }

    /// Convert inches to yards.
    #[inline(always)]
    pub const fn inches_yard(x:f32)->f32{
        in_yd(x)
    }

    /// Convert inches to yards.
    #[inline(always)]
    pub const fn inches_yards(x:f32)->f32{
        in_yd(x)
    }

    /// Convert inches to miles.
    #[inline(always)]
    pub const fn in_mile(x:f32)->f32{
        in_mi(x)
    }

    /// Convert inches to miles.
    #[inline(always)]
    pub const fn in_miles(x:f32)->f32{
        in_mi(x)
    }

    /// Convert inches to miles.
    #[inline(always)]
    pub const fn inch_mi(x:f32)->f32{
        in_mi(x)
    }

    /// Convert inches to miles.
    #[inline(always)]
    pub const fn inch_mile(x:f32)->f32{
        in_mi(x)
    }

    /// Convert inches to miles.
    #[inline(always)]
    pub const fn inch_miles(x:f32)->f32{
        in_mi(x)
    }

    /// Convert inches to miles.
    #[inline(always)]
    pub const fn inches_mi(x:f32)->f32{
        in_mi(x)
    }

    /// Convert inches to miles.
    #[inline(always)]
    pub const fn inches_mile(x:f32)->f32{
        in_mi(x)
    }

    /// Convert inches to miles.
    #[inline(always)]
    pub const fn inches_miles(x:f32)->f32{
        in_mi(x)
    }

    /// Convert inches to nautical miles.
    #[inline(always)]
    pub const fn in_nautical(x:f32)->f32{
        in_nmi(x)
    }

    /// Convert inches to nautical miles.
    #[inline(always)]
    pub const fn in_nauticalmile(x:f32)->f32{
        in_nmi(x)
    }

    /// Convert inches to nautical miles.
    #[inline(always)]
    pub const fn in_nauticalmiles(x:f32)->f32{
        in_nmi(x)
    }

    /// Convert inches to nautical miles.
    #[inline(always)]
    pub const fn inch_nmi(x:f32)->f32{
        in_nmi(x)
    }

    /// Convert inches to nautical miles.
    #[inline(always)]
    pub const fn inch_nautical(x:f32)->f32{
        in_nmi(x)
    }

    /// Convert inches to nautical miles.
    #[inline(always)]
    pub const fn inch_nauticalmile(x:f32)->f32{
        in_nmi(x)
    }

    /// Convert inches to nautical miles.
    #[inline(always)]
    pub const fn inch_nauticalmiles(x:f32)->f32{
        in_nmi(x)
    }

    /// Convert inches to nautical miles.
    #[inline(always)]
    pub const fn inches_nmi(x:f32)->f32{
        in_nmi(x)
    }

    /// Convert inches to nautical miles.
    #[inline(always)]
    pub const fn inches_nautical(x:f32)->f32{
        in_nmi(x)
    }

    /// Convert inches to nautical miles.
    #[inline(always)]
    pub const fn inches_nauticalmile(x:f32)->f32{
        in_nmi(x)
    }

    /// Convert inches to nautical miles.
    #[inline(always)]
    pub const fn inches_nauticalmiles(x:f32)->f32{
        in_nmi(x)
    }

    /// Convert feet to millimeters.
    #[inline(always)]
    pub const fn ft_millimeter(x:f32)->f32{
        ft_mm(x)
    }

    /// Convert feet to millimeters.
    #[inline(always)]
    pub const fn ft_millimeters(x:f32)->f32{
        ft_mm(x)
    }

    /// Convert feet to millimeters.
    #[inline(always)]
    pub const fn foot_mm(x:f32)->f32{
        ft_mm(x)
    }

    /// Convert feet to millimeters.
    #[inline(always)]
    pub const fn foot_millimeter(x:f32)->f32{
        ft_mm(x)
    }

    /// Convert feet to millimeters.
    #[inline(always)]
    pub const fn foot_millimeters(x:f32)->f32{
        ft_mm(x)
    }

    /// Convert feet to millimeters.
    #[inline(always)]
    pub const fn feet_mm(x:f32)->f32{
        ft_mm(x)
    }

    /// Convert feet to millimeters.
    #[inline(always)]
    pub const fn feet_millimeter(x:f32)->f32{
        ft_mm(x)
    }

    /// Convert feet to millimeters.
    #[inline(always)]
    pub const fn feet_millimeters(x:f32)->f32{
        ft_mm(x)
    }

    /// Convert feet to centimeters.
    #[inline(always)]
    pub const fn ft_centimeter(x:f32)->f32{
        ft_cm(x)
    }

    /// Convert feet to centimeters.
    #[inline(always)]
    pub const fn ft_centimeters(x:f32)->f32{
        ft_cm(x)
    }

    /// Convert feet to centimeters.
    #[inline(always)]
    pub const fn foot_cm(x:f32)->f32{
        ft_cm(x)
    }

    /// Convert feet to centimeters.
    #[inline(always)]
    pub const fn foot_centimeter(x:f32)->f32{
        ft_cm(x)
    }

    /// Convert feet to centimeters.
    #[inline(always)]
    pub const fn foot_centimeters(x:f32)->f32{
        ft_cm(x)
    }

    /// Convert feet to centimeters.
    #[inline(always)]
    pub const fn feet_cm(x:f32)->f32{
        ft_cm(x)
    }

    /// Convert feet to centimeters.
    #[inline(always)]
    pub const fn feet_centimeter(x:f32)->f32{
        ft_cm(x)
    }

    /// Convert feet to centimeters.
    #[inline(always)]
    pub const fn feet_centimeters(x:f32)->f32{
        ft_cm(x)
    }

    /// Convert feet to decimeters.
    #[inline(always)]
    pub const fn ft_decimeter(x:f32)->f32{
        ft_dm(x)
    }

    /// Convert feet to decimeters.
    #[inline(always)]
    pub const fn ft_decimeters(x:f32)->f32{
        ft_dm(x)
    }

    /// Convert feet to decimeters.
    #[inline(always)]
    pub const fn foot_dm(x:f32)->f32{
        ft_dm(x)
    }

    /// Convert feet to decimeters.
    #[inline(always)]
    pub const fn foot_decimeter(x:f32)->f32{
        ft_dm(x)
    }

    /// Convert feet to decimeters.
    #[inline(always)]
    pub const fn foot_decimeters(x:f32)->f32{
        ft_dm(x)
    }

    /// Convert feet to decimeters.
    #[inline(always)]
    pub const fn feet_dm(x:f32)->f32{
        ft_dm(x)
    }

    /// Convert feet to decimeters.
    #[inline(always)]
    pub const fn feet_decimeter(x:f32)->f32{
        ft_dm(x)
    }

    /// Convert feet to decimeters.
    #[inline(always)]
    pub const fn feet_decimeters(x:f32)->f32{
        ft_dm(x)
    }

    /// Convert feet to meters.
    #[inline(always)]
    pub const fn ft_meter(x:f32)->f32{
        ft_m(x)
    }

    /// Convert feet to meters.
    #[inline(always)]
    pub const fn ft_meters(x:f32)->f32{
        ft_m(x)
    }

    /// Convert feet to meters.
    #[inline(always)]
    pub const fn foot_m(x:f32)->f32{
        ft_m(x)
    }

    /// Convert feet to meters.
    #[inline(always)]
    pub const fn foot_meter(x:f32)->f32{
        ft_m(x)
    }

    /// Convert feet to meters.
    #[inline(always)]
    pub const fn foot_meters(x:f32)->f32{
        ft_m(x)
    }

    /// Convert feet to meters.
    #[inline(always)]
    pub const fn feet_m(x:f32)->f32{
        ft_m(x)
    }

    /// Convert feet to meters.
    #[inline(always)]
    pub const fn feet_meter(x:f32)->f32{
        ft_m(x)
    }

    /// Convert feet to meters.
    #[inline(always)]
    pub const fn feet_meters(x:f32)->f32{
        ft_m(x)
    }

    /// Convert feet to kilometers.
    #[inline(always)]
    pub const fn ft_kilometer(x:f32)->f32{
        ft_km(x)
    }

    /// Convert feet to kilometers.
    #[inline(always)]
    pub const fn ft_kilometers(x:f32)->f32{
        ft_km(x)
    }

    /// Convert feet to kilometers.
    #[inline(always)]
    pub const fn foot_km(x:f32)->f32{
        ft_km(x)
    }

    /// Convert feet to kilometers.
    #[inline(always)]
    pub const fn foot_kilometer(x:f32)->f32{
        ft_km(x)
    }

    /// Convert feet to kilometers.
    #[inline(always)]
    pub const fn foot_kilometers(x:f32)->f32{
        ft_km(x)
    }

    /// Convert feet to kilometers.
    #[inline(always)]
    pub const fn feet_km(x:f32)->f32{
        ft_km(x)
    }

    /// Convert feet to kilometers.
    #[inline(always)]
    pub const fn feet_kilometer(x:f32)->f32{
        ft_km(x)
    }

    /// Convert feet to kilometers.
    #[inline(always)]
    pub const fn feet_kilometers(x:f32)->f32{
        ft_km(x)
    }

    /// Convert feet to inches.
    #[inline(always)]
    pub const fn ft_inch(x:f32)->f32{
        ft_in(x)
    }

    /// Convert feet to inches.
    #[inline(always)]
    pub const fn ft_inches(x:f32)->f32{
        ft_in(x)
    }

    /// Convert feet to inches.
    #[inline(always)]
    pub const fn foot_in(x:f32)->f32{
        ft_in(x)
    }

    /// Convert feet to inches.
    #[inline(always)]
    pub const fn foot_inch(x:f32)->f32{
        ft_in(x)
    }

    /// Convert feet to inches.
    #[inline(always)]
    pub const fn foot_inches(x:f32)->f32{
        ft_in(x)
    }

    /// Convert feet to inches.
    #[inline(always)]
    pub const fn feet_in(x:f32)->f32{
        ft_in(x)
    }

    /// Convert feet to inches.
    #[inline(always)]
    pub const fn feet_inch(x:f32)->f32{
        ft_in(x)
    }

    /// Convert feet to inches.
    #[inline(always)]
    pub const fn feet_inches(x:f32)->f32{
        ft_in(x)
    }

    /// Convert feet to yards.
    #[inline(always)]
    pub const fn ft_yard(x:f32)->f32{
        ft_yd(x)
    }

    /// Convert feet to yards.
    #[inline(always)]
    pub const fn ft_yards(x:f32)->f32{
        ft_yd(x)
    }

    /// Convert feet to yards.
    #[inline(always)]
    pub const fn foot_yd(x:f32)->f32{
        ft_yd(x)
    }

    /// Convert feet to yards.
    #[inline(always)]
    pub const fn foot_yard(x:f32)->f32{
        ft_yd(x)
    }

    /// Convert feet to yards.
    #[inline(always)]
    pub const fn foot_yards(x:f32)->f32{
        ft_yd(x)
    }

    /// Convert feet to yards.
    #[inline(always)]
    pub const fn feet_yd(x:f32)->f32{
        ft_yd(x)
    }

    /// Convert feet to yards.
    #[inline(always)]
    pub const fn feet_yard(x:f32)->f32{
        ft_yd(x)
    }

    /// Convert feet to yards.
    #[inline(always)]
    pub const fn feet_yards(x:f32)->f32{
        ft_yd(x)
    }

    /// Convert feet to miles.
    #[inline(always)]
    pub const fn ft_mile(x:f32)->f32{
        ft_mi(x)
    }

    /// Convert feet to miles.
    #[inline(always)]
    pub const fn ft_miles(x:f32)->f32{
        ft_mi(x)
    }

    /// Convert feet to miles.
    #[inline(always)]
    pub const fn foot_mi(x:f32)->f32{
        ft_mi(x)
    }

    /// Convert feet to miles.
    #[inline(always)]
    pub const fn foot_mile(x:f32)->f32{
        ft_mi(x)
    }

    /// Convert feet to miles.
    #[inline(always)]
    pub const fn foot_miles(x:f32)->f32{
        ft_mi(x)
    }

    /// Convert feet to miles.
    #[inline(always)]
    pub const fn feet_mi(x:f32)->f32{
        ft_mi(x)
    }

    /// Convert feet to miles.
    #[inline(always)]
    pub const fn feet_mile(x:f32)->f32{
        ft_mi(x)
    }

    /// Convert feet to miles.
    #[inline(always)]
    pub const fn feet_miles(x:f32)->f32{
        ft_mi(x)
    }

    /// Convert feet to nautical miles.
    #[inline(always)]
    pub const fn ft_nautical(x:f32)->f32{
        ft_nmi(x)
    }

    /// Convert feet to nautical miles.
    #[inline(always)]
    pub const fn ft_nauticalmile(x:f32)->f32{
        ft_nmi(x)
    }

    /// Convert feet to nautical miles.
    #[inline(always)]
    pub const fn ft_nauticalmiles(x:f32)->f32{
        ft_nmi(x)
    }

    /// Convert feet to nautical miles.
    #[inline(always)]
    pub const fn foot_nmi(x:f32)->f32{
        ft_nmi(x)
    }

    /// Convert feet to nautical miles.
    #[inline(always)]
    pub const fn foot_nautical(x:f32)->f32{
        ft_nmi(x)
    }

    /// Convert feet to nautical miles.
    #[inline(always)]
    pub const fn foot_nauticalmile(x:f32)->f32{
        ft_nmi(x)
    }

    /// Convert feet to nautical miles.
    #[inline(always)]
    pub const fn foot_nauticalmiles(x:f32)->f32{
        ft_nmi(x)
    }

    /// Convert feet to nautical miles.
    #[inline(always)]
    pub const fn feet_nmi(x:f32)->f32{
        ft_nmi(x)
    }

    /// Convert feet to nautical miles.
    #[inline(always)]
    pub const fn feet_nautical(x:f32)->f32{
        ft_nmi(x)
    }

    /// Convert feet to nautical miles.
    #[inline(always)]
    pub const fn feet_nauticalmile(x:f32)->f32{
        ft_nmi(x)
    }

    /// Convert feet to nautical miles.
    #[inline(always)]
    pub const fn feet_nauticalmiles(x:f32)->f32{
        ft_nmi(x)
    }

    /// Convert yards to millimeters.
    #[inline(always)]
    pub const fn yd_millimeter(x:f32)->f32{
        yd_mm(x)
    }

    /// Convert yards to millimeters.
    #[inline(always)]
    pub const fn yd_millimeters(x:f32)->f32{
        yd_mm(x)
    }

    /// Convert yards to millimeters.
    #[inline(always)]
    pub const fn yard_mm(x:f32)->f32{
        yd_mm(x)
    }

    /// Convert yards to millimeters.
    #[inline(always)]
    pub const fn yard_millimeter(x:f32)->f32{
        yd_mm(x)
    }

    /// Convert yards to millimeters.
    #[inline(always)]
    pub const fn yard_millimeters(x:f32)->f32{
        yd_mm(x)
    }

    /// Convert yards to millimeters.
    #[inline(always)]
    pub const fn yards_mm(x:f32)->f32{
        yd_mm(x)
    }

    /// Convert yards to millimeters.
    #[inline(always)]
    pub const fn yards_millimeter(x:f32)->f32{
        yd_mm(x)
    }

    /// Convert yards to millimeters.
    #[inline(always)]
    pub const fn yards_millimeters(x:f32)->f32{
        yd_mm(x)
    }

    /// Convert yards to centimeters.
    #[inline(always)]
    pub const fn yd_centimeter(x:f32)->f32{
        yd_cm(x)
    }

    /// Convert yards to centimeters.
    #[inline(always)]
    pub const fn yd_centimeters(x:f32)->f32{
        yd_cm(x)
    }

    /// Convert yards to centimeters.
    #[inline(always)]
    pub const fn yard_cm(x:f32)->f32{
        yd_cm(x)
    }

    /// Convert yards to centimeters.
    #[inline(always)]
    pub const fn yard_centimeter(x:f32)->f32{
        yd_cm(x)
    }

    /// Convert yards to centimeters.
    #[inline(always)]
    pub const fn yard_centimeters(x:f32)->f32{
        yd_cm(x)
    }

    /// Convert yards to centimeters.
    #[inline(always)]
    pub const fn yards_cm(x:f32)->f32{
        yd_cm(x)
    }

    /// Convert yards to centimeters.
    #[inline(always)]
    pub const fn yards_centimeter(x:f32)->f32{
        yd_cm(x)
    }

    /// Convert yards to centimeters.
    #[inline(always)]
    pub const fn yards_centimeters(x:f32)->f32{
        yd_cm(x)
    }

    /// Convert yards to decimeters.
    #[inline(always)]
    pub const fn yd_decimeter(x:f32)->f32{
        yd_dm(x)
    }

    /// Convert yards to decimeters.
    #[inline(always)]
    pub const fn yd_decimeters(x:f32)->f32{
        yd_dm(x)
    }

    /// Convert yards to decimeters.
    #[inline(always)]
    pub const fn yard_dm(x:f32)->f32{
        yd_dm(x)
    }

    /// Convert yards to decimeters.
    #[inline(always)]
    pub const fn yard_decimeter(x:f32)->f32{
        yd_dm(x)
    }

    /// Convert yards to decimeters.
    #[inline(always)]
    pub const fn yard_decimeters(x:f32)->f32{
        yd_dm(x)
    }

    /// Convert yards to decimeters.
    #[inline(always)]
    pub const fn yards_dm(x:f32)->f32{
        yd_dm(x)
    }

    /// Convert yards to decimeters.
    #[inline(always)]
    pub const fn yards_decimeter(x:f32)->f32{
        yd_dm(x)
    }

    /// Convert yards to decimeters.
    #[inline(always)]
    pub const fn yards_decimeters(x:f32)->f32{
        yd_dm(x)
    }

    /// Convert yards to meters.
    #[inline(always)]
    pub const fn yd_meter(x:f32)->f32{
        yd_m(x)
    }

    /// Convert yards to meters.
    #[inline(always)]
    pub const fn yd_meters(x:f32)->f32{
        yd_m(x)
    }

    /// Convert yards to meters.
    #[inline(always)]
    pub const fn yard_m(x:f32)->f32{
        yd_m(x)
    }

    /// Convert yards to meters.
    #[inline(always)]
    pub const fn yard_meter(x:f32)->f32{
        yd_m(x)
    }

    /// Convert yards to meters.
    #[inline(always)]
    pub const fn yard_meters(x:f32)->f32{
        yd_m(x)
    }

    /// Convert yards to meters.
    #[inline(always)]
    pub const fn yards_m(x:f32)->f32{
        yd_m(x)
    }

    /// Convert yards to meters.
    #[inline(always)]
    pub const fn yards_meter(x:f32)->f32{
        yd_m(x)
    }

    /// Convert yards to meters.
    #[inline(always)]
    pub const fn yards_meters(x:f32)->f32{
        yd_m(x)
    }

    /// Convert yards to kilometers.
    #[inline(always)]
    pub const fn yd_kilometer(x:f32)->f32{
        yd_km(x)
    }

    /// Convert yards to kilometers.
    #[inline(always)]
    pub const fn yd_kilometers(x:f32)->f32{
        yd_km(x)
    }

    /// Convert yards to kilometers.
    #[inline(always)]
    pub const fn yard_km(x:f32)->f32{
        yd_km(x)
    }

    /// Convert yards to kilometers.
    #[inline(always)]
    pub const fn yard_kilometer(x:f32)->f32{
        yd_km(x)
    }

    /// Convert yards to kilometers.
    #[inline(always)]
    pub const fn yard_kilometers(x:f32)->f32{
        yd_km(x)
    }

    /// Convert yards to kilometers.
    #[inline(always)]
    pub const fn yards_km(x:f32)->f32{
        yd_km(x)
    }

    /// Convert yards to kilometers.
    #[inline(always)]
    pub const fn yards_kilometer(x:f32)->f32{
        yd_km(x)
    }

    /// Convert yards to kilometers.
    #[inline(always)]
    pub const fn yards_kilometers(x:f32)->f32{
        yd_km(x)
    }

    /// Convert yards to inches.
    #[inline(always)]
    pub const fn yd_inch(x:f32)->f32{
        yd_in(x)
    }

    /// Convert yards to inches.
    #[inline(always)]
    pub const fn yd_inches(x:f32)->f32{
        yd_in(x)
    }

    /// Convert yards to inches.
    #[inline(always)]
    pub const fn yard_in(x:f32)->f32{
        yd_in(x)
    }

    /// Convert yards to inches.
    #[inline(always)]
    pub const fn yard_inch(x:f32)->f32{
        yd_in(x)
    }

    /// Convert yards to inches.
    #[inline(always)]
    pub const fn yard_inches(x:f32)->f32{
        yd_in(x)
    }

    /// Convert yards to inches.
    #[inline(always)]
    pub const fn yards_in(x:f32)->f32{
        yd_in(x)
    }

    /// Convert yards to inches.
    #[inline(always)]
    pub const fn yards_inch(x:f32)->f32{
        yd_in(x)
    }

    /// Convert yards to inches.
    #[inline(always)]
    pub const fn yards_inches(x:f32)->f32{
        yd_in(x)
    }

    /// Convert yards to feet.
    #[inline(always)]
    pub const fn yd_foot(x:f32)->f32{
        yd_ft(x)
    }

    /// Convert yards to feet.
    #[inline(always)]
    pub const fn yd_feet(x:f32)->f32{
        yd_ft(x)
    }

    /// Convert yards to feet.
    #[inline(always)]
    pub const fn yard_ft(x:f32)->f32{
        yd_ft(x)
    }

    /// Convert yards to feet.
    #[inline(always)]
    pub const fn yard_foot(x:f32)->f32{
        yd_ft(x)
    }

    /// Convert yards to feet.
    #[inline(always)]
    pub const fn yard_feet(x:f32)->f32{
        yd_ft(x)
    }

    /// Convert yards to feet.
    #[inline(always)]
    pub const fn yards_ft(x:f32)->f32{
        yd_ft(x)
    }

    /// Convert yards to feet.
    #[inline(always)]
    pub const fn yards_foot(x:f32)->f32{
        yd_ft(x)
    }

    /// Convert yards to feet.
    #[inline(always)]
    pub const fn yards_feet(x:f32)->f32{
        yd_ft(x)
    }

    /// Convert yards to miles.
    #[inline(always)]
    pub const fn yd_mile(x:f32)->f32{
        yd_mi(x)
    }

    /// Convert yards to miles.
    #[inline(always)]
    pub const fn yd_miles(x:f32)->f32{
        yd_mi(x)
    }

    /// Convert yards to miles.
    #[inline(always)]
    pub const fn yard_mi(x:f32)->f32{
        yd_mi(x)
    }

    /// Convert yards to miles.
    #[inline(always)]
    pub const fn yard_mile(x:f32)->f32{
        yd_mi(x)
    }

    /// Convert yards to miles.
    #[inline(always)]
    pub const fn yard_miles(x:f32)->f32{
        yd_mi(x)
    }

    /// Convert yards to miles.
    #[inline(always)]
    pub const fn yards_mi(x:f32)->f32{
        yd_mi(x)
    }

    /// Convert yards to miles.
    #[inline(always)]
    pub const fn yards_mile(x:f32)->f32{
        yd_mi(x)
    }

    /// Convert yards to miles.
    #[inline(always)]
    pub const fn yards_miles(x:f32)->f32{
        yd_mi(x)
    }

    /// Convert yards to nautical miles.
    #[inline(always)]
    pub const fn yd_nautical(x:f32)->f32{
        yd_nmi(x)
    }

    /// Convert yards to nautical miles.
    #[inline(always)]
    pub const fn yd_nauticalmile(x:f32)->f32{
        yd_nmi(x)
    }

    /// Convert yards to nautical miles.
    #[inline(always)]
    pub const fn yd_nauticalmiles(x:f32)->f32{
        yd_nmi(x)
    }

    /// Convert yards to nautical miles.
    #[inline(always)]
    pub const fn yard_nmi(x:f32)->f32{
        yd_nmi(x)
    }

    /// Convert yards to nautical miles.
    #[inline(always)]
    pub const fn yard_nautical(x:f32)->f32{
        yd_nmi(x)
    }

    /// Convert yards to nautical miles.
    #[inline(always)]
    pub const fn yard_nauticalmile(x:f32)->f32{
        yd_nmi(x)
    }

    /// Convert yards to nautical miles.
    #[inline(always)]
    pub const fn yard_nauticalmiles(x:f32)->f32{
        yd_nmi(x)
    }

    /// Convert yards to nautical miles.
    #[inline(always)]
    pub const fn yards_nmi(x:f32)->f32{
        yd_nmi(x)
    }

    /// Convert yards to nautical miles.
    #[inline(always)]
    pub const fn yards_nautical(x:f32)->f32{
        yd_nmi(x)
    }

    /// Convert yards to nautical miles.
    #[inline(always)]
    pub const fn yards_nauticalmile(x:f32)->f32{
        yd_nmi(x)
    }

    /// Convert yards to nautical miles.
    #[inline(always)]
    pub const fn yards_nauticalmiles(x:f32)->f32{
        yd_nmi(x)
    }

    /// Convert miles to millimeters.
    #[inline(always)]
    pub const fn mi_millimeter(x:f32)->f32{
        mi_mm(x)
    }

    /// Convert miles to millimeters.
    #[inline(always)]
    pub const fn mi_millimeters(x:f32)->f32{
        mi_mm(x)
    }

    /// Convert miles to millimeters.
    #[inline(always)]
    pub const fn mile_mm(x:f32)->f32{
        mi_mm(x)
    }

    /// Convert miles to millimeters.
    #[inline(always)]
    pub const fn mile_millimeter(x:f32)->f32{
        mi_mm(x)
    }

    /// Convert miles to millimeters.
    #[inline(always)]
    pub const fn mile_millimeters(x:f32)->f32{
        mi_mm(x)
    }

    /// Convert miles to millimeters.
    #[inline(always)]
    pub const fn miles_mm(x:f32)->f32{
        mi_mm(x)
    }

    /// Convert miles to millimeters.
    #[inline(always)]
    pub const fn miles_millimeter(x:f32)->f32{
        mi_mm(x)
    }

    /// Convert miles to millimeters.
    #[inline(always)]
    pub const fn miles_millimeters(x:f32)->f32{
        mi_mm(x)
    }

    /// Convert miles to centimeters.
    #[inline(always)]
    pub const fn mi_centimeter(x:f32)->f32{
        mi_cm(x)
    }

    /// Convert miles to centimeters.
    #[inline(always)]
    pub const fn mi_centimeters(x:f32)->f32{
        mi_cm(x)
    }

    /// Convert miles to centimeters.
    #[inline(always)]
    pub const fn mile_cm(x:f32)->f32{
        mi_cm(x)
    }

    /// Convert miles to centimeters.
    #[inline(always)]
    pub const fn mile_centimeter(x:f32)->f32{
        mi_cm(x)
    }

    /// Convert miles to centimeters.
    #[inline(always)]
    pub const fn mile_centimeters(x:f32)->f32{
        mi_cm(x)
    }

    /// Convert miles to centimeters.
    #[inline(always)]
    pub const fn miles_cm(x:f32)->f32{
        mi_cm(x)
    }

    /// Convert miles to centimeters.
    #[inline(always)]
    pub const fn miles_centimeter(x:f32)->f32{
        mi_cm(x)
    }

    /// Convert miles to centimeters.
    #[inline(always)]
    pub const fn miles_centimeters(x:f32)->f32{
        mi_cm(x)
    }

    /// Convert miles to decimeters.
    #[inline(always)]
    pub const fn mi_decimeter(x:f32)->f32{
        mi_dm(x)
    }

    /// Convert miles to decimeters.
    #[inline(always)]
    pub const fn mi_decimeters(x:f32)->f32{
        mi_dm(x)
    }

    /// Convert miles to decimeters.
    #[inline(always)]
    pub const fn mile_dm(x:f32)->f32{
        mi_dm(x)
    }

    /// Convert miles to decimeters.
    #[inline(always)]
    pub const fn mile_decimeter(x:f32)->f32{
        mi_dm(x)
    }

    /// Convert miles to decimeters.
    #[inline(always)]
    pub const fn mile_decimeters(x:f32)->f32{
        mi_dm(x)
    }

    /// Convert miles to decimeters.
    #[inline(always)]
    pub const fn miles_dm(x:f32)->f32{
        mi_dm(x)
    }

    /// Convert miles to decimeters.
    #[inline(always)]
    pub const fn miles_decimeter(x:f32)->f32{
        mi_dm(x)
    }

    /// Convert miles to decimeters.
    #[inline(always)]
    pub const fn miles_decimeters(x:f32)->f32{
        mi_dm(x)
    }

    /// Convert miles to meters.
    #[inline(always)]
    pub const fn mi_meter(x:f32)->f32{
        mi_m(x)
    }

    /// Convert miles to meters.
    #[inline(always)]
    pub const fn mi_meters(x:f32)->f32{
        mi_m(x)
    }

    /// Convert miles to meters.
    #[inline(always)]
    pub const fn mile_m(x:f32)->f32{
        mi_m(x)
    }

    /// Convert miles to meters.
    #[inline(always)]
    pub const fn mile_meter(x:f32)->f32{
        mi_m(x)
    }

    /// Convert miles to meters.
    #[inline(always)]
    pub const fn mile_meters(x:f32)->f32{
        mi_m(x)
    }

    /// Convert miles to meters.
    #[inline(always)]
    pub const fn miles_m(x:f32)->f32{
        mi_m(x)
    }

    /// Convert miles to meters.
    #[inline(always)]
    pub const fn miles_meter(x:f32)->f32{
        mi_m(x)
    }

    /// Convert miles to meters.
    #[inline(always)]
    pub const fn miles_meters(x:f32)->f32{
        mi_m(x)
    }

    /// Convert miles to kilometers.
    #[inline(always)]
    pub const fn mi_kilometer(x:f32)->f32{
        mi_km(x)
    }

    /// Convert miles to kilometers.
    #[inline(always)]
    pub const fn mi_kilometers(x:f32)->f32{
        mi_km(x)
    }

    /// Convert miles to kilometers.
    #[inline(always)]
    pub const fn mile_km(x:f32)->f32{
        mi_km(x)
    }

    /// Convert miles to kilometers.
    #[inline(always)]
    pub const fn mile_kilometer(x:f32)->f32{
        mi_km(x)
    }

    /// Convert miles to kilometers.
    #[inline(always)]
    pub const fn mile_kilometers(x:f32)->f32{
        mi_km(x)
    }

    /// Convert miles to kilometers.
    #[inline(always)]
    pub const fn miles_km(x:f32)->f32{
        mi_km(x)
    }

    /// Convert miles to kilometers.
    #[inline(always)]
    pub const fn miles_kilometer(x:f32)->f32{
        mi_km(x)
    }

    /// Convert miles to kilometers.
    #[inline(always)]
    pub const fn miles_kilometers(x:f32)->f32{
        mi_km(x)
    }

    /// Convert miles to inches.
    #[inline(always)]
    pub const fn mi_inch(x:f32)->f32{
        mi_in(x)
    }

    /// Convert miles to inches.
    #[inline(always)]
    pub const fn mi_inches(x:f32)->f32{
        mi_in(x)
    }

    /// Convert miles to inches.
    #[inline(always)]
    pub const fn mile_in(x:f32)->f32{
        mi_in(x)
    }

    /// Convert miles to inches.
    #[inline(always)]
    pub const fn mile_inch(x:f32)->f32{
        mi_in(x)
    }

    /// Convert miles to inches.
    #[inline(always)]
    pub const fn mile_inches(x:f32)->f32{
        mi_in(x)
    }

    /// Convert miles to inches.
    #[inline(always)]
    pub const fn miles_in(x:f32)->f32{
        mi_in(x)
    }

    /// Convert miles to inches.
    #[inline(always)]
    pub const fn miles_inch(x:f32)->f32{
        mi_in(x)
    }

    /// Convert miles to inches.
    #[inline(always)]
    pub const fn miles_inches(x:f32)->f32{
        mi_in(x)
    }

    /// Convert miles to feet.
    #[inline(always)]
    pub const fn mi_foot(x:f32)->f32{
        mi_ft(x)
    }

    /// Convert miles to feet.
    #[inline(always)]
    pub const fn mi_feet(x:f32)->f32{
        mi_ft(x)
    }

    /// Convert miles to feet.
    #[inline(always)]
    pub const fn mile_ft(x:f32)->f32{
        mi_ft(x)
    }

    /// Convert miles to feet.
    #[inline(always)]
    pub const fn mile_foot(x:f32)->f32{
        mi_ft(x)
    }

    /// Convert miles to feet.
    #[inline(always)]
    pub const fn mile_feet(x:f32)->f32{
        mi_ft(x)
    }

    /// Convert miles to feet.
    #[inline(always)]
    pub const fn miles_ft(x:f32)->f32{
        mi_ft(x)
    }

    /// Convert miles to feet.
    #[inline(always)]
    pub const fn miles_foot(x:f32)->f32{
        mi_ft(x)
    }

    /// Convert miles to feet.
    #[inline(always)]
    pub const fn miles_feet(x:f32)->f32{
        mi_ft(x)
    }

    /// Convert miles to yards.
    #[inline(always)]
    pub const fn mi_yard(x:f32)->f32{
        mi_yd(x)
    }

    /// Convert miles to yards.
    #[inline(always)]
    pub const fn mi_yards(x:f32)->f32{
        mi_yd(x)
    }

    /// Convert miles to yards.
    #[inline(always)]
    pub const fn mile_yd(x:f32)->f32{
        mi_yd(x)
    }

    /// Convert miles to yards.
    #[inline(always)]
    pub const fn mile_yard(x:f32)->f32{
        mi_yd(x)
    }

    /// Convert miles to yards.
    #[inline(always)]
    pub const fn mile_yards(x:f32)->f32{
        mi_yd(x)
    }

    /// Convert miles to yards.
    #[inline(always)]
    pub const fn miles_yd(x:f32)->f32{
        mi_yd(x)
    }

    /// Convert miles to yards.
    #[inline(always)]
    pub const fn miles_yard(x:f32)->f32{
        mi_yd(x)
    }

    /// Convert miles to yards.
    #[inline(always)]
    pub const fn miles_yards(x:f32)->f32{
        mi_yd(x)
    }

    /// Convert miles to nautical miles.
    #[inline(always)]
    pub const fn mi_nautical(x:f32)->f32{
        mi_nmi(x)
    }

    /// Convert miles to nautical miles.
    #[inline(always)]
    pub const fn mi_nauticalmile(x:f32)->f32{
        mi_nmi(x)
    }

    /// Convert miles to nautical miles.
    #[inline(always)]
    pub const fn mi_nauticalmiles(x:f32)->f32{
        mi_nmi(x)
    }

    /// Convert miles to nautical miles.
    #[inline(always)]
    pub const fn mile_nmi(x:f32)->f32{
        mi_nmi(x)
    }

    /// Convert miles to nautical miles.
    #[inline(always)]
    pub const fn mile_nautical(x:f32)->f32{
        mi_nmi(x)
    }

    /// Convert miles to nautical miles.
    #[inline(always)]
    pub const fn mile_nauticalmile(x:f32)->f32{
        mi_nmi(x)
    }

    /// Convert miles to nautical miles.
    #[inline(always)]
    pub const fn mile_nauticalmiles(x:f32)->f32{
        mi_nmi(x)
    }

    /// Convert miles to nautical miles.
    #[inline(always)]
    pub const fn miles_nmi(x:f32)->f32{
        mi_nmi(x)
    }

    /// Convert miles to nautical miles.
    #[inline(always)]
    pub const fn miles_nautical(x:f32)->f32{
        mi_nmi(x)
    }

    /// Convert miles to nautical miles.
    #[inline(always)]
    pub const fn miles_nauticalmile(x:f32)->f32{
        mi_nmi(x)
    }

    /// Convert miles to nautical miles.
    #[inline(always)]
    pub const fn miles_nauticalmiles(x:f32)->f32{
        mi_nmi(x)
    }

    /// Convert nautical miles to millimeters.
    #[inline(always)]
    pub const fn nmi_millimeter(x:f32)->f32{
        nmi_mm(x)
    }

    /// Convert nautical miles to millimeters.
    #[inline(always)]
    pub const fn nmi_millimeters(x:f32)->f32{
        nmi_mm(x)
    }

    /// Convert nautical miles to millimeters.
    #[inline(always)]
    pub const fn nautical_mm(x:f32)->f32{
        nmi_mm(x)
    }

    /// Convert nautical miles to millimeters.
    #[inline(always)]
    pub const fn nautical_millimeter(x:f32)->f32{
        nmi_mm(x)
    }

    /// Convert nautical miles to millimeters.
    #[inline(always)]
    pub const fn nautical_millimeters(x:f32)->f32{
        nmi_mm(x)
    }

    /// Convert nautical miles to millimeters.
    #[inline(always)]
    pub const fn nauticalmile_mm(x:f32)->f32{
        nmi_mm(x)
    }

    /// Convert nautical miles to millimeters.
    #[inline(always)]
    pub const fn nauticalmile_millimeter(x:f32)->f32{
        nmi_mm(x)
    }

    /// Convert nautical miles to millimeters.
    #[inline(always)]
    pub const fn nauticalmile_millimeters(x:f32)->f32{
        nmi_mm(x)
    }

    /// Convert nautical miles to millimeters.
    #[inline(always)]
    pub const fn nauticalmiles_mm(x:f32)->f32{
        nmi_mm(x)
    }

    /// Convert nautical miles to millimeters.
    #[inline(always)]
    pub const fn nauticalmiles_millimeter(x:f32)->f32{
        nmi_mm(x)
    }

    /// Convert nautical miles to millimeters.
    #[inline(always)]
    pub const fn nauticalmiles_millimeters(x:f32)->f32{
        nmi_mm(x)
    }

    /// Convert nautical miles to centimeters.
    #[inline(always)]
    pub const fn nmi_centimeter(x:f32)->f32{
        nmi_cm(x)
    }

    /// Convert nautical miles to centimeters.
    #[inline(always)]
    pub const fn nmi_centimeters(x:f32)->f32{
        nmi_cm(x)
    }

    /// Convert nautical miles to centimeters.
    #[inline(always)]
    pub const fn nautical_cm(x:f32)->f32{
        nmi_cm(x)
    }

    /// Convert nautical miles to centimeters.
    #[inline(always)]
    pub const fn nautical_centimeter(x:f32)->f32{
        nmi_cm(x)
    }

    /// Convert nautical miles to centimeters.
    #[inline(always)]
    pub const fn nautical_centimeters(x:f32)->f32{
        nmi_cm(x)
    }

    /// Convert nautical miles to centimeters.
    #[inline(always)]
    pub const fn nauticalmile_cm(x:f32)->f32{
        nmi_cm(x)
    }

    /// Convert nautical miles to centimeters.
    #[inline(always)]
    pub const fn nauticalmile_centimeter(x:f32)->f32{
        nmi_cm(x)
    }

    /// Convert nautical miles to centimeters.
    #[inline(always)]
    pub const fn nauticalmile_centimeters(x:f32)->f32{
        nmi_cm(x)
    }

    /// Convert nautical miles to centimeters.
    #[inline(always)]
    pub const fn nauticalmiles_cm(x:f32)->f32{
        nmi_cm(x)
    }

    /// Convert nautical miles to centimeters.
    #[inline(always)]
    pub const fn nauticalmiles_centimeter(x:f32)->f32{
        nmi_cm(x)
    }

    /// Convert nautical miles to centimeters.
    #[inline(always)]
    pub const fn nauticalmiles_centimeters(x:f32)->f32{
        nmi_cm(x)
    }

    /// Convert nautical miles to decimeters.
    #[inline(always)]
    pub const fn nmi_decimeter(x:f32)->f32{
        nmi_dm(x)
    }

    /// Convert nautical miles to decimeters.
    #[inline(always)]
    pub const fn nmi_decimeters(x:f32)->f32{
        nmi_dm(x)
    }

    /// Convert nautical miles to decimeters.
    #[inline(always)]
    pub const fn nautical_dm(x:f32)->f32{
        nmi_dm(x)
    }

    /// Convert nautical miles to decimeters.
    #[inline(always)]
    pub const fn nautical_decimeter(x:f32)->f32{
        nmi_dm(x)
    }

    /// Convert nautical miles to decimeters.
    #[inline(always)]
    pub const fn nautical_decimeters(x:f32)->f32{
        nmi_dm(x)
    }

    /// Convert nautical miles to decimeters.
    #[inline(always)]
    pub const fn nauticalmile_dm(x:f32)->f32{
        nmi_dm(x)
    }

    /// Convert nautical miles to decimeters.
    #[inline(always)]
    pub const fn nauticalmile_decimeter(x:f32)->f32{
        nmi_dm(x)
    }

    /// Convert nautical miles to decimeters.
    #[inline(always)]
    pub const fn nauticalmile_decimeters(x:f32)->f32{
        nmi_dm(x)
    }

    /// Convert nautical miles to decimeters.
    #[inline(always)]
    pub const fn nauticalmiles_dm(x:f32)->f32{
        nmi_dm(x)
    }

    /// Convert nautical miles to decimeters.
    #[inline(always)]
    pub const fn nauticalmiles_decimeter(x:f32)->f32{
        nmi_dm(x)
    }

    /// Convert nautical miles to decimeters.
    #[inline(always)]
    pub const fn nauticalmiles_decimeters(x:f32)->f32{
        nmi_dm(x)
    }

    /// Convert nautical miles to meters.
    #[inline(always)]
    pub const fn nmi_meter(x:f32)->f32{
        nmi_m(x)
    }

    /// Convert nautical miles to meters.
    #[inline(always)]
    pub const fn nmi_meters(x:f32)->f32{
        nmi_m(x)
    }

    /// Convert nautical miles to meters.
    #[inline(always)]
    pub const fn nautical_m(x:f32)->f32{
        nmi_m(x)
    }

    /// Convert nautical miles to meters.
    #[inline(always)]
    pub const fn nautical_meter(x:f32)->f32{
        nmi_m(x)
    }

    /// Convert nautical miles to meters.
    #[inline(always)]
    pub const fn nautical_meters(x:f32)->f32{
        nmi_m(x)
    }

    /// Convert nautical miles to meters.
    #[inline(always)]
    pub const fn nauticalmile_m(x:f32)->f32{
        nmi_m(x)
    }

    /// Convert nautical miles to meters.
    #[inline(always)]
    pub const fn nauticalmile_meter(x:f32)->f32{
        nmi_m(x)
    }

    /// Convert nautical miles to meters.
    #[inline(always)]
    pub const fn nauticalmile_meters(x:f32)->f32{
        nmi_m(x)
    }

    /// Convert nautical miles to meters.
    #[inline(always)]
    pub const fn nauticalmiles_m(x:f32)->f32{
        nmi_m(x)
    }

    /// Convert nautical miles to meters.
    #[inline(always)]
    pub const fn nauticalmiles_meter(x:f32)->f32{
        nmi_m(x)
    }

    /// Convert nautical miles to meters.
    #[inline(always)]
    pub const fn nauticalmiles_meters(x:f32)->f32{
        nmi_m(x)
    }

    /// Convert nautical miles to kilometers.
    #[inline(always)]
    pub const fn nmi_kilometer(x:f32)->f32{
        nmi_km(x)
    }

    /// Convert nautical miles to kilometers.
    #[inline(always)]
    pub const fn nmi_kilometers(x:f32)->f32{
        nmi_km(x)
    }

    /// Convert nautical miles to kilometers.
    #[inline(always)]
    pub const fn nautical_km(x:f32)->f32{
        nmi_km(x)
    }

    /// Convert nautical miles to kilometers.
    #[inline(always)]
    pub const fn nautical_kilometer(x:f32)->f32{
        nmi_km(x)
    }

    /// Convert nautical miles to kilometers.
    #[inline(always)]
    pub const fn nautical_kilometers(x:f32)->f32{
        nmi_km(x)
    }

    /// Convert nautical miles to kilometers.
    #[inline(always)]
    pub const fn nauticalmile_km(x:f32)->f32{
        nmi_km(x)
    }

    /// Convert nautical miles to kilometers.
    #[inline(always)]
    pub const fn nauticalmile_kilometer(x:f32)->f32{
        nmi_km(x)
    }

    /// Convert nautical miles to kilometers.
    #[inline(always)]
    pub const fn nauticalmile_kilometers(x:f32)->f32{
        nmi_km(x)
    }

    /// Convert nautical miles to kilometers.
    #[inline(always)]
    pub const fn nauticalmiles_km(x:f32)->f32{
        nmi_km(x)
    }

    /// Convert nautical miles to kilometers.
    #[inline(always)]
    pub const fn nauticalmiles_kilometer(x:f32)->f32{
        nmi_km(x)
    }

    /// Convert nautical miles to kilometers.
    #[inline(always)]
    pub const fn nauticalmiles_kilometers(x:f32)->f32{
        nmi_km(x)
    }

    /// Convert nautical miles to inches.
    #[inline(always)]
    pub const fn nmi_inch(x:f32)->f32{
        nmi_in(x)
    }

    /// Convert nautical miles to inches.
    #[inline(always)]
    pub const fn nmi_inches(x:f32)->f32{
        nmi_in(x)
    }

    /// Convert nautical miles to inches.
    #[inline(always)]
    pub const fn nautical_in(x:f32)->f32{
        nmi_in(x)
    }

    /// Convert nautical miles to inches.
    #[inline(always)]
    pub const fn nautical_inch(x:f32)->f32{
        nmi_in(x)
    }

    /// Convert nautical miles to inches.
    #[inline(always)]
    pub const fn nautical_inches(x:f32)->f32{
        nmi_in(x)
    }

    /// Convert nautical miles to inches.
    #[inline(always)]
    pub const fn nauticalmile_in(x:f32)->f32{
        nmi_in(x)
    }

    /// Convert nautical miles to inches.
    #[inline(always)]
    pub const fn nauticalmile_inch(x:f32)->f32{
        nmi_in(x)
    }

    /// Convert nautical miles to inches.
    #[inline(always)]
    pub const fn nauticalmile_inches(x:f32)->f32{
        nmi_in(x)
    }

    /// Convert nautical miles to inches.
    #[inline(always)]
    pub const fn nauticalmiles_in(x:f32)->f32{
        nmi_in(x)
    }

    /// Convert nautical miles to inches.
    #[inline(always)]
    pub const fn nauticalmiles_inch(x:f32)->f32{
        nmi_in(x)
    }

    /// Convert nautical miles to inches.
    #[inline(always)]
    pub const fn nauticalmiles_inches(x:f32)->f32{
        nmi_in(x)
    }

    /// Convert nautical miles to feet.
    #[inline(always)]
    pub const fn nmi_foot(x:f32)->f32{
        nmi_ft(x)
    }

    /// Convert nautical miles to feet.
    #[inline(always)]
    pub const fn nmi_feet(x:f32)->f32{
        nmi_ft(x)
    }

    /// Convert nautical miles to feet.
    #[inline(always)]
    pub const fn nautical_ft(x:f32)->f32{
        nmi_ft(x)
    }

    /// Convert nautical miles to feet.
    #[inline(always)]
    pub const fn nautical_foot(x:f32)->f32{
        nmi_ft(x)
    }

    /// Convert nautical miles to feet.
    #[inline(always)]
    pub const fn nautical_feet(x:f32)->f32{
        nmi_ft(x)
    }

    /// Convert nautical miles to feet.
    #[inline(always)]
    pub const fn nauticalmile_ft(x:f32)->f32{
        nmi_ft(x)
    }

    /// Convert nautical miles to feet.
    #[inline(always)]
    pub const fn nauticalmile_foot(x:f32)->f32{
        nmi_ft(x)
    }

    /// Convert nautical miles to feet.
    #[inline(always)]
    pub const fn nauticalmile_feet(x:f32)->f32{
        nmi_ft(x)
    }

    /// Convert nautical miles to feet.
    #[inline(always)]
    pub const fn nauticalmiles_ft(x:f32)->f32{
        nmi_ft(x)
    }

    /// Convert nautical miles to feet.
    #[inline(always)]
    pub const fn nauticalmiles_foot(x:f32)->f32{
        nmi_ft(x)
    }

    /// Convert nautical miles to feet.
    #[inline(always)]
    pub const fn nauticalmiles_feet(x:f32)->f32{
        nmi_ft(x)
    }

    /// Convert nautical miles to yards.
    #[inline(always)]
    pub const fn nmi_yard(x:f32)->f32{
        nmi_yd(x)
    }

    /// Convert nautical miles to yards.
    #[inline(always)]
    pub const fn nmi_yards(x:f32)->f32{
        nmi_yd(x)
    }

    /// Convert nautical miles to yards.
    #[inline(always)]
    pub const fn nautical_yd(x:f32)->f32{
        nmi_yd(x)
    }

    /// Convert nautical miles to yards.
    #[inline(always)]
    pub const fn nautical_yard(x:f32)->f32{
        nmi_yd(x)
    }

    /// Convert nautical miles to yards.
    #[inline(always)]
    pub const fn nautical_yards(x:f32)->f32{
        nmi_yd(x)
    }

    /// Convert nautical miles to yards.
    #[inline(always)]
    pub const fn nauticalmile_yd(x:f32)->f32{
        nmi_yd(x)
    }

    /// Convert nautical miles to yards.
    #[inline(always)]
    pub const fn nauticalmile_yard(x:f32)->f32{
        nmi_yd(x)
    }

    /// Convert nautical miles to yards.
    #[inline(always)]
    pub const fn nauticalmile_yards(x:f32)->f32{
        nmi_yd(x)
    }

    /// Convert nautical miles to yards.
    #[inline(always)]
    pub const fn nauticalmiles_yd(x:f32)->f32{
        nmi_yd(x)
    }

    /// Convert nautical miles to yards.
    #[inline(always)]
    pub const fn nauticalmiles_yard(x:f32)->f32{
        nmi_yd(x)
    }

    /// Convert nautical miles to yards.
    #[inline(always)]
    pub const fn nauticalmiles_yards(x:f32)->f32{
        nmi_yd(x)
    }

    /// Convert nautical miles to miles.
    #[inline(always)]
    pub const fn nmi_mile(x:f32)->f32{
        nmi_mi(x)
    }

    /// Convert nautical miles to miles.
    #[inline(always)]
    pub const fn nmi_miles(x:f32)->f32{
        nmi_mi(x)
    }

    /// Convert nautical miles to miles.
    #[inline(always)]
    pub const fn nautical_mi(x:f32)->f32{
        nmi_mi(x)
    }

    /// Convert nautical miles to miles.
    #[inline(always)]
    pub const fn nautical_mile(x:f32)->f32{
        nmi_mi(x)
    }

    /// Convert nautical miles to miles.
    #[inline(always)]
    pub const fn nautical_miles(x:f32)->f32{
        nmi_mi(x)
    }

    /// Convert nautical miles to miles.
    #[inline(always)]
    pub const fn nauticalmile_mi(x:f32)->f32{
        nmi_mi(x)
    }

    /// Convert nautical miles to miles.
    #[inline(always)]
    pub const fn nauticalmile_mile(x:f32)->f32{
        nmi_mi(x)
    }

    /// Convert nautical miles to miles.
    #[inline(always)]
    pub const fn nauticalmile_miles(x:f32)->f32{
        nmi_mi(x)
    }

    /// Convert nautical miles to miles.
    #[inline(always)]
    pub const fn nauticalmiles_mi(x:f32)->f32{
        nmi_mi(x)
    }

    /// Convert nautical miles to miles.
    #[inline(always)]
    pub const fn nauticalmiles_mile(x:f32)->f32{
        nmi_mi(x)
    }

    /// Convert nautical miles to miles.
    #[inline(always)]
    pub const fn nauticalmiles_miles(x:f32)->f32{
        nmi_mi(x)
    }
}

pub mod weight{
    /// Convert milligrams to grams.
    #[inline]
    pub const fn mg_g(x:f32)->f32{
        x*0.001
    }

    /// Convert milligrams to kilograms.
    #[inline]
    pub const fn mg_kg(x:f32)->f32{
        x*0.000001
    }

    /// Convert milligrams to ounces.
    #[inline]
    pub const fn mg_oz(x:f32)->f32{
        x*0.00003527396195
    }

    /// Convert milligrams to pounds.
    #[inline]
    pub const fn mg_lb(x:f32)->f32{
        x*0.00000220462262
    }

    /// Convert milligrams to stones.
    #[inline]
    pub const fn mg_st(x:f32)->f32{
        x*0.00000015747304
    }

    /// Convert milligrams to tons.
    #[inline]
    pub const fn mg_tn(x:f32)->f32{
        x*0.00000000110231
    }

    /// Convert milligrams to tonnes.
    #[inline]
    pub const fn mg_t(x:f32)->f32{
        x*0.000000001
    }

    /// Convert grams to milligrams.
    #[inline]
    pub const fn g_mg(x:f32)->f32{
        x*1000.0
    }

    /// Convert grams to kilograms.
    #[inline]
    pub const fn g_kg(x:f32)->f32{
        x*0.001
    }

    /// Convert grams to ounces.
    #[inline]
    pub const fn g_oz(x:f32)->f32{
        x*0.03527396194958
    }

    /// Convert grams to pounds.
    #[inline]
    pub const fn g_lb(x:f32)->f32{
        x*0.00220462262185
    }

    /// Convert grams to stones.
    #[inline]
    pub const fn g_st(x:f32)->f32{
        x*0.00015747304442
    }

    /// Convert grams to tons.
    #[inline]
    pub const fn g_tn(x:f32)->f32{
        x*0.00000110231131
    }

    /// Convert grams to tonnes.
    #[inline]
    pub const fn g_t(x:f32)->f32{
        x*0.000001
    }

    /// Convert kilograms to milligrams.
    #[inline]
    pub const fn kg_mg(x:f32)->f32{
        x*1000000.0
    }

    /// Convert kilograms to grams.
    #[inline]
    pub const fn kg_g(x:f32)->f32{
        x*1000.0
    }

    /// Convert kilograms to ounces.
    #[inline]
    pub const fn kg_oz(x:f32)->f32{
        x*35.27396194958041
    }

    /// Convert kilograms to pounds.
    #[inline]
    pub const fn kg_lb(x:f32)->f32{
        x*2.20462262184878
    }

    /// Convert kilograms to stones.
    #[inline]
    pub const fn kg_st(x:f32)->f32{
        x*0.15747304441777
    }

    /// Convert kilograms to tons.
    #[inline]
    pub const fn kg_tn(x:f32)->f32{
        x*0.00110231131092
    }

    /// Convert kilograms to tonnes.
    #[inline]
    pub const fn kg_t(x:f32)->f32{
        x*0.001
    }

    /// Convert ounces to milligrams.
    #[inline]
    pub const fn oz_mg(x:f32)->f32{
        x*28349.52312499999971
    }

    /// Convert ounces to grams.
    #[inline]
    pub const fn oz_g(x:f32)->f32{
        x*28.349523125
    }

    /// Convert ounces to kilograms.
    #[inline]
    pub const fn oz_kg(x:f32)->f32{
        x*0.028349523125
    }

    /// Convert ounces to pounds.
    #[inline]
    pub const fn oz_lb(x:f32)->f32{
        x*0.0625
    }

    /// Convert ounces to stones.
    #[inline]
    pub const fn oz_st(x:f32)->f32{
        x*0.00446428571429
    }

    /// Convert ounces to tons.
    #[inline]
    pub const fn oz_tn(x:f32)->f32{
        x*0.00003125
    }

    /// Convert ounces to tonnes.
    #[inline]
    pub const fn oz_t(x:f32)->f32{
        x*0.00002834952313
    }

    /// Convert pounds to milligrams.
    #[inline]
    pub const fn lb_mg(x:f32)->f32{
        x*453592.36999999999534
    }

    /// Convert pounds to grams.
    #[inline]
    pub const fn lb_g(x:f32)->f32{
        x*453.59237000000002
    }

    /// Convert pounds to kilograms.
    #[inline]
    pub const fn lb_kg(x:f32)->f32{
        x*0.45359237
    }

    /// Convert pounds to ounces.
    #[inline]
    pub const fn lb_oz(x:f32)->f32{
        x*16.0
    }

    /// Convert pounds to stones.
    #[inline]
    pub const fn lb_st(x:f32)->f32{
        x*0.07142857142857
    }

    /// Convert pounds to tons.
    #[inline]
    pub const fn lb_tn(x:f32)->f32{
        x*0.0005
    }

    /// Convert pounds to tonnes.
    #[inline]
    pub const fn lb_t(x:f32)->f32{
        x*0.00045359237
    }

    /// Convert stones to milligrams.
    #[inline]
    pub const fn st_mg(x:f32)->f32{
        x*6350293.17999999970198
    }

    /// Convert stones to grams.
    #[inline]
    pub const fn st_g(x:f32)->f32{
        x*6350.29317999999967
    }

    /// Convert stones to kilograms.
    #[inline]
    pub const fn st_kg(x:f32)->f32{
        x*6.35029318
    }

    /// Convert stones to ounces.
    #[inline]
    pub const fn st_oz(x:f32)->f32{
        x*223.99999999999997
    }

    /// Convert stones to pounds.
    #[inline]
    pub const fn st_lb(x:f32)->f32{
        x*14.0
    }

    /// Convert stones to tons.
    #[inline]
    pub const fn st_tn(x:f32)->f32{
        x*0.007
    }

    /// Convert stones to tonnes.
    #[inline]
    pub const fn st_t(x:f32)->f32{
        x*0.00635029318
    }

    /// Convert tons to milligrams.
    #[inline]
    pub const fn tn_mg(x:f32)->f32{
        x*907184740.0
    }

    /// Convert tons to grams.
    #[inline]
    pub const fn tn_g(x:f32)->f32{
        x*907184.73999999999069
    }

    /// Convert tons to kilograms.
    #[inline]
    pub const fn tn_kg(x:f32)->f32{
        x*907.18474000000003
    }

    /// Convert tons to ounces.
    #[inline]
    pub const fn tn_oz(x:f32)->f32{
        x*32000.0
    }

    /// Convert tons to pounds.
    #[inline]
    pub const fn tn_lb(x:f32)->f32{
        x*2000.0
    }

    /// Convert tons to stones.
    #[inline]
    pub const fn tn_st(x:f32)->f32{
        x*142.85714285714286
    }

    /// Convert tons to tonnes.
    #[inline]
    pub const fn tn_t(x:f32)->f32{
        x*0.90718474
    }

    /// Convert tonnes to milligrams.
    #[inline]
    pub const fn t_mg(x:f32)->f32{
        x*1000000000.0
    }

    /// Convert tonnes to grams.
    #[inline]
    pub const fn t_g(x:f32)->f32{
        x*1000000.0
    }

    /// Convert tonnes to kilograms.
    #[inline]
    pub const fn t_kg(x:f32)->f32{
        x*1000.0
    }

    /// Convert tonnes to ounces.
    #[inline]
    pub const fn t_oz(x:f32)->f32{
        x*35273.96194958041451
    }

    /// Convert tonnes to pounds.
    #[inline]
    pub const fn t_lb(x:f32)->f32{
        x*2204.62262184877591
    }

    /// Convert tonnes to stones.
    #[inline]
    pub const fn t_st(x:f32)->f32{
        x*157.4730444177697
    }

    /// Convert tonnes to tons.
    #[inline]
    pub const fn t_tn(x:f32)->f32{
        x*1.10231131092439
    }

    /// Convert milligrams to grams.
    #[inline(always)]
    pub const fn mg_gram(x:f32)->f32{
        mg_g(x)
    }

    /// Convert milligrams to grams.
    #[inline(always)]
    pub const fn mg_grams(x:f32)->f32{
        mg_g(x)
    }

    /// Convert milligrams to grams.
    #[inline(always)]
    pub const fn milligram_g(x:f32)->f32{
        mg_g(x)
    }

    /// Convert milligrams to grams.
    #[inline(always)]
    pub const fn milligram_gram(x:f32)->f32{
        mg_g(x)
    }

    /// Convert milligrams to grams.
    #[inline(always)]
    pub const fn milligram_grams(x:f32)->f32{
        mg_g(x)
    }

    /// Convert milligrams to grams.
    #[inline(always)]
    pub const fn milligrams_g(x:f32)->f32{
        mg_g(x)
    }

    /// Convert milligrams to grams.
    #[inline(always)]
    pub const fn milligrams_gram(x:f32)->f32{
        mg_g(x)
    }

    /// Convert milligrams to grams.
    #[inline(always)]
    pub const fn milligrams_grams(x:f32)->f32{
        mg_g(x)
    }

    /// Convert milligrams to kilograms.
    #[inline(always)]
    pub const fn mg_kilogram(x:f32)->f32{
        mg_kg(x)
    }

    /// Convert milligrams to kilograms.
    #[inline(always)]
    pub const fn mg_kilograms(x:f32)->f32{
        mg_kg(x)
    }

    /// Convert milligrams to kilograms.
    #[inline(always)]
    pub const fn milligram_kg(x:f32)->f32{
        mg_kg(x)
    }

    /// Convert milligrams to kilograms.
    #[inline(always)]
    pub const fn milligram_kilogram(x:f32)->f32{
        mg_kg(x)
    }

    /// Convert milligrams to kilograms.
    #[inline(always)]
    pub const fn milligram_kilograms(x:f32)->f32{
        mg_kg(x)
    }

    /// Convert milligrams to kilograms.
    #[inline(always)]
    pub const fn milligrams_kg(x:f32)->f32{
        mg_kg(x)
    }

    /// Convert milligrams to kilograms.
    #[inline(always)]
    pub const fn milligrams_kilogram(x:f32)->f32{
        mg_kg(x)
    }

    /// Convert milligrams to kilograms.
    #[inline(always)]
    pub const fn milligrams_kilograms(x:f32)->f32{
        mg_kg(x)
    }

    /// Convert milligrams to ounces.
    #[inline(always)]
    pub const fn mg_ounce(x:f32)->f32{
        mg_oz(x)
    }

    /// Convert milligrams to ounces.
    #[inline(always)]
    pub const fn mg_ounces(x:f32)->f32{
        mg_oz(x)
    }

    /// Convert milligrams to ounces.
    #[inline(always)]
    pub const fn milligram_oz(x:f32)->f32{
        mg_oz(x)
    }

    /// Convert milligrams to ounces.
    #[inline(always)]
    pub const fn milligram_ounce(x:f32)->f32{
        mg_oz(x)
    }

    /// Convert milligrams to ounces.
    #[inline(always)]
    pub const fn milligram_ounces(x:f32)->f32{
        mg_oz(x)
    }

    /// Convert milligrams to ounces.
    #[inline(always)]
    pub const fn milligrams_oz(x:f32)->f32{
        mg_oz(x)
    }

    /// Convert milligrams to ounces.
    #[inline(always)]
    pub const fn milligrams_ounce(x:f32)->f32{
        mg_oz(x)
    }

    /// Convert milligrams to ounces.
    #[inline(always)]
    pub const fn milligrams_ounces(x:f32)->f32{
        mg_oz(x)
    }

    /// Convert milligrams to pounds.
    #[inline(always)]
    pub const fn mg_pound(x:f32)->f32{
        mg_lb(x)
    }

    /// Convert milligrams to pounds.
    #[inline(always)]
    pub const fn mg_pounds(x:f32)->f32{
        mg_lb(x)
    }

    /// Convert milligrams to pounds.
    #[inline(always)]
    pub const fn milligram_lb(x:f32)->f32{
        mg_lb(x)
    }

    /// Convert milligrams to pounds.
    #[inline(always)]
    pub const fn milligram_pound(x:f32)->f32{
        mg_lb(x)
    }

    /// Convert milligrams to pounds.
    #[inline(always)]
    pub const fn milligram_pounds(x:f32)->f32{
        mg_lb(x)
    }

    /// Convert milligrams to pounds.
    #[inline(always)]
    pub const fn milligrams_lb(x:f32)->f32{
        mg_lb(x)
    }

    /// Convert milligrams to pounds.
    #[inline(always)]
    pub const fn milligrams_pound(x:f32)->f32{
        mg_lb(x)
    }

    /// Convert milligrams to pounds.
    #[inline(always)]
    pub const fn milligrams_pounds(x:f32)->f32{
        mg_lb(x)
    }

    /// Convert milligrams to stones.
    #[inline(always)]
    pub const fn mg_stone(x:f32)->f32{
        mg_st(x)
    }

    /// Convert milligrams to stones.
    #[inline(always)]
    pub const fn mg_stones(x:f32)->f32{
        mg_st(x)
    }

    /// Convert milligrams to stones.
    #[inline(always)]
    pub const fn milligram_st(x:f32)->f32{
        mg_st(x)
    }

    /// Convert milligrams to stones.
    #[inline(always)]
    pub const fn milligram_stone(x:f32)->f32{
        mg_st(x)
    }

    /// Convert milligrams to stones.
    #[inline(always)]
    pub const fn milligram_stones(x:f32)->f32{
        mg_st(x)
    }

    /// Convert milligrams to stones.
    #[inline(always)]
    pub const fn milligrams_st(x:f32)->f32{
        mg_st(x)
    }

    /// Convert milligrams to stones.
    #[inline(always)]
    pub const fn milligrams_stone(x:f32)->f32{
        mg_st(x)
    }

    /// Convert milligrams to stones.
    #[inline(always)]
    pub const fn milligrams_stones(x:f32)->f32{
        mg_st(x)
    }

    /// Convert milligrams to tons.
    #[inline(always)]
    pub const fn mg_ton(x:f32)->f32{
        mg_tn(x)
    }

    /// Convert milligrams to tons.
    #[inline(always)]
    pub const fn mg_tons(x:f32)->f32{
        mg_tn(x)
    }

    /// Convert milligrams to tons.
    #[inline(always)]
    pub const fn milligram_tn(x:f32)->f32{
        mg_tn(x)
    }

    /// Convert milligrams to tons.
    #[inline(always)]
    pub const fn milligram_ton(x:f32)->f32{
        mg_tn(x)
    }

    /// Convert milligrams to tons.
    #[inline(always)]
    pub const fn milligram_tons(x:f32)->f32{
        mg_tn(x)
    }

    /// Convert milligrams to tons.
    #[inline(always)]
    pub const fn milligrams_tn(x:f32)->f32{
        mg_tn(x)
    }

    /// Convert milligrams to tons.
    #[inline(always)]
    pub const fn milligrams_ton(x:f32)->f32{
        mg_tn(x)
    }

    /// Convert milligrams to tons.
    #[inline(always)]
    pub const fn milligrams_tons(x:f32)->f32{
        mg_tn(x)
    }

    /// Convert milligrams to tonnes.
    #[inline(always)]
    pub const fn mg_tonne(x:f32)->f32{
        mg_t(x)
    }

    /// Convert milligrams to tonnes.
    #[inline(always)]
    pub const fn mg_tonnes(x:f32)->f32{
        mg_t(x)
    }

    /// Convert milligrams to tonnes.
    #[inline(always)]
    pub const fn milligram_t(x:f32)->f32{
        mg_t(x)
    }

    /// Convert milligrams to tonnes.
    #[inline(always)]
    pub const fn milligram_tonne(x:f32)->f32{
        mg_t(x)
    }

    /// Convert milligrams to tonnes.
    #[inline(always)]
    pub const fn milligram_tonnes(x:f32)->f32{
        mg_t(x)
    }

    /// Convert milligrams to tonnes.
    #[inline(always)]
    pub const fn milligrams_t(x:f32)->f32{
        mg_t(x)
    }

    /// Convert milligrams to tonnes.
    #[inline(always)]
    pub const fn milligrams_tonne(x:f32)->f32{
        mg_t(x)
    }

    /// Convert milligrams to tonnes.
    #[inline(always)]
    pub const fn milligrams_tonnes(x:f32)->f32{
        mg_t(x)
    }

    /// Convert grams to milligrams.
    #[inline(always)]
    pub const fn g_milligram(x:f32)->f32{
        g_mg(x)
    }

    /// Convert grams to milligrams.
    #[inline(always)]
    pub const fn g_milligrams(x:f32)->f32{
        g_mg(x)
    }

    /// Convert grams to milligrams.
    #[inline(always)]
    pub const fn gram_mg(x:f32)->f32{
        g_mg(x)
    }

    /// Convert grams to milligrams.
    #[inline(always)]
    pub const fn gram_milligram(x:f32)->f32{
        g_mg(x)
    }

    /// Convert grams to milligrams.
    #[inline(always)]
    pub const fn gram_milligrams(x:f32)->f32{
        g_mg(x)
    }

    /// Convert grams to milligrams.
    #[inline(always)]
    pub const fn grams_mg(x:f32)->f32{
        g_mg(x)
    }

    /// Convert grams to milligrams.
    #[inline(always)]
    pub const fn grams_milligram(x:f32)->f32{
        g_mg(x)
    }

    /// Convert grams to milligrams.
    #[inline(always)]
    pub const fn grams_milligrams(x:f32)->f32{
        g_mg(x)
    }

    /// Convert grams to kilograms.
    #[inline(always)]
    pub const fn g_kilogram(x:f32)->f32{
        g_kg(x)
    }

    /// Convert grams to kilograms.
    #[inline(always)]
    pub const fn g_kilograms(x:f32)->f32{
        g_kg(x)
    }

    /// Convert grams to kilograms.
    #[inline(always)]
    pub const fn gram_kg(x:f32)->f32{
        g_kg(x)
    }

    /// Convert grams to kilograms.
    #[inline(always)]
    pub const fn gram_kilogram(x:f32)->f32{
        g_kg(x)
    }

    /// Convert grams to kilograms.
    #[inline(always)]
    pub const fn gram_kilograms(x:f32)->f32{
        g_kg(x)
    }

    /// Convert grams to kilograms.
    #[inline(always)]
    pub const fn grams_kg(x:f32)->f32{
        g_kg(x)
    }

    /// Convert grams to kilograms.
    #[inline(always)]
    pub const fn grams_kilogram(x:f32)->f32{
        g_kg(x)
    }

    /// Convert grams to kilograms.
    #[inline(always)]
    pub const fn grams_kilograms(x:f32)->f32{
        g_kg(x)
    }

    /// Convert grams to ounces.
    #[inline(always)]
    pub const fn g_ounce(x:f32)->f32{
        g_oz(x)
    }

    /// Convert grams to ounces.
    #[inline(always)]
    pub const fn g_ounces(x:f32)->f32{
        g_oz(x)
    }

    /// Convert grams to ounces.
    #[inline(always)]
    pub const fn gram_oz(x:f32)->f32{
        g_oz(x)
    }

    /// Convert grams to ounces.
    #[inline(always)]
    pub const fn gram_ounce(x:f32)->f32{
        g_oz(x)
    }

    /// Convert grams to ounces.
    #[inline(always)]
    pub const fn gram_ounces(x:f32)->f32{
        g_oz(x)
    }

    /// Convert grams to ounces.
    #[inline(always)]
    pub const fn grams_oz(x:f32)->f32{
        g_oz(x)
    }

    /// Convert grams to ounces.
    #[inline(always)]
    pub const fn grams_ounce(x:f32)->f32{
        g_oz(x)
    }

    /// Convert grams to ounces.
    #[inline(always)]
    pub const fn grams_ounces(x:f32)->f32{
        g_oz(x)
    }

    /// Convert grams to pounds.
    #[inline(always)]
    pub const fn g_pound(x:f32)->f32{
        g_lb(x)
    }

    /// Convert grams to pounds.
    #[inline(always)]
    pub const fn g_pounds(x:f32)->f32{
        g_lb(x)
    }

    /// Convert grams to pounds.
    #[inline(always)]
    pub const fn gram_lb(x:f32)->f32{
        g_lb(x)
    }

    /// Convert grams to pounds.
    #[inline(always)]
    pub const fn gram_pound(x:f32)->f32{
        g_lb(x)
    }

    /// Convert grams to pounds.
    #[inline(always)]
    pub const fn gram_pounds(x:f32)->f32{
        g_lb(x)
    }

    /// Convert grams to pounds.
    #[inline(always)]
    pub const fn grams_lb(x:f32)->f32{
        g_lb(x)
    }

    /// Convert grams to pounds.
    #[inline(always)]
    pub const fn grams_pound(x:f32)->f32{
        g_lb(x)
    }

    /// Convert grams to pounds.
    #[inline(always)]
    pub const fn grams_pounds(x:f32)->f32{
        g_lb(x)
    }

    /// Convert grams to stones.
    #[inline(always)]
    pub const fn g_stone(x:f32)->f32{
        g_st(x)
    }

    /// Convert grams to stones.
    #[inline(always)]
    pub const fn g_stones(x:f32)->f32{
        g_st(x)
    }

    /// Convert grams to stones.
    #[inline(always)]
    pub const fn gram_st(x:f32)->f32{
        g_st(x)
    }

    /// Convert grams to stones.
    #[inline(always)]
    pub const fn gram_stone(x:f32)->f32{
        g_st(x)
    }

    /// Convert grams to stones.
    #[inline(always)]
    pub const fn gram_stones(x:f32)->f32{
        g_st(x)
    }

    /// Convert grams to stones.
    #[inline(always)]
    pub const fn grams_st(x:f32)->f32{
        g_st(x)
    }

    /// Convert grams to stones.
    #[inline(always)]
    pub const fn grams_stone(x:f32)->f32{
        g_st(x)
    }

    /// Convert grams to stones.
    #[inline(always)]
    pub const fn grams_stones(x:f32)->f32{
        g_st(x)
    }

    /// Convert grams to tons.
    #[inline(always)]
    pub const fn g_ton(x:f32)->f32{
        g_tn(x)
    }

    /// Convert grams to tons.
    #[inline(always)]
    pub const fn g_tons(x:f32)->f32{
        g_tn(x)
    }

    /// Convert grams to tons.
    #[inline(always)]
    pub const fn gram_tn(x:f32)->f32{
        g_tn(x)
    }

    /// Convert grams to tons.
    #[inline(always)]
    pub const fn gram_ton(x:f32)->f32{
        g_tn(x)
    }

    /// Convert grams to tons.
    #[inline(always)]
    pub const fn gram_tons(x:f32)->f32{
        g_tn(x)
    }

    /// Convert grams to tons.
    #[inline(always)]
    pub const fn grams_tn(x:f32)->f32{
        g_tn(x)
    }

    /// Convert grams to tons.
    #[inline(always)]
    pub const fn grams_ton(x:f32)->f32{
        g_tn(x)
    }

    /// Convert grams to tons.
    #[inline(always)]
    pub const fn grams_tons(x:f32)->f32{
        g_tn(x)
    }

    /// Convert grams to tonnes.
    #[inline(always)]
    pub const fn g_tonne(x:f32)->f32{
        g_t(x)
    }

    /// Convert grams to tonnes.
    #[inline(always)]
    pub const fn g_tonnes(x:f32)->f32{
        g_t(x)
    }

    /// Convert grams to tonnes.
    #[inline(always)]
    pub const fn gram_t(x:f32)->f32{
        g_t(x)
    }

    /// Convert grams to tonnes.
    #[inline(always)]
    pub const fn gram_tonne(x:f32)->f32{
        g_t(x)
    }

    /// Convert grams to tonnes.
    #[inline(always)]
    pub const fn gram_tonnes(x:f32)->f32{
        g_t(x)
    }

    /// Convert grams to tonnes.
    #[inline(always)]
    pub const fn grams_t(x:f32)->f32{
        g_t(x)
    }

    /// Convert grams to tonnes.
    #[inline(always)]
    pub const fn grams_tonne(x:f32)->f32{
        g_t(x)
    }

    /// Convert grams to tonnes.
    #[inline(always)]
    pub const fn grams_tonnes(x:f32)->f32{
        g_t(x)
    }

    /// Convert kilograms to milligrams.
    #[inline(always)]
    pub const fn kg_milligram(x:f32)->f32{
        kg_mg(x)
    }

    /// Convert kilograms to milligrams.
    #[inline(always)]
    pub const fn kg_milligrams(x:f32)->f32{
        kg_mg(x)
    }

    /// Convert kilograms to milligrams.
    #[inline(always)]
    pub const fn kilogram_mg(x:f32)->f32{
        kg_mg(x)
    }

    /// Convert kilograms to milligrams.
    #[inline(always)]
    pub const fn kilogram_milligram(x:f32)->f32{
        kg_mg(x)
    }

    /// Convert kilograms to milligrams.
    #[inline(always)]
    pub const fn kilogram_milligrams(x:f32)->f32{
        kg_mg(x)
    }

    /// Convert kilograms to milligrams.
    #[inline(always)]
    pub const fn kilograms_mg(x:f32)->f32{
        kg_mg(x)
    }

    /// Convert kilograms to milligrams.
    #[inline(always)]
    pub const fn kilograms_milligram(x:f32)->f32{
        kg_mg(x)
    }

    /// Convert kilograms to milligrams.
    #[inline(always)]
    pub const fn kilograms_milligrams(x:f32)->f32{
        kg_mg(x)
    }

    /// Convert kilograms to grams.
    #[inline(always)]
    pub const fn kg_gram(x:f32)->f32{
        kg_g(x)
    }

    /// Convert kilograms to grams.
    #[inline(always)]
    pub const fn kg_grams(x:f32)->f32{
        kg_g(x)
    }

    /// Convert kilograms to grams.
    #[inline(always)]
    pub const fn kilogram_g(x:f32)->f32{
        kg_g(x)
    }

    /// Convert kilograms to grams.
    #[inline(always)]
    pub const fn kilogram_gram(x:f32)->f32{
        kg_g(x)
    }

    /// Convert kilograms to grams.
    #[inline(always)]
    pub const fn kilogram_grams(x:f32)->f32{
        kg_g(x)
    }

    /// Convert kilograms to grams.
    #[inline(always)]
    pub const fn kilograms_g(x:f32)->f32{
        kg_g(x)
    }

    /// Convert kilograms to grams.
    #[inline(always)]
    pub const fn kilograms_gram(x:f32)->f32{
        kg_g(x)
    }

    /// Convert kilograms to grams.
    #[inline(always)]
    pub const fn kilograms_grams(x:f32)->f32{
        kg_g(x)
    }

    /// Convert kilograms to ounces.
    #[inline(always)]
    pub const fn kg_ounce(x:f32)->f32{
        kg_oz(x)
    }

    /// Convert kilograms to ounces.
    #[inline(always)]
    pub const fn kg_ounces(x:f32)->f32{
        kg_oz(x)
    }

    /// Convert kilograms to ounces.
    #[inline(always)]
    pub const fn kilogram_oz(x:f32)->f32{
        kg_oz(x)
    }

    /// Convert kilograms to ounces.
    #[inline(always)]
    pub const fn kilogram_ounce(x:f32)->f32{
        kg_oz(x)
    }

    /// Convert kilograms to ounces.
    #[inline(always)]
    pub const fn kilogram_ounces(x:f32)->f32{
        kg_oz(x)
    }

    /// Convert kilograms to ounces.
    #[inline(always)]
    pub const fn kilograms_oz(x:f32)->f32{
        kg_oz(x)
    }

    /// Convert kilograms to ounces.
    #[inline(always)]
    pub const fn kilograms_ounce(x:f32)->f32{
        kg_oz(x)
    }

    /// Convert kilograms to ounces.
    #[inline(always)]
    pub const fn kilograms_ounces(x:f32)->f32{
        kg_oz(x)
    }

    /// Convert kilograms to pounds.
    #[inline(always)]
    pub const fn kg_pound(x:f32)->f32{
        kg_lb(x)
    }

    /// Convert kilograms to pounds.
    #[inline(always)]
    pub const fn kg_pounds(x:f32)->f32{
        kg_lb(x)
    }

    /// Convert kilograms to pounds.
    #[inline(always)]
    pub const fn kilogram_lb(x:f32)->f32{
        kg_lb(x)
    }

    /// Convert kilograms to pounds.
    #[inline(always)]
    pub const fn kilogram_pound(x:f32)->f32{
        kg_lb(x)
    }

    /// Convert kilograms to pounds.
    #[inline(always)]
    pub const fn kilogram_pounds(x:f32)->f32{
        kg_lb(x)
    }

    /// Convert kilograms to pounds.
    #[inline(always)]
    pub const fn kilograms_lb(x:f32)->f32{
        kg_lb(x)
    }

    /// Convert kilograms to pounds.
    #[inline(always)]
    pub const fn kilograms_pound(x:f32)->f32{
        kg_lb(x)
    }

    /// Convert kilograms to pounds.
    #[inline(always)]
    pub const fn kilograms_pounds(x:f32)->f32{
        kg_lb(x)
    }

    /// Convert kilograms to stones.
    #[inline(always)]
    pub const fn kg_stone(x:f32)->f32{
        kg_st(x)
    }

    /// Convert kilograms to stones.
    #[inline(always)]
    pub const fn kg_stones(x:f32)->f32{
        kg_st(x)
    }

    /// Convert kilograms to stones.
    #[inline(always)]
    pub const fn kilogram_st(x:f32)->f32{
        kg_st(x)
    }

    /// Convert kilograms to stones.
    #[inline(always)]
    pub const fn kilogram_stone(x:f32)->f32{
        kg_st(x)
    }

    /// Convert kilograms to stones.
    #[inline(always)]
    pub const fn kilogram_stones(x:f32)->f32{
        kg_st(x)
    }

    /// Convert kilograms to stones.
    #[inline(always)]
    pub const fn kilograms_st(x:f32)->f32{
        kg_st(x)
    }

    /// Convert kilograms to stones.
    #[inline(always)]
    pub const fn kilograms_stone(x:f32)->f32{
        kg_st(x)
    }

    /// Convert kilograms to stones.
    #[inline(always)]
    pub const fn kilograms_stones(x:f32)->f32{
        kg_st(x)
    }

    /// Convert kilograms to tons.
    #[inline(always)]
    pub const fn kg_ton(x:f32)->f32{
        kg_tn(x)
    }

    /// Convert kilograms to tons.
    #[inline(always)]
    pub const fn kg_tons(x:f32)->f32{
        kg_tn(x)
    }

    /// Convert kilograms to tons.
    #[inline(always)]
    pub const fn kilogram_tn(x:f32)->f32{
        kg_tn(x)
    }

    /// Convert kilograms to tons.
    #[inline(always)]
    pub const fn kilogram_ton(x:f32)->f32{
        kg_tn(x)
    }

    /// Convert kilograms to tons.
    #[inline(always)]
    pub const fn kilogram_tons(x:f32)->f32{
        kg_tn(x)
    }

    /// Convert kilograms to tons.
    #[inline(always)]
    pub const fn kilograms_tn(x:f32)->f32{
        kg_tn(x)
    }

    /// Convert kilograms to tons.
    #[inline(always)]
    pub const fn kilograms_ton(x:f32)->f32{
        kg_tn(x)
    }

    /// Convert kilograms to tons.
    #[inline(always)]
    pub const fn kilograms_tons(x:f32)->f32{
        kg_tn(x)
    }

    /// Convert kilograms to tonnes.
    #[inline(always)]
    pub const fn kg_tonne(x:f32)->f32{
        kg_t(x)
    }

    /// Convert kilograms to tonnes.
    #[inline(always)]
    pub const fn kg_tonnes(x:f32)->f32{
        kg_t(x)
    }

    /// Convert kilograms to tonnes.
    #[inline(always)]
    pub const fn kilogram_t(x:f32)->f32{
        kg_t(x)
    }

    /// Convert kilograms to tonnes.
    #[inline(always)]
    pub const fn kilogram_tonne(x:f32)->f32{
        kg_t(x)
    }

    /// Convert kilograms to tonnes.
    #[inline(always)]
    pub const fn kilogram_tonnes(x:f32)->f32{
        kg_t(x)
    }

    /// Convert kilograms to tonnes.
    #[inline(always)]
    pub const fn kilograms_t(x:f32)->f32{
        kg_t(x)
    }

    /// Convert kilograms to tonnes.
    #[inline(always)]
    pub const fn kilograms_tonne(x:f32)->f32{
        kg_t(x)
    }

    /// Convert kilograms to tonnes.
    #[inline(always)]
    pub const fn kilograms_tonnes(x:f32)->f32{
        kg_t(x)
    }

    /// Convert ounces to milligrams.
    #[inline(always)]
    pub const fn oz_milligram(x:f32)->f32{
        oz_mg(x)
    }

    /// Convert ounces to milligrams.
    #[inline(always)]
    pub const fn oz_milligrams(x:f32)->f32{
        oz_mg(x)
    }

    /// Convert ounces to milligrams.
    #[inline(always)]
    pub const fn ounce_mg(x:f32)->f32{
        oz_mg(x)
    }

    /// Convert ounces to milligrams.
    #[inline(always)]
    pub const fn ounce_milligram(x:f32)->f32{
        oz_mg(x)
    }

    /// Convert ounces to milligrams.
    #[inline(always)]
    pub const fn ounce_milligrams(x:f32)->f32{
        oz_mg(x)
    }

    /// Convert ounces to milligrams.
    #[inline(always)]
    pub const fn ounces_mg(x:f32)->f32{
        oz_mg(x)
    }

    /// Convert ounces to milligrams.
    #[inline(always)]
    pub const fn ounces_milligram(x:f32)->f32{
        oz_mg(x)
    }

    /// Convert ounces to milligrams.
    #[inline(always)]
    pub const fn ounces_milligrams(x:f32)->f32{
        oz_mg(x)
    }

    /// Convert ounces to grams.
    #[inline(always)]
    pub const fn oz_gram(x:f32)->f32{
        oz_g(x)
    }

    /// Convert ounces to grams.
    #[inline(always)]
    pub const fn oz_grams(x:f32)->f32{
        oz_g(x)
    }

    /// Convert ounces to grams.
    #[inline(always)]
    pub const fn ounce_g(x:f32)->f32{
        oz_g(x)
    }

    /// Convert ounces to grams.
    #[inline(always)]
    pub const fn ounce_gram(x:f32)->f32{
        oz_g(x)
    }

    /// Convert ounces to grams.
    #[inline(always)]
    pub const fn ounce_grams(x:f32)->f32{
        oz_g(x)
    }

    /// Convert ounces to grams.
    #[inline(always)]
    pub const fn ounces_g(x:f32)->f32{
        oz_g(x)
    }

    /// Convert ounces to grams.
    #[inline(always)]
    pub const fn ounces_gram(x:f32)->f32{
        oz_g(x)
    }

    /// Convert ounces to grams.
    #[inline(always)]
    pub const fn ounces_grams(x:f32)->f32{
        oz_g(x)
    }

    /// Convert ounces to kilograms.
    #[inline(always)]
    pub const fn oz_kilogram(x:f32)->f32{
        oz_kg(x)
    }

    /// Convert ounces to kilograms.
    #[inline(always)]
    pub const fn oz_kilograms(x:f32)->f32{
        oz_kg(x)
    }

    /// Convert ounces to kilograms.
    #[inline(always)]
    pub const fn ounce_kg(x:f32)->f32{
        oz_kg(x)
    }

    /// Convert ounces to kilograms.
    #[inline(always)]
    pub const fn ounce_kilogram(x:f32)->f32{
        oz_kg(x)
    }

    /// Convert ounces to kilograms.
    #[inline(always)]
    pub const fn ounce_kilograms(x:f32)->f32{
        oz_kg(x)
    }

    /// Convert ounces to kilograms.
    #[inline(always)]
    pub const fn ounces_kg(x:f32)->f32{
        oz_kg(x)
    }

    /// Convert ounces to kilograms.
    #[inline(always)]
    pub const fn ounces_kilogram(x:f32)->f32{
        oz_kg(x)
    }

    /// Convert ounces to kilograms.
    #[inline(always)]
    pub const fn ounces_kilograms(x:f32)->f32{
        oz_kg(x)
    }

    /// Convert ounces to pounds.
    #[inline(always)]
    pub const fn oz_pound(x:f32)->f32{
        oz_lb(x)
    }

    /// Convert ounces to pounds.
    #[inline(always)]
    pub const fn oz_pounds(x:f32)->f32{
        oz_lb(x)
    }

    /// Convert ounces to pounds.
    #[inline(always)]
    pub const fn ounce_lb(x:f32)->f32{
        oz_lb(x)
    }

    /// Convert ounces to pounds.
    #[inline(always)]
    pub const fn ounce_pound(x:f32)->f32{
        oz_lb(x)
    }

    /// Convert ounces to pounds.
    #[inline(always)]
    pub const fn ounce_pounds(x:f32)->f32{
        oz_lb(x)
    }

    /// Convert ounces to pounds.
    #[inline(always)]
    pub const fn ounces_lb(x:f32)->f32{
        oz_lb(x)
    }

    /// Convert ounces to pounds.
    #[inline(always)]
    pub const fn ounces_pound(x:f32)->f32{
        oz_lb(x)
    }

    /// Convert ounces to pounds.
    #[inline(always)]
    pub const fn ounces_pounds(x:f32)->f32{
        oz_lb(x)
    }

    /// Convert ounces to stones.
    #[inline(always)]
    pub const fn oz_stone(x:f32)->f32{
        oz_st(x)
    }

    /// Convert ounces to stones.
    #[inline(always)]
    pub const fn oz_stones(x:f32)->f32{
        oz_st(x)
    }

    /// Convert ounces to stones.
    #[inline(always)]
    pub const fn ounce_st(x:f32)->f32{
        oz_st(x)
    }

    /// Convert ounces to stones.
    #[inline(always)]
    pub const fn ounce_stone(x:f32)->f32{
        oz_st(x)
    }

    /// Convert ounces to stones.
    #[inline(always)]
    pub const fn ounce_stones(x:f32)->f32{
        oz_st(x)
    }

    /// Convert ounces to stones.
    #[inline(always)]
    pub const fn ounces_st(x:f32)->f32{
        oz_st(x)
    }

    /// Convert ounces to stones.
    #[inline(always)]
    pub const fn ounces_stone(x:f32)->f32{
        oz_st(x)
    }

    /// Convert ounces to stones.
    #[inline(always)]
    pub const fn ounces_stones(x:f32)->f32{
        oz_st(x)
    }

    /// Convert ounces to tons.
    #[inline(always)]
    pub const fn oz_ton(x:f32)->f32{
        oz_tn(x)
    }

    /// Convert ounces to tons.
    #[inline(always)]
    pub const fn oz_tons(x:f32)->f32{
        oz_tn(x)
    }

    /// Convert ounces to tons.
    #[inline(always)]
    pub const fn ounce_tn(x:f32)->f32{
        oz_tn(x)
    }

    /// Convert ounces to tons.
    #[inline(always)]
    pub const fn ounce_ton(x:f32)->f32{
        oz_tn(x)
    }

    /// Convert ounces to tons.
    #[inline(always)]
    pub const fn ounce_tons(x:f32)->f32{
        oz_tn(x)
    }

    /// Convert ounces to tons.
    #[inline(always)]
    pub const fn ounces_tn(x:f32)->f32{
        oz_tn(x)
    }

    /// Convert ounces to tons.
    #[inline(always)]
    pub const fn ounces_ton(x:f32)->f32{
        oz_tn(x)
    }

    /// Convert ounces to tons.
    #[inline(always)]
    pub const fn ounces_tons(x:f32)->f32{
        oz_tn(x)
    }

    /// Convert ounces to tonnes.
    #[inline(always)]
    pub const fn oz_tonne(x:f32)->f32{
        oz_t(x)
    }

    /// Convert ounces to tonnes.
    #[inline(always)]
    pub const fn oz_tonnes(x:f32)->f32{
        oz_t(x)
    }

    /// Convert ounces to tonnes.
    #[inline(always)]
    pub const fn ounce_t(x:f32)->f32{
        oz_t(x)
    }

    /// Convert ounces to tonnes.
    #[inline(always)]
    pub const fn ounce_tonne(x:f32)->f32{
        oz_t(x)
    }

    /// Convert ounces to tonnes.
    #[inline(always)]
    pub const fn ounce_tonnes(x:f32)->f32{
        oz_t(x)
    }

    /// Convert ounces to tonnes.
    #[inline(always)]
    pub const fn ounces_t(x:f32)->f32{
        oz_t(x)
    }

    /// Convert ounces to tonnes.
    #[inline(always)]
    pub const fn ounces_tonne(x:f32)->f32{
        oz_t(x)
    }

    /// Convert ounces to tonnes.
    #[inline(always)]
    pub const fn ounces_tonnes(x:f32)->f32{
        oz_t(x)
    }

    /// Convert pounds to milligrams.
    #[inline(always)]
    pub const fn lb_milligram(x:f32)->f32{
        lb_mg(x)
    }

    /// Convert pounds to milligrams.
    #[inline(always)]
    pub const fn lb_milligrams(x:f32)->f32{
        lb_mg(x)
    }

    /// Convert pounds to milligrams.
    #[inline(always)]
    pub const fn pound_mg(x:f32)->f32{
        lb_mg(x)
    }

    /// Convert pounds to milligrams.
    #[inline(always)]
    pub const fn pound_milligram(x:f32)->f32{
        lb_mg(x)
    }

    /// Convert pounds to milligrams.
    #[inline(always)]
    pub const fn pound_milligrams(x:f32)->f32{
        lb_mg(x)
    }

    /// Convert pounds to milligrams.
    #[inline(always)]
    pub const fn pounds_mg(x:f32)->f32{
        lb_mg(x)
    }

    /// Convert pounds to milligrams.
    #[inline(always)]
    pub const fn pounds_milligram(x:f32)->f32{
        lb_mg(x)
    }

    /// Convert pounds to milligrams.
    #[inline(always)]
    pub const fn pounds_milligrams(x:f32)->f32{
        lb_mg(x)
    }

    /// Convert pounds to grams.
    #[inline(always)]
    pub const fn lb_gram(x:f32)->f32{
        lb_g(x)
    }

    /// Convert pounds to grams.
    #[inline(always)]
    pub const fn lb_grams(x:f32)->f32{
        lb_g(x)
    }

    /// Convert pounds to grams.
    #[inline(always)]
    pub const fn pound_g(x:f32)->f32{
        lb_g(x)
    }

    /// Convert pounds to grams.
    #[inline(always)]
    pub const fn pound_gram(x:f32)->f32{
        lb_g(x)
    }

    /// Convert pounds to grams.
    #[inline(always)]
    pub const fn pound_grams(x:f32)->f32{
        lb_g(x)
    }

    /// Convert pounds to grams.
    #[inline(always)]
    pub const fn pounds_g(x:f32)->f32{
        lb_g(x)
    }

    /// Convert pounds to grams.
    #[inline(always)]
    pub const fn pounds_gram(x:f32)->f32{
        lb_g(x)
    }

    /// Convert pounds to grams.
    #[inline(always)]
    pub const fn pounds_grams(x:f32)->f32{
        lb_g(x)
    }

    /// Convert pounds to kilograms.
    #[inline(always)]
    pub const fn lb_kilogram(x:f32)->f32{
        lb_kg(x)
    }

    /// Convert pounds to kilograms.
    #[inline(always)]
    pub const fn lb_kilograms(x:f32)->f32{
        lb_kg(x)
    }

    /// Convert pounds to kilograms.
    #[inline(always)]
    pub const fn pound_kg(x:f32)->f32{
        lb_kg(x)
    }

    /// Convert pounds to kilograms.
    #[inline(always)]
    pub const fn pound_kilogram(x:f32)->f32{
        lb_kg(x)
    }

    /// Convert pounds to kilograms.
    #[inline(always)]
    pub const fn pound_kilograms(x:f32)->f32{
        lb_kg(x)
    }

    /// Convert pounds to kilograms.
    #[inline(always)]
    pub const fn pounds_kg(x:f32)->f32{
        lb_kg(x)
    }

    /// Convert pounds to kilograms.
    #[inline(always)]
    pub const fn pounds_kilogram(x:f32)->f32{
        lb_kg(x)
    }

    /// Convert pounds to kilograms.
    #[inline(always)]
    pub const fn pounds_kilograms(x:f32)->f32{
        lb_kg(x)
    }

    /// Convert pounds to ounces.
    #[inline(always)]
    pub const fn lb_ounce(x:f32)->f32{
        lb_oz(x)
    }

    /// Convert pounds to ounces.
    #[inline(always)]
    pub const fn lb_ounces(x:f32)->f32{
        lb_oz(x)
    }

    /// Convert pounds to ounces.
    #[inline(always)]
    pub const fn pound_oz(x:f32)->f32{
        lb_oz(x)
    }

    /// Convert pounds to ounces.
    #[inline(always)]
    pub const fn pound_ounce(x:f32)->f32{
        lb_oz(x)
    }

    /// Convert pounds to ounces.
    #[inline(always)]
    pub const fn pound_ounces(x:f32)->f32{
        lb_oz(x)
    }

    /// Convert pounds to ounces.
    #[inline(always)]
    pub const fn pounds_oz(x:f32)->f32{
        lb_oz(x)
    }

    /// Convert pounds to ounces.
    #[inline(always)]
    pub const fn pounds_ounce(x:f32)->f32{
        lb_oz(x)
    }

    /// Convert pounds to ounces.
    #[inline(always)]
    pub const fn pounds_ounces(x:f32)->f32{
        lb_oz(x)
    }

    /// Convert pounds to stones.
    #[inline(always)]
    pub const fn lb_stone(x:f32)->f32{
        lb_st(x)
    }

    /// Convert pounds to stones.
    #[inline(always)]
    pub const fn lb_stones(x:f32)->f32{
        lb_st(x)
    }

    /// Convert pounds to stones.
    #[inline(always)]
    pub const fn pound_st(x:f32)->f32{
        lb_st(x)
    }

    /// Convert pounds to stones.
    #[inline(always)]
    pub const fn pound_stone(x:f32)->f32{
        lb_st(x)
    }

    /// Convert pounds to stones.
    #[inline(always)]
    pub const fn pound_stones(x:f32)->f32{
        lb_st(x)
    }

    /// Convert pounds to stones.
    #[inline(always)]
    pub const fn pounds_st(x:f32)->f32{
        lb_st(x)
    }

    /// Convert pounds to stones.
    #[inline(always)]
    pub const fn pounds_stone(x:f32)->f32{
        lb_st(x)
    }

    /// Convert pounds to stones.
    #[inline(always)]
    pub const fn pounds_stones(x:f32)->f32{
        lb_st(x)
    }

    /// Convert pounds to tons.
    #[inline(always)]
    pub const fn lb_ton(x:f32)->f32{
        lb_tn(x)
    }

    /// Convert pounds to tons.
    #[inline(always)]
    pub const fn lb_tons(x:f32)->f32{
        lb_tn(x)
    }

    /// Convert pounds to tons.
    #[inline(always)]
    pub const fn pound_tn(x:f32)->f32{
        lb_tn(x)
    }

    /// Convert pounds to tons.
    #[inline(always)]
    pub const fn pound_ton(x:f32)->f32{
        lb_tn(x)
    }

    /// Convert pounds to tons.
    #[inline(always)]
    pub const fn pound_tons(x:f32)->f32{
        lb_tn(x)
    }

    /// Convert pounds to tons.
    #[inline(always)]
    pub const fn pounds_tn(x:f32)->f32{
        lb_tn(x)
    }

    /// Convert pounds to tons.
    #[inline(always)]
    pub const fn pounds_ton(x:f32)->f32{
        lb_tn(x)
    }

    /// Convert pounds to tons.
    #[inline(always)]
    pub const fn pounds_tons(x:f32)->f32{
        lb_tn(x)
    }

    /// Convert pounds to tonnes.
    #[inline(always)]
    pub const fn lb_tonne(x:f32)->f32{
        lb_t(x)
    }

    /// Convert pounds to tonnes.
    #[inline(always)]
    pub const fn lb_tonnes(x:f32)->f32{
        lb_t(x)
    }

    /// Convert pounds to tonnes.
    #[inline(always)]
    pub const fn pound_t(x:f32)->f32{
        lb_t(x)
    }

    /// Convert pounds to tonnes.
    #[inline(always)]
    pub const fn pound_tonne(x:f32)->f32{
        lb_t(x)
    }

    /// Convert pounds to tonnes.
    #[inline(always)]
    pub const fn pound_tonnes(x:f32)->f32{
        lb_t(x)
    }

    /// Convert pounds to tonnes.
    #[inline(always)]
    pub const fn pounds_t(x:f32)->f32{
        lb_t(x)
    }

    /// Convert pounds to tonnes.
    #[inline(always)]
    pub const fn pounds_tonne(x:f32)->f32{
        lb_t(x)
    }

    /// Convert pounds to tonnes.
    #[inline(always)]
    pub const fn pounds_tonnes(x:f32)->f32{
        lb_t(x)
    }

    /// Convert stones to milligrams.
    #[inline(always)]
    pub const fn st_milligram(x:f32)->f32{
        st_mg(x)
    }

    /// Convert stones to milligrams.
    #[inline(always)]
    pub const fn st_milligrams(x:f32)->f32{
        st_mg(x)
    }

    /// Convert stones to milligrams.
    #[inline(always)]
    pub const fn stone_mg(x:f32)->f32{
        st_mg(x)
    }

    /// Convert stones to milligrams.
    #[inline(always)]
    pub const fn stone_milligram(x:f32)->f32{
        st_mg(x)
    }

    /// Convert stones to milligrams.
    #[inline(always)]
    pub const fn stone_milligrams(x:f32)->f32{
        st_mg(x)
    }

    /// Convert stones to milligrams.
    #[inline(always)]
    pub const fn stones_mg(x:f32)->f32{
        st_mg(x)
    }

    /// Convert stones to milligrams.
    #[inline(always)]
    pub const fn stones_milligram(x:f32)->f32{
        st_mg(x)
    }

    /// Convert stones to milligrams.
    #[inline(always)]
    pub const fn stones_milligrams(x:f32)->f32{
        st_mg(x)
    }

    /// Convert stones to grams.
    #[inline(always)]
    pub const fn st_gram(x:f32)->f32{
        st_g(x)
    }

    /// Convert stones to grams.
    #[inline(always)]
    pub const fn st_grams(x:f32)->f32{
        st_g(x)
    }

    /// Convert stones to grams.
    #[inline(always)]
    pub const fn stone_g(x:f32)->f32{
        st_g(x)
    }

    /// Convert stones to grams.
    #[inline(always)]
    pub const fn stone_gram(x:f32)->f32{
        st_g(x)
    }

    /// Convert stones to grams.
    #[inline(always)]
    pub const fn stone_grams(x:f32)->f32{
        st_g(x)
    }

    /// Convert stones to grams.
    #[inline(always)]
    pub const fn stones_g(x:f32)->f32{
        st_g(x)
    }

    /// Convert stones to grams.
    #[inline(always)]
    pub const fn stones_gram(x:f32)->f32{
        st_g(x)
    }

    /// Convert stones to grams.
    #[inline(always)]
    pub const fn stones_grams(x:f32)->f32{
        st_g(x)
    }

    /// Convert stones to kilograms.
    #[inline(always)]
    pub const fn st_kilogram(x:f32)->f32{
        st_kg(x)
    }

    /// Convert stones to kilograms.
    #[inline(always)]
    pub const fn st_kilograms(x:f32)->f32{
        st_kg(x)
    }

    /// Convert stones to kilograms.
    #[inline(always)]
    pub const fn stone_kg(x:f32)->f32{
        st_kg(x)
    }

    /// Convert stones to kilograms.
    #[inline(always)]
    pub const fn stone_kilogram(x:f32)->f32{
        st_kg(x)
    }

    /// Convert stones to kilograms.
    #[inline(always)]
    pub const fn stone_kilograms(x:f32)->f32{
        st_kg(x)
    }

    /// Convert stones to kilograms.
    #[inline(always)]
    pub const fn stones_kg(x:f32)->f32{
        st_kg(x)
    }

    /// Convert stones to kilograms.
    #[inline(always)]
    pub const fn stones_kilogram(x:f32)->f32{
        st_kg(x)
    }

    /// Convert stones to kilograms.
    #[inline(always)]
    pub const fn stones_kilograms(x:f32)->f32{
        st_kg(x)
    }

    /// Convert stones to ounces.
    #[inline(always)]
    pub const fn st_ounce(x:f32)->f32{
        st_oz(x)
    }

    /// Convert stones to ounces.
    #[inline(always)]
    pub const fn st_ounces(x:f32)->f32{
        st_oz(x)
    }

    /// Convert stones to ounces.
    #[inline(always)]
    pub const fn stone_oz(x:f32)->f32{
        st_oz(x)
    }

    /// Convert stones to ounces.
    #[inline(always)]
    pub const fn stone_ounce(x:f32)->f32{
        st_oz(x)
    }

    /// Convert stones to ounces.
    #[inline(always)]
    pub const fn stone_ounces(x:f32)->f32{
        st_oz(x)
    }

    /// Convert stones to ounces.
    #[inline(always)]
    pub const fn stones_oz(x:f32)->f32{
        st_oz(x)
    }

    /// Convert stones to ounces.
    #[inline(always)]
    pub const fn stones_ounce(x:f32)->f32{
        st_oz(x)
    }

    /// Convert stones to ounces.
    #[inline(always)]
    pub const fn stones_ounces(x:f32)->f32{
        st_oz(x)
    }

    /// Convert stones to pounds.
    #[inline(always)]
    pub const fn st_pound(x:f32)->f32{
        st_lb(x)
    }

    /// Convert stones to pounds.
    #[inline(always)]
    pub const fn st_pounds(x:f32)->f32{
        st_lb(x)
    }

    /// Convert stones to pounds.
    #[inline(always)]
    pub const fn stone_lb(x:f32)->f32{
        st_lb(x)
    }

    /// Convert stones to pounds.
    #[inline(always)]
    pub const fn stone_pound(x:f32)->f32{
        st_lb(x)
    }

    /// Convert stones to pounds.
    #[inline(always)]
    pub const fn stone_pounds(x:f32)->f32{
        st_lb(x)
    }

    /// Convert stones to pounds.
    #[inline(always)]
    pub const fn stones_lb(x:f32)->f32{
        st_lb(x)
    }

    /// Convert stones to pounds.
    #[inline(always)]
    pub const fn stones_pound(x:f32)->f32{
        st_lb(x)
    }

    /// Convert stones to pounds.
    #[inline(always)]
    pub const fn stones_pounds(x:f32)->f32{
        st_lb(x)
    }

    /// Convert stones to tons.
    #[inline(always)]
    pub const fn st_ton(x:f32)->f32{
        st_tn(x)
    }

    /// Convert stones to tons.
    #[inline(always)]
    pub const fn st_tons(x:f32)->f32{
        st_tn(x)
    }

    /// Convert stones to tons.
    #[inline(always)]
    pub const fn stone_tn(x:f32)->f32{
        st_tn(x)
    }

    /// Convert stones to tons.
    #[inline(always)]
    pub const fn stone_ton(x:f32)->f32{
        st_tn(x)
    }

    /// Convert stones to tons.
    #[inline(always)]
    pub const fn stone_tons(x:f32)->f32{
        st_tn(x)
    }

    /// Convert stones to tons.
    #[inline(always)]
    pub const fn stones_tn(x:f32)->f32{
        st_tn(x)
    }

    /// Convert stones to tons.
    #[inline(always)]
    pub const fn stones_ton(x:f32)->f32{
        st_tn(x)
    }

    /// Convert stones to tons.
    #[inline(always)]
    pub const fn stones_tons(x:f32)->f32{
        st_tn(x)
    }

    /// Convert stones to tonnes.
    #[inline(always)]
    pub const fn st_tonne(x:f32)->f32{
        st_t(x)
    }

    /// Convert stones to tonnes.
    #[inline(always)]
    pub const fn st_tonnes(x:f32)->f32{
        st_t(x)
    }

    /// Convert stones to tonnes.
    #[inline(always)]
    pub const fn stone_t(x:f32)->f32{
        st_t(x)
    }

    /// Convert stones to tonnes.
    #[inline(always)]
    pub const fn stone_tonne(x:f32)->f32{
        st_t(x)
    }

    /// Convert stones to tonnes.
    #[inline(always)]
    pub const fn stone_tonnes(x:f32)->f32{
        st_t(x)
    }

    /// Convert stones to tonnes.
    #[inline(always)]
    pub const fn stones_t(x:f32)->f32{
        st_t(x)
    }

    /// Convert stones to tonnes.
    #[inline(always)]
    pub const fn stones_tonne(x:f32)->f32{
        st_t(x)
    }

    /// Convert stones to tonnes.
    #[inline(always)]
    pub const fn stones_tonnes(x:f32)->f32{
        st_t(x)
    }

    /// Convert tons to milligrams.
    #[inline(always)]
    pub const fn tn_milligram(x:f32)->f32{
        tn_mg(x)
    }

    /// Convert tons to milligrams.
    #[inline(always)]
    pub const fn tn_milligrams(x:f32)->f32{
        tn_mg(x)
    }

    /// Convert tons to milligrams.
    #[inline(always)]
    pub const fn ton_mg(x:f32)->f32{
        tn_mg(x)
    }

    /// Convert tons to milligrams.
    #[inline(always)]
    pub const fn ton_milligram(x:f32)->f32{
        tn_mg(x)
    }

    /// Convert tons to milligrams.
    #[inline(always)]
    pub const fn ton_milligrams(x:f32)->f32{
        tn_mg(x)
    }

    /// Convert tons to milligrams.
    #[inline(always)]
    pub const fn tons_mg(x:f32)->f32{
        tn_mg(x)
    }

    /// Convert tons to milligrams.
    #[inline(always)]
    pub const fn tons_milligram(x:f32)->f32{
        tn_mg(x)
    }

    /// Convert tons to milligrams.
    #[inline(always)]
    pub const fn tons_milligrams(x:f32)->f32{
        tn_mg(x)
    }

    /// Convert tons to grams.
    #[inline(always)]
    pub const fn tn_gram(x:f32)->f32{
        tn_g(x)
    }

    /// Convert tons to grams.
    #[inline(always)]
    pub const fn tn_grams(x:f32)->f32{
        tn_g(x)
    }

    /// Convert tons to grams.
    #[inline(always)]
    pub const fn ton_g(x:f32)->f32{
        tn_g(x)
    }

    /// Convert tons to grams.
    #[inline(always)]
    pub const fn ton_gram(x:f32)->f32{
        tn_g(x)
    }

    /// Convert tons to grams.
    #[inline(always)]
    pub const fn ton_grams(x:f32)->f32{
        tn_g(x)
    }

    /// Convert tons to grams.
    #[inline(always)]
    pub const fn tons_g(x:f32)->f32{
        tn_g(x)
    }

    /// Convert tons to grams.
    #[inline(always)]
    pub const fn tons_gram(x:f32)->f32{
        tn_g(x)
    }

    /// Convert tons to grams.
    #[inline(always)]
    pub const fn tons_grams(x:f32)->f32{
        tn_g(x)
    }

    /// Convert tons to kilograms.
    #[inline(always)]
    pub const fn tn_kilogram(x:f32)->f32{
        tn_kg(x)
    }

    /// Convert tons to kilograms.
    #[inline(always)]
    pub const fn tn_kilograms(x:f32)->f32{
        tn_kg(x)
    }

    /// Convert tons to kilograms.
    #[inline(always)]
    pub const fn ton_kg(x:f32)->f32{
        tn_kg(x)
    }

    /// Convert tons to kilograms.
    #[inline(always)]
    pub const fn ton_kilogram(x:f32)->f32{
        tn_kg(x)
    }

    /// Convert tons to kilograms.
    #[inline(always)]
    pub const fn ton_kilograms(x:f32)->f32{
        tn_kg(x)
    }

    /// Convert tons to kilograms.
    #[inline(always)]
    pub const fn tons_kg(x:f32)->f32{
        tn_kg(x)
    }

    /// Convert tons to kilograms.
    #[inline(always)]
    pub const fn tons_kilogram(x:f32)->f32{
        tn_kg(x)
    }

    /// Convert tons to kilograms.
    #[inline(always)]
    pub const fn tons_kilograms(x:f32)->f32{
        tn_kg(x)
    }

    /// Convert tons to ounces.
    #[inline(always)]
    pub const fn tn_ounce(x:f32)->f32{
        tn_oz(x)
    }

    /// Convert tons to ounces.
    #[inline(always)]
    pub const fn tn_ounces(x:f32)->f32{
        tn_oz(x)
    }

    /// Convert tons to ounces.
    #[inline(always)]
    pub const fn ton_oz(x:f32)->f32{
        tn_oz(x)
    }

    /// Convert tons to ounces.
    #[inline(always)]
    pub const fn ton_ounce(x:f32)->f32{
        tn_oz(x)
    }

    /// Convert tons to ounces.
    #[inline(always)]
    pub const fn ton_ounces(x:f32)->f32{
        tn_oz(x)
    }

    /// Convert tons to ounces.
    #[inline(always)]
    pub const fn tons_oz(x:f32)->f32{
        tn_oz(x)
    }

    /// Convert tons to ounces.
    #[inline(always)]
    pub const fn tons_ounce(x:f32)->f32{
        tn_oz(x)
    }

    /// Convert tons to ounces.
    #[inline(always)]
    pub const fn tons_ounces(x:f32)->f32{
        tn_oz(x)
    }

    /// Convert tons to pounds.
    #[inline(always)]
    pub const fn tn_pound(x:f32)->f32{
        tn_lb(x)
    }

    /// Convert tons to pounds.
    #[inline(always)]
    pub const fn tn_pounds(x:f32)->f32{
        tn_lb(x)
    }

    /// Convert tons to pounds.
    #[inline(always)]
    pub const fn ton_lb(x:f32)->f32{
        tn_lb(x)
    }

    /// Convert tons to pounds.
    #[inline(always)]
    pub const fn ton_pound(x:f32)->f32{
        tn_lb(x)
    }

    /// Convert tons to pounds.
    #[inline(always)]
    pub const fn ton_pounds(x:f32)->f32{
        tn_lb(x)
    }

    /// Convert tons to pounds.
    #[inline(always)]
    pub const fn tons_lb(x:f32)->f32{
        tn_lb(x)
    }

    /// Convert tons to pounds.
    #[inline(always)]
    pub const fn tons_pound(x:f32)->f32{
        tn_lb(x)
    }

    /// Convert tons to pounds.
    #[inline(always)]
    pub const fn tons_pounds(x:f32)->f32{
        tn_lb(x)
    }

    /// Convert tons to stones.
    #[inline(always)]
    pub const fn tn_stone(x:f32)->f32{
        tn_st(x)
    }

    /// Convert tons to stones.
    #[inline(always)]
    pub const fn tn_stones(x:f32)->f32{
        tn_st(x)
    }

    /// Convert tons to stones.
    #[inline(always)]
    pub const fn ton_st(x:f32)->f32{
        tn_st(x)
    }

    /// Convert tons to stones.
    #[inline(always)]
    pub const fn ton_stone(x:f32)->f32{
        tn_st(x)
    }

    /// Convert tons to stones.
    #[inline(always)]
    pub const fn ton_stones(x:f32)->f32{
        tn_st(x)
    }

    /// Convert tons to stones.
    #[inline(always)]
    pub const fn tons_st(x:f32)->f32{
        tn_st(x)
    }

    /// Convert tons to stones.
    #[inline(always)]
    pub const fn tons_stone(x:f32)->f32{
        tn_st(x)
    }

    /// Convert tons to stones.
    #[inline(always)]
    pub const fn tons_stones(x:f32)->f32{
        tn_st(x)
    }

    /// Convert tons to tonnes.
    #[inline(always)]
    pub const fn tn_tonne(x:f32)->f32{
        tn_t(x)
    }

    /// Convert tons to tonnes.
    #[inline(always)]
    pub const fn tn_tonnes(x:f32)->f32{
        tn_t(x)
    }

    /// Convert tons to tonnes.
    #[inline(always)]
    pub const fn ton_t(x:f32)->f32{
        tn_t(x)
    }

    /// Convert tons to tonnes.
    #[inline(always)]
    pub const fn ton_tonne(x:f32)->f32{
        tn_t(x)
    }

    /// Convert tons to tonnes.
    #[inline(always)]
    pub const fn ton_tonnes(x:f32)->f32{
        tn_t(x)
    }

    /// Convert tons to tonnes.
    #[inline(always)]
    pub const fn tons_t(x:f32)->f32{
        tn_t(x)
    }

    /// Convert tons to tonnes.
    #[inline(always)]
    pub const fn tons_tonne(x:f32)->f32{
        tn_t(x)
    }

    /// Convert tons to tonnes.
    #[inline(always)]
    pub const fn tons_tonnes(x:f32)->f32{
        tn_t(x)
    }

    /// Convert tonnes to milligrams.
    #[inline(always)]
    pub const fn t_milligram(x:f32)->f32{
        t_mg(x)
    }

    /// Convert tonnes to milligrams.
    #[inline(always)]
    pub const fn t_milligrams(x:f32)->f32{
        t_mg(x)
    }

    /// Convert tonnes to milligrams.
    #[inline(always)]
    pub const fn tonne_mg(x:f32)->f32{
        t_mg(x)
    }

    /// Convert tonnes to milligrams.
    #[inline(always)]
    pub const fn tonne_milligram(x:f32)->f32{
        t_mg(x)
    }

    /// Convert tonnes to milligrams.
    #[inline(always)]
    pub const fn tonne_milligrams(x:f32)->f32{
        t_mg(x)
    }

    /// Convert tonnes to milligrams.
    #[inline(always)]
    pub const fn tonnes_mg(x:f32)->f32{
        t_mg(x)
    }

    /// Convert tonnes to milligrams.
    #[inline(always)]
    pub const fn tonnes_milligram(x:f32)->f32{
        t_mg(x)
    }

    /// Convert tonnes to milligrams.
    #[inline(always)]
    pub const fn tonnes_milligrams(x:f32)->f32{
        t_mg(x)
    }

    /// Convert tonnes to grams.
    #[inline(always)]
    pub const fn t_gram(x:f32)->f32{
        t_g(x)
    }

    /// Convert tonnes to grams.
    #[inline(always)]
    pub const fn t_grams(x:f32)->f32{
        t_g(x)
    }

    /// Convert tonnes to grams.
    #[inline(always)]
    pub const fn tonne_g(x:f32)->f32{
        t_g(x)
    }

    /// Convert tonnes to grams.
    #[inline(always)]
    pub const fn tonne_gram(x:f32)->f32{
        t_g(x)
    }

    /// Convert tonnes to grams.
    #[inline(always)]
    pub const fn tonne_grams(x:f32)->f32{
        t_g(x)
    }

    /// Convert tonnes to grams.
    #[inline(always)]
    pub const fn tonnes_g(x:f32)->f32{
        t_g(x)
    }

    /// Convert tonnes to grams.
    #[inline(always)]
    pub const fn tonnes_gram(x:f32)->f32{
        t_g(x)
    }

    /// Convert tonnes to grams.
    #[inline(always)]
    pub const fn tonnes_grams(x:f32)->f32{
        t_g(x)
    }

    /// Convert tonnes to kilograms.
    #[inline(always)]
    pub const fn t_kilogram(x:f32)->f32{
        t_kg(x)
    }

    /// Convert tonnes to kilograms.
    #[inline(always)]
    pub const fn t_kilograms(x:f32)->f32{
        t_kg(x)
    }

    /// Convert tonnes to kilograms.
    #[inline(always)]
    pub const fn tonne_kg(x:f32)->f32{
        t_kg(x)
    }

    /// Convert tonnes to kilograms.
    #[inline(always)]
    pub const fn tonne_kilogram(x:f32)->f32{
        t_kg(x)
    }

    /// Convert tonnes to kilograms.
    #[inline(always)]
    pub const fn tonne_kilograms(x:f32)->f32{
        t_kg(x)
    }

    /// Convert tonnes to kilograms.
    #[inline(always)]
    pub const fn tonnes_kg(x:f32)->f32{
        t_kg(x)
    }

    /// Convert tonnes to kilograms.
    #[inline(always)]
    pub const fn tonnes_kilogram(x:f32)->f32{
        t_kg(x)
    }

    /// Convert tonnes to kilograms.
    #[inline(always)]
    pub const fn tonnes_kilograms(x:f32)->f32{
        t_kg(x)
    }

    /// Convert tonnes to ounces.
    #[inline(always)]
    pub const fn t_ounce(x:f32)->f32{
        t_oz(x)
    }

    /// Convert tonnes to ounces.
    #[inline(always)]
    pub const fn t_ounces(x:f32)->f32{
        t_oz(x)
    }

    /// Convert tonnes to ounces.
    #[inline(always)]
    pub const fn tonne_oz(x:f32)->f32{
        t_oz(x)
    }

    /// Convert tonnes to ounces.
    #[inline(always)]
    pub const fn tonne_ounce(x:f32)->f32{
        t_oz(x)
    }

    /// Convert tonnes to ounces.
    #[inline(always)]
    pub const fn tonne_ounces(x:f32)->f32{
        t_oz(x)
    }

    /// Convert tonnes to ounces.
    #[inline(always)]
    pub const fn tonnes_oz(x:f32)->f32{
        t_oz(x)
    }

    /// Convert tonnes to ounces.
    #[inline(always)]
    pub const fn tonnes_ounce(x:f32)->f32{
        t_oz(x)
    }

    /// Convert tonnes to ounces.
    #[inline(always)]
    pub const fn tonnes_ounces(x:f32)->f32{
        t_oz(x)
    }

    /// Convert tonnes to pounds.
    #[inline(always)]
    pub const fn t_pound(x:f32)->f32{
        t_lb(x)
    }

    /// Convert tonnes to pounds.
    #[inline(always)]
    pub const fn t_pounds(x:f32)->f32{
        t_lb(x)
    }

    /// Convert tonnes to pounds.
    #[inline(always)]
    pub const fn tonne_lb(x:f32)->f32{
        t_lb(x)
    }

    /// Convert tonnes to pounds.
    #[inline(always)]
    pub const fn tonne_pound(x:f32)->f32{
        t_lb(x)
    }

    /// Convert tonnes to pounds.
    #[inline(always)]
    pub const fn tonne_pounds(x:f32)->f32{
        t_lb(x)
    }

    /// Convert tonnes to pounds.
    #[inline(always)]
    pub const fn tonnes_lb(x:f32)->f32{
        t_lb(x)
    }

    /// Convert tonnes to pounds.
    #[inline(always)]
    pub const fn tonnes_pound(x:f32)->f32{
        t_lb(x)
    }

    /// Convert tonnes to pounds.
    #[inline(always)]
    pub const fn tonnes_pounds(x:f32)->f32{
        t_lb(x)
    }

    /// Convert tonnes to stones.
    #[inline(always)]
    pub const fn t_stone(x:f32)->f32{
        t_st(x)
    }

    /// Convert tonnes to stones.
    #[inline(always)]
    pub const fn t_stones(x:f32)->f32{
        t_st(x)
    }

    /// Convert tonnes to stones.
    #[inline(always)]
    pub const fn tonne_st(x:f32)->f32{
        t_st(x)
    }

    /// Convert tonnes to stones.
    #[inline(always)]
    pub const fn tonne_stone(x:f32)->f32{
        t_st(x)
    }

    /// Convert tonnes to stones.
    #[inline(always)]
    pub const fn tonne_stones(x:f32)->f32{
        t_st(x)
    }

    /// Convert tonnes to stones.
    #[inline(always)]
    pub const fn tonnes_st(x:f32)->f32{
        t_st(x)
    }

    /// Convert tonnes to stones.
    #[inline(always)]
    pub const fn tonnes_stone(x:f32)->f32{
        t_st(x)
    }

    /// Convert tonnes to stones.
    #[inline(always)]
    pub const fn tonnes_stones(x:f32)->f32{
        t_st(x)
    }

    /// Convert tonnes to tons.
    #[inline(always)]
    pub const fn t_ton(x:f32)->f32{
        t_tn(x)
    }

    /// Convert tonnes to tons.
    #[inline(always)]
    pub const fn t_tons(x:f32)->f32{
        t_tn(x)
    }

    /// Convert tonnes to tons.
    #[inline(always)]
    pub const fn tonne_tn(x:f32)->f32{
        t_tn(x)
    }

    /// Convert tonnes to tons.
    #[inline(always)]
    pub const fn tonne_ton(x:f32)->f32{
        t_tn(x)
    }

    /// Convert tonnes to tons.
    #[inline(always)]
    pub const fn tonne_tons(x:f32)->f32{
        t_tn(x)
    }

    /// Convert tonnes to tons.
    #[inline(always)]
    pub const fn tonnes_tn(x:f32)->f32{
        t_tn(x)
    }

    /// Convert tonnes to tons.
    #[inline(always)]
    pub const fn tonnes_ton(x:f32)->f32{
        t_tn(x)
    }

    /// Convert tonnes to tons.
    #[inline(always)]
    pub const fn tonnes_tons(x:f32)->f32{
        t_tn(x)
    }
}

pub mod volume{
    /// Convert milliliters to liters.
    #[inline]
    pub const fn ml_l(x:f32)->f32{
        x*0.001
    }

    /// Convert milliliters to fluid ounces.
    #[inline]
    pub const fn ml_floz(x:f32)->f32{
        x*0.03381402
    }

    /// Convert milliliters to cups.
    #[inline]
    pub const fn ml_cp(x:f32)->f32{
        x*0.00422675
    }

    /// Convert milliliters to pints.
    #[inline]
    pub const fn ml_pt(x:f32)->f32{
        x*0.00211338
    }

    /// Convert milliliters to quarts.
    #[inline]
    pub const fn ml_qt(x:f32)->f32{
        x*0.00105669
    }

    /// Convert milliliters to gallons.
    #[inline]
    pub const fn ml_gal(x:f32)->f32{
        x*0.00026417
    }

    /// Convert liters to milliliters.
    #[inline]
    pub const fn l_ml(x:f32)->f32{
        x*1000.0
    }

    /// Convert liters to fluid ounces.
    #[inline]
    pub const fn l_floz(x:f32)->f32{
        x*33.8140227
    }

    /// Convert liters to cups.
    #[inline]
    pub const fn l_cp(x:f32)->f32{
        x*4.22675284
    }

    /// Convert liters to pints.
    #[inline]
    pub const fn l_pt(x:f32)->f32{
        x*2.11337642
    }

    /// Convert liters to quarts.
    #[inline]
    pub const fn l_qt(x:f32)->f32{
        x*1.05668821
    }

    /// Convert liters to gallons.
    #[inline]
    pub const fn l_gal(x:f32)->f32{
        x*0.26417205
    }

    /// Convert fluid ounces to milliliters.
    #[inline]
    pub const fn floz_ml(x:f32)->f32{
        x*29.57352956
    }

    /// Convert fluid ounces to liters.
    #[inline]
    pub const fn floz_l(x:f32)->f32{
        x*0.02957353
    }

    /// Convert fluid ounces to cups.
    #[inline]
    pub const fn floz_cp(x:f32)->f32{
        x*0.125
    }

    /// Convert fluid ounces to pints.
    #[inline]
    pub const fn floz_pt(x:f32)->f32{
        x*0.0625
    }

    /// Convert fluid ounces to quarts.
    #[inline]
    pub const fn floz_qt(x:f32)->f32{
        x*0.03125
    }

    /// Convert fluid ounces to gallons.
    #[inline]
    pub const fn floz_gal(x:f32)->f32{
        x*0.0078125
    }

    /// Convert cups to milliliters.
    #[inline]
    pub const fn cp_ml(x:f32)->f32{
        x*236.5882365
    }

    /// Convert cups to liters.
    #[inline]
    pub const fn cp_l(x:f32)->f32{
        x*0.23658824
    }

    /// Convert cups to fluid ounces.
    #[inline]
    pub const fn cp_floz(x:f32)->f32{
        x*8.0
    }

    /// Convert cups to pints.
    #[inline]
    pub const fn cp_pt(x:f32)->f32{
        x*0.5
    }

    /// Convert cups to quarts.
    #[inline]
    pub const fn cp_qt(x:f32)->f32{
        x*0.25
    }

    /// Convert cups to gallons.
    #[inline]
    pub const fn cp_gal(x:f32)->f32{
        x*0.0625
    }

    /// Convert pints to milliliters.
    #[inline]
    pub const fn pt_ml(x:f32)->f32{
        x*473.176473
    }

    /// Convert pints to liters.
    #[inline]
    pub const fn pt_l(x:f32)->f32{
        x*0.47317647
    }

    /// Convert pints to fluid ounces.
    #[inline]
    pub const fn pt_floz(x:f32)->f32{
        x*16.0
    }

    /// Convert pints to cups.
    #[inline]
    pub const fn pt_cp(x:f32)->f32{
        x*2.0
    }

    /// Convert pints to quarts.
    #[inline]
    pub const fn pt_qt(x:f32)->f32{
        x*0.5
    }

    /// Convert pints to gallons.
    #[inline]
    pub const fn pt_gal(x:f32)->f32{
        x*0.125
    }

    /// Convert quarts to milliliters.
    #[inline]
    pub const fn qt_ml(x:f32)->f32{
        x*946.352946
    }

    /// Convert quarts to liters.
    #[inline]
    pub const fn qt_l(x:f32)->f32{
        x*0.94635295
    }

    /// Convert quarts to fluid ounces.
    #[inline]
    pub const fn qt_floz(x:f32)->f32{
        x*32.0
    }

    /// Convert quarts to cups.
    #[inline]
    pub const fn qt_cp(x:f32)->f32{
        x*4.0
    }

    /// Convert quarts to pints.
    #[inline]
    pub const fn qt_pt(x:f32)->f32{
        x*2.0
    }

    /// Convert quarts to gallons.
    #[inline]
    pub const fn qt_gal(x:f32)->f32{
        x*0.25
    }

    /// Convert gallons to milliliters.
    #[inline]
    pub const fn gal_ml(x:f32)->f32{
        x*3785.411784
    }

    /// Convert gallons to liters.
    #[inline]
    pub const fn gal_l(x:f32)->f32{
        x*3.78541178
    }

    /// Convert gallons to fluid ounces.
    #[inline]
    pub const fn gal_floz(x:f32)->f32{
        x*128.0
    }

    /// Convert gallons to cups.
    #[inline]
    pub const fn gal_cp(x:f32)->f32{
        x*16.0
    }

    /// Convert gallons to pints.
    #[inline]
    pub const fn gal_pt(x:f32)->f32{
        x*8.0
    }

    /// Convert gallons to quarts.
    #[inline]
    pub const fn gal_qt(x:f32)->f32{
        x*4.0
    }

    /// Convert milliliters to liters.
    #[inline(always)]
    pub const fn ml_liter(x:f32)->f32{
        ml_l(x)
    }

    /// Convert milliliters to liters.
    #[inline(always)]
    pub const fn ml_liters(x:f32)->f32{
        ml_l(x)
    }

    /// Convert milliliters to liters.
    #[inline(always)]
    pub const fn milliliter_l(x:f32)->f32{
        ml_l(x)
    }

    /// Convert milliliters to liters.
    #[inline(always)]
    pub const fn milliliter_liter(x:f32)->f32{
        ml_l(x)
    }

    /// Convert milliliters to liters.
    #[inline(always)]
    pub const fn milliliter_liters(x:f32)->f32{
        ml_l(x)
    }

    /// Convert milliliters to liters.
    #[inline(always)]
    pub const fn milliliters_l(x:f32)->f32{
        ml_l(x)
    }

    /// Convert milliliters to liters.
    #[inline(always)]
    pub const fn milliliters_liter(x:f32)->f32{
        ml_l(x)
    }

    /// Convert milliliters to liters.
    #[inline(always)]
    pub const fn milliliters_liters(x:f32)->f32{
        ml_l(x)
    }

    /// Convert milliliters to fluid ounces.
    #[inline(always)]
    pub const fn ml_fluidounce(x:f32)->f32{
        ml_floz(x)
    }

    /// Convert milliliters to fluid ounces.
    #[inline(always)]
    pub const fn ml_fluidounces(x:f32)->f32{
        ml_floz(x)
    }

    /// Convert milliliters to fluid ounces.
    #[inline(always)]
    pub const fn milliliter_floz(x:f32)->f32{
        ml_floz(x)
    }

    /// Convert milliliters to fluid ounces.
    #[inline(always)]
    pub const fn milliliter_fluidounce(x:f32)->f32{
        ml_floz(x)
    }

    /// Convert milliliters to fluid ounces.
    #[inline(always)]
    pub const fn milliliter_fluidounces(x:f32)->f32{
        ml_floz(x)
    }

    /// Convert milliliters to fluid ounces.
    #[inline(always)]
    pub const fn milliliters_floz(x:f32)->f32{
        ml_floz(x)
    }

    /// Convert milliliters to fluid ounces.
    #[inline(always)]
    pub const fn milliliters_fluidounce(x:f32)->f32{
        ml_floz(x)
    }

    /// Convert milliliters to fluid ounces.
    #[inline(always)]
    pub const fn milliliters_fluidounces(x:f32)->f32{
        ml_floz(x)
    }

    /// Convert milliliters to cups.
    #[inline(always)]
    pub const fn ml_cup(x:f32)->f32{
        ml_cp(x)
    }

    /// Convert milliliters to cups.
    #[inline(always)]
    pub const fn ml_cups(x:f32)->f32{
        ml_cp(x)
    }

    /// Convert milliliters to cups.
    #[inline(always)]
    pub const fn milliliter_cp(x:f32)->f32{
        ml_cp(x)
    }

    /// Convert milliliters to cups.
    #[inline(always)]
    pub const fn milliliter_cup(x:f32)->f32{
        ml_cp(x)
    }

    /// Convert milliliters to cups.
    #[inline(always)]
    pub const fn milliliter_cups(x:f32)->f32{
        ml_cp(x)
    }

    /// Convert milliliters to cups.
    #[inline(always)]
    pub const fn milliliters_cp(x:f32)->f32{
        ml_cp(x)
    }

    /// Convert milliliters to cups.
    #[inline(always)]
    pub const fn milliliters_cup(x:f32)->f32{
        ml_cp(x)
    }

    /// Convert milliliters to cups.
    #[inline(always)]
    pub const fn milliliters_cups(x:f32)->f32{
        ml_cp(x)
    }

    /// Convert milliliters to pints.
    #[inline(always)]
    pub const fn ml_pint(x:f32)->f32{
        ml_pt(x)
    }

    /// Convert milliliters to pints.
    #[inline(always)]
    pub const fn ml_pints(x:f32)->f32{
        ml_pt(x)
    }

    /// Convert milliliters to pints.
    #[inline(always)]
    pub const fn milliliter_pt(x:f32)->f32{
        ml_pt(x)
    }

    /// Convert milliliters to pints.
    #[inline(always)]
    pub const fn milliliter_pint(x:f32)->f32{
        ml_pt(x)
    }

    /// Convert milliliters to pints.
    #[inline(always)]
    pub const fn milliliter_pints(x:f32)->f32{
        ml_pt(x)
    }

    /// Convert milliliters to pints.
    #[inline(always)]
    pub const fn milliliters_pt(x:f32)->f32{
        ml_pt(x)
    }

    /// Convert milliliters to pints.
    #[inline(always)]
    pub const fn milliliters_pint(x:f32)->f32{
        ml_pt(x)
    }

    /// Convert milliliters to pints.
    #[inline(always)]
    pub const fn milliliters_pints(x:f32)->f32{
        ml_pt(x)
    }

    /// Convert milliliters to quarts.
    #[inline(always)]
    pub const fn ml_quart(x:f32)->f32{
        ml_qt(x)
    }

    /// Convert milliliters to quarts.
    #[inline(always)]
    pub const fn ml_quarts(x:f32)->f32{
        ml_qt(x)
    }

    /// Convert milliliters to quarts.
    #[inline(always)]
    pub const fn milliliter_qt(x:f32)->f32{
        ml_qt(x)
    }

    /// Convert milliliters to quarts.
    #[inline(always)]
    pub const fn milliliter_quart(x:f32)->f32{
        ml_qt(x)
    }

    /// Convert milliliters to quarts.
    #[inline(always)]
    pub const fn milliliter_quarts(x:f32)->f32{
        ml_qt(x)
    }

    /// Convert milliliters to quarts.
    #[inline(always)]
    pub const fn milliliters_qt(x:f32)->f32{
        ml_qt(x)
    }

    /// Convert milliliters to quarts.
    #[inline(always)]
    pub const fn milliliters_quart(x:f32)->f32{
        ml_qt(x)
    }

    /// Convert milliliters to quarts.
    #[inline(always)]
    pub const fn milliliters_quarts(x:f32)->f32{
        ml_qt(x)
    }

    /// Convert milliliters to gallons.
    #[inline(always)]
    pub const fn ml_gallon(x:f32)->f32{
        ml_gal(x)
    }

    /// Convert milliliters to gallons.
    #[inline(always)]
    pub const fn ml_gallons(x:f32)->f32{
        ml_gal(x)
    }

    /// Convert milliliters to gallons.
    #[inline(always)]
    pub const fn milliliter_gal(x:f32)->f32{
        ml_gal(x)
    }

    /// Convert milliliters to gallons.
    #[inline(always)]
    pub const fn milliliter_gallon(x:f32)->f32{
        ml_gal(x)
    }

    /// Convert milliliters to gallons.
    #[inline(always)]
    pub const fn milliliter_gallons(x:f32)->f32{
        ml_gal(x)
    }

    /// Convert milliliters to gallons.
    #[inline(always)]
    pub const fn milliliters_gal(x:f32)->f32{
        ml_gal(x)
    }

    /// Convert milliliters to gallons.
    #[inline(always)]
    pub const fn milliliters_gallon(x:f32)->f32{
        ml_gal(x)
    }

    /// Convert milliliters to gallons.
    #[inline(always)]
    pub const fn milliliters_gallons(x:f32)->f32{
        ml_gal(x)
    }

    /// Convert liters to milliliters.
    #[inline(always)]
    pub const fn l_milliliter(x:f32)->f32{
        l_ml(x)
    }

    /// Convert liters to milliliters.
    #[inline(always)]
    pub const fn l_milliliters(x:f32)->f32{
        l_ml(x)
    }

    /// Convert liters to milliliters.
    #[inline(always)]
    pub const fn liter_ml(x:f32)->f32{
        l_ml(x)
    }

    /// Convert liters to milliliters.
    #[inline(always)]
    pub const fn liter_milliliter(x:f32)->f32{
        l_ml(x)
    }

    /// Convert liters to milliliters.
    #[inline(always)]
    pub const fn liter_milliliters(x:f32)->f32{
        l_ml(x)
    }

    /// Convert liters to milliliters.
    #[inline(always)]
    pub const fn liters_ml(x:f32)->f32{
        l_ml(x)
    }

    /// Convert liters to milliliters.
    #[inline(always)]
    pub const fn liters_milliliter(x:f32)->f32{
        l_ml(x)
    }

    /// Convert liters to milliliters.
    #[inline(always)]
    pub const fn liters_milliliters(x:f32)->f32{
        l_ml(x)
    }

    /// Convert liters to fluid ounces.
    #[inline(always)]
    pub const fn l_fluidounce(x:f32)->f32{
        l_floz(x)
    }

    /// Convert liters to fluid ounces.
    #[inline(always)]
    pub const fn l_fluidounces(x:f32)->f32{
        l_floz(x)
    }

    /// Convert liters to fluid ounces.
    #[inline(always)]
    pub const fn liter_floz(x:f32)->f32{
        l_floz(x)
    }

    /// Convert liters to fluid ounces.
    #[inline(always)]
    pub const fn liter_fluidounce(x:f32)->f32{
        l_floz(x)
    }

    /// Convert liters to fluid ounces.
    #[inline(always)]
    pub const fn liter_fluidounces(x:f32)->f32{
        l_floz(x)
    }

    /// Convert liters to fluid ounces.
    #[inline(always)]
    pub const fn liters_floz(x:f32)->f32{
        l_floz(x)
    }

    /// Convert liters to fluid ounces.
    #[inline(always)]
    pub const fn liters_fluidounce(x:f32)->f32{
        l_floz(x)
    }

    /// Convert liters to fluid ounces.
    #[inline(always)]
    pub const fn liters_fluidounces(x:f32)->f32{
        l_floz(x)
    }

    /// Convert liters to cups.
    #[inline(always)]
    pub const fn l_cup(x:f32)->f32{
        l_cp(x)
    }

    /// Convert liters to cups.
    #[inline(always)]
    pub const fn l_cups(x:f32)->f32{
        l_cp(x)
    }

    /// Convert liters to cups.
    #[inline(always)]
    pub const fn liter_cp(x:f32)->f32{
        l_cp(x)
    }

    /// Convert liters to cups.
    #[inline(always)]
    pub const fn liter_cup(x:f32)->f32{
        l_cp(x)
    }

    /// Convert liters to cups.
    #[inline(always)]
    pub const fn liter_cups(x:f32)->f32{
        l_cp(x)
    }

    /// Convert liters to cups.
    #[inline(always)]
    pub const fn liters_cp(x:f32)->f32{
        l_cp(x)
    }

    /// Convert liters to cups.
    #[inline(always)]
    pub const fn liters_cup(x:f32)->f32{
        l_cp(x)
    }

    /// Convert liters to cups.
    #[inline(always)]
    pub const fn liters_cups(x:f32)->f32{
        l_cp(x)
    }

    /// Convert liters to pints.
    #[inline(always)]
    pub const fn l_pint(x:f32)->f32{
        l_pt(x)
    }

    /// Convert liters to pints.
    #[inline(always)]
    pub const fn l_pints(x:f32)->f32{
        l_pt(x)
    }

    /// Convert liters to pints.
    #[inline(always)]
    pub const fn liter_pt(x:f32)->f32{
        l_pt(x)
    }

    /// Convert liters to pints.
    #[inline(always)]
    pub const fn liter_pint(x:f32)->f32{
        l_pt(x)
    }

    /// Convert liters to pints.
    #[inline(always)]
    pub const fn liter_pints(x:f32)->f32{
        l_pt(x)
    }

    /// Convert liters to pints.
    #[inline(always)]
    pub const fn liters_pt(x:f32)->f32{
        l_pt(x)
    }

    /// Convert liters to pints.
    #[inline(always)]
    pub const fn liters_pint(x:f32)->f32{
        l_pt(x)
    }

    /// Convert liters to pints.
    #[inline(always)]
    pub const fn liters_pints(x:f32)->f32{
        l_pt(x)
    }

    /// Convert liters to quarts.
    #[inline(always)]
    pub const fn l_quart(x:f32)->f32{
        l_qt(x)
    }

    /// Convert liters to quarts.
    #[inline(always)]
    pub const fn l_quarts(x:f32)->f32{
        l_qt(x)
    }

    /// Convert liters to quarts.
    #[inline(always)]
    pub const fn liter_qt(x:f32)->f32{
        l_qt(x)
    }

    /// Convert liters to quarts.
    #[inline(always)]
    pub const fn liter_quart(x:f32)->f32{
        l_qt(x)
    }

    /// Convert liters to quarts.
    #[inline(always)]
    pub const fn liter_quarts(x:f32)->f32{
        l_qt(x)
    }

    /// Convert liters to quarts.
    #[inline(always)]
    pub const fn liters_qt(x:f32)->f32{
        l_qt(x)
    }

    /// Convert liters to quarts.
    #[inline(always)]
    pub const fn liters_quart(x:f32)->f32{
        l_qt(x)
    }

    /// Convert liters to quarts.
    #[inline(always)]
    pub const fn liters_quarts(x:f32)->f32{
        l_qt(x)
    }

    /// Convert liters to gallons.
    #[inline(always)]
    pub const fn l_gallon(x:f32)->f32{
        l_gal(x)
    }

    /// Convert liters to gallons.
    #[inline(always)]
    pub const fn l_gallons(x:f32)->f32{
        l_gal(x)
    }

    /// Convert liters to gallons.
    #[inline(always)]
    pub const fn liter_gal(x:f32)->f32{
        l_gal(x)
    }

    /// Convert liters to gallons.
    #[inline(always)]
    pub const fn liter_gallon(x:f32)->f32{
        l_gal(x)
    }

    /// Convert liters to gallons.
    #[inline(always)]
    pub const fn liter_gallons(x:f32)->f32{
        l_gal(x)
    }

    /// Convert liters to gallons.
    #[inline(always)]
    pub const fn liters_gal(x:f32)->f32{
        l_gal(x)
    }

    /// Convert liters to gallons.
    #[inline(always)]
    pub const fn liters_gallon(x:f32)->f32{
        l_gal(x)
    }

    /// Convert liters to gallons.
    #[inline(always)]
    pub const fn liters_gallons(x:f32)->f32{
        l_gal(x)
    }

    /// Convert fluid ounces to milliliters.
    #[inline(always)]
    pub const fn floz_milliliter(x:f32)->f32{
        floz_ml(x)
    }

    /// Convert fluid ounces to milliliters.
    #[inline(always)]
    pub const fn floz_milliliters(x:f32)->f32{
        floz_ml(x)
    }

    /// Convert fluid ounces to milliliters.
    #[inline(always)]
    pub const fn fluidounce_ml(x:f32)->f32{
        floz_ml(x)
    }

    /// Convert fluid ounces to milliliters.
    #[inline(always)]
    pub const fn fluidounce_milliliter(x:f32)->f32{
        floz_ml(x)
    }

    /// Convert fluid ounces to milliliters.
    #[inline(always)]
    pub const fn fluidounce_milliliters(x:f32)->f32{
        floz_ml(x)
    }

    /// Convert fluid ounces to milliliters.
    #[inline(always)]
    pub const fn fluidounces_ml(x:f32)->f32{
        floz_ml(x)
    }

    /// Convert fluid ounces to milliliters.
    #[inline(always)]
    pub const fn fluidounces_milliliter(x:f32)->f32{
        floz_ml(x)
    }

    /// Convert fluid ounces to milliliters.
    #[inline(always)]
    pub const fn fluidounces_milliliters(x:f32)->f32{
        floz_ml(x)
    }

    /// Convert fluid ounces to liters.
    #[inline(always)]
    pub const fn floz_liter(x:f32)->f32{
        floz_l(x)
    }

    /// Convert fluid ounces to liters.
    #[inline(always)]
    pub const fn floz_liters(x:f32)->f32{
        floz_l(x)
    }

    /// Convert fluid ounces to liters.
    #[inline(always)]
    pub const fn fluidounce_l(x:f32)->f32{
        floz_l(x)
    }

    /// Convert fluid ounces to liters.
    #[inline(always)]
    pub const fn fluidounce_liter(x:f32)->f32{
        floz_l(x)
    }

    /// Convert fluid ounces to liters.
    #[inline(always)]
    pub const fn fluidounce_liters(x:f32)->f32{
        floz_l(x)
    }

    /// Convert fluid ounces to liters.
    #[inline(always)]
    pub const fn fluidounces_l(x:f32)->f32{
        floz_l(x)
    }

    /// Convert fluid ounces to liters.
    #[inline(always)]
    pub const fn fluidounces_liter(x:f32)->f32{
        floz_l(x)
    }

    /// Convert fluid ounces to liters.
    #[inline(always)]
    pub const fn fluidounces_liters(x:f32)->f32{
        floz_l(x)
    }

    /// Convert fluid ounces to cups.
    #[inline(always)]
    pub const fn floz_cup(x:f32)->f32{
        floz_cp(x)
    }

    /// Convert fluid ounces to cups.
    #[inline(always)]
    pub const fn floz_cups(x:f32)->f32{
        floz_cp(x)
    }

    /// Convert fluid ounces to cups.
    #[inline(always)]
    pub const fn fluidounce_cp(x:f32)->f32{
        floz_cp(x)
    }

    /// Convert fluid ounces to cups.
    #[inline(always)]
    pub const fn fluidounce_cup(x:f32)->f32{
        floz_cp(x)
    }

    /// Convert fluid ounces to cups.
    #[inline(always)]
    pub const fn fluidounce_cups(x:f32)->f32{
        floz_cp(x)
    }

    /// Convert fluid ounces to cups.
    #[inline(always)]
    pub const fn fluidounces_cp(x:f32)->f32{
        floz_cp(x)
    }

    /// Convert fluid ounces to cups.
    #[inline(always)]
    pub const fn fluidounces_cup(x:f32)->f32{
        floz_cp(x)
    }

    /// Convert fluid ounces to cups.
    #[inline(always)]
    pub const fn fluidounces_cups(x:f32)->f32{
        floz_cp(x)
    }

    /// Convert fluid ounces to pints.
    #[inline(always)]
    pub const fn floz_pint(x:f32)->f32{
        floz_pt(x)
    }

    /// Convert fluid ounces to pints.
    #[inline(always)]
    pub const fn floz_pints(x:f32)->f32{
        floz_pt(x)
    }

    /// Convert fluid ounces to pints.
    #[inline(always)]
    pub const fn fluidounce_pt(x:f32)->f32{
        floz_pt(x)
    }

    /// Convert fluid ounces to pints.
    #[inline(always)]
    pub const fn fluidounce_pint(x:f32)->f32{
        floz_pt(x)
    }

    /// Convert fluid ounces to pints.
    #[inline(always)]
    pub const fn fluidounce_pints(x:f32)->f32{
        floz_pt(x)
    }

    /// Convert fluid ounces to pints.
    #[inline(always)]
    pub const fn fluidounces_pt(x:f32)->f32{
        floz_pt(x)
    }

    /// Convert fluid ounces to pints.
    #[inline(always)]
    pub const fn fluidounces_pint(x:f32)->f32{
        floz_pt(x)
    }

    /// Convert fluid ounces to pints.
    #[inline(always)]
    pub const fn fluidounces_pints(x:f32)->f32{
        floz_pt(x)
    }

    /// Convert fluid ounces to quarts.
    #[inline(always)]
    pub const fn floz_quart(x:f32)->f32{
        floz_qt(x)
    }

    /// Convert fluid ounces to quarts.
    #[inline(always)]
    pub const fn floz_quarts(x:f32)->f32{
        floz_qt(x)
    }

    /// Convert fluid ounces to quarts.
    #[inline(always)]
    pub const fn fluidounce_qt(x:f32)->f32{
        floz_qt(x)
    }

    /// Convert fluid ounces to quarts.
    #[inline(always)]
    pub const fn fluidounce_quart(x:f32)->f32{
        floz_qt(x)
    }

    /// Convert fluid ounces to quarts.
    #[inline(always)]
    pub const fn fluidounce_quarts(x:f32)->f32{
        floz_qt(x)
    }

    /// Convert fluid ounces to quarts.
    #[inline(always)]
    pub const fn fluidounces_qt(x:f32)->f32{
        floz_qt(x)
    }

    /// Convert fluid ounces to quarts.
    #[inline(always)]
    pub const fn fluidounces_quart(x:f32)->f32{
        floz_qt(x)
    }

    /// Convert fluid ounces to quarts.
    #[inline(always)]
    pub const fn fluidounces_quarts(x:f32)->f32{
        floz_qt(x)
    }

    /// Convert fluid ounces to gallons.
    #[inline(always)]
    pub const fn floz_gallon(x:f32)->f32{
        floz_gal(x)
    }

    /// Convert fluid ounces to gallons.
    #[inline(always)]
    pub const fn floz_gallons(x:f32)->f32{
        floz_gal(x)
    }

    /// Convert fluid ounces to gallons.
    #[inline(always)]
    pub const fn fluidounce_gal(x:f32)->f32{
        floz_gal(x)
    }

    /// Convert fluid ounces to gallons.
    #[inline(always)]
    pub const fn fluidounce_gallon(x:f32)->f32{
        floz_gal(x)
    }

    /// Convert fluid ounces to gallons.
    #[inline(always)]
    pub const fn fluidounce_gallons(x:f32)->f32{
        floz_gal(x)
    }

    /// Convert fluid ounces to gallons.
    #[inline(always)]
    pub const fn fluidounces_gal(x:f32)->f32{
        floz_gal(x)
    }

    /// Convert fluid ounces to gallons.
    #[inline(always)]
    pub const fn fluidounces_gallon(x:f32)->f32{
        floz_gal(x)
    }

    /// Convert fluid ounces to gallons.
    #[inline(always)]
    pub const fn fluidounces_gallons(x:f32)->f32{
        floz_gal(x)
    }

    /// Convert cups to milliliters.
    #[inline(always)]
    pub const fn cp_milliliter(x:f32)->f32{
        cp_ml(x)
    }

    /// Convert cups to milliliters.
    #[inline(always)]
    pub const fn cp_milliliters(x:f32)->f32{
        cp_ml(x)
    }

    /// Convert cups to milliliters.
    #[inline(always)]
    pub const fn cup_ml(x:f32)->f32{
        cp_ml(x)
    }

    /// Convert cups to milliliters.
    #[inline(always)]
    pub const fn cup_milliliter(x:f32)->f32{
        cp_ml(x)
    }

    /// Convert cups to milliliters.
    #[inline(always)]
    pub const fn cup_milliliters(x:f32)->f32{
        cp_ml(x)
    }

    /// Convert cups to milliliters.
    #[inline(always)]
    pub const fn cups_ml(x:f32)->f32{
        cp_ml(x)
    }

    /// Convert cups to milliliters.
    #[inline(always)]
    pub const fn cups_milliliter(x:f32)->f32{
        cp_ml(x)
    }

    /// Convert cups to milliliters.
    #[inline(always)]
    pub const fn cups_milliliters(x:f32)->f32{
        cp_ml(x)
    }

    /// Convert cups to liters.
    #[inline(always)]
    pub const fn cp_liter(x:f32)->f32{
        cp_l(x)
    }

    /// Convert cups to liters.
    #[inline(always)]
    pub const fn cp_liters(x:f32)->f32{
        cp_l(x)
    }

    /// Convert cups to liters.
    #[inline(always)]
    pub const fn cup_l(x:f32)->f32{
        cp_l(x)
    }

    /// Convert cups to liters.
    #[inline(always)]
    pub const fn cup_liter(x:f32)->f32{
        cp_l(x)
    }

    /// Convert cups to liters.
    #[inline(always)]
    pub const fn cup_liters(x:f32)->f32{
        cp_l(x)
    }

    /// Convert cups to liters.
    #[inline(always)]
    pub const fn cups_l(x:f32)->f32{
        cp_l(x)
    }

    /// Convert cups to liters.
    #[inline(always)]
    pub const fn cups_liter(x:f32)->f32{
        cp_l(x)
    }

    /// Convert cups to liters.
    #[inline(always)]
    pub const fn cups_liters(x:f32)->f32{
        cp_l(x)
    }

    /// Convert cups to fluid ounces.
    #[inline(always)]
    pub const fn cp_fluidounce(x:f32)->f32{
        cp_floz(x)
    }

    /// Convert cups to fluid ounces.
    #[inline(always)]
    pub const fn cp_fluidounces(x:f32)->f32{
        cp_floz(x)
    }

    /// Convert cups to fluid ounces.
    #[inline(always)]
    pub const fn cup_floz(x:f32)->f32{
        cp_floz(x)
    }

    /// Convert cups to fluid ounces.
    #[inline(always)]
    pub const fn cup_fluidounce(x:f32)->f32{
        cp_floz(x)
    }

    /// Convert cups to fluid ounces.
    #[inline(always)]
    pub const fn cup_fluidounces(x:f32)->f32{
        cp_floz(x)
    }

    /// Convert cups to fluid ounces.
    #[inline(always)]
    pub const fn cups_floz(x:f32)->f32{
        cp_floz(x)
    }

    /// Convert cups to fluid ounces.
    #[inline(always)]
    pub const fn cups_fluidounce(x:f32)->f32{
        cp_floz(x)
    }

    /// Convert cups to fluid ounces.
    #[inline(always)]
    pub const fn cups_fluidounces(x:f32)->f32{
        cp_floz(x)
    }

    /// Convert cups to pints.
    #[inline(always)]
    pub const fn cp_pint(x:f32)->f32{
        cp_pt(x)
    }

    /// Convert cups to pints.
    #[inline(always)]
    pub const fn cp_pints(x:f32)->f32{
        cp_pt(x)
    }

    /// Convert cups to pints.
    #[inline(always)]
    pub const fn cup_pt(x:f32)->f32{
        cp_pt(x)
    }

    /// Convert cups to pints.
    #[inline(always)]
    pub const fn cup_pint(x:f32)->f32{
        cp_pt(x)
    }

    /// Convert cups to pints.
    #[inline(always)]
    pub const fn cup_pints(x:f32)->f32{
        cp_pt(x)
    }

    /// Convert cups to pints.
    #[inline(always)]
    pub const fn cups_pt(x:f32)->f32{
        cp_pt(x)
    }

    /// Convert cups to pints.
    #[inline(always)]
    pub const fn cups_pint(x:f32)->f32{
        cp_pt(x)
    }

    /// Convert cups to pints.
    #[inline(always)]
    pub const fn cups_pints(x:f32)->f32{
        cp_pt(x)
    }

    /// Convert cups to quarts.
    #[inline(always)]
    pub const fn cp_quart(x:f32)->f32{
        cp_qt(x)
    }

    /// Convert cups to quarts.
    #[inline(always)]
    pub const fn cp_quarts(x:f32)->f32{
        cp_qt(x)
    }

    /// Convert cups to quarts.
    #[inline(always)]
    pub const fn cup_qt(x:f32)->f32{
        cp_qt(x)
    }

    /// Convert cups to quarts.
    #[inline(always)]
    pub const fn cup_quart(x:f32)->f32{
        cp_qt(x)
    }

    /// Convert cups to quarts.
    #[inline(always)]
    pub const fn cup_quarts(x:f32)->f32{
        cp_qt(x)
    }

    /// Convert cups to quarts.
    #[inline(always)]
    pub const fn cups_qt(x:f32)->f32{
        cp_qt(x)
    }

    /// Convert cups to quarts.
    #[inline(always)]
    pub const fn cups_quart(x:f32)->f32{
        cp_qt(x)
    }

    /// Convert cups to quarts.
    #[inline(always)]
    pub const fn cups_quarts(x:f32)->f32{
        cp_qt(x)
    }

    /// Convert cups to gallons.
    #[inline(always)]
    pub const fn cp_gallon(x:f32)->f32{
        cp_gal(x)
    }

    /// Convert cups to gallons.
    #[inline(always)]
    pub const fn cp_gallons(x:f32)->f32{
        cp_gal(x)
    }

    /// Convert cups to gallons.
    #[inline(always)]
    pub const fn cup_gal(x:f32)->f32{
        cp_gal(x)
    }

    /// Convert cups to gallons.
    #[inline(always)]
    pub const fn cup_gallon(x:f32)->f32{
        cp_gal(x)
    }

    /// Convert cups to gallons.
    #[inline(always)]
    pub const fn cup_gallons(x:f32)->f32{
        cp_gal(x)
    }

    /// Convert cups to gallons.
    #[inline(always)]
    pub const fn cups_gal(x:f32)->f32{
        cp_gal(x)
    }

    /// Convert cups to gallons.
    #[inline(always)]
    pub const fn cups_gallon(x:f32)->f32{
        cp_gal(x)
    }

    /// Convert cups to gallons.
    #[inline(always)]
    pub const fn cups_gallons(x:f32)->f32{
        cp_gal(x)
    }

    /// Convert pints to milliliters.
    #[inline(always)]
    pub const fn pt_milliliter(x:f32)->f32{
        pt_ml(x)
    }

    /// Convert pints to milliliters.
    #[inline(always)]
    pub const fn pt_milliliters(x:f32)->f32{
        pt_ml(x)
    }

    /// Convert pints to milliliters.
    #[inline(always)]
    pub const fn pint_ml(x:f32)->f32{
        pt_ml(x)
    }

    /// Convert pints to milliliters.
    #[inline(always)]
    pub const fn pint_milliliter(x:f32)->f32{
        pt_ml(x)
    }

    /// Convert pints to milliliters.
    #[inline(always)]
    pub const fn pint_milliliters(x:f32)->f32{
        pt_ml(x)
    }

    /// Convert pints to milliliters.
    #[inline(always)]
    pub const fn pints_ml(x:f32)->f32{
        pt_ml(x)
    }

    /// Convert pints to milliliters.
    #[inline(always)]
    pub const fn pints_milliliter(x:f32)->f32{
        pt_ml(x)
    }

    /// Convert pints to milliliters.
    #[inline(always)]
    pub const fn pints_milliliters(x:f32)->f32{
        pt_ml(x)
    }

    /// Convert pints to liters.
    #[inline(always)]
    pub const fn pt_liter(x:f32)->f32{
        pt_l(x)
    }

    /// Convert pints to liters.
    #[inline(always)]
    pub const fn pt_liters(x:f32)->f32{
        pt_l(x)
    }

    /// Convert pints to liters.
    #[inline(always)]
    pub const fn pint_l(x:f32)->f32{
        pt_l(x)
    }

    /// Convert pints to liters.
    #[inline(always)]
    pub const fn pint_liter(x:f32)->f32{
        pt_l(x)
    }

    /// Convert pints to liters.
    #[inline(always)]
    pub const fn pint_liters(x:f32)->f32{
        pt_l(x)
    }

    /// Convert pints to liters.
    #[inline(always)]
    pub const fn pints_l(x:f32)->f32{
        pt_l(x)
    }

    /// Convert pints to liters.
    #[inline(always)]
    pub const fn pints_liter(x:f32)->f32{
        pt_l(x)
    }

    /// Convert pints to liters.
    #[inline(always)]
    pub const fn pints_liters(x:f32)->f32{
        pt_l(x)
    }

    /// Convert pints to fluid ounces.
    #[inline(always)]
    pub const fn pt_fluidounce(x:f32)->f32{
        pt_floz(x)
    }

    /// Convert pints to fluid ounces.
    #[inline(always)]
    pub const fn pt_fluidounces(x:f32)->f32{
        pt_floz(x)
    }

    /// Convert pints to fluid ounces.
    #[inline(always)]
    pub const fn pint_floz(x:f32)->f32{
        pt_floz(x)
    }

    /// Convert pints to fluid ounces.
    #[inline(always)]
    pub const fn pint_fluidounce(x:f32)->f32{
        pt_floz(x)
    }

    /// Convert pints to fluid ounces.
    #[inline(always)]
    pub const fn pint_fluidounces(x:f32)->f32{
        pt_floz(x)
    }

    /// Convert pints to fluid ounces.
    #[inline(always)]
    pub const fn pints_floz(x:f32)->f32{
        pt_floz(x)
    }

    /// Convert pints to fluid ounces.
    #[inline(always)]
    pub const fn pints_fluidounce(x:f32)->f32{
        pt_floz(x)
    }

    /// Convert pints to fluid ounces.
    #[inline(always)]
    pub const fn pints_fluidounces(x:f32)->f32{
        pt_floz(x)
    }

    /// Convert pints to cups.
    #[inline(always)]
    pub const fn pt_cup(x:f32)->f32{
        pt_cp(x)
    }

    /// Convert pints to cups.
    #[inline(always)]
    pub const fn pt_cups(x:f32)->f32{
        pt_cp(x)
    }

    /// Convert pints to cups.
    #[inline(always)]
    pub const fn pint_cp(x:f32)->f32{
        pt_cp(x)
    }

    /// Convert pints to cups.
    #[inline(always)]
    pub const fn pint_cup(x:f32)->f32{
        pt_cp(x)
    }

    /// Convert pints to cups.
    #[inline(always)]
    pub const fn pint_cups(x:f32)->f32{
        pt_cp(x)
    }

    /// Convert pints to cups.
    #[inline(always)]
    pub const fn pints_cp(x:f32)->f32{
        pt_cp(x)
    }

    /// Convert pints to cups.
    #[inline(always)]
    pub const fn pints_cup(x:f32)->f32{
        pt_cp(x)
    }

    /// Convert pints to cups.
    #[inline(always)]
    pub const fn pints_cups(x:f32)->f32{
        pt_cp(x)
    }

    /// Convert pints to quarts.
    #[inline(always)]
    pub const fn pt_quart(x:f32)->f32{
        pt_qt(x)
    }

    /// Convert pints to quarts.
    #[inline(always)]
    pub const fn pt_quarts(x:f32)->f32{
        pt_qt(x)
    }

    /// Convert pints to quarts.
    #[inline(always)]
    pub const fn pint_qt(x:f32)->f32{
        pt_qt(x)
    }

    /// Convert pints to quarts.
    #[inline(always)]
    pub const fn pint_quart(x:f32)->f32{
        pt_qt(x)
    }

    /// Convert pints to quarts.
    #[inline(always)]
    pub const fn pint_quarts(x:f32)->f32{
        pt_qt(x)
    }

    /// Convert pints to quarts.
    #[inline(always)]
    pub const fn pints_qt(x:f32)->f32{
        pt_qt(x)
    }

    /// Convert pints to quarts.
    #[inline(always)]
    pub const fn pints_quart(x:f32)->f32{
        pt_qt(x)
    }

    /// Convert pints to quarts.
    #[inline(always)]
    pub const fn pints_quarts(x:f32)->f32{
        pt_qt(x)
    }

    /// Convert pints to gallons.
    #[inline(always)]
    pub const fn pt_gallon(x:f32)->f32{
        pt_gal(x)
    }

    /// Convert pints to gallons.
    #[inline(always)]
    pub const fn pt_gallons(x:f32)->f32{
        pt_gal(x)
    }

    /// Convert pints to gallons.
    #[inline(always)]
    pub const fn pint_gal(x:f32)->f32{
        pt_gal(x)
    }

    /// Convert pints to gallons.
    #[inline(always)]
    pub const fn pint_gallon(x:f32)->f32{
        pt_gal(x)
    }

    /// Convert pints to gallons.
    #[inline(always)]
    pub const fn pint_gallons(x:f32)->f32{
        pt_gal(x)
    }

    /// Convert pints to gallons.
    #[inline(always)]
    pub const fn pints_gal(x:f32)->f32{
        pt_gal(x)
    }

    /// Convert pints to gallons.
    #[inline(always)]
    pub const fn pints_gallon(x:f32)->f32{
        pt_gal(x)
    }

    /// Convert pints to gallons.
    #[inline(always)]
    pub const fn pints_gallons(x:f32)->f32{
        pt_gal(x)
    }

    /// Convert quarts to milliliters.
    #[inline(always)]
    pub const fn qt_milliliter(x:f32)->f32{
        qt_ml(x)
    }

    /// Convert quarts to milliliters.
    #[inline(always)]
    pub const fn qt_milliliters(x:f32)->f32{
        qt_ml(x)
    }

    /// Convert quarts to milliliters.
    #[inline(always)]
    pub const fn quart_ml(x:f32)->f32{
        qt_ml(x)
    }

    /// Convert quarts to milliliters.
    #[inline(always)]
    pub const fn quart_milliliter(x:f32)->f32{
        qt_ml(x)
    }

    /// Convert quarts to milliliters.
    #[inline(always)]
    pub const fn quart_milliliters(x:f32)->f32{
        qt_ml(x)
    }

    /// Convert quarts to milliliters.
    #[inline(always)]
    pub const fn quarts_ml(x:f32)->f32{
        qt_ml(x)
    }

    /// Convert quarts to milliliters.
    #[inline(always)]
    pub const fn quarts_milliliter(x:f32)->f32{
        qt_ml(x)
    }

    /// Convert quarts to milliliters.
    #[inline(always)]
    pub const fn quarts_milliliters(x:f32)->f32{
        qt_ml(x)
    }

    /// Convert quarts to liters.
    #[inline(always)]
    pub const fn qt_liter(x:f32)->f32{
        qt_l(x)
    }

    /// Convert quarts to liters.
    #[inline(always)]
    pub const fn qt_liters(x:f32)->f32{
        qt_l(x)
    }

    /// Convert quarts to liters.
    #[inline(always)]
    pub const fn quart_l(x:f32)->f32{
        qt_l(x)
    }

    /// Convert quarts to liters.
    #[inline(always)]
    pub const fn quart_liter(x:f32)->f32{
        qt_l(x)
    }

    /// Convert quarts to liters.
    #[inline(always)]
    pub const fn quart_liters(x:f32)->f32{
        qt_l(x)
    }

    /// Convert quarts to liters.
    #[inline(always)]
    pub const fn quarts_l(x:f32)->f32{
        qt_l(x)
    }

    /// Convert quarts to liters.
    #[inline(always)]
    pub const fn quarts_liter(x:f32)->f32{
        qt_l(x)
    }

    /// Convert quarts to liters.
    #[inline(always)]
    pub const fn quarts_liters(x:f32)->f32{
        qt_l(x)
    }

    /// Convert quarts to fluid ounces.
    #[inline(always)]
    pub const fn qt_fluidounce(x:f32)->f32{
        qt_floz(x)
    }

    /// Convert quarts to fluid ounces.
    #[inline(always)]
    pub const fn qt_fluidounces(x:f32)->f32{
        qt_floz(x)
    }

    /// Convert quarts to fluid ounces.
    #[inline(always)]
    pub const fn quart_floz(x:f32)->f32{
        qt_floz(x)
    }

    /// Convert quarts to fluid ounces.
    #[inline(always)]
    pub const fn quart_fluidounce(x:f32)->f32{
        qt_floz(x)
    }

    /// Convert quarts to fluid ounces.
    #[inline(always)]
    pub const fn quart_fluidounces(x:f32)->f32{
        qt_floz(x)
    }

    /// Convert quarts to fluid ounces.
    #[inline(always)]
    pub const fn quarts_floz(x:f32)->f32{
        qt_floz(x)
    }

    /// Convert quarts to fluid ounces.
    #[inline(always)]
    pub const fn quarts_fluidounce(x:f32)->f32{
        qt_floz(x)
    }

    /// Convert quarts to fluid ounces.
    #[inline(always)]
    pub const fn quarts_fluidounces(x:f32)->f32{
        qt_floz(x)
    }

    /// Convert quarts to cups.
    #[inline(always)]
    pub const fn qt_cup(x:f32)->f32{
        qt_cp(x)
    }

    /// Convert quarts to cups.
    #[inline(always)]
    pub const fn qt_cups(x:f32)->f32{
        qt_cp(x)
    }

    /// Convert quarts to cups.
    #[inline(always)]
    pub const fn quart_cp(x:f32)->f32{
        qt_cp(x)
    }

    /// Convert quarts to cups.
    #[inline(always)]
    pub const fn quart_cup(x:f32)->f32{
        qt_cp(x)
    }

    /// Convert quarts to cups.
    #[inline(always)]
    pub const fn quart_cups(x:f32)->f32{
        qt_cp(x)
    }

    /// Convert quarts to cups.
    #[inline(always)]
    pub const fn quarts_cp(x:f32)->f32{
        qt_cp(x)
    }

    /// Convert quarts to cups.
    #[inline(always)]
    pub const fn quarts_cup(x:f32)->f32{
        qt_cp(x)
    }

    /// Convert quarts to cups.
    #[inline(always)]
    pub const fn quarts_cups(x:f32)->f32{
        qt_cp(x)
    }

    /// Convert quarts to pints.
    #[inline(always)]
    pub const fn qt_pint(x:f32)->f32{
        qt_pt(x)
    }

    /// Convert quarts to pints.
    #[inline(always)]
    pub const fn qt_pints(x:f32)->f32{
        qt_pt(x)
    }

    /// Convert quarts to pints.
    #[inline(always)]
    pub const fn quart_pt(x:f32)->f32{
        qt_pt(x)
    }

    /// Convert quarts to pints.
    #[inline(always)]
    pub const fn quart_pint(x:f32)->f32{
        qt_pt(x)
    }

    /// Convert quarts to pints.
    #[inline(always)]
    pub const fn quart_pints(x:f32)->f32{
        qt_pt(x)
    }

    /// Convert quarts to pints.
    #[inline(always)]
    pub const fn quarts_pt(x:f32)->f32{
        qt_pt(x)
    }

    /// Convert quarts to pints.
    #[inline(always)]
    pub const fn quarts_pint(x:f32)->f32{
        qt_pt(x)
    }

    /// Convert quarts to pints.
    #[inline(always)]
    pub const fn quarts_pints(x:f32)->f32{
        qt_pt(x)
    }

    /// Convert quarts to gallons.
    #[inline(always)]
    pub const fn qt_gallon(x:f32)->f32{
        qt_gal(x)
    }

    /// Convert quarts to gallons.
    #[inline(always)]
    pub const fn qt_gallons(x:f32)->f32{
        qt_gal(x)
    }

    /// Convert quarts to gallons.
    #[inline(always)]
    pub const fn quart_gal(x:f32)->f32{
        qt_gal(x)
    }

    /// Convert quarts to gallons.
    #[inline(always)]
    pub const fn quart_gallon(x:f32)->f32{
        qt_gal(x)
    }

    /// Convert quarts to gallons.
    #[inline(always)]
    pub const fn quart_gallons(x:f32)->f32{
        qt_gal(x)
    }

    /// Convert quarts to gallons.
    #[inline(always)]
    pub const fn quarts_gal(x:f32)->f32{
        qt_gal(x)
    }

    /// Convert quarts to gallons.
    #[inline(always)]
    pub const fn quarts_gallon(x:f32)->f32{
        qt_gal(x)
    }

    /// Convert quarts to gallons.
    #[inline(always)]
    pub const fn quarts_gallons(x:f32)->f32{
        qt_gal(x)
    }

    /// Convert gallons to milliliters.
    #[inline(always)]
    pub const fn gal_milliliter(x:f32)->f32{
        gal_ml(x)
    }

    /// Convert gallons to milliliters.
    #[inline(always)]
    pub const fn gal_milliliters(x:f32)->f32{
        gal_ml(x)
    }

    /// Convert gallons to milliliters.
    #[inline(always)]
    pub const fn gallon_ml(x:f32)->f32{
        gal_ml(x)
    }

    /// Convert gallons to milliliters.
    #[inline(always)]
    pub const fn gallon_milliliter(x:f32)->f32{
        gal_ml(x)
    }

    /// Convert gallons to milliliters.
    #[inline(always)]
    pub const fn gallon_milliliters(x:f32)->f32{
        gal_ml(x)
    }

    /// Convert gallons to milliliters.
    #[inline(always)]
    pub const fn gallons_ml(x:f32)->f32{
        gal_ml(x)
    }

    /// Convert gallons to milliliters.
    #[inline(always)]
    pub const fn gallons_milliliter(x:f32)->f32{
        gal_ml(x)
    }

    /// Convert gallons to milliliters.
    #[inline(always)]
    pub const fn gallons_milliliters(x:f32)->f32{
        gal_ml(x)
    }

    /// Convert gallons to liters.
    #[inline(always)]
    pub const fn gal_liter(x:f32)->f32{
        gal_l(x)
    }

    /// Convert gallons to liters.
    #[inline(always)]
    pub const fn gal_liters(x:f32)->f32{
        gal_l(x)
    }

    /// Convert gallons to liters.
    #[inline(always)]
    pub const fn gallon_l(x:f32)->f32{
        gal_l(x)
    }

    /// Convert gallons to liters.
    #[inline(always)]
    pub const fn gallon_liter(x:f32)->f32{
        gal_l(x)
    }

    /// Convert gallons to liters.
    #[inline(always)]
    pub const fn gallon_liters(x:f32)->f32{
        gal_l(x)
    }

    /// Convert gallons to liters.
    #[inline(always)]
    pub const fn gallons_l(x:f32)->f32{
        gal_l(x)
    }

    /// Convert gallons to liters.
    #[inline(always)]
    pub const fn gallons_liter(x:f32)->f32{
        gal_l(x)
    }

    /// Convert gallons to liters.
    #[inline(always)]
    pub const fn gallons_liters(x:f32)->f32{
        gal_l(x)
    }

    /// Convert gallons to fluid ounces.
    #[inline(always)]
    pub const fn gal_fluidounce(x:f32)->f32{
        gal_floz(x)
    }

    /// Convert gallons to fluid ounces.
    #[inline(always)]
    pub const fn gal_fluidounces(x:f32)->f32{
        gal_floz(x)
    }

    /// Convert gallons to fluid ounces.
    #[inline(always)]
    pub const fn gallon_floz(x:f32)->f32{
        gal_floz(x)
    }

    /// Convert gallons to fluid ounces.
    #[inline(always)]
    pub const fn gallon_fluidounce(x:f32)->f32{
        gal_floz(x)
    }

    /// Convert gallons to fluid ounces.
    #[inline(always)]
    pub const fn gallon_fluidounces(x:f32)->f32{
        gal_floz(x)
    }

    /// Convert gallons to fluid ounces.
    #[inline(always)]
    pub const fn gallons_floz(x:f32)->f32{
        gal_floz(x)
    }

    /// Convert gallons to fluid ounces.
    #[inline(always)]
    pub const fn gallons_fluidounce(x:f32)->f32{
        gal_floz(x)
    }

    /// Convert gallons to fluid ounces.
    #[inline(always)]
    pub const fn gallons_fluidounces(x:f32)->f32{
        gal_floz(x)
    }

    /// Convert gallons to cups.
    #[inline(always)]
    pub const fn gal_cup(x:f32)->f32{
        gal_cp(x)
    }

    /// Convert gallons to cups.
    #[inline(always)]
    pub const fn gal_cups(x:f32)->f32{
        gal_cp(x)
    }

    /// Convert gallons to cups.
    #[inline(always)]
    pub const fn gallon_cp(x:f32)->f32{
        gal_cp(x)
    }

    /// Convert gallons to cups.
    #[inline(always)]
    pub const fn gallon_cup(x:f32)->f32{
        gal_cp(x)
    }

    /// Convert gallons to cups.
    #[inline(always)]
    pub const fn gallon_cups(x:f32)->f32{
        gal_cp(x)
    }

    /// Convert gallons to cups.
    #[inline(always)]
    pub const fn gallons_cp(x:f32)->f32{
        gal_cp(x)
    }

    /// Convert gallons to cups.
    #[inline(always)]
    pub const fn gallons_cup(x:f32)->f32{
        gal_cp(x)
    }

    /// Convert gallons to cups.
    #[inline(always)]
    pub const fn gallons_cups(x:f32)->f32{
        gal_cp(x)
    }

    /// Convert gallons to pints.
    #[inline(always)]
    pub const fn gal_pint(x:f32)->f32{
        gal_pt(x)
    }

    /// Convert gallons to pints.
    #[inline(always)]
    pub const fn gal_pints(x:f32)->f32{
        gal_pt(x)
    }

    /// Convert gallons to pints.
    #[inline(always)]
    pub const fn gallon_pt(x:f32)->f32{
        gal_pt(x)
    }

    /// Convert gallons to pints.
    #[inline(always)]
    pub const fn gallon_pint(x:f32)->f32{
        gal_pt(x)
    }

    /// Convert gallons to pints.
    #[inline(always)]
    pub const fn gallon_pints(x:f32)->f32{
        gal_pt(x)
    }

    /// Convert gallons to pints.
    #[inline(always)]
    pub const fn gallons_pt(x:f32)->f32{
        gal_pt(x)
    }

    /// Convert gallons to pints.
    #[inline(always)]
    pub const fn gallons_pint(x:f32)->f32{
        gal_pt(x)
    }

    /// Convert gallons to pints.
    #[inline(always)]
    pub const fn gallons_pints(x:f32)->f32{
        gal_pt(x)
    }

    /// Convert gallons to quarts.
    #[inline(always)]
    pub const fn gal_quart(x:f32)->f32{
        gal_qt(x)
    }

    /// Convert gallons to quarts.
    #[inline(always)]
    pub const fn gal_quarts(x:f32)->f32{
        gal_qt(x)
    }

    /// Convert gallons to quarts.
    #[inline(always)]
    pub const fn gallon_qt(x:f32)->f32{
        gal_qt(x)
    }

    /// Convert gallons to quarts.
    #[inline(always)]
    pub const fn gallon_quart(x:f32)->f32{
        gal_qt(x)
    }

    /// Convert gallons to quarts.
    #[inline(always)]
    pub const fn gallon_quarts(x:f32)->f32{
        gal_qt(x)
    }

    /// Convert gallons to quarts.
    #[inline(always)]
    pub const fn gallons_qt(x:f32)->f32{
        gal_qt(x)
    }

    /// Convert gallons to quarts.
    #[inline(always)]
    pub const fn gallons_quart(x:f32)->f32{
        gal_qt(x)
    }

    /// Convert gallons to quarts.
    #[inline(always)]
    pub const fn gallons_quarts(x:f32)->f32{
        gal_qt(x)
    }
}

pub mod speed{
    /// Convert meters per second to kilometers per hour.
    #[inline]
    pub const fn mps_kmh(x:f32)->f32{
        x*3.59999999999997
    }

    /// Convert meters per second to miles per hour.
    #[inline]
    pub const fn mps_mph(x:f32)->f32{
        x*2.2369362920544
    }

    /// Convert meters per second to knots.
    #[inline]
    pub const fn mps_kt(x:f32)->f32{
        x*1.94384449244062
    }

    /// Convert meters per second to mach.
    #[inline]
    pub const fn mps_mach(x:f32)->f32{
        x*0.00291545189504
    }

    /// Convert kilometers per hour to meters per second.
    #[inline]
    pub const fn kmh_mps(x:f32)->f32{
        x*0.27777777777778
    }

    /// Convert kilometers per hour to miles per hour.
    #[inline]
    pub const fn kmh_mph(x:f32)->f32{
        x*0.62137119223734
    }

    /// Convert kilometers per hour to knots.
    #[inline]
    pub const fn kmh_kt(x:f32)->f32{
        x*0.53995680345573
    }

    /// Convert kilometers per hour to mach.
    #[inline]
    pub const fn kmh_mach(x:f32)->f32{
        x*0.00080984774862
    }

    /// Convert miles per hour to meters per second.
    #[inline]
    pub const fn mph_mps(x:f32)->f32{
        x*0.44704
    }

    /// Convert miles per hour to kilometers per hour.
    #[inline]
    pub const fn mph_kmh(x:f32)->f32{
        x*1.60934399999999
    }

    /// Convert miles per hour to knots.
    #[inline]
    pub const fn mph_kt(x:f32)->f32{
        x*0.86897624190066
    }

    /// Convert miles per hour to mach.
    #[inline]
    pub const fn mph_mach(x:f32)->f32{
        x*0.00130332361516
    }

    /// Convert knots to meters per second.
    #[inline]
    pub const fn kt_mps(x:f32)->f32{
        x*0.51444444444444
    }

    /// Convert knots to kilometers per hour.
    #[inline]
    pub const fn kt_kmh(x:f32)->f32{
        x*1.85199999999997
    }

    /// Convert knots to miles per hour.
    #[inline]
    pub const fn kt_mph(x:f32)->f32{
        x*1.15077944802353
    }

    /// Convert knots to mach.
    #[inline]
    pub const fn kt_mach(x:f32)->f32{
        x*0.00149983803045
    }

    /// Convert mach to meters per second.
    #[inline]
    pub const fn mach_mps(x:f32)->f32{
        x*343.0
    }

    /// Convert mach to kilometers per hour.
    #[inline]
    pub const fn mach_kmh(x:f32)->f32{
        x*1234.79999999999018
    }

    /// Convert mach to miles per hour.
    #[inline]
    pub const fn mach_mph(x:f32)->f32{
        x*767.26914817466002
    }

    /// Convert mach to knots.
    #[inline]
    pub const fn mach_kt(x:f32)->f32{
        x*666.73866090713318
    }

    /// Convert meters per second to kilometers per hour.
    #[inline(always)]
    pub const fn mps_kilometershour(x:f32)->f32{
        mps_kmh(x)
    }

    /// Convert meters per second to kilometers per hour.
    #[inline(always)]
    pub const fn mps_kilometerhour(x:f32)->f32{
        mps_kmh(x)
    }

    /// Convert meters per second to kilometers per hour.
    #[inline(always)]
    pub const fn mps_kilometerperhour(x:f32)->f32{
        mps_kmh(x)
    }

    /// Convert meters per second to kilometers per hour.
    #[inline(always)]
    pub const fn mps_kilometersperhour(x:f32)->f32{
        mps_kmh(x)
    }

    /// Convert meters per second to kilometers per hour.
    #[inline(always)]
    pub const fn meterssecond_kmh(x:f32)->f32{
        mps_kmh(x)
    }

    /// Convert meters per second to kilometers per hour.
    #[inline(always)]
    pub const fn meterssecond_kilometershour(x:f32)->f32{
        mps_kmh(x)
    }

    /// Convert meters per second to kilometers per hour.
    #[inline(always)]
    pub const fn meterssecond_kilometerhour(x:f32)->f32{
        mps_kmh(x)
    }

    /// Convert meters per second to kilometers per hour.
    #[inline(always)]
    pub const fn meterssecond_kilometerperhour(x:f32)->f32{
        mps_kmh(x)
    }

    /// Convert meters per second to kilometers per hour.
    #[inline(always)]
    pub const fn meterssecond_kilometersperhour(x:f32)->f32{
        mps_kmh(x)
    }

    /// Convert meters per second to kilometers per hour.
    #[inline(always)]
    pub const fn metersecond_kmh(x:f32)->f32{
        mps_kmh(x)
    }

    /// Convert meters per second to kilometers per hour.
    #[inline(always)]
    pub const fn metersecond_kilometershour(x:f32)->f32{
        mps_kmh(x)
    }

    /// Convert meters per second to kilometers per hour.
    #[inline(always)]
    pub const fn metersecond_kilometerhour(x:f32)->f32{
        mps_kmh(x)
    }

    /// Convert meters per second to kilometers per hour.
    #[inline(always)]
    pub const fn metersecond_kilometerperhour(x:f32)->f32{
        mps_kmh(x)
    }

    /// Convert meters per second to kilometers per hour.
    #[inline(always)]
    pub const fn metersecond_kilometersperhour(x:f32)->f32{
        mps_kmh(x)
    }

    /// Convert meters per second to kilometers per hour.
    #[inline(always)]
    pub const fn meterpersecond_kmh(x:f32)->f32{
        mps_kmh(x)
    }

    /// Convert meters per second to kilometers per hour.
    #[inline(always)]
    pub const fn meterpersecond_kilometershour(x:f32)->f32{
        mps_kmh(x)
    }

    /// Convert meters per second to kilometers per hour.
    #[inline(always)]
    pub const fn meterpersecond_kilometerhour(x:f32)->f32{
        mps_kmh(x)
    }

    /// Convert meters per second to kilometers per hour.
    #[inline(always)]
    pub const fn meterpersecond_kilometerperhour(x:f32)->f32{
        mps_kmh(x)
    }

    /// Convert meters per second to kilometers per hour.
    #[inline(always)]
    pub const fn meterpersecond_kilometersperhour(x:f32)->f32{
        mps_kmh(x)
    }

    /// Convert meters per second to kilometers per hour.
    #[inline(always)]
    pub const fn meterspersecond_kmh(x:f32)->f32{
        mps_kmh(x)
    }

    /// Convert meters per second to kilometers per hour.
    #[inline(always)]
    pub const fn meterspersecond_kilometershour(x:f32)->f32{
        mps_kmh(x)
    }

    /// Convert meters per second to kilometers per hour.
    #[inline(always)]
    pub const fn meterspersecond_kilometerhour(x:f32)->f32{
        mps_kmh(x)
    }

    /// Convert meters per second to kilometers per hour.
    #[inline(always)]
    pub const fn meterspersecond_kilometerperhour(x:f32)->f32{
        mps_kmh(x)
    }

    /// Convert meters per second to kilometers per hour.
    #[inline(always)]
    pub const fn meterspersecond_kilometersperhour(x:f32)->f32{
        mps_kmh(x)
    }

    /// Convert meters per second to miles per hour.
    #[inline(always)]
    pub const fn mps_mileshour(x:f32)->f32{
        mps_mph(x)
    }

    /// Convert meters per second to miles per hour.
    #[inline(always)]
    pub const fn mps_milehour(x:f32)->f32{
        mps_mph(x)
    }

    /// Convert meters per second to miles per hour.
    #[inline(always)]
    pub const fn mps_mileperhour(x:f32)->f32{
        mps_mph(x)
    }

    /// Convert meters per second to miles per hour.
    #[inline(always)]
    pub const fn mps_milesperhour(x:f32)->f32{
        mps_mph(x)
    }

    /// Convert meters per second to miles per hour.
    #[inline(always)]
    pub const fn meterssecond_mph(x:f32)->f32{
        mps_mph(x)
    }

    /// Convert meters per second to miles per hour.
    #[inline(always)]
    pub const fn meterssecond_mileshour(x:f32)->f32{
        mps_mph(x)
    }

    /// Convert meters per second to miles per hour.
    #[inline(always)]
    pub const fn meterssecond_milehour(x:f32)->f32{
        mps_mph(x)
    }

    /// Convert meters per second to miles per hour.
    #[inline(always)]
    pub const fn meterssecond_mileperhour(x:f32)->f32{
        mps_mph(x)
    }

    /// Convert meters per second to miles per hour.
    #[inline(always)]
    pub const fn meterssecond_milesperhour(x:f32)->f32{
        mps_mph(x)
    }

    /// Convert meters per second to miles per hour.
    #[inline(always)]
    pub const fn metersecond_mph(x:f32)->f32{
        mps_mph(x)
    }

    /// Convert meters per second to miles per hour.
    #[inline(always)]
    pub const fn metersecond_mileshour(x:f32)->f32{
        mps_mph(x)
    }

    /// Convert meters per second to miles per hour.
    #[inline(always)]
    pub const fn metersecond_milehour(x:f32)->f32{
        mps_mph(x)
    }

    /// Convert meters per second to miles per hour.
    #[inline(always)]
    pub const fn metersecond_mileperhour(x:f32)->f32{
        mps_mph(x)
    }

    /// Convert meters per second to miles per hour.
    #[inline(always)]
    pub const fn metersecond_milesperhour(x:f32)->f32{
        mps_mph(x)
    }

    /// Convert meters per second to miles per hour.
    #[inline(always)]
    pub const fn meterpersecond_mph(x:f32)->f32{
        mps_mph(x)
    }

    /// Convert meters per second to miles per hour.
    #[inline(always)]
    pub const fn meterpersecond_mileshour(x:f32)->f32{
        mps_mph(x)
    }

    /// Convert meters per second to miles per hour.
    #[inline(always)]
    pub const fn meterpersecond_milehour(x:f32)->f32{
        mps_mph(x)
    }

    /// Convert meters per second to miles per hour.
    #[inline(always)]
    pub const fn meterpersecond_mileperhour(x:f32)->f32{
        mps_mph(x)
    }

    /// Convert meters per second to miles per hour.
    #[inline(always)]
    pub const fn meterpersecond_milesperhour(x:f32)->f32{
        mps_mph(x)
    }

    /// Convert meters per second to miles per hour.
    #[inline(always)]
    pub const fn meterspersecond_mph(x:f32)->f32{
        mps_mph(x)
    }

    /// Convert meters per second to miles per hour.
    #[inline(always)]
    pub const fn meterspersecond_mileshour(x:f32)->f32{
        mps_mph(x)
    }

    /// Convert meters per second to miles per hour.
    #[inline(always)]
    pub const fn meterspersecond_milehour(x:f32)->f32{
        mps_mph(x)
    }

    /// Convert meters per second to miles per hour.
    #[inline(always)]
    pub const fn meterspersecond_mileperhour(x:f32)->f32{
        mps_mph(x)
    }

    /// Convert meters per second to miles per hour.
    #[inline(always)]
    pub const fn meterspersecond_milesperhour(x:f32)->f32{
        mps_mph(x)
    }

    /// Convert meters per second to knots.
    #[inline(always)]
    pub const fn mps_knot(x:f32)->f32{
        mps_kt(x)
    }

    /// Convert meters per second to knots.
    #[inline(always)]
    pub const fn mps_knots(x:f32)->f32{
        mps_kt(x)
    }

    /// Convert meters per second to knots.
    #[inline(always)]
    pub const fn meterssecond_kt(x:f32)->f32{
        mps_kt(x)
    }

    /// Convert meters per second to knots.
    #[inline(always)]
    pub const fn meterssecond_knot(x:f32)->f32{
        mps_kt(x)
    }

    /// Convert meters per second to knots.
    #[inline(always)]
    pub const fn meterssecond_knots(x:f32)->f32{
        mps_kt(x)
    }

    /// Convert meters per second to knots.
    #[inline(always)]
    pub const fn metersecond_kt(x:f32)->f32{
        mps_kt(x)
    }

    /// Convert meters per second to knots.
    #[inline(always)]
    pub const fn metersecond_knot(x:f32)->f32{
        mps_kt(x)
    }

    /// Convert meters per second to knots.
    #[inline(always)]
    pub const fn metersecond_knots(x:f32)->f32{
        mps_kt(x)
    }

    /// Convert meters per second to knots.
    #[inline(always)]
    pub const fn meterpersecond_kt(x:f32)->f32{
        mps_kt(x)
    }

    /// Convert meters per second to knots.
    #[inline(always)]
    pub const fn meterpersecond_knot(x:f32)->f32{
        mps_kt(x)
    }

    /// Convert meters per second to knots.
    #[inline(always)]
    pub const fn meterpersecond_knots(x:f32)->f32{
        mps_kt(x)
    }

    /// Convert meters per second to knots.
    #[inline(always)]
    pub const fn meterspersecond_kt(x:f32)->f32{
        mps_kt(x)
    }

    /// Convert meters per second to knots.
    #[inline(always)]
    pub const fn meterspersecond_knot(x:f32)->f32{
        mps_kt(x)
    }

    /// Convert meters per second to knots.
    #[inline(always)]
    pub const fn meterspersecond_knots(x:f32)->f32{
        mps_kt(x)
    }

    /// Convert meters per second to mach.
    #[inline(always)]
    pub const fn meterssecond_mach(x:f32)->f32{
        mps_mach(x)
    }

    /// Convert meters per second to mach.
    #[inline(always)]
    pub const fn metersecond_mach(x:f32)->f32{
        mps_mach(x)
    }

    /// Convert meters per second to mach.
    #[inline(always)]
    pub const fn meterpersecond_mach(x:f32)->f32{
        mps_mach(x)
    }

    /// Convert meters per second to mach.
    #[inline(always)]
    pub const fn meterspersecond_mach(x:f32)->f32{
        mps_mach(x)
    }

    /// Convert kilometers per hour to meters per second.
    #[inline(always)]
    pub const fn kmh_meterssecond(x:f32)->f32{
        kmh_mps(x)
    }

    /// Convert kilometers per hour to meters per second.
    #[inline(always)]
    pub const fn kmh_metersecond(x:f32)->f32{
        kmh_mps(x)
    }

    /// Convert kilometers per hour to meters per second.
    #[inline(always)]
    pub const fn kmh_meterpersecond(x:f32)->f32{
        kmh_mps(x)
    }

    /// Convert kilometers per hour to meters per second.
    #[inline(always)]
    pub const fn kmh_meterspersecond(x:f32)->f32{
        kmh_mps(x)
    }

    /// Convert kilometers per hour to meters per second.
    #[inline(always)]
    pub const fn kilometershour_mps(x:f32)->f32{
        kmh_mps(x)
    }

    /// Convert kilometers per hour to meters per second.
    #[inline(always)]
    pub const fn kilometershour_meterssecond(x:f32)->f32{
        kmh_mps(x)
    }

    /// Convert kilometers per hour to meters per second.
    #[inline(always)]
    pub const fn kilometershour_metersecond(x:f32)->f32{
        kmh_mps(x)
    }

    /// Convert kilometers per hour to meters per second.
    #[inline(always)]
    pub const fn kilometershour_meterpersecond(x:f32)->f32{
        kmh_mps(x)
    }

    /// Convert kilometers per hour to meters per second.
    #[inline(always)]
    pub const fn kilometershour_meterspersecond(x:f32)->f32{
        kmh_mps(x)
    }

    /// Convert kilometers per hour to meters per second.
    #[inline(always)]
    pub const fn kilometerhour_mps(x:f32)->f32{
        kmh_mps(x)
    }

    /// Convert kilometers per hour to meters per second.
    #[inline(always)]
    pub const fn kilometerhour_meterssecond(x:f32)->f32{
        kmh_mps(x)
    }

    /// Convert kilometers per hour to meters per second.
    #[inline(always)]
    pub const fn kilometerhour_metersecond(x:f32)->f32{
        kmh_mps(x)
    }

    /// Convert kilometers per hour to meters per second.
    #[inline(always)]
    pub const fn kilometerhour_meterpersecond(x:f32)->f32{
        kmh_mps(x)
    }

    /// Convert kilometers per hour to meters per second.
    #[inline(always)]
    pub const fn kilometerhour_meterspersecond(x:f32)->f32{
        kmh_mps(x)
    }

    /// Convert kilometers per hour to meters per second.
    #[inline(always)]
    pub const fn kilometerperhour_mps(x:f32)->f32{
        kmh_mps(x)
    }

    /// Convert kilometers per hour to meters per second.
    #[inline(always)]
    pub const fn kilometerperhour_meterssecond(x:f32)->f32{
        kmh_mps(x)
    }

    /// Convert kilometers per hour to meters per second.
    #[inline(always)]
    pub const fn kilometerperhour_metersecond(x:f32)->f32{
        kmh_mps(x)
    }

    /// Convert kilometers per hour to meters per second.
    #[inline(always)]
    pub const fn kilometerperhour_meterpersecond(x:f32)->f32{
        kmh_mps(x)
    }

    /// Convert kilometers per hour to meters per second.
    #[inline(always)]
    pub const fn kilometerperhour_meterspersecond(x:f32)->f32{
        kmh_mps(x)
    }

    /// Convert kilometers per hour to meters per second.
    #[inline(always)]
    pub const fn kilometersperhour_mps(x:f32)->f32{
        kmh_mps(x)
    }

    /// Convert kilometers per hour to meters per second.
    #[inline(always)]
    pub const fn kilometersperhour_meterssecond(x:f32)->f32{
        kmh_mps(x)
    }

    /// Convert kilometers per hour to meters per second.
    #[inline(always)]
    pub const fn kilometersperhour_metersecond(x:f32)->f32{
        kmh_mps(x)
    }

    /// Convert kilometers per hour to meters per second.
    #[inline(always)]
    pub const fn kilometersperhour_meterpersecond(x:f32)->f32{
        kmh_mps(x)
    }

    /// Convert kilometers per hour to meters per second.
    #[inline(always)]
    pub const fn kilometersperhour_meterspersecond(x:f32)->f32{
        kmh_mps(x)
    }

    /// Convert kilometers per hour to miles per hour.
    #[inline(always)]
    pub const fn kmh_mileshour(x:f32)->f32{
        kmh_mph(x)
    }

    /// Convert kilometers per hour to miles per hour.
    #[inline(always)]
    pub const fn kmh_milehour(x:f32)->f32{
        kmh_mph(x)
    }

    /// Convert kilometers per hour to miles per hour.
    #[inline(always)]
    pub const fn kmh_mileperhour(x:f32)->f32{
        kmh_mph(x)
    }

    /// Convert kilometers per hour to miles per hour.
    #[inline(always)]
    pub const fn kmh_milesperhour(x:f32)->f32{
        kmh_mph(x)
    }

    /// Convert kilometers per hour to miles per hour.
    #[inline(always)]
    pub const fn kilometershour_mph(x:f32)->f32{
        kmh_mph(x)
    }

    /// Convert kilometers per hour to miles per hour.
    #[inline(always)]
    pub const fn kilometershour_mileshour(x:f32)->f32{
        kmh_mph(x)
    }

    /// Convert kilometers per hour to miles per hour.
    #[inline(always)]
    pub const fn kilometershour_milehour(x:f32)->f32{
        kmh_mph(x)
    }

    /// Convert kilometers per hour to miles per hour.
    #[inline(always)]
    pub const fn kilometershour_mileperhour(x:f32)->f32{
        kmh_mph(x)
    }

    /// Convert kilometers per hour to miles per hour.
    #[inline(always)]
    pub const fn kilometershour_milesperhour(x:f32)->f32{
        kmh_mph(x)
    }

    /// Convert kilometers per hour to miles per hour.
    #[inline(always)]
    pub const fn kilometerhour_mph(x:f32)->f32{
        kmh_mph(x)
    }

    /// Convert kilometers per hour to miles per hour.
    #[inline(always)]
    pub const fn kilometerhour_mileshour(x:f32)->f32{
        kmh_mph(x)
    }

    /// Convert kilometers per hour to miles per hour.
    #[inline(always)]
    pub const fn kilometerhour_milehour(x:f32)->f32{
        kmh_mph(x)
    }

    /// Convert kilometers per hour to miles per hour.
    #[inline(always)]
    pub const fn kilometerhour_mileperhour(x:f32)->f32{
        kmh_mph(x)
    }

    /// Convert kilometers per hour to miles per hour.
    #[inline(always)]
    pub const fn kilometerhour_milesperhour(x:f32)->f32{
        kmh_mph(x)
    }

    /// Convert kilometers per hour to miles per hour.
    #[inline(always)]
    pub const fn kilometerperhour_mph(x:f32)->f32{
        kmh_mph(x)
    }

    /// Convert kilometers per hour to miles per hour.
    #[inline(always)]
    pub const fn kilometerperhour_mileshour(x:f32)->f32{
        kmh_mph(x)
    }

    /// Convert kilometers per hour to miles per hour.
    #[inline(always)]
    pub const fn kilometerperhour_milehour(x:f32)->f32{
        kmh_mph(x)
    }

    /// Convert kilometers per hour to miles per hour.
    #[inline(always)]
    pub const fn kilometerperhour_mileperhour(x:f32)->f32{
        kmh_mph(x)
    }

    /// Convert kilometers per hour to miles per hour.
    #[inline(always)]
    pub const fn kilometerperhour_milesperhour(x:f32)->f32{
        kmh_mph(x)
    }

    /// Convert kilometers per hour to miles per hour.
    #[inline(always)]
    pub const fn kilometersperhour_mph(x:f32)->f32{
        kmh_mph(x)
    }

    /// Convert kilometers per hour to miles per hour.
    #[inline(always)]
    pub const fn kilometersperhour_mileshour(x:f32)->f32{
        kmh_mph(x)
    }

    /// Convert kilometers per hour to miles per hour.
    #[inline(always)]
    pub const fn kilometersperhour_milehour(x:f32)->f32{
        kmh_mph(x)
    }

    /// Convert kilometers per hour to miles per hour.
    #[inline(always)]
    pub const fn kilometersperhour_mileperhour(x:f32)->f32{
        kmh_mph(x)
    }

    /// Convert kilometers per hour to miles per hour.
    #[inline(always)]
    pub const fn kilometersperhour_milesperhour(x:f32)->f32{
        kmh_mph(x)
    }

    /// Convert kilometers per hour to knots.
    #[inline(always)]
    pub const fn kmh_knot(x:f32)->f32{
        kmh_kt(x)
    }

    /// Convert kilometers per hour to knots.
    #[inline(always)]
    pub const fn kmh_knots(x:f32)->f32{
        kmh_kt(x)
    }

    /// Convert kilometers per hour to knots.
    #[inline(always)]
    pub const fn kilometershour_kt(x:f32)->f32{
        kmh_kt(x)
    }

    /// Convert kilometers per hour to knots.
    #[inline(always)]
    pub const fn kilometershour_knot(x:f32)->f32{
        kmh_kt(x)
    }

    /// Convert kilometers per hour to knots.
    #[inline(always)]
    pub const fn kilometershour_knots(x:f32)->f32{
        kmh_kt(x)
    }

    /// Convert kilometers per hour to knots.
    #[inline(always)]
    pub const fn kilometerhour_kt(x:f32)->f32{
        kmh_kt(x)
    }

    /// Convert kilometers per hour to knots.
    #[inline(always)]
    pub const fn kilometerhour_knot(x:f32)->f32{
        kmh_kt(x)
    }

    /// Convert kilometers per hour to knots.
    #[inline(always)]
    pub const fn kilometerhour_knots(x:f32)->f32{
        kmh_kt(x)
    }

    /// Convert kilometers per hour to knots.
    #[inline(always)]
    pub const fn kilometerperhour_kt(x:f32)->f32{
        kmh_kt(x)
    }

    /// Convert kilometers per hour to knots.
    #[inline(always)]
    pub const fn kilometerperhour_knot(x:f32)->f32{
        kmh_kt(x)
    }

    /// Convert kilometers per hour to knots.
    #[inline(always)]
    pub const fn kilometerperhour_knots(x:f32)->f32{
        kmh_kt(x)
    }

    /// Convert kilometers per hour to knots.
    #[inline(always)]
    pub const fn kilometersperhour_kt(x:f32)->f32{
        kmh_kt(x)
    }

    /// Convert kilometers per hour to knots.
    #[inline(always)]
    pub const fn kilometersperhour_knot(x:f32)->f32{
        kmh_kt(x)
    }

    /// Convert kilometers per hour to knots.
    #[inline(always)]
    pub const fn kilometersperhour_knots(x:f32)->f32{
        kmh_kt(x)
    }

    /// Convert kilometers per hour to mach.
    #[inline(always)]
    pub const fn kilometershour_mach(x:f32)->f32{
        kmh_mach(x)
    }

    /// Convert kilometers per hour to mach.
    #[inline(always)]
    pub const fn kilometerhour_mach(x:f32)->f32{
        kmh_mach(x)
    }

    /// Convert kilometers per hour to mach.
    #[inline(always)]
    pub const fn kilometerperhour_mach(x:f32)->f32{
        kmh_mach(x)
    }

    /// Convert kilometers per hour to mach.
    #[inline(always)]
    pub const fn kilometersperhour_mach(x:f32)->f32{
        kmh_mach(x)
    }

    /// Convert miles per hour to meters per second.
    #[inline(always)]
    pub const fn mph_meterssecond(x:f32)->f32{
        mph_mps(x)
    }

    /// Convert miles per hour to meters per second.
    #[inline(always)]
    pub const fn mph_metersecond(x:f32)->f32{
        mph_mps(x)
    }

    /// Convert miles per hour to meters per second.
    #[inline(always)]
    pub const fn mph_meterpersecond(x:f32)->f32{
        mph_mps(x)
    }

    /// Convert miles per hour to meters per second.
    #[inline(always)]
    pub const fn mph_meterspersecond(x:f32)->f32{
        mph_mps(x)
    }

    /// Convert miles per hour to meters per second.
    #[inline(always)]
    pub const fn mileshour_mps(x:f32)->f32{
        mph_mps(x)
    }

    /// Convert miles per hour to meters per second.
    #[inline(always)]
    pub const fn mileshour_meterssecond(x:f32)->f32{
        mph_mps(x)
    }

    /// Convert miles per hour to meters per second.
    #[inline(always)]
    pub const fn mileshour_metersecond(x:f32)->f32{
        mph_mps(x)
    }

    /// Convert miles per hour to meters per second.
    #[inline(always)]
    pub const fn mileshour_meterpersecond(x:f32)->f32{
        mph_mps(x)
    }

    /// Convert miles per hour to meters per second.
    #[inline(always)]
    pub const fn mileshour_meterspersecond(x:f32)->f32{
        mph_mps(x)
    }

    /// Convert miles per hour to meters per second.
    #[inline(always)]
    pub const fn milehour_mps(x:f32)->f32{
        mph_mps(x)
    }

    /// Convert miles per hour to meters per second.
    #[inline(always)]
    pub const fn milehour_meterssecond(x:f32)->f32{
        mph_mps(x)
    }

    /// Convert miles per hour to meters per second.
    #[inline(always)]
    pub const fn milehour_metersecond(x:f32)->f32{
        mph_mps(x)
    }

    /// Convert miles per hour to meters per second.
    #[inline(always)]
    pub const fn milehour_meterpersecond(x:f32)->f32{
        mph_mps(x)
    }

    /// Convert miles per hour to meters per second.
    #[inline(always)]
    pub const fn milehour_meterspersecond(x:f32)->f32{
        mph_mps(x)
    }

    /// Convert miles per hour to meters per second.
    #[inline(always)]
    pub const fn mileperhour_mps(x:f32)->f32{
        mph_mps(x)
    }

    /// Convert miles per hour to meters per second.
    #[inline(always)]
    pub const fn mileperhour_meterssecond(x:f32)->f32{
        mph_mps(x)
    }

    /// Convert miles per hour to meters per second.
    #[inline(always)]
    pub const fn mileperhour_metersecond(x:f32)->f32{
        mph_mps(x)
    }

    /// Convert miles per hour to meters per second.
    #[inline(always)]
    pub const fn mileperhour_meterpersecond(x:f32)->f32{
        mph_mps(x)
    }

    /// Convert miles per hour to meters per second.
    #[inline(always)]
    pub const fn mileperhour_meterspersecond(x:f32)->f32{
        mph_mps(x)
    }

    /// Convert miles per hour to meters per second.
    #[inline(always)]
    pub const fn milesperhour_mps(x:f32)->f32{
        mph_mps(x)
    }

    /// Convert miles per hour to meters per second.
    #[inline(always)]
    pub const fn milesperhour_meterssecond(x:f32)->f32{
        mph_mps(x)
    }

    /// Convert miles per hour to meters per second.
    #[inline(always)]
    pub const fn milesperhour_metersecond(x:f32)->f32{
        mph_mps(x)
    }

    /// Convert miles per hour to meters per second.
    #[inline(always)]
    pub const fn milesperhour_meterpersecond(x:f32)->f32{
        mph_mps(x)
    }

    /// Convert miles per hour to meters per second.
    #[inline(always)]
    pub const fn milesperhour_meterspersecond(x:f32)->f32{
        mph_mps(x)
    }

    /// Convert miles per hour to kilometers per hour.
    #[inline(always)]
    pub const fn mph_kilometershour(x:f32)->f32{
        mph_kmh(x)
    }

    /// Convert miles per hour to kilometers per hour.
    #[inline(always)]
    pub const fn mph_kilometerhour(x:f32)->f32{
        mph_kmh(x)
    }

    /// Convert miles per hour to kilometers per hour.
    #[inline(always)]
    pub const fn mph_kilometerperhour(x:f32)->f32{
        mph_kmh(x)
    }

    /// Convert miles per hour to kilometers per hour.
    #[inline(always)]
    pub const fn mph_kilometersperhour(x:f32)->f32{
        mph_kmh(x)
    }

    /// Convert miles per hour to kilometers per hour.
    #[inline(always)]
    pub const fn mileshour_kmh(x:f32)->f32{
        mph_kmh(x)
    }

    /// Convert miles per hour to kilometers per hour.
    #[inline(always)]
    pub const fn mileshour_kilometershour(x:f32)->f32{
        mph_kmh(x)
    }

    /// Convert miles per hour to kilometers per hour.
    #[inline(always)]
    pub const fn mileshour_kilometerhour(x:f32)->f32{
        mph_kmh(x)
    }

    /// Convert miles per hour to kilometers per hour.
    #[inline(always)]
    pub const fn mileshour_kilometerperhour(x:f32)->f32{
        mph_kmh(x)
    }

    /// Convert miles per hour to kilometers per hour.
    #[inline(always)]
    pub const fn mileshour_kilometersperhour(x:f32)->f32{
        mph_kmh(x)
    }

    /// Convert miles per hour to kilometers per hour.
    #[inline(always)]
    pub const fn milehour_kmh(x:f32)->f32{
        mph_kmh(x)
    }

    /// Convert miles per hour to kilometers per hour.
    #[inline(always)]
    pub const fn milehour_kilometershour(x:f32)->f32{
        mph_kmh(x)
    }

    /// Convert miles per hour to kilometers per hour.
    #[inline(always)]
    pub const fn milehour_kilometerhour(x:f32)->f32{
        mph_kmh(x)
    }

    /// Convert miles per hour to kilometers per hour.
    #[inline(always)]
    pub const fn milehour_kilometerperhour(x:f32)->f32{
        mph_kmh(x)
    }

    /// Convert miles per hour to kilometers per hour.
    #[inline(always)]
    pub const fn milehour_kilometersperhour(x:f32)->f32{
        mph_kmh(x)
    }

    /// Convert miles per hour to kilometers per hour.
    #[inline(always)]
    pub const fn mileperhour_kmh(x:f32)->f32{
        mph_kmh(x)
    }

    /// Convert miles per hour to kilometers per hour.
    #[inline(always)]
    pub const fn mileperhour_kilometershour(x:f32)->f32{
        mph_kmh(x)
    }

    /// Convert miles per hour to kilometers per hour.
    #[inline(always)]
    pub const fn mileperhour_kilometerhour(x:f32)->f32{
        mph_kmh(x)
    }

    /// Convert miles per hour to kilometers per hour.
    #[inline(always)]
    pub const fn mileperhour_kilometerperhour(x:f32)->f32{
        mph_kmh(x)
    }

    /// Convert miles per hour to kilometers per hour.
    #[inline(always)]
    pub const fn mileperhour_kilometersperhour(x:f32)->f32{
        mph_kmh(x)
    }

    /// Convert miles per hour to kilometers per hour.
    #[inline(always)]
    pub const fn milesperhour_kmh(x:f32)->f32{
        mph_kmh(x)
    }

    /// Convert miles per hour to kilometers per hour.
    #[inline(always)]
    pub const fn milesperhour_kilometershour(x:f32)->f32{
        mph_kmh(x)
    }

    /// Convert miles per hour to kilometers per hour.
    #[inline(always)]
    pub const fn milesperhour_kilometerhour(x:f32)->f32{
        mph_kmh(x)
    }

    /// Convert miles per hour to kilometers per hour.
    #[inline(always)]
    pub const fn milesperhour_kilometerperhour(x:f32)->f32{
        mph_kmh(x)
    }

    /// Convert miles per hour to kilometers per hour.
    #[inline(always)]
    pub const fn milesperhour_kilometersperhour(x:f32)->f32{
        mph_kmh(x)
    }

    /// Convert miles per hour to knots.
    #[inline(always)]
    pub const fn mph_knot(x:f32)->f32{
        mph_kt(x)
    }

    /// Convert miles per hour to knots.
    #[inline(always)]
    pub const fn mph_knots(x:f32)->f32{
        mph_kt(x)
    }

    /// Convert miles per hour to knots.
    #[inline(always)]
    pub const fn mileshour_kt(x:f32)->f32{
        mph_kt(x)
    }

    /// Convert miles per hour to knots.
    #[inline(always)]
    pub const fn mileshour_knot(x:f32)->f32{
        mph_kt(x)
    }

    /// Convert miles per hour to knots.
    #[inline(always)]
    pub const fn mileshour_knots(x:f32)->f32{
        mph_kt(x)
    }

    /// Convert miles per hour to knots.
    #[inline(always)]
    pub const fn milehour_kt(x:f32)->f32{
        mph_kt(x)
    }

    /// Convert miles per hour to knots.
    #[inline(always)]
    pub const fn milehour_knot(x:f32)->f32{
        mph_kt(x)
    }

    /// Convert miles per hour to knots.
    #[inline(always)]
    pub const fn milehour_knots(x:f32)->f32{
        mph_kt(x)
    }

    /// Convert miles per hour to knots.
    #[inline(always)]
    pub const fn mileperhour_kt(x:f32)->f32{
        mph_kt(x)
    }

    /// Convert miles per hour to knots.
    #[inline(always)]
    pub const fn mileperhour_knot(x:f32)->f32{
        mph_kt(x)
    }

    /// Convert miles per hour to knots.
    #[inline(always)]
    pub const fn mileperhour_knots(x:f32)->f32{
        mph_kt(x)
    }

    /// Convert miles per hour to knots.
    #[inline(always)]
    pub const fn milesperhour_kt(x:f32)->f32{
        mph_kt(x)
    }

    /// Convert miles per hour to knots.
    #[inline(always)]
    pub const fn milesperhour_knot(x:f32)->f32{
        mph_kt(x)
    }

    /// Convert miles per hour to knots.
    #[inline(always)]
    pub const fn milesperhour_knots(x:f32)->f32{
        mph_kt(x)
    }

    /// Convert miles per hour to mach.
    #[inline(always)]
    pub const fn mileshour_mach(x:f32)->f32{
        mph_mach(x)
    }

    /// Convert miles per hour to mach.
    #[inline(always)]
    pub const fn milehour_mach(x:f32)->f32{
        mph_mach(x)
    }

    /// Convert miles per hour to mach.
    #[inline(always)]
    pub const fn mileperhour_mach(x:f32)->f32{
        mph_mach(x)
    }

    /// Convert miles per hour to mach.
    #[inline(always)]
    pub const fn milesperhour_mach(x:f32)->f32{
        mph_mach(x)
    }

    /// Convert knots to meters per second.
    #[inline(always)]
    pub const fn kt_meterssecond(x:f32)->f32{
        kt_mps(x)
    }

    /// Convert knots to meters per second.
    #[inline(always)]
    pub const fn kt_metersecond(x:f32)->f32{
        kt_mps(x)
    }

    /// Convert knots to meters per second.
    #[inline(always)]
    pub const fn kt_meterpersecond(x:f32)->f32{
        kt_mps(x)
    }

    /// Convert knots to meters per second.
    #[inline(always)]
    pub const fn kt_meterspersecond(x:f32)->f32{
        kt_mps(x)
    }

    /// Convert knots to meters per second.
    #[inline(always)]
    pub const fn knot_mps(x:f32)->f32{
        kt_mps(x)
    }

    /// Convert knots to meters per second.
    #[inline(always)]
    pub const fn knot_meterssecond(x:f32)->f32{
        kt_mps(x)
    }

    /// Convert knots to meters per second.
    #[inline(always)]
    pub const fn knot_metersecond(x:f32)->f32{
        kt_mps(x)
    }

    /// Convert knots to meters per second.
    #[inline(always)]
    pub const fn knot_meterpersecond(x:f32)->f32{
        kt_mps(x)
    }

    /// Convert knots to meters per second.
    #[inline(always)]
    pub const fn knot_meterspersecond(x:f32)->f32{
        kt_mps(x)
    }

    /// Convert knots to meters per second.
    #[inline(always)]
    pub const fn knots_mps(x:f32)->f32{
        kt_mps(x)
    }

    /// Convert knots to meters per second.
    #[inline(always)]
    pub const fn knots_meterssecond(x:f32)->f32{
        kt_mps(x)
    }

    /// Convert knots to meters per second.
    #[inline(always)]
    pub const fn knots_metersecond(x:f32)->f32{
        kt_mps(x)
    }

    /// Convert knots to meters per second.
    #[inline(always)]
    pub const fn knots_meterpersecond(x:f32)->f32{
        kt_mps(x)
    }

    /// Convert knots to meters per second.
    #[inline(always)]
    pub const fn knots_meterspersecond(x:f32)->f32{
        kt_mps(x)
    }

    /// Convert knots to kilometers per hour.
    #[inline(always)]
    pub const fn kt_kilometershour(x:f32)->f32{
        kt_kmh(x)
    }

    /// Convert knots to kilometers per hour.
    #[inline(always)]
    pub const fn kt_kilometerhour(x:f32)->f32{
        kt_kmh(x)
    }

    /// Convert knots to kilometers per hour.
    #[inline(always)]
    pub const fn kt_kilometerperhour(x:f32)->f32{
        kt_kmh(x)
    }

    /// Convert knots to kilometers per hour.
    #[inline(always)]
    pub const fn kt_kilometersperhour(x:f32)->f32{
        kt_kmh(x)
    }

    /// Convert knots to kilometers per hour.
    #[inline(always)]
    pub const fn knot_kmh(x:f32)->f32{
        kt_kmh(x)
    }

    /// Convert knots to kilometers per hour.
    #[inline(always)]
    pub const fn knot_kilometershour(x:f32)->f32{
        kt_kmh(x)
    }

    /// Convert knots to kilometers per hour.
    #[inline(always)]
    pub const fn knot_kilometerhour(x:f32)->f32{
        kt_kmh(x)
    }

    /// Convert knots to kilometers per hour.
    #[inline(always)]
    pub const fn knot_kilometerperhour(x:f32)->f32{
        kt_kmh(x)
    }

    /// Convert knots to kilometers per hour.
    #[inline(always)]
    pub const fn knot_kilometersperhour(x:f32)->f32{
        kt_kmh(x)
    }

    /// Convert knots to kilometers per hour.
    #[inline(always)]
    pub const fn knots_kmh(x:f32)->f32{
        kt_kmh(x)
    }

    /// Convert knots to kilometers per hour.
    #[inline(always)]
    pub const fn knots_kilometershour(x:f32)->f32{
        kt_kmh(x)
    }

    /// Convert knots to kilometers per hour.
    #[inline(always)]
    pub const fn knots_kilometerhour(x:f32)->f32{
        kt_kmh(x)
    }

    /// Convert knots to kilometers per hour.
    #[inline(always)]
    pub const fn knots_kilometerperhour(x:f32)->f32{
        kt_kmh(x)
    }

    /// Convert knots to kilometers per hour.
    #[inline(always)]
    pub const fn knots_kilometersperhour(x:f32)->f32{
        kt_kmh(x)
    }

    /// Convert knots to miles per hour.
    #[inline(always)]
    pub const fn kt_mileshour(x:f32)->f32{
        kt_mph(x)
    }

    /// Convert knots to miles per hour.
    #[inline(always)]
    pub const fn kt_milehour(x:f32)->f32{
        kt_mph(x)
    }

    /// Convert knots to miles per hour.
    #[inline(always)]
    pub const fn kt_mileperhour(x:f32)->f32{
        kt_mph(x)
    }

    /// Convert knots to miles per hour.
    #[inline(always)]
    pub const fn kt_milesperhour(x:f32)->f32{
        kt_mph(x)
    }

    /// Convert knots to miles per hour.
    #[inline(always)]
    pub const fn knot_mph(x:f32)->f32{
        kt_mph(x)
    }

    /// Convert knots to miles per hour.
    #[inline(always)]
    pub const fn knot_mileshour(x:f32)->f32{
        kt_mph(x)
    }

    /// Convert knots to miles per hour.
    #[inline(always)]
    pub const fn knot_milehour(x:f32)->f32{
        kt_mph(x)
    }

    /// Convert knots to miles per hour.
    #[inline(always)]
    pub const fn knot_mileperhour(x:f32)->f32{
        kt_mph(x)
    }

    /// Convert knots to miles per hour.
    #[inline(always)]
    pub const fn knot_milesperhour(x:f32)->f32{
        kt_mph(x)
    }

    /// Convert knots to miles per hour.
    #[inline(always)]
    pub const fn knots_mph(x:f32)->f32{
        kt_mph(x)
    }

    /// Convert knots to miles per hour.
    #[inline(always)]
    pub const fn knots_mileshour(x:f32)->f32{
        kt_mph(x)
    }

    /// Convert knots to miles per hour.
    #[inline(always)]
    pub const fn knots_milehour(x:f32)->f32{
        kt_mph(x)
    }

    /// Convert knots to miles per hour.
    #[inline(always)]
    pub const fn knots_mileperhour(x:f32)->f32{
        kt_mph(x)
    }

    /// Convert knots to miles per hour.
    #[inline(always)]
    pub const fn knots_milesperhour(x:f32)->f32{
        kt_mph(x)
    }

    /// Convert knots to mach.
    #[inline(always)]
    pub const fn knot_mach(x:f32)->f32{
        kt_mach(x)
    }

    /// Convert knots to mach.
    #[inline(always)]
    pub const fn knots_mach(x:f32)->f32{
        kt_mach(x)
    }

    /// Convert mach to meters per second.
    #[inline(always)]
    pub const fn mach_meterssecond(x:f32)->f32{
        mach_mps(x)
    }

    /// Convert mach to meters per second.
    #[inline(always)]
    pub const fn mach_metersecond(x:f32)->f32{
        mach_mps(x)
    }

    /// Convert mach to meters per second.
    #[inline(always)]
    pub const fn mach_meterpersecond(x:f32)->f32{
        mach_mps(x)
    }

    /// Convert mach to meters per second.
    #[inline(always)]
    pub const fn mach_meterspersecond(x:f32)->f32{
        mach_mps(x)
    }

    /// Convert mach to kilometers per hour.
    #[inline(always)]
    pub const fn mach_kilometershour(x:f32)->f32{
        mach_kmh(x)
    }

    /// Convert mach to kilometers per hour.
    #[inline(always)]
    pub const fn mach_kilometerhour(x:f32)->f32{
        mach_kmh(x)
    }

    /// Convert mach to kilometers per hour.
    #[inline(always)]
    pub const fn mach_kilometerperhour(x:f32)->f32{
        mach_kmh(x)
    }

    /// Convert mach to kilometers per hour.
    #[inline(always)]
    pub const fn mach_kilometersperhour(x:f32)->f32{
        mach_kmh(x)
    }

    /// Convert mach to miles per hour.
    #[inline(always)]
    pub const fn mach_mileshour(x:f32)->f32{
        mach_mph(x)
    }

    /// Convert mach to miles per hour.
    #[inline(always)]
    pub const fn mach_milehour(x:f32)->f32{
        mach_mph(x)
    }

    /// Convert mach to miles per hour.
    #[inline(always)]
    pub const fn mach_mileperhour(x:f32)->f32{
        mach_mph(x)
    }

    /// Convert mach to miles per hour.
    #[inline(always)]
    pub const fn mach_milesperhour(x:f32)->f32{
        mach_mph(x)
    }

    /// Convert mach to knots.
    #[inline(always)]
    pub const fn mach_knot(x:f32)->f32{
        mach_kt(x)
    }

    /// Convert mach to knots.
    #[inline(always)]
    pub const fn mach_knots(x:f32)->f32{
        mach_kt(x)
    }
}

pub mod pressure{
    /// Convert pascals to pounds per square inch.
    #[inline]
    pub const fn pa_psi(x:f32)->f32{
        x*0.00014504
    }

    /// Convert pascals to bar.
    #[inline]
    pub const fn pa_b(x:f32)->f32{
        x*0.00001
    }

    /// Convert pascals to atmospheres.
    #[inline]
    pub const fn pa_atm(x:f32)->f32{
        x*0.00000987
    }

    /// Convert pounds per square inch to pascals.
    #[inline]
    pub const fn psi_pa(x:f32)->f32{
        x*6894.75729317
    }

    /// Convert pounds per square inch to bar.
    #[inline]
    pub const fn psi_b(x:f32)->f32{
        x*0.06894757
    }

    /// Convert pounds per square inch to atmospheres.
    #[inline]
    pub const fn psi_atm(x:f32)->f32{
        x*0.06804596
    }

    /// Convert bar to pascals.
    #[inline]
    pub const fn b_pa(x:f32)->f32{
        x*100000.0
    }

    /// Convert bar to pounds per square inch.
    #[inline]
    pub const fn b_psi(x:f32)->f32{
        x*14.50377377
    }

    /// Convert bar to atmospheres.
    #[inline]
    pub const fn b_atm(x:f32)->f32{
        x*0.98692327
    }

    /// Convert atmospheres to pascals.
    #[inline]
    pub const fn atm_pa(x:f32)->f32{
        x*101325.0
    }

    /// Convert atmospheres to pounds per square inch.
    #[inline]
    pub const fn atm_psi(x:f32)->f32{
        x*14.69594878
    }

    /// Convert atmospheres to bar.
    #[inline]
    pub const fn atm_b(x:f32)->f32{
        x*1.01325
    }

    /// Convert pascals to pounds per square inch.
    #[inline(always)]
    pub const fn pa_poundsquareinch(x:f32)->f32{
        pa_psi(x)
    }

    /// Convert pascals to pounds per square inch.
    #[inline(always)]
    pub const fn pa_poundssquareinch(x:f32)->f32{
        pa_psi(x)
    }

    /// Convert pascals to pounds per square inch.
    #[inline(always)]
    pub const fn pa_poundspersquareinch(x:f32)->f32{
        pa_psi(x)
    }

    /// Convert pascals to pounds per square inch.
    #[inline(always)]
    pub const fn pascal_psi(x:f32)->f32{
        pa_psi(x)
    }

    /// Convert pascals to pounds per square inch.
    #[inline(always)]
    pub const fn pascal_poundsquareinch(x:f32)->f32{
        pa_psi(x)
    }

    /// Convert pascals to pounds per square inch.
    #[inline(always)]
    pub const fn pascal_poundssquareinch(x:f32)->f32{
        pa_psi(x)
    }

    /// Convert pascals to pounds per square inch.
    #[inline(always)]
    pub const fn pascal_poundspersquareinch(x:f32)->f32{
        pa_psi(x)
    }

    /// Convert pascals to pounds per square inch.
    #[inline(always)]
    pub const fn pascals_psi(x:f32)->f32{
        pa_psi(x)
    }

    /// Convert pascals to pounds per square inch.
    #[inline(always)]
    pub const fn pascals_poundsquareinch(x:f32)->f32{
        pa_psi(x)
    }

    /// Convert pascals to pounds per square inch.
    #[inline(always)]
    pub const fn pascals_poundssquareinch(x:f32)->f32{
        pa_psi(x)
    }

    /// Convert pascals to pounds per square inch.
    #[inline(always)]
    pub const fn pascals_poundspersquareinch(x:f32)->f32{
        pa_psi(x)
    }

    /// Convert pascals to bar.
    #[inline(always)]
    pub const fn pa_bar(x:f32)->f32{
        pa_b(x)
    }

    /// Convert pascals to bar.
    #[inline(always)]
    pub const fn pascal_b(x:f32)->f32{
        pa_b(x)
    }

    /// Convert pascals to bar.
    #[inline(always)]
    pub const fn pascal_bar(x:f32)->f32{
        pa_b(x)
    }

    /// Convert pascals to bar.
    #[inline(always)]
    pub const fn pascals_b(x:f32)->f32{
        pa_b(x)
    }

    /// Convert pascals to bar.
    #[inline(always)]
    pub const fn pascals_bar(x:f32)->f32{
        pa_b(x)
    }

    /// Convert pascals to atmospheres.
    #[inline(always)]
    pub const fn pa_atmosphere(x:f32)->f32{
        pa_atm(x)
    }

    /// Convert pascals to atmospheres.
    #[inline(always)]
    pub const fn pa_atmospheres(x:f32)->f32{
        pa_atm(x)
    }

    /// Convert pascals to atmospheres.
    #[inline(always)]
    pub const fn pascal_atm(x:f32)->f32{
        pa_atm(x)
    }

    /// Convert pascals to atmospheres.
    #[inline(always)]
    pub const fn pascal_atmosphere(x:f32)->f32{
        pa_atm(x)
    }

    /// Convert pascals to atmospheres.
    #[inline(always)]
    pub const fn pascal_atmospheres(x:f32)->f32{
        pa_atm(x)
    }

    /// Convert pascals to atmospheres.
    #[inline(always)]
    pub const fn pascals_atm(x:f32)->f32{
        pa_atm(x)
    }

    /// Convert pascals to atmospheres.
    #[inline(always)]
    pub const fn pascals_atmosphere(x:f32)->f32{
        pa_atm(x)
    }

    /// Convert pascals to atmospheres.
    #[inline(always)]
    pub const fn pascals_atmospheres(x:f32)->f32{
        pa_atm(x)
    }

    /// Convert pounds per square inch to pascals.
    #[inline(always)]
    pub const fn psi_pascal(x:f32)->f32{
        psi_pa(x)
    }

    /// Convert pounds per square inch to pascals.
    #[inline(always)]
    pub const fn psi_pascals(x:f32)->f32{
        psi_pa(x)
    }

    /// Convert pounds per square inch to pascals.
    #[inline(always)]
    pub const fn poundsquareinch_pa(x:f32)->f32{
        psi_pa(x)
    }

    /// Convert pounds per square inch to pascals.
    #[inline(always)]
    pub const fn poundsquareinch_pascal(x:f32)->f32{
        psi_pa(x)
    }

    /// Convert pounds per square inch to pascals.
    #[inline(always)]
    pub const fn poundsquareinch_pascals(x:f32)->f32{
        psi_pa(x)
    }

    /// Convert pounds per square inch to pascals.
    #[inline(always)]
    pub const fn poundssquareinch_pa(x:f32)->f32{
        psi_pa(x)
    }

    /// Convert pounds per square inch to pascals.
    #[inline(always)]
    pub const fn poundssquareinch_pascal(x:f32)->f32{
        psi_pa(x)
    }

    /// Convert pounds per square inch to pascals.
    #[inline(always)]
    pub const fn poundssquareinch_pascals(x:f32)->f32{
        psi_pa(x)
    }

    /// Convert pounds per square inch to pascals.
    #[inline(always)]
    pub const fn poundspersquareinch_pa(x:f32)->f32{
        psi_pa(x)
    }

    /// Convert pounds per square inch to pascals.
    #[inline(always)]
    pub const fn poundspersquareinch_pascal(x:f32)->f32{
        psi_pa(x)
    }

    /// Convert pounds per square inch to pascals.
    #[inline(always)]
    pub const fn poundspersquareinch_pascals(x:f32)->f32{
        psi_pa(x)
    }

    /// Convert pounds per square inch to bar.
    #[inline(always)]
    pub const fn psi_bar(x:f32)->f32{
        psi_b(x)
    }

    /// Convert pounds per square inch to bar.
    #[inline(always)]
    pub const fn poundsquareinch_b(x:f32)->f32{
        psi_b(x)
    }

    /// Convert pounds per square inch to bar.
    #[inline(always)]
    pub const fn poundsquareinch_bar(x:f32)->f32{
        psi_b(x)
    }

    /// Convert pounds per square inch to bar.
    #[inline(always)]
    pub const fn poundssquareinch_b(x:f32)->f32{
        psi_b(x)
    }

    /// Convert pounds per square inch to bar.
    #[inline(always)]
    pub const fn poundssquareinch_bar(x:f32)->f32{
        psi_b(x)
    }

    /// Convert pounds per square inch to bar.
    #[inline(always)]
    pub const fn poundspersquareinch_b(x:f32)->f32{
        psi_b(x)
    }

    /// Convert pounds per square inch to bar.
    #[inline(always)]
    pub const fn poundspersquareinch_bar(x:f32)->f32{
        psi_b(x)
    }

    /// Convert pounds per square inch to atmospheres.
    #[inline(always)]
    pub const fn psi_atmosphere(x:f32)->f32{
        psi_atm(x)
    }

    /// Convert pounds per square inch to atmospheres.
    #[inline(always)]
    pub const fn psi_atmospheres(x:f32)->f32{
        psi_atm(x)
    }

    /// Convert pounds per square inch to atmospheres.
    #[inline(always)]
    pub const fn poundsquareinch_atm(x:f32)->f32{
        psi_atm(x)
    }

    /// Convert pounds per square inch to atmospheres.
    #[inline(always)]
    pub const fn poundsquareinch_atmosphere(x:f32)->f32{
        psi_atm(x)
    }

    /// Convert pounds per square inch to atmospheres.
    #[inline(always)]
    pub const fn poundsquareinch_atmospheres(x:f32)->f32{
        psi_atm(x)
    }

    /// Convert pounds per square inch to atmospheres.
    #[inline(always)]
    pub const fn poundssquareinch_atm(x:f32)->f32{
        psi_atm(x)
    }

    /// Convert pounds per square inch to atmospheres.
    #[inline(always)]
    pub const fn poundssquareinch_atmosphere(x:f32)->f32{
        psi_atm(x)
    }

    /// Convert pounds per square inch to atmospheres.
    #[inline(always)]
    pub const fn poundssquareinch_atmospheres(x:f32)->f32{
        psi_atm(x)
    }

    /// Convert pounds per square inch to atmospheres.
    #[inline(always)]
    pub const fn poundspersquareinch_atm(x:f32)->f32{
        psi_atm(x)
    }

    /// Convert pounds per square inch to atmospheres.
    #[inline(always)]
    pub const fn poundspersquareinch_atmosphere(x:f32)->f32{
        psi_atm(x)
    }

    /// Convert pounds per square inch to atmospheres.
    #[inline(always)]
    pub const fn poundspersquareinch_atmospheres(x:f32)->f32{
        psi_atm(x)
    }

    /// Convert bar to pascals.
    #[inline(always)]
    pub const fn b_pascal(x:f32)->f32{
        b_pa(x)
    }

    /// Convert bar to pascals.
    #[inline(always)]
    pub const fn b_pascals(x:f32)->f32{
        b_pa(x)
    }

    /// Convert bar to pascals.
    #[inline(always)]
    pub const fn bar_pa(x:f32)->f32{
        b_pa(x)
    }

    /// Convert bar to pascals.
    #[inline(always)]
    pub const fn bar_pascal(x:f32)->f32{
        b_pa(x)
    }

    /// Convert bar to pascals.
    #[inline(always)]
    pub const fn bar_pascals(x:f32)->f32{
        b_pa(x)
    }

    /// Convert bar to pounds per square inch.
    #[inline(always)]
    pub const fn b_poundsquareinch(x:f32)->f32{
        b_psi(x)
    }

    /// Convert bar to pounds per square inch.
    #[inline(always)]
    pub const fn b_poundssquareinch(x:f32)->f32{
        b_psi(x)
    }

    /// Convert bar to pounds per square inch.
    #[inline(always)]
    pub const fn b_poundspersquareinch(x:f32)->f32{
        b_psi(x)
    }

    /// Convert bar to pounds per square inch.
    #[inline(always)]
    pub const fn bar_psi(x:f32)->f32{
        b_psi(x)
    }

    /// Convert bar to pounds per square inch.
    #[inline(always)]
    pub const fn bar_poundsquareinch(x:f32)->f32{
        b_psi(x)
    }

    /// Convert bar to pounds per square inch.
    #[inline(always)]
    pub const fn bar_poundssquareinch(x:f32)->f32{
        b_psi(x)
    }

    /// Convert bar to pounds per square inch.
    #[inline(always)]
    pub const fn bar_poundspersquareinch(x:f32)->f32{
        b_psi(x)
    }

    /// Convert bar to atmospheres.
    #[inline(always)]
    pub const fn b_atmosphere(x:f32)->f32{
        b_atm(x)
    }

    /// Convert bar to atmospheres.
    #[inline(always)]
    pub const fn b_atmospheres(x:f32)->f32{
        b_atm(x)
    }

    /// Convert bar to atmospheres.
    #[inline(always)]
    pub const fn bar_atm(x:f32)->f32{
        b_atm(x)
    }

    /// Convert bar to atmospheres.
    #[inline(always)]
    pub const fn bar_atmosphere(x:f32)->f32{
        b_atm(x)
    }

    /// Convert bar to atmospheres.
    #[inline(always)]
    pub const fn bar_atmospheres(x:f32)->f32{
        b_atm(x)
    }

    /// Convert atmospheres to pascals.
    #[inline(always)]
    pub const fn atm_pascal(x:f32)->f32{
        atm_pa(x)
    }

    /// Convert atmospheres to pascals.
    #[inline(always)]
    pub const fn atm_pascals(x:f32)->f32{
        atm_pa(x)
    }

    /// Convert atmospheres to pascals.
    #[inline(always)]
    pub const fn atmosphere_pa(x:f32)->f32{
        atm_pa(x)
    }

    /// Convert atmospheres to pascals.
    #[inline(always)]
    pub const fn atmosphere_pascal(x:f32)->f32{
        atm_pa(x)
    }

    /// Convert atmospheres to pascals.
    #[inline(always)]
    pub const fn atmosphere_pascals(x:f32)->f32{
        atm_pa(x)
    }

    /// Convert atmospheres to pascals.
    #[inline(always)]
    pub const fn atmospheres_pa(x:f32)->f32{
        atm_pa(x)
    }

    /// Convert atmospheres to pascals.
    #[inline(always)]
    pub const fn atmospheres_pascal(x:f32)->f32{
        atm_pa(x)
    }

    /// Convert atmospheres to pascals.
    #[inline(always)]
    pub const fn atmospheres_pascals(x:f32)->f32{
        atm_pa(x)
    }

    /// Convert atmospheres to pounds per square inch.
    #[inline(always)]
    pub const fn atm_poundsquareinch(x:f32)->f32{
        atm_psi(x)
    }

    /// Convert atmospheres to pounds per square inch.
    #[inline(always)]
    pub const fn atm_poundssquareinch(x:f32)->f32{
        atm_psi(x)
    }

    /// Convert atmospheres to pounds per square inch.
    #[inline(always)]
    pub const fn atm_poundspersquareinch(x:f32)->f32{
        atm_psi(x)
    }

    /// Convert atmospheres to pounds per square inch.
    #[inline(always)]
    pub const fn atmosphere_psi(x:f32)->f32{
        atm_psi(x)
    }

    /// Convert atmospheres to pounds per square inch.
    #[inline(always)]
    pub const fn atmosphere_poundsquareinch(x:f32)->f32{
        atm_psi(x)
    }

    /// Convert atmospheres to pounds per square inch.
    #[inline(always)]
    pub const fn atmosphere_poundssquareinch(x:f32)->f32{
        atm_psi(x)
    }

    /// Convert atmospheres to pounds per square inch.
    #[inline(always)]
    pub const fn atmosphere_poundspersquareinch(x:f32)->f32{
        atm_psi(x)
    }

    /// Convert atmospheres to pounds per square inch.
    #[inline(always)]
    pub const fn atmospheres_psi(x:f32)->f32{
        atm_psi(x)
    }

    /// Convert atmospheres to pounds per square inch.
    #[inline(always)]
    pub const fn atmospheres_poundsquareinch(x:f32)->f32{
        atm_psi(x)
    }

    /// Convert atmospheres to pounds per square inch.
    #[inline(always)]
    pub const fn atmospheres_poundssquareinch(x:f32)->f32{
        atm_psi(x)
    }

    /// Convert atmospheres to pounds per square inch.
    #[inline(always)]
    pub const fn atmospheres_poundspersquareinch(x:f32)->f32{
        atm_psi(x)
    }

    /// Convert atmospheres to bar.
    #[inline(always)]
    pub const fn atm_bar(x:f32)->f32{
        atm_b(x)
    }

    /// Convert atmospheres to bar.
    #[inline(always)]
    pub const fn atmosphere_b(x:f32)->f32{
        atm_b(x)
    }

    /// Convert atmospheres to bar.
    #[inline(always)]
    pub const fn atmosphere_bar(x:f32)->f32{
        atm_b(x)
    }

    /// Convert atmospheres to bar.
    #[inline(always)]
    pub const fn atmospheres_b(x:f32)->f32{
        atm_b(x)
    }

    /// Convert atmospheres to bar.
    #[inline(always)]
    pub const fn atmospheres_bar(x:f32)->f32{
        atm_b(x)
    }
}

pub mod energy{
    /// Convert joules to calories.
    #[inline]
    pub const fn j_cal(x:f32)->f32{
        x*0.23900574
    }

    /// Convert joules to horsepower.
    #[inline]
    pub const fn j_hp(x:f32)->f32{
        x*0.00134102
    }

    /// Convert joules to watts.
    #[inline]
    pub const fn j_w(x:f32)->f32{
        x*1.0
    }

    /// Convert calories to joules.
    #[inline]
    pub const fn cal_j(x:f32)->f32{
        x*4.184
    }

    /// Convert calories to horsepower.
    #[inline]
    pub const fn cal_hp(x:f32)->f32{
        x*0.00561084
    }

    /// Convert calories to watts.
    #[inline]
    pub const fn cal_w(x:f32)->f32{
        x*4.184
    }

    /// Convert horsepower to joules.
    #[inline]
    pub const fn hp_j(x:f32)->f32{
        x*745.699872
    }

    /// Convert horsepower to calories.
    #[inline]
    pub const fn hp_cal(x:f32)->f32{
        x*178.22654685
    }

    /// Convert horsepower to watts.
    #[inline]
    pub const fn hp_w(x:f32)->f32{
        x*745.699872
    }

    /// Convert watts to joules.
    #[inline]
    pub const fn w_j(x:f32)->f32{
        x*1.0
    }

    /// Convert watts to calories.
    #[inline]
    pub const fn w_cal(x:f32)->f32{
        x*0.23900574
    }

    /// Convert watts to horsepower.
    #[inline]
    pub const fn w_hp(x:f32)->f32{
        x*0.00134102
    }

    /// Convert joules to calories.
    #[inline(always)]
    pub const fn j_calorie(x:f32)->f32{
        j_cal(x)
    }

    /// Convert joules to calories.
    #[inline(always)]
    pub const fn j_calories(x:f32)->f32{
        j_cal(x)
    }

    /// Convert joules to calories.
    #[inline(always)]
    pub const fn joule_cal(x:f32)->f32{
        j_cal(x)
    }

    /// Convert joules to calories.
    #[inline(always)]
    pub const fn joule_calorie(x:f32)->f32{
        j_cal(x)
    }

    /// Convert joules to calories.
    #[inline(always)]
    pub const fn joule_calories(x:f32)->f32{
        j_cal(x)
    }

    /// Convert joules to calories.
    #[inline(always)]
    pub const fn joules_cal(x:f32)->f32{
        j_cal(x)
    }

    /// Convert joules to calories.
    #[inline(always)]
    pub const fn joules_calorie(x:f32)->f32{
        j_cal(x)
    }

    /// Convert joules to calories.
    #[inline(always)]
    pub const fn joules_calories(x:f32)->f32{
        j_cal(x)
    }

    /// Convert joules to horsepower.
    #[inline(always)]
    pub const fn j_horsepower(x:f32)->f32{
        j_hp(x)
    }

    /// Convert joules to horsepower.
    #[inline(always)]
    pub const fn joule_hp(x:f32)->f32{
        j_hp(x)
    }

    /// Convert joules to horsepower.
    #[inline(always)]
    pub const fn joule_horsepower(x:f32)->f32{
        j_hp(x)
    }

    /// Convert joules to horsepower.
    #[inline(always)]
    pub const fn joules_hp(x:f32)->f32{
        j_hp(x)
    }

    /// Convert joules to horsepower.
    #[inline(always)]
    pub const fn joules_horsepower(x:f32)->f32{
        j_hp(x)
    }

    /// Convert joules to watts.
    #[inline(always)]
    pub const fn j_watt(x:f32)->f32{
        j_w(x)
    }

    /// Convert joules to watts.
    #[inline(always)]
    pub const fn j_watts(x:f32)->f32{
        j_w(x)
    }

    /// Convert joules to watts.
    #[inline(always)]
    pub const fn joule_w(x:f32)->f32{
        j_w(x)
    }

    /// Convert joules to watts.
    #[inline(always)]
    pub const fn joule_watt(x:f32)->f32{
        j_w(x)
    }

    /// Convert joules to watts.
    #[inline(always)]
    pub const fn joule_watts(x:f32)->f32{
        j_w(x)
    }

    /// Convert joules to watts.
    #[inline(always)]
    pub const fn joules_w(x:f32)->f32{
        j_w(x)
    }

    /// Convert joules to watts.
    #[inline(always)]
    pub const fn joules_watt(x:f32)->f32{
        j_w(x)
    }

    /// Convert joules to watts.
    #[inline(always)]
    pub const fn joules_watts(x:f32)->f32{
        j_w(x)
    }

    /// Convert calories to joules.
    #[inline(always)]
    pub const fn cal_joule(x:f32)->f32{
        cal_j(x)
    }

    /// Convert calories to joules.
    #[inline(always)]
    pub const fn cal_joules(x:f32)->f32{
        cal_j(x)
    }

    /// Convert calories to joules.
    #[inline(always)]
    pub const fn calorie_j(x:f32)->f32{
        cal_j(x)
    }

    /// Convert calories to joules.
    #[inline(always)]
    pub const fn calorie_joule(x:f32)->f32{
        cal_j(x)
    }

    /// Convert calories to joules.
    #[inline(always)]
    pub const fn calorie_joules(x:f32)->f32{
        cal_j(x)
    }

    /// Convert calories to joules.
    #[inline(always)]
    pub const fn calories_j(x:f32)->f32{
        cal_j(x)
    }

    /// Convert calories to joules.
    #[inline(always)]
    pub const fn calories_joule(x:f32)->f32{
        cal_j(x)
    }

    /// Convert calories to joules.
    #[inline(always)]
    pub const fn calories_joules(x:f32)->f32{
        cal_j(x)
    }

    /// Convert calories to horsepower.
    #[inline(always)]
    pub const fn cal_horsepower(x:f32)->f32{
        cal_hp(x)
    }

    /// Convert calories to horsepower.
    #[inline(always)]
    pub const fn calorie_hp(x:f32)->f32{
        cal_hp(x)
    }

    /// Convert calories to horsepower.
    #[inline(always)]
    pub const fn calorie_horsepower(x:f32)->f32{
        cal_hp(x)
    }

    /// Convert calories to horsepower.
    #[inline(always)]
    pub const fn calories_hp(x:f32)->f32{
        cal_hp(x)
    }

    /// Convert calories to horsepower.
    #[inline(always)]
    pub const fn calories_horsepower(x:f32)->f32{
        cal_hp(x)
    }

    /// Convert calories to watts.
    #[inline(always)]
    pub const fn cal_watt(x:f32)->f32{
        cal_w(x)
    }

    /// Convert calories to watts.
    #[inline(always)]
    pub const fn cal_watts(x:f32)->f32{
        cal_w(x)
    }

    /// Convert calories to watts.
    #[inline(always)]
    pub const fn calorie_w(x:f32)->f32{
        cal_w(x)
    }

    /// Convert calories to watts.
    #[inline(always)]
    pub const fn calorie_watt(x:f32)->f32{
        cal_w(x)
    }

    /// Convert calories to watts.
    #[inline(always)]
    pub const fn calorie_watts(x:f32)->f32{
        cal_w(x)
    }

    /// Convert calories to watts.
    #[inline(always)]
    pub const fn calories_w(x:f32)->f32{
        cal_w(x)
    }

    /// Convert calories to watts.
    #[inline(always)]
    pub const fn calories_watt(x:f32)->f32{
        cal_w(x)
    }

    /// Convert calories to watts.
    #[inline(always)]
    pub const fn calories_watts(x:f32)->f32{
        cal_w(x)
    }

    /// Convert horsepower to joules.
    #[inline(always)]
    pub const fn hp_joule(x:f32)->f32{
        hp_j(x)
    }

    /// Convert horsepower to joules.
    #[inline(always)]
    pub const fn hp_joules(x:f32)->f32{
        hp_j(x)
    }

    /// Convert horsepower to joules.
    #[inline(always)]
    pub const fn horsepower_j(x:f32)->f32{
        hp_j(x)
    }

    /// Convert horsepower to joules.
    #[inline(always)]
    pub const fn horsepower_joule(x:f32)->f32{
        hp_j(x)
    }

    /// Convert horsepower to joules.
    #[inline(always)]
    pub const fn horsepower_joules(x:f32)->f32{
        hp_j(x)
    }

    /// Convert horsepower to calories.
    #[inline(always)]
    pub const fn hp_calorie(x:f32)->f32{
        hp_cal(x)
    }

    /// Convert horsepower to calories.
    #[inline(always)]
    pub const fn hp_calories(x:f32)->f32{
        hp_cal(x)
    }

    /// Convert horsepower to calories.
    #[inline(always)]
    pub const fn horsepower_cal(x:f32)->f32{
        hp_cal(x)
    }

    /// Convert horsepower to calories.
    #[inline(always)]
    pub const fn horsepower_calorie(x:f32)->f32{
        hp_cal(x)
    }

    /// Convert horsepower to calories.
    #[inline(always)]
    pub const fn horsepower_calories(x:f32)->f32{
        hp_cal(x)
    }

    /// Convert horsepower to watts.
    #[inline(always)]
    pub const fn hp_watt(x:f32)->f32{
        hp_w(x)
    }

    /// Convert horsepower to watts.
    #[inline(always)]
    pub const fn hp_watts(x:f32)->f32{
        hp_w(x)
    }

    /// Convert horsepower to watts.
    #[inline(always)]
    pub const fn horsepower_w(x:f32)->f32{
        hp_w(x)
    }

    /// Convert horsepower to watts.
    #[inline(always)]
    pub const fn horsepower_watt(x:f32)->f32{
        hp_w(x)
    }

    /// Convert horsepower to watts.
    #[inline(always)]
    pub const fn horsepower_watts(x:f32)->f32{
        hp_w(x)
    }

    /// Convert watts to joules.
    #[inline(always)]
    pub const fn w_joule(x:f32)->f32{
        w_j(x)
    }

    /// Convert watts to joules.
    #[inline(always)]
    pub const fn w_joules(x:f32)->f32{
        w_j(x)
    }

    /// Convert watts to joules.
    #[inline(always)]
    pub const fn watt_j(x:f32)->f32{
        w_j(x)
    }

    /// Convert watts to joules.
    #[inline(always)]
    pub const fn watt_joule(x:f32)->f32{
        w_j(x)
    }

    /// Convert watts to joules.
    #[inline(always)]
    pub const fn watt_joules(x:f32)->f32{
        w_j(x)
    }

    /// Convert watts to joules.
    #[inline(always)]
    pub const fn watts_j(x:f32)->f32{
        w_j(x)
    }

    /// Convert watts to joules.
    #[inline(always)]
    pub const fn watts_joule(x:f32)->f32{
        w_j(x)
    }

    /// Convert watts to joules.
    #[inline(always)]
    pub const fn watts_joules(x:f32)->f32{
        w_j(x)
    }

    /// Convert watts to calories.
    #[inline(always)]
    pub const fn w_calorie(x:f32)->f32{
        w_cal(x)
    }

    /// Convert watts to calories.
    #[inline(always)]
    pub const fn w_calories(x:f32)->f32{
        w_cal(x)
    }

    /// Convert watts to calories.
    #[inline(always)]
    pub const fn watt_cal(x:f32)->f32{
        w_cal(x)
    }

    /// Convert watts to calories.
    #[inline(always)]
    pub const fn watt_calorie(x:f32)->f32{
        w_cal(x)
    }

    /// Convert watts to calories.
    #[inline(always)]
    pub const fn watt_calories(x:f32)->f32{
        w_cal(x)
    }

    /// Convert watts to calories.
    #[inline(always)]
    pub const fn watts_cal(x:f32)->f32{
        w_cal(x)
    }

    /// Convert watts to calories.
    #[inline(always)]
    pub const fn watts_calorie(x:f32)->f32{
        w_cal(x)
    }

    /// Convert watts to calories.
    #[inline(always)]
    pub const fn watts_calories(x:f32)->f32{
        w_cal(x)
    }

    /// Convert watts to horsepower.
    #[inline(always)]
    pub const fn w_horsepower(x:f32)->f32{
        w_hp(x)
    }

    /// Convert watts to horsepower.
    #[inline(always)]
    pub const fn watt_hp(x:f32)->f32{
        w_hp(x)
    }

    /// Convert watts to horsepower.
    #[inline(always)]
    pub const fn watt_horsepower(x:f32)->f32{
        w_hp(x)
    }

    /// Convert watts to horsepower.
    #[inline(always)]
    pub const fn watts_hp(x:f32)->f32{
        w_hp(x)
    }

    /// Convert watts to horsepower.
    #[inline(always)]
    pub const fn watts_horsepower(x:f32)->f32{
        w_hp(x)
    }
}

pub mod time{
    /// Convert milliseconds to seconds.
    #[inline]
    pub const fn ms_s(x:f32)->f32{
        x*0.001
    }

    /// Convert milliseconds to minutes.
    #[inline]
    pub const fn ms_min(x:f32)->f32{
        x*0.00001666666667
    }

    /// Convert milliseconds to hours.
    #[inline]
    pub const fn ms_h(x:f32)->f32{
        x*0.00000027777778
    }

    /// Convert milliseconds to days.
    #[inline]
    pub const fn ms_d(x:f32)->f32{
        x*0.00000001157407
    }

    /// Convert milliseconds to weeks.
    #[inline]
    pub const fn ms_wk(x:f32)->f32{
        x*0.00000000165344
    }

    /// Convert milliseconds to months.
    #[inline]
    pub const fn ms_mo(x:f32)->f32{
        x*0.00000000038026
    }

    /// Convert milliseconds to years.
    #[inline]
    pub const fn ms_yr(x:f32)->f32{
        x*0.00000000003169
    }

    /// Convert seconds to milliseconds.
    #[inline]
    pub const fn s_ms(x:f32)->f32{
        x*1000.0
    }

    /// Convert seconds to minutes.
    #[inline]
    pub const fn s_min(x:f32)->f32{
        x*0.01666666666667
    }

    /// Convert seconds to hours.
    #[inline]
    pub const fn s_h(x:f32)->f32{
        x*0.00027777777778
    }

    /// Convert seconds to days.
    #[inline]
    pub const fn s_d(x:f32)->f32{
        x*0.00001157407407
    }

    /// Convert seconds to weeks.
    #[inline]
    pub const fn s_wk(x:f32)->f32{
        x*0.00000165343915
    }

    /// Convert seconds to months.
    #[inline]
    pub const fn s_mo(x:f32)->f32{
        x*0.00000038026486
    }

    /// Convert seconds to years.
    #[inline]
    pub const fn s_yr(x:f32)->f32{
        x*0.00000003168874
    }

    /// Convert minutes to milliseconds.
    #[inline]
    pub const fn min_ms(x:f32)->f32{
        x*60000.0
    }

    /// Convert minutes to seconds.
    #[inline]
    pub const fn min_s(x:f32)->f32{
        x*60.0
    }

    /// Convert minutes to hours.
    #[inline]
    pub const fn min_h(x:f32)->f32{
        x*0.01666666666667
    }

    /// Convert minutes to days.
    #[inline]
    pub const fn min_d(x:f32)->f32{
        x*0.00069444444444
    }

    /// Convert minutes to weeks.
    #[inline]
    pub const fn min_wk(x:f32)->f32{
        x*0.00009920634921
    }

    /// Convert minutes to months.
    #[inline]
    pub const fn min_mo(x:f32)->f32{
        x*0.00002281589172
    }

    /// Convert minutes to years.
    #[inline]
    pub const fn min_yr(x:f32)->f32{
        x*0.00000190132431
    }

    /// Convert hours to milliseconds.
    #[inline]
    pub const fn h_ms(x:f32)->f32{
        x*3600000.0
    }

    /// Convert hours to seconds.
    #[inline]
    pub const fn h_s(x:f32)->f32{
        x*3600.0
    }

    /// Convert hours to minutes.
    #[inline]
    pub const fn h_min(x:f32)->f32{
        x*60.0
    }

    /// Convert hours to days.
    #[inline]
    pub const fn h_d(x:f32)->f32{
        x*0.04166666666667
    }

    /// Convert hours to weeks.
    #[inline]
    pub const fn h_wk(x:f32)->f32{
        x*0.00595238095238
    }

    /// Convert hours to months.
    #[inline]
    pub const fn h_mo(x:f32)->f32{
        x*0.00136895350349
    }

    /// Convert hours to years.
    #[inline]
    pub const fn h_yr(x:f32)->f32{
        x*0.00011407945862
    }

    /// Convert days to milliseconds.
    #[inline]
    pub const fn d_ms(x:f32)->f32{
        x*86400000.0
    }

    /// Convert days to seconds.
    #[inline]
    pub const fn d_s(x:f32)->f32{
        x*86400.0
    }

    /// Convert days to minutes.
    #[inline]
    pub const fn d_min(x:f32)->f32{
        x*1440.0
    }

    /// Convert days to hours.
    #[inline]
    pub const fn d_h(x:f32)->f32{
        x*24.0
    }

    /// Convert days to weeks.
    #[inline]
    pub const fn d_wk(x:f32)->f32{
        x*0.14285714285714
    }

    /// Convert days to months.
    #[inline]
    pub const fn d_mo(x:f32)->f32{
        x*0.03285488408386
    }

    /// Convert days to years.
    #[inline]
    pub const fn d_yr(x:f32)->f32{
        x*0.00273790700699
    }

    /// Convert weeks to milliseconds.
    #[inline]
    pub const fn wk_ms(x:f32)->f32{
        x*604800000.0
    }

    /// Convert weeks to seconds.
    #[inline]
    pub const fn wk_s(x:f32)->f32{
        x*604800.0
    }

    /// Convert weeks to minutes.
    #[inline]
    pub const fn wk_min(x:f32)->f32{
        x*10080.0
    }

    /// Convert weeks to hours.
    #[inline]
    pub const fn wk_h(x:f32)->f32{
        x*168.0
    }

    /// Convert weeks to days.
    #[inline]
    pub const fn wk_d(x:f32)->f32{
        x*7.0
    }

    /// Convert weeks to months.
    #[inline]
    pub const fn wk_mo(x:f32)->f32{
        x*0.22998418858703
    }

    /// Convert weeks to years.
    #[inline]
    pub const fn wk_yr(x:f32)->f32{
        x*0.01916534904892
    }

    /// Convert months to milliseconds.
    #[inline]
    pub const fn mo_ms(x:f32)->f32{
        x*2629746000.0
    }

    /// Convert months to seconds.
    #[inline]
    pub const fn mo_s(x:f32)->f32{
        x*2629746.0
    }

    /// Convert months to minutes.
    #[inline]
    pub const fn mo_min(x:f32)->f32{
        x*43829.09999999999854
    }

    /// Convert months to hours.
    #[inline]
    pub const fn mo_h(x:f32)->f32{
        x*730.485
    }

    /// Convert months to days.
    #[inline]
    pub const fn mo_d(x:f32)->f32{
        x*30.436875
    }

    /// Convert months to weeks.
    #[inline]
    pub const fn mo_wk(x:f32)->f32{
        x*4.348125
    }

    /// Convert months to years.
    #[inline]
    pub const fn mo_yr(x:f32)->f32{
        x*0.08333333333333
    }

    /// Convert years to milliseconds.
    #[inline]
    pub const fn yr_ms(x:f32)->f32{
        x*31556952000.0
    }

    /// Convert years to seconds.
    #[inline]
    pub const fn yr_s(x:f32)->f32{
        x*31556952.0
    }

    /// Convert years to minutes.
    #[inline]
    pub const fn yr_min(x:f32)->f32{
        x*525949.19999999995343
    }

    /// Convert years to hours.
    #[inline]
    pub const fn yr_h(x:f32)->f32{
        x*8765.81999999999971
    }

    /// Convert years to days.
    #[inline]
    pub const fn yr_d(x:f32)->f32{
        x*365.24250000000001
    }

    /// Convert years to weeks.
    #[inline]
    pub const fn yr_wk(x:f32)->f32{
        x*52.1775
    }

    /// Convert years to months.
    #[inline]
    pub const fn yr_mo(x:f32)->f32{
        x*12.0
    }

    /// Convert milliseconds to seconds.
    #[inline(always)]
    pub const fn ms_second(x:f32)->f32{
        ms_s(x)
    }

    /// Convert milliseconds to seconds.
    #[inline(always)]
    pub const fn ms_seconds(x:f32)->f32{
        ms_s(x)
    }

    /// Convert milliseconds to seconds.
    #[inline(always)]
    pub const fn millisecond_s(x:f32)->f32{
        ms_s(x)
    }

    /// Convert milliseconds to seconds.
    #[inline(always)]
    pub const fn millisecond_second(x:f32)->f32{
        ms_s(x)
    }

    /// Convert milliseconds to seconds.
    #[inline(always)]
    pub const fn millisecond_seconds(x:f32)->f32{
        ms_s(x)
    }

    /// Convert milliseconds to seconds.
    #[inline(always)]
    pub const fn milliseconds_s(x:f32)->f32{
        ms_s(x)
    }

    /// Convert milliseconds to seconds.
    #[inline(always)]
    pub const fn milliseconds_second(x:f32)->f32{
        ms_s(x)
    }

    /// Convert milliseconds to seconds.
    #[inline(always)]
    pub const fn milliseconds_seconds(x:f32)->f32{
        ms_s(x)
    }

    /// Convert milliseconds to minutes.
    #[inline(always)]
    pub const fn ms_minute(x:f32)->f32{
        ms_min(x)
    }

    /// Convert milliseconds to minutes.
    #[inline(always)]
    pub const fn ms_minutes(x:f32)->f32{
        ms_min(x)
    }

    /// Convert milliseconds to minutes.
    #[inline(always)]
    pub const fn millisecond_min(x:f32)->f32{
        ms_min(x)
    }

    /// Convert milliseconds to minutes.
    #[inline(always)]
    pub const fn millisecond_minute(x:f32)->f32{
        ms_min(x)
    }

    /// Convert milliseconds to minutes.
    #[inline(always)]
    pub const fn millisecond_minutes(x:f32)->f32{
        ms_min(x)
    }

    /// Convert milliseconds to minutes.
    #[inline(always)]
    pub const fn milliseconds_min(x:f32)->f32{
        ms_min(x)
    }

    /// Convert milliseconds to minutes.
    #[inline(always)]
    pub const fn milliseconds_minute(x:f32)->f32{
        ms_min(x)
    }

    /// Convert milliseconds to minutes.
    #[inline(always)]
    pub const fn milliseconds_minutes(x:f32)->f32{
        ms_min(x)
    }

    /// Convert milliseconds to hours.
    #[inline(always)]
    pub const fn ms_hour(x:f32)->f32{
        ms_h(x)
    }

    /// Convert milliseconds to hours.
    #[inline(always)]
    pub const fn ms_hours(x:f32)->f32{
        ms_h(x)
    }

    /// Convert milliseconds to hours.
    #[inline(always)]
    pub const fn millisecond_h(x:f32)->f32{
        ms_h(x)
    }

    /// Convert milliseconds to hours.
    #[inline(always)]
    pub const fn millisecond_hour(x:f32)->f32{
        ms_h(x)
    }

    /// Convert milliseconds to hours.
    #[inline(always)]
    pub const fn millisecond_hours(x:f32)->f32{
        ms_h(x)
    }

    /// Convert milliseconds to hours.
    #[inline(always)]
    pub const fn milliseconds_h(x:f32)->f32{
        ms_h(x)
    }

    /// Convert milliseconds to hours.
    #[inline(always)]
    pub const fn milliseconds_hour(x:f32)->f32{
        ms_h(x)
    }

    /// Convert milliseconds to hours.
    #[inline(always)]
    pub const fn milliseconds_hours(x:f32)->f32{
        ms_h(x)
    }

    /// Convert milliseconds to days.
    #[inline(always)]
    pub const fn ms_day(x:f32)->f32{
        ms_d(x)
    }

    /// Convert milliseconds to days.
    #[inline(always)]
    pub const fn ms_days(x:f32)->f32{
        ms_d(x)
    }

    /// Convert milliseconds to days.
    #[inline(always)]
    pub const fn millisecond_d(x:f32)->f32{
        ms_d(x)
    }

    /// Convert milliseconds to days.
    #[inline(always)]
    pub const fn millisecond_day(x:f32)->f32{
        ms_d(x)
    }

    /// Convert milliseconds to days.
    #[inline(always)]
    pub const fn millisecond_days(x:f32)->f32{
        ms_d(x)
    }

    /// Convert milliseconds to days.
    #[inline(always)]
    pub const fn milliseconds_d(x:f32)->f32{
        ms_d(x)
    }

    /// Convert milliseconds to days.
    #[inline(always)]
    pub const fn milliseconds_day(x:f32)->f32{
        ms_d(x)
    }

    /// Convert milliseconds to days.
    #[inline(always)]
    pub const fn milliseconds_days(x:f32)->f32{
        ms_d(x)
    }

    /// Convert milliseconds to weeks.
    #[inline(always)]
    pub const fn ms_week(x:f32)->f32{
        ms_wk(x)
    }

    /// Convert milliseconds to weeks.
    #[inline(always)]
    pub const fn ms_weeks(x:f32)->f32{
        ms_wk(x)
    }

    /// Convert milliseconds to weeks.
    #[inline(always)]
    pub const fn millisecond_wk(x:f32)->f32{
        ms_wk(x)
    }

    /// Convert milliseconds to weeks.
    #[inline(always)]
    pub const fn millisecond_week(x:f32)->f32{
        ms_wk(x)
    }

    /// Convert milliseconds to weeks.
    #[inline(always)]
    pub const fn millisecond_weeks(x:f32)->f32{
        ms_wk(x)
    }

    /// Convert milliseconds to weeks.
    #[inline(always)]
    pub const fn milliseconds_wk(x:f32)->f32{
        ms_wk(x)
    }

    /// Convert milliseconds to weeks.
    #[inline(always)]
    pub const fn milliseconds_week(x:f32)->f32{
        ms_wk(x)
    }

    /// Convert milliseconds to weeks.
    #[inline(always)]
    pub const fn milliseconds_weeks(x:f32)->f32{
        ms_wk(x)
    }

    /// Convert milliseconds to months.
    #[inline(always)]
    pub const fn ms_month(x:f32)->f32{
        ms_mo(x)
    }

    /// Convert milliseconds to months.
    #[inline(always)]
    pub const fn ms_months(x:f32)->f32{
        ms_mo(x)
    }

    /// Convert milliseconds to months.
    #[inline(always)]
    pub const fn millisecond_mo(x:f32)->f32{
        ms_mo(x)
    }

    /// Convert milliseconds to months.
    #[inline(always)]
    pub const fn millisecond_month(x:f32)->f32{
        ms_mo(x)
    }

    /// Convert milliseconds to months.
    #[inline(always)]
    pub const fn millisecond_months(x:f32)->f32{
        ms_mo(x)
    }

    /// Convert milliseconds to months.
    #[inline(always)]
    pub const fn milliseconds_mo(x:f32)->f32{
        ms_mo(x)
    }

    /// Convert milliseconds to months.
    #[inline(always)]
    pub const fn milliseconds_month(x:f32)->f32{
        ms_mo(x)
    }

    /// Convert milliseconds to months.
    #[inline(always)]
    pub const fn milliseconds_months(x:f32)->f32{
        ms_mo(x)
    }

    /// Convert milliseconds to years.
    #[inline(always)]
    pub const fn ms_year(x:f32)->f32{
        ms_yr(x)
    }

    /// Convert milliseconds to years.
    #[inline(always)]
    pub const fn ms_years(x:f32)->f32{
        ms_yr(x)
    }

    /// Convert milliseconds to years.
    #[inline(always)]
    pub const fn millisecond_yr(x:f32)->f32{
        ms_yr(x)
    }

    /// Convert milliseconds to years.
    #[inline(always)]
    pub const fn millisecond_year(x:f32)->f32{
        ms_yr(x)
    }

    /// Convert milliseconds to years.
    #[inline(always)]
    pub const fn millisecond_years(x:f32)->f32{
        ms_yr(x)
    }

    /// Convert milliseconds to years.
    #[inline(always)]
    pub const fn milliseconds_yr(x:f32)->f32{
        ms_yr(x)
    }

    /// Convert milliseconds to years.
    #[inline(always)]
    pub const fn milliseconds_year(x:f32)->f32{
        ms_yr(x)
    }

    /// Convert milliseconds to years.
    #[inline(always)]
    pub const fn milliseconds_years(x:f32)->f32{
        ms_yr(x)
    }

    /// Convert seconds to milliseconds.
    #[inline(always)]
    pub const fn s_millisecond(x:f32)->f32{
        s_ms(x)
    }

    /// Convert seconds to milliseconds.
    #[inline(always)]
    pub const fn s_milliseconds(x:f32)->f32{
        s_ms(x)
    }

    /// Convert seconds to milliseconds.
    #[inline(always)]
    pub const fn second_ms(x:f32)->f32{
        s_ms(x)
    }

    /// Convert seconds to milliseconds.
    #[inline(always)]
    pub const fn second_millisecond(x:f32)->f32{
        s_ms(x)
    }

    /// Convert seconds to milliseconds.
    #[inline(always)]
    pub const fn second_milliseconds(x:f32)->f32{
        s_ms(x)
    }

    /// Convert seconds to milliseconds.
    #[inline(always)]
    pub const fn seconds_ms(x:f32)->f32{
        s_ms(x)
    }

    /// Convert seconds to milliseconds.
    #[inline(always)]
    pub const fn seconds_millisecond(x:f32)->f32{
        s_ms(x)
    }

    /// Convert seconds to milliseconds.
    #[inline(always)]
    pub const fn seconds_milliseconds(x:f32)->f32{
        s_ms(x)
    }

    /// Convert seconds to minutes.
    #[inline(always)]
    pub const fn s_minute(x:f32)->f32{
        s_min(x)
    }

    /// Convert seconds to minutes.
    #[inline(always)]
    pub const fn s_minutes(x:f32)->f32{
        s_min(x)
    }

    /// Convert seconds to minutes.
    #[inline(always)]
    pub const fn second_min(x:f32)->f32{
        s_min(x)
    }

    /// Convert seconds to minutes.
    #[inline(always)]
    pub const fn second_minute(x:f32)->f32{
        s_min(x)
    }

    /// Convert seconds to minutes.
    #[inline(always)]
    pub const fn second_minutes(x:f32)->f32{
        s_min(x)
    }

    /// Convert seconds to minutes.
    #[inline(always)]
    pub const fn seconds_min(x:f32)->f32{
        s_min(x)
    }

    /// Convert seconds to minutes.
    #[inline(always)]
    pub const fn seconds_minute(x:f32)->f32{
        s_min(x)
    }

    /// Convert seconds to minutes.
    #[inline(always)]
    pub const fn seconds_minutes(x:f32)->f32{
        s_min(x)
    }

    /// Convert seconds to hours.
    #[inline(always)]
    pub const fn s_hour(x:f32)->f32{
        s_h(x)
    }

    /// Convert seconds to hours.
    #[inline(always)]
    pub const fn s_hours(x:f32)->f32{
        s_h(x)
    }

    /// Convert seconds to hours.
    #[inline(always)]
    pub const fn second_h(x:f32)->f32{
        s_h(x)
    }

    /// Convert seconds to hours.
    #[inline(always)]
    pub const fn second_hour(x:f32)->f32{
        s_h(x)
    }

    /// Convert seconds to hours.
    #[inline(always)]
    pub const fn second_hours(x:f32)->f32{
        s_h(x)
    }

    /// Convert seconds to hours.
    #[inline(always)]
    pub const fn seconds_h(x:f32)->f32{
        s_h(x)
    }

    /// Convert seconds to hours.
    #[inline(always)]
    pub const fn seconds_hour(x:f32)->f32{
        s_h(x)
    }

    /// Convert seconds to hours.
    #[inline(always)]
    pub const fn seconds_hours(x:f32)->f32{
        s_h(x)
    }

    /// Convert seconds to days.
    #[inline(always)]
    pub const fn s_day(x:f32)->f32{
        s_d(x)
    }

    /// Convert seconds to days.
    #[inline(always)]
    pub const fn s_days(x:f32)->f32{
        s_d(x)
    }

    /// Convert seconds to days.
    #[inline(always)]
    pub const fn second_d(x:f32)->f32{
        s_d(x)
    }

    /// Convert seconds to days.
    #[inline(always)]
    pub const fn second_day(x:f32)->f32{
        s_d(x)
    }

    /// Convert seconds to days.
    #[inline(always)]
    pub const fn second_days(x:f32)->f32{
        s_d(x)
    }

    /// Convert seconds to days.
    #[inline(always)]
    pub const fn seconds_d(x:f32)->f32{
        s_d(x)
    }

    /// Convert seconds to days.
    #[inline(always)]
    pub const fn seconds_day(x:f32)->f32{
        s_d(x)
    }

    /// Convert seconds to days.
    #[inline(always)]
    pub const fn seconds_days(x:f32)->f32{
        s_d(x)
    }

    /// Convert seconds to weeks.
    #[inline(always)]
    pub const fn s_week(x:f32)->f32{
        s_wk(x)
    }

    /// Convert seconds to weeks.
    #[inline(always)]
    pub const fn s_weeks(x:f32)->f32{
        s_wk(x)
    }

    /// Convert seconds to weeks.
    #[inline(always)]
    pub const fn second_wk(x:f32)->f32{
        s_wk(x)
    }

    /// Convert seconds to weeks.
    #[inline(always)]
    pub const fn second_week(x:f32)->f32{
        s_wk(x)
    }

    /// Convert seconds to weeks.
    #[inline(always)]
    pub const fn second_weeks(x:f32)->f32{
        s_wk(x)
    }

    /// Convert seconds to weeks.
    #[inline(always)]
    pub const fn seconds_wk(x:f32)->f32{
        s_wk(x)
    }

    /// Convert seconds to weeks.
    #[inline(always)]
    pub const fn seconds_week(x:f32)->f32{
        s_wk(x)
    }

    /// Convert seconds to weeks.
    #[inline(always)]
    pub const fn seconds_weeks(x:f32)->f32{
        s_wk(x)
    }

    /// Convert seconds to months.
    #[inline(always)]
    pub const fn s_month(x:f32)->f32{
        s_mo(x)
    }

    /// Convert seconds to months.
    #[inline(always)]
    pub const fn s_months(x:f32)->f32{
        s_mo(x)
    }

    /// Convert seconds to months.
    #[inline(always)]
    pub const fn second_mo(x:f32)->f32{
        s_mo(x)
    }

    /// Convert seconds to months.
    #[inline(always)]
    pub const fn second_month(x:f32)->f32{
        s_mo(x)
    }

    /// Convert seconds to months.
    #[inline(always)]
    pub const fn second_months(x:f32)->f32{
        s_mo(x)
    }

    /// Convert seconds to months.
    #[inline(always)]
    pub const fn seconds_mo(x:f32)->f32{
        s_mo(x)
    }

    /// Convert seconds to months.
    #[inline(always)]
    pub const fn seconds_month(x:f32)->f32{
        s_mo(x)
    }

    /// Convert seconds to months.
    #[inline(always)]
    pub const fn seconds_months(x:f32)->f32{
        s_mo(x)
    }

    /// Convert seconds to years.
    #[inline(always)]
    pub const fn s_year(x:f32)->f32{
        s_yr(x)
    }

    /// Convert seconds to years.
    #[inline(always)]
    pub const fn s_years(x:f32)->f32{
        s_yr(x)
    }

    /// Convert seconds to years.
    #[inline(always)]
    pub const fn second_yr(x:f32)->f32{
        s_yr(x)
    }

    /// Convert seconds to years.
    #[inline(always)]
    pub const fn second_year(x:f32)->f32{
        s_yr(x)
    }

    /// Convert seconds to years.
    #[inline(always)]
    pub const fn second_years(x:f32)->f32{
        s_yr(x)
    }

    /// Convert seconds to years.
    #[inline(always)]
    pub const fn seconds_yr(x:f32)->f32{
        s_yr(x)
    }

    /// Convert seconds to years.
    #[inline(always)]
    pub const fn seconds_year(x:f32)->f32{
        s_yr(x)
    }

    /// Convert seconds to years.
    #[inline(always)]
    pub const fn seconds_years(x:f32)->f32{
        s_yr(x)
    }

    /// Convert minutes to milliseconds.
    #[inline(always)]
    pub const fn min_millisecond(x:f32)->f32{
        min_ms(x)
    }

    /// Convert minutes to milliseconds.
    #[inline(always)]
    pub const fn min_milliseconds(x:f32)->f32{
        min_ms(x)
    }

    /// Convert minutes to milliseconds.
    #[inline(always)]
    pub const fn minute_ms(x:f32)->f32{
        min_ms(x)
    }

    /// Convert minutes to milliseconds.
    #[inline(always)]
    pub const fn minute_millisecond(x:f32)->f32{
        min_ms(x)
    }

    /// Convert minutes to milliseconds.
    #[inline(always)]
    pub const fn minute_milliseconds(x:f32)->f32{
        min_ms(x)
    }

    /// Convert minutes to milliseconds.
    #[inline(always)]
    pub const fn minutes_ms(x:f32)->f32{
        min_ms(x)
    }

    /// Convert minutes to milliseconds.
    #[inline(always)]
    pub const fn minutes_millisecond(x:f32)->f32{
        min_ms(x)
    }

    /// Convert minutes to milliseconds.
    #[inline(always)]
    pub const fn minutes_milliseconds(x:f32)->f32{
        min_ms(x)
    }

    /// Convert minutes to seconds.
    #[inline(always)]
    pub const fn min_second(x:f32)->f32{
        min_s(x)
    }

    /// Convert minutes to seconds.
    #[inline(always)]
    pub const fn min_seconds(x:f32)->f32{
        min_s(x)
    }

    /// Convert minutes to seconds.
    #[inline(always)]
    pub const fn minute_s(x:f32)->f32{
        min_s(x)
    }

    /// Convert minutes to seconds.
    #[inline(always)]
    pub const fn minute_second(x:f32)->f32{
        min_s(x)
    }

    /// Convert minutes to seconds.
    #[inline(always)]
    pub const fn minute_seconds(x:f32)->f32{
        min_s(x)
    }

    /// Convert minutes to seconds.
    #[inline(always)]
    pub const fn minutes_s(x:f32)->f32{
        min_s(x)
    }

    /// Convert minutes to seconds.
    #[inline(always)]
    pub const fn minutes_second(x:f32)->f32{
        min_s(x)
    }

    /// Convert minutes to seconds.
    #[inline(always)]
    pub const fn minutes_seconds(x:f32)->f32{
        min_s(x)
    }

    /// Convert minutes to hours.
    #[inline(always)]
    pub const fn min_hour(x:f32)->f32{
        min_h(x)
    }

    /// Convert minutes to hours.
    #[inline(always)]
    pub const fn min_hours(x:f32)->f32{
        min_h(x)
    }

    /// Convert minutes to hours.
    #[inline(always)]
    pub const fn minute_h(x:f32)->f32{
        min_h(x)
    }

    /// Convert minutes to hours.
    #[inline(always)]
    pub const fn minute_hour(x:f32)->f32{
        min_h(x)
    }

    /// Convert minutes to hours.
    #[inline(always)]
    pub const fn minute_hours(x:f32)->f32{
        min_h(x)
    }

    /// Convert minutes to hours.
    #[inline(always)]
    pub const fn minutes_h(x:f32)->f32{
        min_h(x)
    }

    /// Convert minutes to hours.
    #[inline(always)]
    pub const fn minutes_hour(x:f32)->f32{
        min_h(x)
    }

    /// Convert minutes to hours.
    #[inline(always)]
    pub const fn minutes_hours(x:f32)->f32{
        min_h(x)
    }

    /// Convert minutes to days.
    #[inline(always)]
    pub const fn min_day(x:f32)->f32{
        min_d(x)
    }

    /// Convert minutes to days.
    #[inline(always)]
    pub const fn min_days(x:f32)->f32{
        min_d(x)
    }

    /// Convert minutes to days.
    #[inline(always)]
    pub const fn minute_d(x:f32)->f32{
        min_d(x)
    }

    /// Convert minutes to days.
    #[inline(always)]
    pub const fn minute_day(x:f32)->f32{
        min_d(x)
    }

    /// Convert minutes to days.
    #[inline(always)]
    pub const fn minute_days(x:f32)->f32{
        min_d(x)
    }

    /// Convert minutes to days.
    #[inline(always)]
    pub const fn minutes_d(x:f32)->f32{
        min_d(x)
    }

    /// Convert minutes to days.
    #[inline(always)]
    pub const fn minutes_day(x:f32)->f32{
        min_d(x)
    }

    /// Convert minutes to days.
    #[inline(always)]
    pub const fn minutes_days(x:f32)->f32{
        min_d(x)
    }

    /// Convert minutes to weeks.
    #[inline(always)]
    pub const fn min_week(x:f32)->f32{
        min_wk(x)
    }

    /// Convert minutes to weeks.
    #[inline(always)]
    pub const fn min_weeks(x:f32)->f32{
        min_wk(x)
    }

    /// Convert minutes to weeks.
    #[inline(always)]
    pub const fn minute_wk(x:f32)->f32{
        min_wk(x)
    }

    /// Convert minutes to weeks.
    #[inline(always)]
    pub const fn minute_week(x:f32)->f32{
        min_wk(x)
    }

    /// Convert minutes to weeks.
    #[inline(always)]
    pub const fn minute_weeks(x:f32)->f32{
        min_wk(x)
    }

    /// Convert minutes to weeks.
    #[inline(always)]
    pub const fn minutes_wk(x:f32)->f32{
        min_wk(x)
    }

    /// Convert minutes to weeks.
    #[inline(always)]
    pub const fn minutes_week(x:f32)->f32{
        min_wk(x)
    }

    /// Convert minutes to weeks.
    #[inline(always)]
    pub const fn minutes_weeks(x:f32)->f32{
        min_wk(x)
    }

    /// Convert minutes to months.
    #[inline(always)]
    pub const fn min_month(x:f32)->f32{
        min_mo(x)
    }

    /// Convert minutes to months.
    #[inline(always)]
    pub const fn min_months(x:f32)->f32{
        min_mo(x)
    }

    /// Convert minutes to months.
    #[inline(always)]
    pub const fn minute_mo(x:f32)->f32{
        min_mo(x)
    }

    /// Convert minutes to months.
    #[inline(always)]
    pub const fn minute_month(x:f32)->f32{
        min_mo(x)
    }

    /// Convert minutes to months.
    #[inline(always)]
    pub const fn minute_months(x:f32)->f32{
        min_mo(x)
    }

    /// Convert minutes to months.
    #[inline(always)]
    pub const fn minutes_mo(x:f32)->f32{
        min_mo(x)
    }

    /// Convert minutes to months.
    #[inline(always)]
    pub const fn minutes_month(x:f32)->f32{
        min_mo(x)
    }

    /// Convert minutes to months.
    #[inline(always)]
    pub const fn minutes_months(x:f32)->f32{
        min_mo(x)
    }

    /// Convert minutes to years.
    #[inline(always)]
    pub const fn min_year(x:f32)->f32{
        min_yr(x)
    }

    /// Convert minutes to years.
    #[inline(always)]
    pub const fn min_years(x:f32)->f32{
        min_yr(x)
    }

    /// Convert minutes to years.
    #[inline(always)]
    pub const fn minute_yr(x:f32)->f32{
        min_yr(x)
    }

    /// Convert minutes to years.
    #[inline(always)]
    pub const fn minute_year(x:f32)->f32{
        min_yr(x)
    }

    /// Convert minutes to years.
    #[inline(always)]
    pub const fn minute_years(x:f32)->f32{
        min_yr(x)
    }

    /// Convert minutes to years.
    #[inline(always)]
    pub const fn minutes_yr(x:f32)->f32{
        min_yr(x)
    }

    /// Convert minutes to years.
    #[inline(always)]
    pub const fn minutes_year(x:f32)->f32{
        min_yr(x)
    }

    /// Convert minutes to years.
    #[inline(always)]
    pub const fn minutes_years(x:f32)->f32{
        min_yr(x)
    }

    /// Convert hours to milliseconds.
    #[inline(always)]
    pub const fn h_millisecond(x:f32)->f32{
        h_ms(x)
    }

    /// Convert hours to milliseconds.
    #[inline(always)]
    pub const fn h_milliseconds(x:f32)->f32{
        h_ms(x)
    }

    /// Convert hours to milliseconds.
    #[inline(always)]
    pub const fn hour_ms(x:f32)->f32{
        h_ms(x)
    }

    /// Convert hours to milliseconds.
    #[inline(always)]
    pub const fn hour_millisecond(x:f32)->f32{
        h_ms(x)
    }

    /// Convert hours to milliseconds.
    #[inline(always)]
    pub const fn hour_milliseconds(x:f32)->f32{
        h_ms(x)
    }

    /// Convert hours to milliseconds.
    #[inline(always)]
    pub const fn hours_ms(x:f32)->f32{
        h_ms(x)
    }

    /// Convert hours to milliseconds.
    #[inline(always)]
    pub const fn hours_millisecond(x:f32)->f32{
        h_ms(x)
    }

    /// Convert hours to milliseconds.
    #[inline(always)]
    pub const fn hours_milliseconds(x:f32)->f32{
        h_ms(x)
    }

    /// Convert hours to seconds.
    #[inline(always)]
    pub const fn h_second(x:f32)->f32{
        h_s(x)
    }

    /// Convert hours to seconds.
    #[inline(always)]
    pub const fn h_seconds(x:f32)->f32{
        h_s(x)
    }

    /// Convert hours to seconds.
    #[inline(always)]
    pub const fn hour_s(x:f32)->f32{
        h_s(x)
    }

    /// Convert hours to seconds.
    #[inline(always)]
    pub const fn hour_second(x:f32)->f32{
        h_s(x)
    }

    /// Convert hours to seconds.
    #[inline(always)]
    pub const fn hour_seconds(x:f32)->f32{
        h_s(x)
    }

    /// Convert hours to seconds.
    #[inline(always)]
    pub const fn hours_s(x:f32)->f32{
        h_s(x)
    }

    /// Convert hours to seconds.
    #[inline(always)]
    pub const fn hours_second(x:f32)->f32{
        h_s(x)
    }

    /// Convert hours to seconds.
    #[inline(always)]
    pub const fn hours_seconds(x:f32)->f32{
        h_s(x)
    }

    /// Convert hours to minutes.
    #[inline(always)]
    pub const fn h_minute(x:f32)->f32{
        h_min(x)
    }

    /// Convert hours to minutes.
    #[inline(always)]
    pub const fn h_minutes(x:f32)->f32{
        h_min(x)
    }

    /// Convert hours to minutes.
    #[inline(always)]
    pub const fn hour_min(x:f32)->f32{
        h_min(x)
    }

    /// Convert hours to minutes.
    #[inline(always)]
    pub const fn hour_minute(x:f32)->f32{
        h_min(x)
    }

    /// Convert hours to minutes.
    #[inline(always)]
    pub const fn hour_minutes(x:f32)->f32{
        h_min(x)
    }

    /// Convert hours to minutes.
    #[inline(always)]
    pub const fn hours_min(x:f32)->f32{
        h_min(x)
    }

    /// Convert hours to minutes.
    #[inline(always)]
    pub const fn hours_minute(x:f32)->f32{
        h_min(x)
    }

    /// Convert hours to minutes.
    #[inline(always)]
    pub const fn hours_minutes(x:f32)->f32{
        h_min(x)
    }

    /// Convert hours to days.
    #[inline(always)]
    pub const fn h_day(x:f32)->f32{
        h_d(x)
    }

    /// Convert hours to days.
    #[inline(always)]
    pub const fn h_days(x:f32)->f32{
        h_d(x)
    }

    /// Convert hours to days.
    #[inline(always)]
    pub const fn hour_d(x:f32)->f32{
        h_d(x)
    }

    /// Convert hours to days.
    #[inline(always)]
    pub const fn hour_day(x:f32)->f32{
        h_d(x)
    }

    /// Convert hours to days.
    #[inline(always)]
    pub const fn hour_days(x:f32)->f32{
        h_d(x)
    }

    /// Convert hours to days.
    #[inline(always)]
    pub const fn hours_d(x:f32)->f32{
        h_d(x)
    }

    /// Convert hours to days.
    #[inline(always)]
    pub const fn hours_day(x:f32)->f32{
        h_d(x)
    }

    /// Convert hours to days.
    #[inline(always)]
    pub const fn hours_days(x:f32)->f32{
        h_d(x)
    }

    /// Convert hours to weeks.
    #[inline(always)]
    pub const fn h_week(x:f32)->f32{
        h_wk(x)
    }

    /// Convert hours to weeks.
    #[inline(always)]
    pub const fn h_weeks(x:f32)->f32{
        h_wk(x)
    }

    /// Convert hours to weeks.
    #[inline(always)]
    pub const fn hour_wk(x:f32)->f32{
        h_wk(x)
    }

    /// Convert hours to weeks.
    #[inline(always)]
    pub const fn hour_week(x:f32)->f32{
        h_wk(x)
    }

    /// Convert hours to weeks.
    #[inline(always)]
    pub const fn hour_weeks(x:f32)->f32{
        h_wk(x)
    }

    /// Convert hours to weeks.
    #[inline(always)]
    pub const fn hours_wk(x:f32)->f32{
        h_wk(x)
    }

    /// Convert hours to weeks.
    #[inline(always)]
    pub const fn hours_week(x:f32)->f32{
        h_wk(x)
    }

    /// Convert hours to weeks.
    #[inline(always)]
    pub const fn hours_weeks(x:f32)->f32{
        h_wk(x)
    }

    /// Convert hours to months.
    #[inline(always)]
    pub const fn h_month(x:f32)->f32{
        h_mo(x)
    }

    /// Convert hours to months.
    #[inline(always)]
    pub const fn h_months(x:f32)->f32{
        h_mo(x)
    }

    /// Convert hours to months.
    #[inline(always)]
    pub const fn hour_mo(x:f32)->f32{
        h_mo(x)
    }

    /// Convert hours to months.
    #[inline(always)]
    pub const fn hour_month(x:f32)->f32{
        h_mo(x)
    }

    /// Convert hours to months.
    #[inline(always)]
    pub const fn hour_months(x:f32)->f32{
        h_mo(x)
    }

    /// Convert hours to months.
    #[inline(always)]
    pub const fn hours_mo(x:f32)->f32{
        h_mo(x)
    }

    /// Convert hours to months.
    #[inline(always)]
    pub const fn hours_month(x:f32)->f32{
        h_mo(x)
    }

    /// Convert hours to months.
    #[inline(always)]
    pub const fn hours_months(x:f32)->f32{
        h_mo(x)
    }

    /// Convert hours to years.
    #[inline(always)]
    pub const fn h_year(x:f32)->f32{
        h_yr(x)
    }

    /// Convert hours to years.
    #[inline(always)]
    pub const fn h_years(x:f32)->f32{
        h_yr(x)
    }

    /// Convert hours to years.
    #[inline(always)]
    pub const fn hour_yr(x:f32)->f32{
        h_yr(x)
    }

    /// Convert hours to years.
    #[inline(always)]
    pub const fn hour_year(x:f32)->f32{
        h_yr(x)
    }

    /// Convert hours to years.
    #[inline(always)]
    pub const fn hour_years(x:f32)->f32{
        h_yr(x)
    }

    /// Convert hours to years.
    #[inline(always)]
    pub const fn hours_yr(x:f32)->f32{
        h_yr(x)
    }

    /// Convert hours to years.
    #[inline(always)]
    pub const fn hours_year(x:f32)->f32{
        h_yr(x)
    }

    /// Convert hours to years.
    #[inline(always)]
    pub const fn hours_years(x:f32)->f32{
        h_yr(x)
    }

    /// Convert days to milliseconds.
    #[inline(always)]
    pub const fn d_millisecond(x:f32)->f32{
        d_ms(x)
    }

    /// Convert days to milliseconds.
    #[inline(always)]
    pub const fn d_milliseconds(x:f32)->f32{
        d_ms(x)
    }

    /// Convert days to milliseconds.
    #[inline(always)]
    pub const fn day_ms(x:f32)->f32{
        d_ms(x)
    }

    /// Convert days to milliseconds.
    #[inline(always)]
    pub const fn day_millisecond(x:f32)->f32{
        d_ms(x)
    }

    /// Convert days to milliseconds.
    #[inline(always)]
    pub const fn day_milliseconds(x:f32)->f32{
        d_ms(x)
    }

    /// Convert days to milliseconds.
    #[inline(always)]
    pub const fn days_ms(x:f32)->f32{
        d_ms(x)
    }

    /// Convert days to milliseconds.
    #[inline(always)]
    pub const fn days_millisecond(x:f32)->f32{
        d_ms(x)
    }

    /// Convert days to milliseconds.
    #[inline(always)]
    pub const fn days_milliseconds(x:f32)->f32{
        d_ms(x)
    }

    /// Convert days to seconds.
    #[inline(always)]
    pub const fn d_second(x:f32)->f32{
        d_s(x)
    }

    /// Convert days to seconds.
    #[inline(always)]
    pub const fn d_seconds(x:f32)->f32{
        d_s(x)
    }

    /// Convert days to seconds.
    #[inline(always)]
    pub const fn day_s(x:f32)->f32{
        d_s(x)
    }

    /// Convert days to seconds.
    #[inline(always)]
    pub const fn day_second(x:f32)->f32{
        d_s(x)
    }

    /// Convert days to seconds.
    #[inline(always)]
    pub const fn day_seconds(x:f32)->f32{
        d_s(x)
    }

    /// Convert days to seconds.
    #[inline(always)]
    pub const fn days_s(x:f32)->f32{
        d_s(x)
    }

    /// Convert days to seconds.
    #[inline(always)]
    pub const fn days_second(x:f32)->f32{
        d_s(x)
    }

    /// Convert days to seconds.
    #[inline(always)]
    pub const fn days_seconds(x:f32)->f32{
        d_s(x)
    }

    /// Convert days to minutes.
    #[inline(always)]
    pub const fn d_minute(x:f32)->f32{
        d_min(x)
    }

    /// Convert days to minutes.
    #[inline(always)]
    pub const fn d_minutes(x:f32)->f32{
        d_min(x)
    }

    /// Convert days to minutes.
    #[inline(always)]
    pub const fn day_min(x:f32)->f32{
        d_min(x)
    }

    /// Convert days to minutes.
    #[inline(always)]
    pub const fn day_minute(x:f32)->f32{
        d_min(x)
    }

    /// Convert days to minutes.
    #[inline(always)]
    pub const fn day_minutes(x:f32)->f32{
        d_min(x)
    }

    /// Convert days to minutes.
    #[inline(always)]
    pub const fn days_min(x:f32)->f32{
        d_min(x)
    }

    /// Convert days to minutes.
    #[inline(always)]
    pub const fn days_minute(x:f32)->f32{
        d_min(x)
    }

    /// Convert days to minutes.
    #[inline(always)]
    pub const fn days_minutes(x:f32)->f32{
        d_min(x)
    }

    /// Convert days to hours.
    #[inline(always)]
    pub const fn d_hour(x:f32)->f32{
        d_h(x)
    }

    /// Convert days to hours.
    #[inline(always)]
    pub const fn d_hours(x:f32)->f32{
        d_h(x)
    }

    /// Convert days to hours.
    #[inline(always)]
    pub const fn day_h(x:f32)->f32{
        d_h(x)
    }

    /// Convert days to hours.
    #[inline(always)]
    pub const fn day_hour(x:f32)->f32{
        d_h(x)
    }

    /// Convert days to hours.
    #[inline(always)]
    pub const fn day_hours(x:f32)->f32{
        d_h(x)
    }

    /// Convert days to hours.
    #[inline(always)]
    pub const fn days_h(x:f32)->f32{
        d_h(x)
    }

    /// Convert days to hours.
    #[inline(always)]
    pub const fn days_hour(x:f32)->f32{
        d_h(x)
    }

    /// Convert days to hours.
    #[inline(always)]
    pub const fn days_hours(x:f32)->f32{
        d_h(x)
    }

    /// Convert days to weeks.
    #[inline(always)]
    pub const fn d_week(x:f32)->f32{
        d_wk(x)
    }

    /// Convert days to weeks.
    #[inline(always)]
    pub const fn d_weeks(x:f32)->f32{
        d_wk(x)
    }

    /// Convert days to weeks.
    #[inline(always)]
    pub const fn day_wk(x:f32)->f32{
        d_wk(x)
    }

    /// Convert days to weeks.
    #[inline(always)]
    pub const fn day_week(x:f32)->f32{
        d_wk(x)
    }

    /// Convert days to weeks.
    #[inline(always)]
    pub const fn day_weeks(x:f32)->f32{
        d_wk(x)
    }

    /// Convert days to weeks.
    #[inline(always)]
    pub const fn days_wk(x:f32)->f32{
        d_wk(x)
    }

    /// Convert days to weeks.
    #[inline(always)]
    pub const fn days_week(x:f32)->f32{
        d_wk(x)
    }

    /// Convert days to weeks.
    #[inline(always)]
    pub const fn days_weeks(x:f32)->f32{
        d_wk(x)
    }

    /// Convert days to months.
    #[inline(always)]
    pub const fn d_month(x:f32)->f32{
        d_mo(x)
    }

    /// Convert days to months.
    #[inline(always)]
    pub const fn d_months(x:f32)->f32{
        d_mo(x)
    }

    /// Convert days to months.
    #[inline(always)]
    pub const fn day_mo(x:f32)->f32{
        d_mo(x)
    }

    /// Convert days to months.
    #[inline(always)]
    pub const fn day_month(x:f32)->f32{
        d_mo(x)
    }

    /// Convert days to months.
    #[inline(always)]
    pub const fn day_months(x:f32)->f32{
        d_mo(x)
    }

    /// Convert days to months.
    #[inline(always)]
    pub const fn days_mo(x:f32)->f32{
        d_mo(x)
    }

    /// Convert days to months.
    #[inline(always)]
    pub const fn days_month(x:f32)->f32{
        d_mo(x)
    }

    /// Convert days to months.
    #[inline(always)]
    pub const fn days_months(x:f32)->f32{
        d_mo(x)
    }

    /// Convert days to years.
    #[inline(always)]
    pub const fn d_year(x:f32)->f32{
        d_yr(x)
    }

    /// Convert days to years.
    #[inline(always)]
    pub const fn d_years(x:f32)->f32{
        d_yr(x)
    }

    /// Convert days to years.
    #[inline(always)]
    pub const fn day_yr(x:f32)->f32{
        d_yr(x)
    }

    /// Convert days to years.
    #[inline(always)]
    pub const fn day_year(x:f32)->f32{
        d_yr(x)
    }

    /// Convert days to years.
    #[inline(always)]
    pub const fn day_years(x:f32)->f32{
        d_yr(x)
    }

    /// Convert days to years.
    #[inline(always)]
    pub const fn days_yr(x:f32)->f32{
        d_yr(x)
    }

    /// Convert days to years.
    #[inline(always)]
    pub const fn days_year(x:f32)->f32{
        d_yr(x)
    }

    /// Convert days to years.
    #[inline(always)]
    pub const fn days_years(x:f32)->f32{
        d_yr(x)
    }

    /// Convert weeks to milliseconds.
    #[inline(always)]
    pub const fn wk_millisecond(x:f32)->f32{
        wk_ms(x)
    }

    /// Convert weeks to milliseconds.
    #[inline(always)]
    pub const fn wk_milliseconds(x:f32)->f32{
        wk_ms(x)
    }

    /// Convert weeks to milliseconds.
    #[inline(always)]
    pub const fn week_ms(x:f32)->f32{
        wk_ms(x)
    }

    /// Convert weeks to milliseconds.
    #[inline(always)]
    pub const fn week_millisecond(x:f32)->f32{
        wk_ms(x)
    }

    /// Convert weeks to milliseconds.
    #[inline(always)]
    pub const fn week_milliseconds(x:f32)->f32{
        wk_ms(x)
    }

    /// Convert weeks to milliseconds.
    #[inline(always)]
    pub const fn weeks_ms(x:f32)->f32{
        wk_ms(x)
    }

    /// Convert weeks to milliseconds.
    #[inline(always)]
    pub const fn weeks_millisecond(x:f32)->f32{
        wk_ms(x)
    }

    /// Convert weeks to milliseconds.
    #[inline(always)]
    pub const fn weeks_milliseconds(x:f32)->f32{
        wk_ms(x)
    }

    /// Convert weeks to seconds.
    #[inline(always)]
    pub const fn wk_second(x:f32)->f32{
        wk_s(x)
    }

    /// Convert weeks to seconds.
    #[inline(always)]
    pub const fn wk_seconds(x:f32)->f32{
        wk_s(x)
    }

    /// Convert weeks to seconds.
    #[inline(always)]
    pub const fn week_s(x:f32)->f32{
        wk_s(x)
    }

    /// Convert weeks to seconds.
    #[inline(always)]
    pub const fn week_second(x:f32)->f32{
        wk_s(x)
    }

    /// Convert weeks to seconds.
    #[inline(always)]
    pub const fn week_seconds(x:f32)->f32{
        wk_s(x)
    }

    /// Convert weeks to seconds.
    #[inline(always)]
    pub const fn weeks_s(x:f32)->f32{
        wk_s(x)
    }

    /// Convert weeks to seconds.
    #[inline(always)]
    pub const fn weeks_second(x:f32)->f32{
        wk_s(x)
    }

    /// Convert weeks to seconds.
    #[inline(always)]
    pub const fn weeks_seconds(x:f32)->f32{
        wk_s(x)
    }

    /// Convert weeks to minutes.
    #[inline(always)]
    pub const fn wk_minute(x:f32)->f32{
        wk_min(x)
    }

    /// Convert weeks to minutes.
    #[inline(always)]
    pub const fn wk_minutes(x:f32)->f32{
        wk_min(x)
    }

    /// Convert weeks to minutes.
    #[inline(always)]
    pub const fn week_min(x:f32)->f32{
        wk_min(x)
    }

    /// Convert weeks to minutes.
    #[inline(always)]
    pub const fn week_minute(x:f32)->f32{
        wk_min(x)
    }

    /// Convert weeks to minutes.
    #[inline(always)]
    pub const fn week_minutes(x:f32)->f32{
        wk_min(x)
    }

    /// Convert weeks to minutes.
    #[inline(always)]
    pub const fn weeks_min(x:f32)->f32{
        wk_min(x)
    }

    /// Convert weeks to minutes.
    #[inline(always)]
    pub const fn weeks_minute(x:f32)->f32{
        wk_min(x)
    }

    /// Convert weeks to minutes.
    #[inline(always)]
    pub const fn weeks_minutes(x:f32)->f32{
        wk_min(x)
    }

    /// Convert weeks to hours.
    #[inline(always)]
    pub const fn wk_hour(x:f32)->f32{
        wk_h(x)
    }

    /// Convert weeks to hours.
    #[inline(always)]
    pub const fn wk_hours(x:f32)->f32{
        wk_h(x)
    }

    /// Convert weeks to hours.
    #[inline(always)]
    pub const fn week_h(x:f32)->f32{
        wk_h(x)
    }

    /// Convert weeks to hours.
    #[inline(always)]
    pub const fn week_hour(x:f32)->f32{
        wk_h(x)
    }

    /// Convert weeks to hours.
    #[inline(always)]
    pub const fn week_hours(x:f32)->f32{
        wk_h(x)
    }

    /// Convert weeks to hours.
    #[inline(always)]
    pub const fn weeks_h(x:f32)->f32{
        wk_h(x)
    }

    /// Convert weeks to hours.
    #[inline(always)]
    pub const fn weeks_hour(x:f32)->f32{
        wk_h(x)
    }

    /// Convert weeks to hours.
    #[inline(always)]
    pub const fn weeks_hours(x:f32)->f32{
        wk_h(x)
    }

    /// Convert weeks to days.
    #[inline(always)]
    pub const fn wk_day(x:f32)->f32{
        wk_d(x)
    }

    /// Convert weeks to days.
    #[inline(always)]
    pub const fn wk_days(x:f32)->f32{
        wk_d(x)
    }

    /// Convert weeks to days.
    #[inline(always)]
    pub const fn week_d(x:f32)->f32{
        wk_d(x)
    }

    /// Convert weeks to days.
    #[inline(always)]
    pub const fn week_day(x:f32)->f32{
        wk_d(x)
    }

    /// Convert weeks to days.
    #[inline(always)]
    pub const fn week_days(x:f32)->f32{
        wk_d(x)
    }

    /// Convert weeks to days.
    #[inline(always)]
    pub const fn weeks_d(x:f32)->f32{
        wk_d(x)
    }

    /// Convert weeks to days.
    #[inline(always)]
    pub const fn weeks_day(x:f32)->f32{
        wk_d(x)
    }

    /// Convert weeks to days.
    #[inline(always)]
    pub const fn weeks_days(x:f32)->f32{
        wk_d(x)
    }

    /// Convert weeks to months.
    #[inline(always)]
    pub const fn wk_month(x:f32)->f32{
        wk_mo(x)
    }

    /// Convert weeks to months.
    #[inline(always)]
    pub const fn wk_months(x:f32)->f32{
        wk_mo(x)
    }

    /// Convert weeks to months.
    #[inline(always)]
    pub const fn week_mo(x:f32)->f32{
        wk_mo(x)
    }

    /// Convert weeks to months.
    #[inline(always)]
    pub const fn week_month(x:f32)->f32{
        wk_mo(x)
    }

    /// Convert weeks to months.
    #[inline(always)]
    pub const fn week_months(x:f32)->f32{
        wk_mo(x)
    }

    /// Convert weeks to months.
    #[inline(always)]
    pub const fn weeks_mo(x:f32)->f32{
        wk_mo(x)
    }

    /// Convert weeks to months.
    #[inline(always)]
    pub const fn weeks_month(x:f32)->f32{
        wk_mo(x)
    }

    /// Convert weeks to months.
    #[inline(always)]
    pub const fn weeks_months(x:f32)->f32{
        wk_mo(x)
    }

    /// Convert weeks to years.
    #[inline(always)]
    pub const fn wk_year(x:f32)->f32{
        wk_yr(x)
    }

    /// Convert weeks to years.
    #[inline(always)]
    pub const fn wk_years(x:f32)->f32{
        wk_yr(x)
    }

    /// Convert weeks to years.
    #[inline(always)]
    pub const fn week_yr(x:f32)->f32{
        wk_yr(x)
    }

    /// Convert weeks to years.
    #[inline(always)]
    pub const fn week_year(x:f32)->f32{
        wk_yr(x)
    }

    /// Convert weeks to years.
    #[inline(always)]
    pub const fn week_years(x:f32)->f32{
        wk_yr(x)
    }

    /// Convert weeks to years.
    #[inline(always)]
    pub const fn weeks_yr(x:f32)->f32{
        wk_yr(x)
    }

    /// Convert weeks to years.
    #[inline(always)]
    pub const fn weeks_year(x:f32)->f32{
        wk_yr(x)
    }

    /// Convert weeks to years.
    #[inline(always)]
    pub const fn weeks_years(x:f32)->f32{
        wk_yr(x)
    }

    /// Convert months to milliseconds.
    #[inline(always)]
    pub const fn mo_millisecond(x:f32)->f32{
        mo_ms(x)
    }

    /// Convert months to milliseconds.
    #[inline(always)]
    pub const fn mo_milliseconds(x:f32)->f32{
        mo_ms(x)
    }

    /// Convert months to milliseconds.
    #[inline(always)]
    pub const fn month_ms(x:f32)->f32{
        mo_ms(x)
    }

    /// Convert months to milliseconds.
    #[inline(always)]
    pub const fn month_millisecond(x:f32)->f32{
        mo_ms(x)
    }

    /// Convert months to milliseconds.
    #[inline(always)]
    pub const fn month_milliseconds(x:f32)->f32{
        mo_ms(x)
    }

    /// Convert months to milliseconds.
    #[inline(always)]
    pub const fn months_ms(x:f32)->f32{
        mo_ms(x)
    }

    /// Convert months to milliseconds.
    #[inline(always)]
    pub const fn months_millisecond(x:f32)->f32{
        mo_ms(x)
    }

    /// Convert months to milliseconds.
    #[inline(always)]
    pub const fn months_milliseconds(x:f32)->f32{
        mo_ms(x)
    }

    /// Convert months to seconds.
    #[inline(always)]
    pub const fn mo_second(x:f32)->f32{
        mo_s(x)
    }

    /// Convert months to seconds.
    #[inline(always)]
    pub const fn mo_seconds(x:f32)->f32{
        mo_s(x)
    }

    /// Convert months to seconds.
    #[inline(always)]
    pub const fn month_s(x:f32)->f32{
        mo_s(x)
    }

    /// Convert months to seconds.
    #[inline(always)]
    pub const fn month_second(x:f32)->f32{
        mo_s(x)
    }

    /// Convert months to seconds.
    #[inline(always)]
    pub const fn month_seconds(x:f32)->f32{
        mo_s(x)
    }

    /// Convert months to seconds.
    #[inline(always)]
    pub const fn months_s(x:f32)->f32{
        mo_s(x)
    }

    /// Convert months to seconds.
    #[inline(always)]
    pub const fn months_second(x:f32)->f32{
        mo_s(x)
    }

    /// Convert months to seconds.
    #[inline(always)]
    pub const fn months_seconds(x:f32)->f32{
        mo_s(x)
    }

    /// Convert months to minutes.
    #[inline(always)]
    pub const fn mo_minute(x:f32)->f32{
        mo_min(x)
    }

    /// Convert months to minutes.
    #[inline(always)]
    pub const fn mo_minutes(x:f32)->f32{
        mo_min(x)
    }

    /// Convert months to minutes.
    #[inline(always)]
    pub const fn month_min(x:f32)->f32{
        mo_min(x)
    }

    /// Convert months to minutes.
    #[inline(always)]
    pub const fn month_minute(x:f32)->f32{
        mo_min(x)
    }

    /// Convert months to minutes.
    #[inline(always)]
    pub const fn month_minutes(x:f32)->f32{
        mo_min(x)
    }

    /// Convert months to minutes.
    #[inline(always)]
    pub const fn months_min(x:f32)->f32{
        mo_min(x)
    }

    /// Convert months to minutes.
    #[inline(always)]
    pub const fn months_minute(x:f32)->f32{
        mo_min(x)
    }

    /// Convert months to minutes.
    #[inline(always)]
    pub const fn months_minutes(x:f32)->f32{
        mo_min(x)
    }

    /// Convert months to hours.
    #[inline(always)]
    pub const fn mo_hour(x:f32)->f32{
        mo_h(x)
    }

    /// Convert months to hours.
    #[inline(always)]
    pub const fn mo_hours(x:f32)->f32{
        mo_h(x)
    }

    /// Convert months to hours.
    #[inline(always)]
    pub const fn month_h(x:f32)->f32{
        mo_h(x)
    }

    /// Convert months to hours.
    #[inline(always)]
    pub const fn month_hour(x:f32)->f32{
        mo_h(x)
    }

    /// Convert months to hours.
    #[inline(always)]
    pub const fn month_hours(x:f32)->f32{
        mo_h(x)
    }

    /// Convert months to hours.
    #[inline(always)]
    pub const fn months_h(x:f32)->f32{
        mo_h(x)
    }

    /// Convert months to hours.
    #[inline(always)]
    pub const fn months_hour(x:f32)->f32{
        mo_h(x)
    }

    /// Convert months to hours.
    #[inline(always)]
    pub const fn months_hours(x:f32)->f32{
        mo_h(x)
    }

    /// Convert months to days.
    #[inline(always)]
    pub const fn mo_day(x:f32)->f32{
        mo_d(x)
    }

    /// Convert months to days.
    #[inline(always)]
    pub const fn mo_days(x:f32)->f32{
        mo_d(x)
    }

    /// Convert months to days.
    #[inline(always)]
    pub const fn month_d(x:f32)->f32{
        mo_d(x)
    }

    /// Convert months to days.
    #[inline(always)]
    pub const fn month_day(x:f32)->f32{
        mo_d(x)
    }

    /// Convert months to days.
    #[inline(always)]
    pub const fn month_days(x:f32)->f32{
        mo_d(x)
    }

    /// Convert months to days.
    #[inline(always)]
    pub const fn months_d(x:f32)->f32{
        mo_d(x)
    }

    /// Convert months to days.
    #[inline(always)]
    pub const fn months_day(x:f32)->f32{
        mo_d(x)
    }

    /// Convert months to days.
    #[inline(always)]
    pub const fn months_days(x:f32)->f32{
        mo_d(x)
    }

    /// Convert months to weeks.
    #[inline(always)]
    pub const fn mo_week(x:f32)->f32{
        mo_wk(x)
    }

    /// Convert months to weeks.
    #[inline(always)]
    pub const fn mo_weeks(x:f32)->f32{
        mo_wk(x)
    }

    /// Convert months to weeks.
    #[inline(always)]
    pub const fn month_wk(x:f32)->f32{
        mo_wk(x)
    }

    /// Convert months to weeks.
    #[inline(always)]
    pub const fn month_week(x:f32)->f32{
        mo_wk(x)
    }

    /// Convert months to weeks.
    #[inline(always)]
    pub const fn month_weeks(x:f32)->f32{
        mo_wk(x)
    }

    /// Convert months to weeks.
    #[inline(always)]
    pub const fn months_wk(x:f32)->f32{
        mo_wk(x)
    }

    /// Convert months to weeks.
    #[inline(always)]
    pub const fn months_week(x:f32)->f32{
        mo_wk(x)
    }

    /// Convert months to weeks.
    #[inline(always)]
    pub const fn months_weeks(x:f32)->f32{
        mo_wk(x)
    }

    /// Convert months to years.
    #[inline(always)]
    pub const fn mo_year(x:f32)->f32{
        mo_yr(x)
    }

    /// Convert months to years.
    #[inline(always)]
    pub const fn mo_years(x:f32)->f32{
        mo_yr(x)
    }

    /// Convert months to years.
    #[inline(always)]
    pub const fn month_yr(x:f32)->f32{
        mo_yr(x)
    }

    /// Convert months to years.
    #[inline(always)]
    pub const fn month_year(x:f32)->f32{
        mo_yr(x)
    }

    /// Convert months to years.
    #[inline(always)]
    pub const fn month_years(x:f32)->f32{
        mo_yr(x)
    }

    /// Convert months to years.
    #[inline(always)]
    pub const fn months_yr(x:f32)->f32{
        mo_yr(x)
    }

    /// Convert months to years.
    #[inline(always)]
    pub const fn months_year(x:f32)->f32{
        mo_yr(x)
    }

    /// Convert months to years.
    #[inline(always)]
    pub const fn months_years(x:f32)->f32{
        mo_yr(x)
    }

    /// Convert years to milliseconds.
    #[inline(always)]
    pub const fn yr_millisecond(x:f32)->f32{
        yr_ms(x)
    }

    /// Convert years to milliseconds.
    #[inline(always)]
    pub const fn yr_milliseconds(x:f32)->f32{
        yr_ms(x)
    }

    /// Convert years to milliseconds.
    #[inline(always)]
    pub const fn year_ms(x:f32)->f32{
        yr_ms(x)
    }

    /// Convert years to milliseconds.
    #[inline(always)]
    pub const fn year_millisecond(x:f32)->f32{
        yr_ms(x)
    }

    /// Convert years to milliseconds.
    #[inline(always)]
    pub const fn year_milliseconds(x:f32)->f32{
        yr_ms(x)
    }

    /// Convert years to milliseconds.
    #[inline(always)]
    pub const fn years_ms(x:f32)->f32{
        yr_ms(x)
    }

    /// Convert years to milliseconds.
    #[inline(always)]
    pub const fn years_millisecond(x:f32)->f32{
        yr_ms(x)
    }

    /// Convert years to milliseconds.
    #[inline(always)]
    pub const fn years_milliseconds(x:f32)->f32{
        yr_ms(x)
    }

    /// Convert years to seconds.
    #[inline(always)]
    pub const fn yr_second(x:f32)->f32{
        yr_s(x)
    }

    /// Convert years to seconds.
    #[inline(always)]
    pub const fn yr_seconds(x:f32)->f32{
        yr_s(x)
    }

    /// Convert years to seconds.
    #[inline(always)]
    pub const fn year_s(x:f32)->f32{
        yr_s(x)
    }

    /// Convert years to seconds.
    #[inline(always)]
    pub const fn year_second(x:f32)->f32{
        yr_s(x)
    }

    /// Convert years to seconds.
    #[inline(always)]
    pub const fn year_seconds(x:f32)->f32{
        yr_s(x)
    }

    /// Convert years to seconds.
    #[inline(always)]
    pub const fn years_s(x:f32)->f32{
        yr_s(x)
    }

    /// Convert years to seconds.
    #[inline(always)]
    pub const fn years_second(x:f32)->f32{
        yr_s(x)
    }

    /// Convert years to seconds.
    #[inline(always)]
    pub const fn years_seconds(x:f32)->f32{
        yr_s(x)
    }

    /// Convert years to minutes.
    #[inline(always)]
    pub const fn yr_minute(x:f32)->f32{
        yr_min(x)
    }

    /// Convert years to minutes.
    #[inline(always)]
    pub const fn yr_minutes(x:f32)->f32{
        yr_min(x)
    }

    /// Convert years to minutes.
    #[inline(always)]
    pub const fn year_min(x:f32)->f32{
        yr_min(x)
    }

    /// Convert years to minutes.
    #[inline(always)]
    pub const fn year_minute(x:f32)->f32{
        yr_min(x)
    }

    /// Convert years to minutes.
    #[inline(always)]
    pub const fn year_minutes(x:f32)->f32{
        yr_min(x)
    }

    /// Convert years to minutes.
    #[inline(always)]
    pub const fn years_min(x:f32)->f32{
        yr_min(x)
    }

    /// Convert years to minutes.
    #[inline(always)]
    pub const fn years_minute(x:f32)->f32{
        yr_min(x)
    }

    /// Convert years to minutes.
    #[inline(always)]
    pub const fn years_minutes(x:f32)->f32{
        yr_min(x)
    }

    /// Convert years to hours.
    #[inline(always)]
    pub const fn yr_hour(x:f32)->f32{
        yr_h(x)
    }

    /// Convert years to hours.
    #[inline(always)]
    pub const fn yr_hours(x:f32)->f32{
        yr_h(x)
    }

    /// Convert years to hours.
    #[inline(always)]
    pub const fn year_h(x:f32)->f32{
        yr_h(x)
    }

    /// Convert years to hours.
    #[inline(always)]
    pub const fn year_hour(x:f32)->f32{
        yr_h(x)
    }

    /// Convert years to hours.
    #[inline(always)]
    pub const fn year_hours(x:f32)->f32{
        yr_h(x)
    }

    /// Convert years to hours.
    #[inline(always)]
    pub const fn years_h(x:f32)->f32{
        yr_h(x)
    }

    /// Convert years to hours.
    #[inline(always)]
    pub const fn years_hour(x:f32)->f32{
        yr_h(x)
    }

    /// Convert years to hours.
    #[inline(always)]
    pub const fn years_hours(x:f32)->f32{
        yr_h(x)
    }

    /// Convert years to days.
    #[inline(always)]
    pub const fn yr_day(x:f32)->f32{
        yr_d(x)
    }

    /// Convert years to days.
    #[inline(always)]
    pub const fn yr_days(x:f32)->f32{
        yr_d(x)
    }

    /// Convert years to days.
    #[inline(always)]
    pub const fn year_d(x:f32)->f32{
        yr_d(x)
    }

    /// Convert years to days.
    #[inline(always)]
    pub const fn year_day(x:f32)->f32{
        yr_d(x)
    }

    /// Convert years to days.
    #[inline(always)]
    pub const fn year_days(x:f32)->f32{
        yr_d(x)
    }

    /// Convert years to days.
    #[inline(always)]
    pub const fn years_d(x:f32)->f32{
        yr_d(x)
    }

    /// Convert years to days.
    #[inline(always)]
    pub const fn years_day(x:f32)->f32{
        yr_d(x)
    }

    /// Convert years to days.
    #[inline(always)]
    pub const fn years_days(x:f32)->f32{
        yr_d(x)
    }

    /// Convert years to weeks.
    #[inline(always)]
    pub const fn yr_week(x:f32)->f32{
        yr_wk(x)
    }

    /// Convert years to weeks.
    #[inline(always)]
    pub const fn yr_weeks(x:f32)->f32{
        yr_wk(x)
    }

    /// Convert years to weeks.
    #[inline(always)]
    pub const fn year_wk(x:f32)->f32{
        yr_wk(x)
    }

    /// Convert years to weeks.
    #[inline(always)]
    pub const fn year_week(x:f32)->f32{
        yr_wk(x)
    }

    /// Convert years to weeks.
    #[inline(always)]
    pub const fn year_weeks(x:f32)->f32{
        yr_wk(x)
    }

    /// Convert years to weeks.
    #[inline(always)]
    pub const fn years_wk(x:f32)->f32{
        yr_wk(x)
    }

    /// Convert years to weeks.
    #[inline(always)]
    pub const fn years_week(x:f32)->f32{
        yr_wk(x)
    }

    /// Convert years to weeks.
    #[inline(always)]
    pub const fn years_weeks(x:f32)->f32{
        yr_wk(x)
    }

    /// Convert years to months.
    #[inline(always)]
    pub const fn yr_month(x:f32)->f32{
        yr_mo(x)
    }

    /// Convert years to months.
    #[inline(always)]
    pub const fn yr_months(x:f32)->f32{
        yr_mo(x)
    }

    /// Convert years to months.
    #[inline(always)]
    pub const fn year_mo(x:f32)->f32{
        yr_mo(x)
    }

    /// Convert years to months.
    #[inline(always)]
    pub const fn year_month(x:f32)->f32{
        yr_mo(x)
    }

    /// Convert years to months.
    #[inline(always)]
    pub const fn year_months(x:f32)->f32{
        yr_mo(x)
    }

    /// Convert years to months.
    #[inline(always)]
    pub const fn years_mo(x:f32)->f32{
        yr_mo(x)
    }

    /// Convert years to months.
    #[inline(always)]
    pub const fn years_month(x:f32)->f32{
        yr_mo(x)
    }

    /// Convert years to months.
    #[inline(always)]
    pub const fn years_months(x:f32)->f32{
        yr_mo(x)
    }
}

pub mod temperature{
        /// Convert celsius to farenheit.
        #[inline]
        pub const fn c_f(x:f32)->f32{
            x*1.8+32.0
        }

        /// Convert celsius to kelvin.
        #[inline]
        pub const fn c_k(x:f32)->f32{
            x+273.15
        }

        /// Convert farenheit to celsius.
        #[inline]
        pub const fn f_c(x:f32)->f32{
            x*0.55555556-17.77777778
        }

        /// Convert farenheit to kelvin.
        #[inline]
        pub const fn f_k(x:f32)->f32{
            x*0.55555556+255.3722222222222
        }

        /// Convert kelvin to celsius.
        #[inline]
        pub const fn k_c(x:f32)->f32{
            x-273.15
        }

        /// Convert kelvin to farenheit.
        #[inline]
        pub const fn k_f(x:f32)->f32{
            x*1.8-459.66999999        }

        /// Convert celsius to farenheit.
        #[inline(always)]
        pub const fn c_farenheit(x:f32)->f32{
            c_f(x)
        }

        /// Convert celsius to farenheit.
        #[inline(always)]
        pub const fn celsius_f(x:f32)->f32{
            c_f(x)
        }

        /// Convert celsius to farenheit.
        #[inline(always)]
        pub const fn celsius_farenheit(x:f32)->f32{
            c_f(x)
        }

        /// Convert celsius to kelvin.
        #[inline(always)]
        pub const fn c_kelvin(x:f32)->f32{
            c_k(x)
        }

        /// Convert celsius to kelvin.
        #[inline(always)]
        pub const fn celsius_k(x:f32)->f32{
            c_k(x)
        }

        /// Convert celsius to kelvin.
        #[inline(always)]
        pub const fn celsius_kelvin(x:f32)->f32{
            c_k(x)
        }

        /// Convert farenheit to celsius.
        #[inline(always)]
        pub const fn f_celsius(x:f32)->f32{
            f_c(x)
        }

        /// Convert farenheit to celsius.
        #[inline(always)]
        pub const fn farenheit_c(x:f32)->f32{
            f_c(x)
        }

        /// Convert farenheit to celsius.
        #[inline(always)]
        pub const fn farenheit_celsius(x:f32)->f32{
            f_c(x)
        }

        /// Convert farenheit to kelvin.
        #[inline(always)]
        pub const fn f_kelvin(x:f32)->f32{
            f_k(x)
        }

        /// Convert farenheit to kelvin.
        #[inline(always)]
        pub const fn farenheit_k(x:f32)->f32{
            f_k(x)
        }

        /// Convert farenheit to kelvin.
        #[inline(always)]
        pub const fn farenheit_kelvin(x:f32)->f32{
            f_k(x)
        }

        /// Convert kelvin to celsius.
        #[inline(always)]
        pub const fn k_celsius(x:f32)->f32{
            k_c(x)
        }

        /// Convert kelvin to celsius.
        #[inline(always)]
        pub const fn kelvin_c(x:f32)->f32{
            k_c(x)
        }

        /// Convert kelvin to celsius.
        #[inline(always)]
        pub const fn kelvin_celsius(x:f32)->f32{
            k_c(x)
        }

        /// Convert kelvin to farenheit.
        #[inline(always)]
        pub const fn k_farenheit(x:f32)->f32{
            k_f(x)
        }

        /// Convert kelvin to farenheit.
        #[inline(always)]
        pub const fn kelvin_f(x:f32)->f32{
            k_f(x)
        }

        /// Convert kelvin to farenheit.
        #[inline(always)]
        pub const fn kelvin_farenheit(x:f32)->f32{
            k_f(x)
        }
}
