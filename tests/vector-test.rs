use smath::vector::*;

#[test]
fn test_vec2() {
    // Constructors
    let mut v2=Vec2::zero();
    assert_eq!(v2.x,0.0);
    assert_eq!(v2.y,0.0);

    let v2_one=Vec2::one();
    assert_eq!(v2_one.x,1.0);
    assert_eq!(v2_one.y,1.0);

    // Set & Shift
    v2.set(5.0,10.0);assert_eq!((v2.x,v2.y),(5.0,10.0));
    v2.shift(1.0,-2.0);assert_eq!((v2.x,v2.y),(6.0,8.0));

    // Math Operators
    let a=Vec2::new(1.0,2.0);
    let b=Vec2::new(3.0,4.0);
    let res_add=a+b;assert_eq!((res_add.x,res_add.y),(4.0,6.0));
    let res_sub=a-b;assert_eq!((res_sub.x,res_sub.y),(-2.0,-2.0));
    let res_mul=a*2.0;assert_eq!((res_mul.x,res_mul.y),(2.0,4.0));
    let res_div=b/2.0;assert_eq!((res_div.x,res_div.y),(1.5,2.0));
    let res_neg=-a;assert_eq!((res_neg.x,res_neg.y),(-1.0,-2.0));

    // Assign Operators
    let mut m=Vec2::new(1.0,2.0);
    m+=Vec2::new(2.0,3.0);assert_eq!((m.x,m.y),(3.0,5.0));
    m-=Vec2::new(1.0,1.0);assert_eq!((m.x,m.y),(2.0,4.0));
    m*=2.0;assert_eq!((m.x,m.y),(4.0,8.0));
    m/=4.0;assert_eq!((m.x,m.y),(1.0,2.0));
}

#[test]
fn test_vec3() {
    // Constructors & Mutation
    let mut v3=Vec3::zero();
    let v3_one=Vec3::one();
    assert_eq!((v3_one.x,v3_one.y,v3_one.z),(1.0,1.0,1.0));

    v3.set(1.0,2.0,3.0);v3.shift(1.0,1.0,1.0);
    assert_eq!((v3.x,v3.y,v3.z),(2.0,3.0,4.0));

    // Math & Assign Operators
    let a=Vec3::new(1.0,2.0,3.0);
    let b=Vec3::new(4.0,5.0,6.0);
    assert_eq!((a+b).x,5.0);
    assert_eq!((a-b).x,-3.0);
    assert_eq!((a*3.0).y,6.0);
    assert_eq!((b/2.0).z,3.0);
    assert_eq!((-a).z,-3.0);

    let mut m=Vec3::new(1.0,1.0,1.0);m+=Vec3::new(1.0,2.0,3.0);
    assert_eq!((m.x,m.y,m.z),(2.0,3.0,4.0));
}

#[test]
fn test_vec4() {
    // Constructors & Mutation
    let mut v4=Vec4::zero();
    let v4_one=Vec4::one();
    assert_eq!((v4_one.x,v4_one.y,v4_one.z,v4_one.w),(1.0,1.0,1.0,1.0));

    v4.set(1.0,2.0,3.0,4.0);v4.shift(1.0,1.0,1.0,1.0);
    assert_eq!((v4.x,v4.y,v4.z,v4.w),(2.0,3.0,4.0,5.0));

    // Math & Assign Operators
    let a=Vec4::new(1.0,2.0,3.0,4.0);
    let b=Vec4::new(2.0,3.0,4.0,5.0);
    assert_eq!((a+b).w,9.0);
    assert_eq!((a-b).w,-1.0);
    assert_eq!((a*2.0).w,8.0);
    assert_eq!((b/2.0).w,2.5);
    assert_eq!((-a).w,-4.0);

    let mut m=Vec4::new(1.0,1.0,1.0,1.0);m+=Vec4::new(1.0,1.0,1.0,1.0);
    assert_eq!((m.x,m.y,m.z,m.w),(2.0,2.0,2.0,2.0));
}

#[test]
    fn test_vec2_operations(){
        let mut v1=Vec2::new(3.0,4.0);
        let v2=Vec2::new(1.0,2.0);

        // Test length and squared length
        assert_eq!(v1.sq_length().round(),25.0);
        assert_eq!(v1.length().round(),5.0);

        // Test dot product
        assert_eq!(v1.dot(v2),11.0); // 3*1 + 4*2

        // Test distance
        assert_eq!(v1.sq_distance(v2),8.0); // (3-1)^2 + (4-2)^2

        // Test normalization
        v1.normalize();
        assert!((v1.length().round()-1.0).abs()<1e-5); // Should be a unit vector
    
    }

    #[test]
    fn test_vec3_cross(){
        let v1=Vec3::new(1.0,0.0,0.0);
        let v2=Vec3::new(0.0,1.0,0.0);
        let cross=v1.cross(v2);

        // Cross product of X and Y axes should point along the Z axis
        assert_eq!(cross.x,0.0);
        assert_eq!(cross.y,0.0);
        assert_eq!(cross.z,1.0);
    }

#[test]
    fn test_vector_angles_verbose(){
        println!("\n");

        let cases=[
            (Vec2::new(1.0,1.7320508),"Quadrant 1 (Top-Right)"),
            (Vec2::new(-2.0,2.0),"Quadrant 2 (Top-Left)"),
            (Vec2::new(-1.0,-1.7320508),"Quadrant 3 (Bottom-Left)"),
            (Vec2::new(3.0,-3.0),"Quadrant 4 (Bottom-Right)"),
            (Vec2::new(0.0,-5.0),"Pure Axis (Straight Down)"),
        ];

        for (v,label) in cases{
            let actual_deg=v.angle_deg();
            let actual_rad_p=v.angle_rad();
            let raw_ratio=if v.x!=0.0{ v.y/v.x }else{ 0.0 };

            println!("\n[{}] Vector: ({}, {})",label,v.x,v.y);
            println!("  Input ratio (y/x): {}",raw_ratio);
            println!("  Your angle_deg() returned: {}",actual_deg);
            println!("  Your angle_rad_p() returned: {}",actual_rad_p);
        }
        println!("\n");
    }