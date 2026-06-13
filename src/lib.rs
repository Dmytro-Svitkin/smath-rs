fn main(){
    // Standard Exact Textbook Values (Internet Reference Data)
    let sin_30_real = 0.5000000;
    let cos_30_real = 0.8660254;
    let tan_30_real = 0.5773503;
    
    let sin_45_real = 0.7071068;
    let cos_45_real = 0.7071068;
    let tan_45_real = 1.0000000;
    let cot_45_real = 1.0000000;

    let sin_60_real = 0.8660254;
    let cos_60_real = 0.5000000;
    let tan_60_real = 1.7320508;

    println!("=== TESTING AT 30 DEGREES (Custom Radian: 30/180 = 0.16666667) ===");
    println!("sin(30):   Approx = {:<12} | Internet Real = {}", sin(30.0), sin_30_real);
    println!("sinr(30):  Approx = {:<12} | Internet Real = {}", sinr(30.0/180.0), sin_30_real);
    println!("cos(30):   Approx = {:<12} | Internet Real = {}", cos(30.0), cos_30_real);
    println!("cosr(30):  Approx = {:<12} | Internet Real = {}", cosr(30.0/180.0), cos_30_real);
    println!("tan(30):   Approx = {:<12} | Internet Real = {}", tan(30.0), tan_30_real);
    println!("tanr(30):  Approx = {:<12} | Internet Real = {}", tanr(30.0/180.0), tan_30_real);

    println!("\n=== TESTING AT 45 DEGREES (Custom Radian: 45/180 = 0.25) ===");
    println!("sin(45):   Approx = {:<12} | Internet Real = {}", sin(45.0), sin_45_real);
    println!("sinr(45):  Approx = {:<12} | Internet Real = {}", sinr(0.25), sin_45_real);
    println!("cos(45):   Approx = {:<12} | Internet Real = {}", cos(45.0), cos_45_real);
    println!("cosr(45):  Approx = {:<12} | Internet Real = {}", cosr(0.25), cos_45_real);
    println!("tan(45):   Approx = {:<12} | Internet Real = {}", tan(45.0), tan_45_real);
    println!("tanr(45):  Approx = {:<12} | Internet Real = {}", tanr(0.25), tan_45_real);
    println!("cotan(45): Approx = {:<12} | Internet Real = {}", cotan(45.0), cot_45_real);
    println!("cotanr(45):Approx = {:<12} | Internet Real = {}", cotanr(0.25), cot_45_real);

    println!("\n=== TESTING AT 60 DEGREES (Custom Radian: 60/180 = 0.33333334) ===");
    println!("sin(60):   Approx = {:<12} | Internet Real = {}", sin(60.0), sin_60_real);
    println!("sinr(60):  Approx = {:<12} | Internet Real = {}", sinr(60.0/180.0), sin_60_real);
    println!("cos(60):   Approx = {:<12} | Internet Real = {}", cos(60.0), cos_60_real);
    println!("cosr(60):  Approx = {:<12} | Internet Real = {}", cosr(60.0/180.0), cos_60_real);
    println!("tan(60):   Approx = {:<12} | Internet Real = {}", tan(60.0), tan_60_real);
    println!("tanr(60):  Approx = {:<12} | Internet Real = {}", tanr(60.0/180.0), tan_60_real);
}

fn sin(x:f32)->f32{
    let x:f32=x-(x*0.0027777778).floor()*360.0;
    if x<180.0{let rcl:f32=x*0.011111111-1.0;let rcl:f32=rcl*rcl;(1.0-rcl)*(1.0-0.2240081*rcl)}
    else{let rcl:f32=x*0.011111111-3.0;let rcl:f32=rcl*rcl;(rcl-1.0)*(1.0-0.2240081*rcl)}
}
/*
fn sin(x:f32)->f32{
    let x:f32=x-(x*0.0027777778).floor()*360.0;
    if x<180.0{let rcl:f32=x*0.011111111-1.0;1.0-rcl*rcl}
    else{let rcl:f32=x*0.011111111-3.0;rcl*rcl-1.0}
}
*/
fn sinr(x:f32)->f32{
    let x:f32=x-(x*0.5).floor()*2.0;
    if x<1.0{let rcl:f32=x*2.0-1.0;let rcl:f32=rcl*rcl;(1.0-rcl)*(1.0-0.2240081*rcl)}
    else{let rcl:f32=x*2.0-3.0;let rcl:f32=rcl*rcl;(rcl-1.0)*(1.0-0.2240081*rcl)}
}
/*
fn sinr(x:f32)->f32{
    let x:f32=x-(x*0.5).floor()*2.0;
    if x<1.0{let rcl:f32=x*2.0-1.0;1.0-rcl*rcl}
    else{let rcl:f32=x*2.0-3.0;rcl*rcl-1.0}
}
*/
fn cos(x:f32)->f32{
    sin(x+90.0)
}

fn cosr(x:f32)->f32{
    sinr(x+0.5)
}

fn tan(x:f32)->f32{
    let x:f32=x*0.0055555556;let x:f32=(x-x.round())*180.0;
    let rcl:f32=x*x;(x*(141.37167-0.0031465*rcl))/(8100.0-rcl)
}

fn tg(x:f32)->f32{tan(x)}

fn tanr(x:f32)->f32{
    let x:f32=x-x.round();
    let rcl:f32=x*x;(x*(0.78539816-0.5663706*rcl))/(0.25-rcl)
}

fn tgr(x:f32)->f32{
    tanr(x)
}

fn cotan(x:f32)->f32{
    -tan(x+90.0)
}

fn ctg(x:f32)->f32{
    cotan(x)
}

fn cotanr(x:f32)->f32{
    -tanr(x+0.5)
}

fn ctnr(x:f32)->f32{
    cotanr(x)
}