pub fn main() {
    let _name: &str = "issam";
    let _age: i32 = 15;

    /* shadowing varibale on Rust*/
    //let age = age + 15;

    /* Const in rust: should assigne a type i32..*/
    const AGE: i32 = 18;
    println!("Hello, world! {}, my age is {}", _name, AGE);

    /*DATA TYPE IN RUST*/
    //integers type: i8, i15, i32, i64, i128 sould positive or negetive.
    //integers type: just postive: u8, u16, u32, u64, u128
    let p_number: i32 = 15;
    let _number = 15_i32;
    let length = -100_000_000;
    let _byte = 0xff;
    let _octal = 0o77;
    let _binary = 0b1111_0000;
    let char_value = b'A';
    println!("the number is {}, {}, {}", char_value, length, p_number);

    //float types: f32, f64
    let _fl1 = 3.9;
    let fl2 = 3.9f64;
    let mut _sum = 5 + 2;
    let _sub = 5 - 2;
    let _mul = 5 * 2;
    let _div = 4 / 2;
    let _rem = 5 % 2;

    _sum += 12;
    println!("the number is {}", fl2);

    //bool : true // false
    let _is_old = false;
    let _isnot_old = true;
    let _combination = true & false; // true | false
    println!("the number is {}", _combination);

    //strings types:
    let crab_emoji = r#"crab"#;
    println!("the number is {}", crab_emoji);

    //tuple and arrays
    let data = (32, 'A', true);
    //get index data: data.0
    println!("data: {:?}", data);
    println!("data: {}", data.0);

    //arrays: with defined types
    let _array: [i32; 3] = [12, 5, 15];
    let long_array = [10; 30];
    // let index: usize = env!(r#"USERINDEX"#)
    //     .parse()
    //     .expect("Expect number of user index");
    println!("array: {:?}", long_array)
}
