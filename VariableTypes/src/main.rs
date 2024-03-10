fn main() {
    let myString = "String"; // String data type 
    let myInt = 10; // Integer data type
    let myFloat = 3.14; // Float data type

    let myBoolean = false; // Boolean data type
    let iconChars = '🤡';

    println!("This is a string data type: {}", myString);
    println!("This is an integer data type: {}", myInt);
    println!("This is a float data type: {}", myFloat);
    println!("This is a boolean data type: {}", myBoolean);
    println!("This is a character data type: {}", iconChars);


    // this is list of some available types
    let mut int:i8 = 0x01;
    let mut int16:i16 = 0x0001;
    let mut int32:i32 = 0x00000001;
    let mut int64:i64 = 0x00000000FFFFFFFF;
    let mut uint:u8 = 0xFF;
    let mut uint16:u16 = 0xFFFF;
    let mut uint32:u32 = 0xFFFFFFFF;
    let mut uint64:u64 = 0xFFFFFFFFFFFFFFFF;
    let mut float:f32 = 0.01;
    let mut char:char = '\x00';
}
