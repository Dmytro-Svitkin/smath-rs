use smath::matrix::*;

#[test]
fn matrix_setup(){
    println!("▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁");
    println!("█ {:?}",Mat::<2,3>::zero());
    println!("█ {:?}",Mat::<3,2>::zero());
    println!("█ {:?}",Mat::<1,1>::zero());
    println!("█ {:?}",Mat::<0,0>::zero());
    println!("█");
    println!("█ {:?}",Mat3::zero());
    println!("█ {:?}",Mat4::zero());

    println!("▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁");
    let mut a: Mat<3,3>=Mat3::new([[1.0,2.0,3.0],[4.0,5.0,6.0],[7.0,8.0,9.0]]);
    println!("█ {:?}",a);
    println!("█ {}",a.get(1,2));
    println!("█");
    a.set(1,2,100.0);
    println!("█ {:?}",a);
    println!("█ {}",a.get(1,2));



}