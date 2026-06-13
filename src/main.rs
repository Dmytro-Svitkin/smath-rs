fn main(){
    println!("{}",cosr(9998.7925))
}

fn sin(x:f32)->f32{
    let x:f32=x-(x*0.0027777778).floor()*360.0;
    if x<180.0{let rcl:f32=x*0.011111111-1.0;1.0-rcl*rcl}
    else{let rcl:f32=x*0.011111111-3.0;rcl*rcl-1.0}
}

fn sinr(x:f32)->f32{
    let x:f32=x-(x*0.5).floor()*2.0;
    if x<1.0{let rcl:f32=x*2.0-1.0;1.0-rcl*rcl}
    else{let rcl:f32=x*2.0-3.0;rcl*rcl-1.0}
}

fn cos(x:f32)->f32{
    sin(x+90.0)
}

fn cosr(x:f32)->f32{
    sinr(x+0.5)
}