# smath

`smath` (Simple Math or Simplified Math) is a lightweight, highly optimized Rust mathematical library designed for performance-critical applications. It provides fast, low-overhead implementations of essential mathematical operations with a primary focus on minimizing instruction counts and avoiding expensive operations.

`smath` features *zero* dependencies and is suitable for `no_std` environments.

## Modules

### Trigonometry (`smath::trigonometry`)
`sin`, `cos`, `tan`, `cotan`, `arcsin`, `arccos`, `arctan`, `arccotan`

### Unit Conversions (`smath::conversion`)
All conversion functions follow a predictable strict shortcut format: `[from]_[to](value)` (e.g., `cm_inches(123.456)`, `second_hours(5.5)`). 
Only primary units are listed below, but all cross-combinations and major aliases (e.g., `inch`, `inches`) are fully supported.

| Category | Module Path | Supported Units (From / To) |
| :--- | :--- | :--- |
| **Length** | `smath::length::` | `mm`, `cm`, `dm`, `m`, `km`, `in`, `ft`, `yd`, `mi`, `nmi` |
| **Weight** | `smath::weight::` | `mg`, `g`, `kg`, `oz`, `lb`, `st`, `tn`, `t` |
| **Volume** | `smath::volume::` | `ml`, `l`, `floz`, `cp`, `pt`, `qt`, `gal` |
| **Speed** | `smath::speed::` | `mps`, `kmh`, `mph`, `kt`, `mach` |
| **Pressure**| `smath::pressure::`| `pa`, `psi`, `bar`, `atm` |
| **Energy** | `smath::energy::` | `j`, `cal`, `hp`, `w` |
| **Time** | `smath::time::` | `ms`, `s`, `min`, `h`, `d`, `wk`, `mo`, `yr` |

```rust
// Example Usage
let millimeters = smath::length::in_mm(2.0); // 2 inches are converted to millimeters
let hours = smath::time::d_h(1.5); // 1.5 of a day is converted to hours
```

## Installation

Add `smath` to your `Cargo.toml` dependencies:

```toml
[dependencies]
smath = "0.2.1"
```

<sub>This project is licensed under the MIT License.</sub>
