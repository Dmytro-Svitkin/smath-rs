# smath

`smath` (Simple Math or Simplified Math) is a lightweight, highly optimized Rust mathematical library designed for performance-critical applications. It provides fast, low-overhead implementations of essential mathematical operations with a primary focus on minimizing instruction counts and avoiding expensive operations.

`smath` features *zero* dependencies and is suitable for `no_std` environments.

## Modules

### Trigonometry (`smath::trigonometry`)
To input an angle in degrees, simply enter the numeric value - for example, `45.0` or `45`.  
<sub>`isize` inputs are faster and more precise than `f32` values because they use lookup tables; `isize` inputs allow creating `const` values.</sub>

| **Function** | **Alias** | **Input** | **Output** |
| --- | --- | --- | --- |
| Sinus | `sin` | Degrees `f32`, Degrees `isize` | `f32`, `f32` |
| Cosinus | `cos` | Degrees `f32`, Degrees `isize` | `f32`, `f32` |
| Tangens | `tan`, `tg` | Degrees `f32`, Degrees `isize` | `f32`, `f32` |
| Cotangens | `cotan`, `ctg` | Degrees `f32`, Degrees `isize` | `f32`, `f32` |
| Arc-Sinus | `arcsin`, `asin` | `f32` | Degrees `f32` |
| Arc-Cosinus | `arccos`, `acos` | `f32` | Degrees `f32` |
| Arc-Tangens | `arctan`, `atan` | `f32` | Degrees `f32` |
| Arc-Cotangens | `arccotan`, `acotan` | `f32` | Degrees `f32` |

To input an angle in radians, enter the numeric value directly - for example, `2.0` (for 2π radians).  
There is no need to multiply the input by π; the functions interpret the value as radian.
| **Function** | **Alias** | **Input** | **Output** |
| --- | --- | --- | --- |
| Sinus | `sinr` | Radians `f32` | `f32` |
| Cosinus | `cosr` | Radians `f32` | `f32` |
| Tangens | `tanr`, `tgr` | Radians `f32` | `f32` |
| Cotangens | `cotanr`, `ctgr` | Radians `f32` | `f32` |
| Arc-Sinus | `arcsinr`, `asinr` | `f32` | Radians `f32` |
| Arc-Cosinus | `arccosr`, `acosr` | `f32` | Radians `f32` |
| Arc-Tangens | `arctanr`, `atanr` | `f32` | Radians `f32` |
| Arc-Cotangens | `arccotanr`, `acotanr` | `f32` | Radians `f32` |

```rust
// Example Usage

const slope = tan(12);       // Slope from a 12° angle (isize is const‑friendly)
let angle_rad = atanr(0.54); // Inverse tangent in radians, returns angle as f32
let sinus_deg = sin(67.5);   // Sinus of a 67.5° angle

```

### Unit Conversions (`smath::conversion`)
All conversion functions follow a predictable strict shortcut format: `[from]_[to](value)` (e.g., `cm_inches(123.456)`, `second_hours(5.5)`). 
Only primary units are listed below, but all cross-combinations and major aliases (e.g., `inch`, `inches`) are fully supported.

| **Category** | **Module Path** | **Supported Units (From / To)** | **Input** | **Output** |
| --- | --- | --- | --- | --- |
| Length | `smath::length::` | `mm`, `cm`, `dm`, `m`, `km`, `in`, `ft`, `yd`, `mi`, `nmi` | `f32` | `f32` |
| Weight | `smath::weight::` | `mg`, `g`, `kg`, `oz`, `lb`, `st`, `tn`, `t` | `f32` | `f32` |
| Volume | `smath::volume::` | `ml`, `l`, `floz`, `cp`, `pt`, `qt`, `gal` | `f32` | `f32` |
| Speed | `smath::speed::` | `mps`, `kmh`, `mph`, `kt`, `mach` | `f32` | `f32` |
| Pressure | `smath::pressure::`| `pa`, `psi`, `bar`, `atm` | `f32` | `f32` |
| Energy | `smath::energy::` | `j`, `cal`, `hp`, `w` | `f32` | `f32` |
| Time | `smath::time::` | `ms`, `s`, `min`, `h`, `d`, `wk`, `mo`, `yr` | `f32` | `f32` |
| Temperature | `smth::temperature::` | `c`, `f`, `k` | `f32` | `f32` |

```rust
// Example Usage

let millimeters = smath::length::in_mm(2.0); // 2 inches are converted to millimeters
let hours = smath::time::d_h(1.5); // 1.5 of a day is converted to hours
```

### Vectors (`smath::vector`)
2D, 3D, and 4D vector types (`Vec2`, `Vec3`, `Vec4`) with a wide range of mathematical and geometric operations.

| **Category** | **Methods** | **Description** |
| --- | --- | --- |
| Creation & Setup | `new`, `zero`, `one`, `set`, `shift` | Create or modify vector instances |
| Math Operations | `dot`, `cross`, `length`, `sq_length`, `distance`, `sq_distance` | Core vector math and magnitude calculations |
| Normalization | `normalize`, `normalized` | Scale vector to unit length |
| Angles & Rotation | `angle_deg`, `angle_rad`, `rotate`, `rotate_deg`, `rotate_rad` | Compute or apply rotations and angles |
| Interpolation & Clamping | `lerp`, `clamp`, `abs`, `midpoint` | Smooth transitions and value constraints |
| Projection & Reflection | `project`, `reject`, `reflect` | Vector projection and reflection utilities |
| 3D Vector Flattening | `flat`, `sp_flat`, `persp` | Convert or project 3D vectors into 2D space using flat, spherical, or perspective flattening |
| Operators | `+`, `-`, `*`, `/`, `+=`, `-=`, `*=`, `/=` | Component‑wise arithmetic via Rust operator overloading |


```rust
// Example Usage

let mut pos = Vec3::zero(); // 3D vector "pos" is equal to {0.0, 0.0, 0.0}
pos.set(10.0, 5.0, 0.0); // "pos" is set to {10.0, 5.0, 0.0}
pos.shift(1.0, 0.0, 0.0); // "pos" is shifted by {1.0, 0.0, 0.0} and is equal to {11.0, 5.0, 0.0}

let velocity = Vec3::new(0.0, 2.0, 1.0); // 3D vector "velocity" is equal to {0.0, 2.0, 1.0}
let next_frame = pos + (velocity * 2.0); // next_frame is equal to {11.0, 9.0, 2.0}
```

## Installation

Add `smath` to your `Cargo.toml` dependencies:

```toml
[dependencies]
smath = "0.3.3"
```

<sub>This project is licensed under the MIT License.</sub>