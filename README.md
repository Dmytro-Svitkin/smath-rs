# smath

`smath` (Simple Math or Simplified Math) is a lightweight, highly optimized Rust mathematical library designed for performance-critical applications. It provides fast, low-overhead implementations of essential mathematical operations with a primary focus on minimizing instruction counts and avoiding expensive operations.

`smath` features *zero* dependencies and is suitable for `no_std` environments.

## Modules

### Trigonometry (`smath::trigonometry`)
`sin`, `cos`, `tan`, `cotan`, `arcsin`, `arccos`, `arctan`, `arccotan`

### Unit Conversions (`smath::conversion`)
`in_mm`, `mm_in`, `ft_m`, `m_ft`, `yd_m`, `m_yd`, `mi_km`, `km_mi`, `nmi_km`, `km_nmi`, `oz_g`, `g_oz`, `lb_kg`, `kg_lb`, `st_kg`, `kg_st`, `tn_t`, `t_tn`, `floz_ml`, `ml_floz`, `cp_ml`, `ml_cp`, `pt_l`, `l_pt`, `qt_l`, `l_qt`, `gal_l`, `l_gal`, `c_f`, `f_c`, `c_k`, `k_c`, `f_k`, `k_f`, `mph_kmh`, `kmh_mph`, `kt_kmh`, `kmh_kt`, `mach_ms`, `ms_mach`, `psi_pa`, `pa_psi`, `bar_pa`, `pa_bar`, `atm_pa`, `pa_atm`, `j_cal`, `cal_j`, `hp_w`, `w_hp`

## Installation

Add `smath` to your `Cargo.toml` dependencies:

```toml
[dependencies]
smath = "0.2.0"
```

<sub>This project is licensed under the MIT License.</sub>
