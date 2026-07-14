use smath::*;

#[test]
fn test_view_matrix(){
    // We place the camera at X=10, looking at the origin, with Y as up.
    let pos:Vec3=Vec3{x:10.0,y:0.0,z:0.0};
    let target:Vec3=Vec3{x:0.0,y:0.0,z:0.0};
    let up:Vec3=Vec3{x:0.0,y:1.0,z:0.0};
    let view_mat:Mat4=Mat4::view(pos,target,up);

    // TEST A: Transforming the camera's own world position into view space.
    // In the camera's local space, its own position is exactly the origin (0, 0, 0).
    let point_at_camera:Vec4=Vec4{x:10.0,y:0.0,z:0.0,w:1.0};
    let view_space_pos:Vec4=view_mat*point_at_camera;
    println!("--- View Matrix Bug Test ---");
    println!("Camera Position in View Space (Should be 0.0, 0.0, 0.0, 1.0)");
    println!("Output: x:{:.1}, y:{:.1}, z:{:.1}, w:{:.1}",view_space_pos.x,view_space_pos.y,view_space_pos.z,view_space_pos.w);

    // TEST B: Transforming the target's world position into view space.
    // Because the camera looks down its local -Z axis, the target (distance 10) 
    // should be at Z = -10 in view space.
    let point_at_target:Vec4=Vec4{x:0.0,y:0.0,z:0.0,w:1.0};
    let view_space_target:Vec4=view_mat*point_at_target;
    println!("Target Position in View Space (Should be 0.0, 0.0, -10.0, 1.0)");
    println!("Output: x:{:.1}, y:{:.1}, z:{:.1}, w:{:.1}",view_space_target.x,view_space_target.y,view_space_target.z,view_space_target.w);
}

#[test]
fn test_translation_matrix(){
    // Simple test to ensure standard multiplication is working as intended.
    let trans_mat:Mat4=Mat4::translation(5.0,-2.0,3.0);
    let point:Vec4=Vec4{x:1.0,y:1.0,z:1.0,w:1.0};
    let result:Vec4=trans_mat*point;
    
    println!("--- Translation Matrix Test ---");
    println!("Point translated by (5, -2, 3) (Should be 6.0, -1.0, 4.0, 1.0)");
    println!("Output: x:{:.1}, y:{:.1}, z:{:.1}, w:{:.1}",result.x,result.y,result.z,result.w);
}

#[test]
fn test_creation_and_access(){
    println!("\n--- Test: Creation and Access ---");
    // Using a 3x3 matrix where each column increments by 10 for easy reading
    let m:Mat3=Mat3::new([[1.0,2.0,3.0],[11.0,12.0,13.0],[21.0,22.0,23.0]]);
    println!("new(): {:?}",m.data);
    
    let z:Mat3=Mat3::zero();
    println!("zero(): {:?}",z.data);
    
    let f:Mat3=Mat3::fill(7.5);
    println!("fill(7.5): {:?}",f.data);
    
    println!("get(1,2) (expected 22.0): {}",m.get(1,2));
    println!("get_column(1) (expected [11.0, 12.0, 13.0]): {:?}",m.get_col(1));
    println!("get_row(1) (expected [2.0, 12.0, 22.0]): {:?}",m.get_row(1));
    println!("get_all(): {:?}",m.get_all());
}

#[test]
fn test_mutation_and_shifting(){
    println!("\n--- Test: Mutation and Shifting ---");
    let mut m:Mat3=Mat3::zero();
    
    m.set(0,1,5.0);
    println!("After set(0,1,5.0): {:?}",m.data);
    
    m.set_col(2,[7.0,8.0,9.0]);
    println!("After set_column(2,...): {:?}",m.data);
    
    m.set_row(1,[1.0,1.0,1.0]);
    println!("After set_row(1,...): {:?}",m.data);
    
    m.shift(0,1,5.0);
    println!("After shift_cell(0,1,5.0) (expected 10.0 at col 1, row 0): {}",m.get(0,1));
    
    m.shift_all(2.0);
    println!("After shift_all(2.0): {:?}",m.data);
    
    m.shift_col(0,[1.0,2.0,3.0]);
    println!("After shift_column(0,...): {:?}",m.data);
    
    m.shift_row(2,[1.0,2.0,3.0]);
    println!("After shift_row(2,...): {:?}",m.data);
}

#[test]
fn test_core_operations(){
    println!("\n--- Test: Core Matrix Operations ---");
    let m:Mat3=Mat3::new([[1.0,2.0,3.0],[4.0,5.0,6.0],[7.0,8.0,9.0]]);
    
    let t:Mat3=m.transpose();
    println!("transpose() of m: {:?}",t.data);
    
    let mut m2:Mat3=m.clone();
    m2.transposed();
    println!("transposed() in-place (should match above): {:?}",m2.data);
    
    let id:Mat3=Mat3::identity();
    println!("identity(): {:?}",id.data);
    
    println!("trace() of m (1.0 + 5.0 + 9.0 = 15.0): {}",m.trace());
}

#[test]
fn test_math_operators(){
    println!("\n--- Test: Math Operators ---");
    let a:Mat3=Mat3::fill(2.0);
    let b:Mat3=Mat3::fill(4.0);
    
    println!("a + b: {:?}",(a+b).data);
    println!("b - a: {:?}",(b-a).data);
    println!("a * 3.0: {:?}",(a*3.0).data);
    println!("b / 2.0: {:?}",(b/2.0).data);
    println!("-a: {:?}",(-a).data);
    
    let m1:Mat3=Mat3::new([[1.0,0.0,0.0],[0.0,2.0,0.0],[0.0,0.0,3.0]]);
    let m2:Mat3=Mat3::new([[2.0,0.0,0.0],[0.0,3.0,0.0],[0.0,0.0,4.0]]);
    println!("m1 * m2 (matrix multiplication): {:?}",(m1*m2).data);
    
    let mut c:Mat3=Mat3::fill(1.0);
    c+=Mat3::fill(2.0);
    println!("c += fill(2.0) (expected 3.0s): {:?}",c.data);
    
    c-=Mat3::fill(1.0);
    println!("c -= fill(1.0) (expected 2.0s): {:?}",c.data);
    
    c*=4.0;
    println!("c *= 4.0 (expected 8.0s): {:?}",c.data);
    
    c/=2.0;
    println!("c /= 2.0 (expected 4.0s): {:?}",c.data);
}

#[test]
fn test_transformations(){
    println!("\n--- Test: 3D Transformations ---");
    let scale:Mat4=Mat4::scale(2.0,3.0,4.0);
    println!("scale(2.0, 3.0, 4.0): {:?}",scale.data);
    
    // 90 degrees in radians
    let rx:Mat4=Mat4::rotate_x(0.5);
    println!("rotate_x(PI/2): {:?}",rx.data);
    
    let ry:Mat4=Mat4::rotate_y(0.5);
    println!("rotate_y(PI/2): {:?}",ry.data);
    
    let proj:Mat4=Mat4::persp(1.570796,16.0/9.0,0.1,100.0);
    println!("perspective(90deg, 16/9, 0.1, 100): {:?}",proj.data);
    
    let vec:Vec4=Vec4{x:1.0,y:2.0,z:3.0,w:1.0};
    let scaled_vec=scale*vec;
    println!("scale * vector (1,2,3,1) (expected 2.0, 6.0, 12.0, 1.0): x:{:.1}, y:{:.1}, z:{:.1}, w:{:.1}",scaled_vec.x,scaled_vec.y,scaled_vec.z,scaled_vec.w);
}