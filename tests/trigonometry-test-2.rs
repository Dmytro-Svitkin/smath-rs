use smath::trigonometry::*;

#[test]
fn print_trig_values(){
    let angles:[isize;14]=[0,15,30,45,60,75,90,105,120,135,150,165,180,-12342];

    for a in angles{
        println!("{}°",a);
        println!("|");

        println!("sin({}.0)   = {}",a as f32,sin(a as f32));
        println!("sin({})     = {}",a,sin(a));
        println!("|");
        assert_eq!(((sin(a as f32)*10.0).round())*0.1,((sin(a)*10.0).round())*0.1);

        println!("cos({}.0)   = {}",a as f32,cos(a as f32));
        println!("cos({})     = {}",a,cos(a));
        println!("|");
        assert_eq!(((cos(a as f32)*10.0).round())*0.1,((cos(a)*10.0).round())*0.1);

        println!("tan({}.0)   = {}",a as f32,tan(a as f32));
        println!("tan({})     = {}",a,tan(a));
        println!("|");
        assert_eq!(((tan(a as f32)*10.0).round())*0.1,((tg(a)*10.0).round())*0.1);

        println!("cotan({}.0) = {}",a as f32,cotan(a as f32));
        println!("cotan({})   = {}",a,cotan(a));
        println!("");
        assert_eq!(((ctg(a as f32)*10.0).round())*0.1,((cotan(a)*10.0).round())*0.1);
    }
}