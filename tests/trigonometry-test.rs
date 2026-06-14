use smath::trigonometry::*;

#[test]
fn test_degrees(){
    assert!((sin(30.0)-0.5000000).abs()<0.001);
    assert!((cos(60.0)-0.5000000).abs()<0.001);
    assert!((tan(45.0)-1.0000000).abs()<0.001);
    assert!((cotan(45.0)-1.0000000).abs()<0.001);
}

#[test]
fn test_radians(){
    assert!((sinr(30.0/180.0)-0.5000000).abs()<0.001);
    assert!((cosr(60.0/180.0)-0.5000000).abs()<0.001);
    assert!((tanr(0.25)-1.0000000).abs()<0.001);
    assert!((cotanr(0.25)-1.0000000).abs()<0.001);
}

#[test]
fn test_aliases(){
    assert_eq!(tan(30.0),tg(30.0));
    assert_eq!(tanr(0.25),tgr(0.25));
    assert_eq!(cotan(30.0),ctg(30.0));
    assert_eq!(cotanr(0.25),ctgr(0.25));
}