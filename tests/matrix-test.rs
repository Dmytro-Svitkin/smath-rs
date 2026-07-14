use smath::*;

const EPSILON: f32 = 1e-5;

#[test]
fn test_identity_multiplication() {
    let m: Mat<4, 4> = Mat4::identity();
    let v: Vec4 = Vec4 { x: 1.0, y: 2.0, z: 3.0, w: 1.0 };
    let res = m*v;

    assert!((res.x - 1.0).abs() < EPSILON);
    assert!((res.y - 2.0).abs() < EPSILON);
    assert!((res.z - 3.0).abs() < EPSILON);
}

#[test]
fn test_matrix_addition() {
    let m1: Mat<4, 4> = Mat4::identity();
    let m2: Mat<4, 4> = Mat4::identity();
    let sum = m1 + m2;
    
    // Identity + Identity = 2.0 on diagonal
    assert!((sum.data[0][0] - 2.0).abs() < EPSILON);
    assert!((sum.data[1][1] - 2.0).abs() < EPSILON);
}

#[test]
fn test_scaling() {
    let mut scale = Mat4::identity();
    scale.data[0][0] = 2.0;
    scale.data[1][1] = 3.0;
    scale.data[2][2] = 4.0;
    
    let v = Vec4 { x: 1.0, y: 1.0, z: 1.0, w: 1.0 };
    let res = scale * v;
    
    assert!((res.x - 2.0).abs() < 1e-5);
    assert!((res.y - 3.0).abs() < 1e-5);
    assert!((res.z - 4.0).abs() < 1e-5);
}
