fn say_hi() {
    println!("hello friend !!");
}

fn my_age(age: i32) {
    println!("hi, my age is {}", age);
}

fn add(nb1: i32, nb2: i32) -> i32 {
    nb1 + nb2
}

fn sub(nb1: i32, nb2: i32) -> i32 {
    nb1 - nb2
}

// return value ad expression with return
fn re_sub(nb1: i32, nb2: i32) -> i32 {
    return nb1 - nb2;
}
