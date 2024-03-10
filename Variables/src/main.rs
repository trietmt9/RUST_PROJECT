fn main() {
    let name = "Me";
    let age = 23; 
    println!("My name is {} and this is my age {}", name, age);

    //mutable variables 
    let mut var:i32 = 25000;
    println!("My variable is {}", var);
    var = 32555; 
    println!("My changed variable is {}", var);

    // test variables 
    let mut int:i8 = 0x01;
    let mut float:f32 = 0.01;
    let mut char:char = '\x00';
}
